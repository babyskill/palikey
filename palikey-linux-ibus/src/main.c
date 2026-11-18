/**
 * @file main.c
 * @brief Standalone executable daemon for ibus-engine-palikey.
 */

#include <ibus.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>
#include "palikey.h"
#include "palikey_engine_ibus.h"

static IBusBus *bus = NULL;
static IBusFactory *factory = NULL;

static void sigterm_cb(int sig) {
    (void)sig;
    ibus_quit();
}

static void bus_disconnected_cb(IBusBus *b, gpointer user_data) {
    (void)b;
    (void)user_data;
    ibus_quit();
}

int main(int argc, char **argv) {
    bool ibus_mode = false;

    for (int i = 1; i < argc; ++i) {
        if (strcmp(argv[i], "--ibus") == 0 || strcmp(argv[i], "-i") == 0) {
            ibus_mode = true;
        } else if (strcmp(argv[i], "--version") == 0 || strcmp(argv[i], "-v") == 0) {
            printf("ibus-engine-palikey 0.1.0 (engine %s)\n", palikey_version());
            return 0;
        } else if (strcmp(argv[i], "--help") == 0 || strcmp(argv[i], "-h") == 0) {
            printf("Usage: ibus-engine-palikey [OPTION...]\n\n"
                   "Options:\n"
                   "  --ibus, -i       Executed by IBus daemon\n"
                   "  --version, -v    Display version information\n"
                   "  --help, -h       Display this help message\n");
            return 0;
        }
    }

    ibus_init();

    signal(SIGINT, sigterm_cb);
    signal(SIGTERM, sigterm_cb);

    bus = ibus_bus_new();
    if (!ibus_bus_is_connected(bus)) {
        g_warning("Could not connect to IBus bus daemon.");
        return 1;
    }

    g_signal_connect(bus, "disconnected", G_CALLBACK(bus_disconnected_cb), NULL);

    factory = ibus_factory_new(ibus_bus_get_connection(bus));
    ibus_factory_add_engine(factory, "palikey", PALIKEY_TYPE_ENGINE_IBUS);

    if (ibus_mode) {
        ibus_bus_request_name(bus, "org.freedesktop.IBus.PaliKey", 0);
    } else {
        IBusComponent *component = ibus_component_new(
            "org.freedesktop.IBus.PaliKey",
            "Pāḷi Keyboard Input Method",
            "0.1.0",
            "MIT",
            "Giac Ngo",
            "https://giacngo.org/tools",
            "",
            "ibus-palikey"
        );
        IBusEngineDesc *desc = ibus_engine_desc_new(
            "palikey",
            "Pāḷi Keyboard",
            "Pāḷi input method engine (Velthuis, PaliTelex, SIL)",
            "pi",
            "MIT",
            "Giac Ngo",
            "palikey",
            "us"
        );
        ibus_component_add_engine(component, desc);
        ibus_bus_register_component(bus, component);
        g_object_unref(component);
    }

    ibus_main();

    g_object_unref(factory);
    g_object_unref(bus);

    return 0;
}
