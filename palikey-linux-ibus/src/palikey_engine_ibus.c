/**
 * @file palikey_engine_ibus.c
 * @brief IBusEngine implementation for PaliKey Pāḷi input method.
 */

#include "palikey_engine_ibus.h"
#include <string.h>

struct _PalikeyEngineIBus {
    IBusEngine parent;
    PalikeyEngine *engine;
    PalikeyConfig config;
};

G_DEFINE_TYPE(PalikeyEngineIBus, palikey_engine_ibus, IBUS_TYPE_ENGINE)

static void palikey_apply_result(PalikeyEngineIBus *self, const PalikeyProcessResult *result) {
    IBusEngine *engine = IBUS_ENGINE(self);

    if (result->backspaces > 0) {
        ibus_engine_delete_surrounding_text(engine, -((gint)result->backspaces), (guint)result->backspaces);
    }

    if (result->commit[0] != '\0') {
        IBusText *commit_text = ibus_text_new_from_string(result->commit);
        ibus_engine_commit_text(engine, commit_text);
        g_object_unref(commit_text);
    }

    if (result->preedit[0] != '\0') {
        IBusText *preedit_text = ibus_text_new_from_string(result->preedit);
        ibus_engine_update_preedit_text(engine, preedit_text, (guint)strlen(result->preedit), TRUE);
        g_object_unref(preedit_text);
    } else {
        ibus_engine_hide_preedit_text(engine);
    }
}

static void palikey_engine_ibus_reload_config(PalikeyEngineIBus *self) {
    PalikeyConfig new_cfg;
    if (palikey_config_load(&new_cfg)) {
        if (new_cfg.niggahita != self->config.niggahita || new_cfg.extended_set != self->config.extended_set) {
            if (self->engine) {
                palikey_free(self->engine);
            }
            self->config = new_cfg;
            self->engine = palikey_new(self->config.scheme, self->config.niggahita, self->config.extended_set);
        } else if (new_cfg.scheme != self->config.scheme) {
            self->config = new_cfg;
            if (self->engine) {
                palikey_set_scheme(self->engine, self->config.scheme);
            }
        } else {
            self->config = new_cfg;
        }
    }
}

static gboolean palikey_engine_ibus_process_key_event(
    IBusEngine *engine,
    guint keyval,
    guint keycode,
    guint state
) {
    (void)keycode;
    PalikeyEngineIBus *self = PALIKEY_ENGINE_IBUS(engine);

    // 1. Ignore key release events
    if (state & IBUS_RELEASE_MASK) {
        return FALSE;
    }

    // 2. Alt / AltGr direct-insert layer (EasyUnicode)
    // IBUS_MOD1_MASK is Alt, IBUS_MOD5_MASK is AltGr / ISO_Level3_Shift
    if ((state & IBUS_MOD1_MASK) || (state & IBUS_MOD5_MASK)) {
        if (!(state & IBUS_CONTROL_MASK)) {
            bool is_shift = (state & IBUS_SHIFT_MASK) != 0;
            const char *mapped = palikey_easyunicode_map(keyval, is_shift, self->config.niggahita);
            if (mapped) {
                if (self->engine) palikey_reset_instance(self->engine);
                IBusText *text = ibus_text_new_from_string(mapped);
                ibus_engine_commit_text(engine, text);
                g_object_unref(text);
                return TRUE;
            }
        }
        // Unmapped combination or Control held: flush and pass through
        if (self->engine) palikey_reset_instance(self->engine);
        return FALSE;
    }

    // 3. Control / Super shortcuts bypass IME and flush composition buffer
    if ((state & IBUS_CONTROL_MASK) || (state & IBUS_SUPER_MASK)) {
        if (self->engine) palikey_reset_instance(self->engine);
        return FALSE;
    }

    // 4. Navigation and termination keys flush composition buffer
    if (keyval == IBUS_KEY_Return || keyval == IBUS_KEY_KP_Enter ||
        keyval == IBUS_KEY_Escape || keyval == IBUS_KEY_Tab ||
        keyval == IBUS_KEY_Left   || keyval == IBUS_KEY_Right ||
        keyval == IBUS_KEY_Up     || keyval == IBUS_KEY_Down ||
        keyval == IBUS_KEY_Home   || keyval == IBUS_KEY_End ||
        keyval == IBUS_KEY_Page_Up|| keyval == IBUS_KEY_Page_Down) {
        if (self->engine) palikey_reset_instance(self->engine);
        return FALSE;
    }

    // 5. Backspace key handling
    if (keyval == IBUS_KEY_BackSpace) {
        if (!self->engine) return FALSE;
        PalikeyProcessResult result;
        if (palikey_process_instance(self->engine, 0, true, &result)) {
            if (result.backspaces > 0 || result.commit[0] != '\0') {
                palikey_apply_result(self, &result);
                return TRUE;
            }
        }
        // No undo sequence available: let default backspace pass through
        return FALSE;
    }

    // 6. Printable character keys
    gunichar ch = ibus_keyval_to_unicode(keyval);
    if (ch >= 0x20) {
        if (!self->engine) return FALSE;
        PalikeyProcessResult result;
        if (palikey_process_instance(self->engine, (uint32_t)ch, false, &result)) {
            palikey_apply_result(self, &result);
            return TRUE;
        }
    }

    // Non-printable or unhandled
    if (self->engine) palikey_reset_instance(self->engine);
    return FALSE;
}

