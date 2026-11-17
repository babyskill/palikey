/**
 * @file palikey_engine_ibus.h
 * @brief IBusEngine GObject subclass header for PaliKey.
 */

#ifndef PALIKEY_ENGINE_IBUS_H
#define PALIKEY_ENGINE_IBUS_H

#include <ibus.h>
#include "palikey.h"
#include "palikey_config.h"
#include "palikey_easyunicode.h"

G_BEGIN_DECLS

#define PALIKEY_TYPE_ENGINE_IBUS (palikey_engine_ibus_get_type())

G_DECLARE_FINAL_TYPE(PalikeyEngineIBus, palikey_engine_ibus, PALIKEY, ENGINE_IBUS, IBusEngine)

G_END_DECLS

#endif /* PALIKEY_ENGINE_IBUS_H */
