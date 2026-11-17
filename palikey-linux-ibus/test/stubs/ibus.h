#ifndef IBUS_STUB_H
#define IBUS_STUB_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#define G_BEGIN_DECLS
#define G_END_DECLS

typedef int gboolean;
#define TRUE 1
#define FALSE 0

typedef uint32_t guint;
typedef int32_t gint;
typedef uint32_t gunichar;
typedef void* gpointer;
typedef size_t gsize;

#define IBUS_RELEASE_MASK  (1 << 30)
#define IBUS_SHIFT_MASK    (1 << 0)
#define IBUS_CONTROL_MASK  (1 << 2)
#define IBUS_MOD1_MASK     (1 << 3)
#define IBUS_SUPER_MASK    (1 << 26)
#define IBUS_MOD5_MASK     (1 << 7)

#define IBUS_KEY_BackSpace 0xff08
#define IBUS_KEY_Tab       0xff09
#define IBUS_KEY_Return    0xff0d
#define IBUS_KEY_Escape    0xff1b
#define IBUS_KEY_KP_Enter  0xff8d
#define IBUS_KEY_Home      0xff50
#define IBUS_KEY_Left      0xff51
#define IBUS_KEY_Up        0xff52
#define IBUS_KEY_Right     0xff53
#define IBUS_KEY_Down      0xff54
#define IBUS_KEY_Page_Up   0xff55
#define IBUS_KEY_Page_Down 0xff56
#define IBUS_KEY_End       0xff57

typedef struct _GTypeInstance GTypeInstance;
typedef struct _GObject GObject;
typedef struct _GObjectClass GObjectClass;

struct _GObjectClass {
    void (*finalize)(GObject *);
};

#define G_OBJECT_CLASS(klass) ((GObjectClass *)(klass))

typedef struct _IBusEngine IBusEngine;
typedef struct _IBusEngineClass IBusEngineClass;

struct _IBusEngine {
    int dummy;
};

struct _IBusEngineClass {
    GObjectClass parent_class;
    gboolean (*process_key_event)(IBusEngine *, guint, guint, guint);
    void (*reset)(IBusEngine *);
    void (*enable)(IBusEngine *);
    void (*disable)(IBusEngine *);
    void (*focus_in)(IBusEngine *);
    void (*focus_out)(IBusEngine *);
};

#define IBUS_ENGINE(obj) ((IBusEngine *)(obj))
#define IBUS_ENGINE_CLASS(klass) ((IBusEngineClass *)(klass))
#define IBUS_TYPE_ENGINE 1

typedef struct _IBusText IBusText;
typedef struct _IBusBus IBusBus;
typedef struct _IBusFactory IBusFactory;
typedef struct _IBusComponent IBusComponent;
typedef struct _IBusEngineDesc IBusEngineDesc;
typedef void (*GCallback)(void);

#define G_CALLBACK(f) ((GCallback)(f))
#define g_signal_connect(instance, detailed_signal, c_handler, data) \
    do { (void)(instance); (void)(detailed_signal); (void)(c_handler); (void)(data); } while(0)
#define g_object_unref(obj) ((void)(obj))
#define g_warning(...) ((void)0)

#define G_DECLARE_FINAL_TYPE(ModuleObjName, module_obj_name, MODULE, OBJ_NAME, ParentName) \
    typedef struct _##ModuleObjName ModuleObjName; \
    typedef struct { ParentName##Class parent_class; } ModuleObjName##Class; \
    uint64_t module_obj_name##_get_type(void); \
    static inline ModuleObjName *MODULE##_##OBJ_NAME(void *obj) { return (ModuleObjName *)obj; }

#define G_DEFINE_TYPE(TN, t_n, T_P) \
    static void t_n##_init(TN *self); \
    static void t_n##_class_init(TN##Class *klass); \
    static void *t_n##_parent_class = (void *)0; \
    uint64_t t_n##_get_type(void) { \
        (void)t_n##_init; \
        (void)t_n##_class_init; \
        (void)t_n##_parent_class; \
        return 42; \
    }

static inline void ibus_init(void) {}
static inline void ibus_main(void) {}
static inline void ibus_quit(void) {}

static inline gunichar ibus_keyval_to_unicode(guint keyval) {
    if (keyval >= 0x20 && keyval <= 0x7e) return (gunichar)keyval;
    return 0;
}

static inline IBusText *ibus_text_new_from_string(const char *str) { (void)str; return (IBusText *)1; }
static inline void ibus_engine_commit_text(IBusEngine *e, IBusText *t) { (void)e; (void)t; }
static inline void ibus_engine_update_preedit_text(IBusEngine *e, IBusText *t, guint c, gboolean v) { (void)e; (void)t; (void)c; (void)v; }
static inline void ibus_engine_hide_preedit_text(IBusEngine *e) { (void)e; }
static inline void ibus_engine_delete_surrounding_text(IBusEngine *e, gint o, guint n) { (void)e; (void)o; (void)n; }

static inline IBusBus *ibus_bus_new(void) { return (IBusBus *)1; }
static inline gboolean ibus_bus_is_connected(IBusBus *b) { (void)b; return TRUE; }
static inline void *ibus_bus_get_connection(IBusBus *b) { (void)b; return (void *)1; }
static inline guint ibus_bus_request_name(IBusBus *b, const char *n, guint f) { (void)b; (void)n; (void)f; return 1; }
static inline gboolean ibus_bus_register_component(IBusBus *b, IBusComponent *c) { (void)b; (void)c; return TRUE; }

static inline IBusFactory *ibus_factory_new(void *c) { (void)c; return (IBusFactory *)1; }
static inline void ibus_factory_add_engine(IBusFactory *f, const char *n, uint64_t t) { (void)f; (void)n; (void)t; }

static inline IBusComponent *ibus_component_new(const char *n, const char *d, const char *v, const char *l, const char *a, const char *h, const char *c, const char *td) {
    (void)n; (void)d; (void)v; (void)l; (void)a; (void)h; (void)c; (void)td; return (IBusComponent *)1;
}
static inline void ibus_component_add_engine(IBusComponent *c, IBusEngineDesc *e) { (void)c; (void)e; }
static inline IBusEngineDesc *ibus_engine_desc_new(const char *n, const char *ln, const char *d, const char *lang, const char *l, const char *a, const char *i, const char *layout) {
    (void)n; (void)ln; (void)d; (void)lang; (void)l; (void)a; (void)i; (void)layout; return (IBusEngineDesc *)1;
}

#ifdef __cplusplus
}
#endif

#endif /* IBUS_STUB_H */