static void palikey_engine_ibus_reset(IBusEngine *engine) {
    PalikeyEngineIBus *self = PALIKEY_ENGINE_IBUS(engine);
    if (self->engine) {
        palikey_reset_instance(self->engine);
    }
    ibus_engine_hide_preedit_text(engine);
}

static void palikey_engine_ibus_enable(IBusEngine *engine) {
    PalikeyEngineIBus *self = PALIKEY_ENGINE_IBUS(engine);
    palikey_engine_ibus_reload_config(self);
    palikey_engine_ibus_reset(engine);
}

static void palikey_engine_ibus_disable(IBusEngine *engine) {
    palikey_engine_ibus_reset(engine);
}

static void palikey_engine_ibus_focus_in(IBusEngine *engine) {
    PalikeyEngineIBus *self = PALIKEY_ENGINE_IBUS(engine);
    palikey_engine_ibus_reload_config(self);
    palikey_engine_ibus_reset(engine);
}

static void palikey_engine_ibus_focus_out(IBusEngine *engine) {
    palikey_engine_ibus_reset(engine);
}

static void palikey_engine_ibus_init(PalikeyEngineIBus *self) {
    palikey_config_load(&self->config);
    self->engine = palikey_new(self->config.scheme, self->config.niggahita, self->config.extended_set);
}

static void palikey_engine_ibus_finalize(GObject *object) {
    PalikeyEngineIBus *self = PALIKEY_ENGINE_IBUS(object);
    if (self->engine) {
        palikey_free(self->engine);
        self->engine = NULL;
    }
    G_OBJECT_CLASS(palikey_engine_ibus_parent_class)->finalize(object);
}

static void palikey_engine_ibus_class_init(PalikeyEngineIBusClass *klass) {
    GObjectClass *object_class = G_OBJECT_CLASS(klass);
    IBusEngineClass *engine_class = IBUS_ENGINE_CLASS(klass);

    object_class->finalize = palikey_engine_ibus_finalize;

    engine_class->process_key_event = palikey_engine_ibus_process_key_event;
    engine_class->reset = palikey_engine_ibus_reset;
    engine_class->enable = palikey_engine_ibus_enable;
    engine_class->disable = palikey_engine_ibus_disable;
    engine_class->focus_in = palikey_engine_ibus_focus_in;
    engine_class->focus_out = palikey_engine_ibus_focus_out;
}
