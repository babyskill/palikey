//
//  main.swift
//  PaliKey
//
//  IMKServer bootstrap and Application Delegate for macOS Input Method.
//

import Cocoa
import InputMethodKit

let kConnectionName = "PaliKey_1_Connection"

@objc(AppDelegate)
public class AppDelegate: NSObject, NSApplicationDelegate {
    public var server: IMKServer?

    public func applicationDidFinishLaunching(_ notification: Notification) {
        let bundleId = Bundle.main.bundleIdentifier ?? "org.giacngo.inputmethod.PaliKey"
        server = IMKServer(name: kConnectionName, bundleIdentifier: bundleId)
        NSLog("PaliKey IMKServer initialized: \(kConnectionName) (\(bundleId))")
    }

    // MARK: - Menu Actions

    @objc public func selectScheme(_ sender: NSMenuItem) {
        let schemeId = UInt32(sender.tag)
        PaliConfig.shared.scheme = schemeId
        PaliConfig.shared.save()
    }

    @objc public func toggleNiggahita(_ sender: NSMenuItem) {
        let variant = UInt32(sender.tag)
        PaliConfig.shared.niggahita = variant
        PaliConfig.shared.save()
    }

    @objc public func toggleExtendedSet(_ sender: NSMenuItem) {
        PaliConfig.shared.extendedSet.toggle()
        PaliConfig.shared.save()
    }

    @objc public func showAbout(_ sender: NSMenuItem) {
        let alert = NSAlert()
        alert.messageText = "Pāḷi Keyboard v\(PaliConfig.shared.version)"
        alert.informativeText = """
        Cross-platform Pāḷi Input Method
        Clean-room MIT Implementation

        Active Schemes:
        • Velthuis: aa->ā, .t->ṭ, "n->ṅ, ~n->ñ, .m->ṃ
        • PaliTelex: aa->ā, tx->ṭ, ngw->ṅ, nw/ny->ñ, mx->ṃ
        • SIL / Keyman: -a->ā, .t->ṭ, >n->ṅ, ~n->ñ, .m->ṃ
        • EasyUnicode: ⌥a->ā, ⌥t->ṭ, ⌥m->ṃ, ⌥g->ṅ, ⌥j->ñ

        Website: https://giacngo.org/tools
        """
        alert.alertStyle = .informational
        alert.addButton(withTitle: "OK")
        alert.runModal()
    }

    // MARK: - Menu Builder

    public static func buildMenu() -> NSMenu {
        let menu = NSMenu(title: "Pāḷi Keyboard")
        let config = PaliConfig.shared

        // Header: Schemes
        let schemesHeader = NSMenuItem(title: "Input Scheme", action: nil, keyEquivalent: "")
        schemesHeader.isEnabled = false
        menu.addItem(schemesHeader)

        let schemes = [
            (0, "Velthuis (aa, .t, \"n, ~n, .m)"),
            (1, "PaliTelex (aa, tx, ngw, nw, mx)"),
            (2, "SIL / Keyman (-a, .t, >n, ~n, .m)")
        ]

        for (id, name) in schemes {
            let item = NSMenuItem(title: name, action: #selector(selectScheme(_:)), keyEquivalent: "")
            item.target = NSApp.delegate
            item.tag = id
            item.state = (config.scheme == UInt32(id)) ? .on : .off
            menu.addItem(item)
        }

        menu.addItem(NSMenuItem.separator())

        // Header: Niggahīta
        let niggahitaHeader = NSMenuItem(title: "Niggahīta Character", action: nil, keyEquivalent: "")
        niggahitaHeader.isEnabled = false
        menu.addItem(niggahitaHeader)

        let nigDotBelow = NSMenuItem(title: "ṃ (Dot Below — PTS / SuttaCentral)", action: #selector(toggleNiggahita(_:)), keyEquivalent: "")
        nigDotBelow.target = NSApp.delegate
        nigDotBelow.tag = 0
        nigDotBelow.state = (config.niggahita == 0) ? .on : .off
        menu.addItem(nigDotBelow)

        let nigDotAbove = NSMenuItem(title: "ṁ (Dot Above — VRI tradition)", action: #selector(toggleNiggahita(_:)), keyEquivalent: "")
        nigDotAbove.target = NSApp.delegate
        nigDotAbove.tag = 1
        nigDotAbove.state = (config.niggahita == 1) ? .on : .off
        menu.addItem(nigDotAbove)

        menu.addItem(NSMenuItem.separator())

        // Extended Sanskrit set
        let extItem = NSMenuItem(title: "Extended Sanskrit Set (ṛ, ṝ, ḷ, ḥ, ś, ṣ, ē, ō)", action: #selector(toggleExtendedSet(_:)), keyEquivalent: "")
        extItem.target = NSApp.delegate
        extItem.state = config.extendedSet ? .on : .off
        menu.addItem(extItem)

        menu.addItem(NSMenuItem.separator())

        // About
        let aboutItem = NSMenuItem(title: "About Pāḷi Keyboard...", action: #selector(showAbout(_:)), keyEquivalent: "")
        aboutItem.target = NSApp.delegate
        menu.addItem(aboutItem)

        return menu
    }
}

// Bootstrap Cocoa Application
let app = NSApplication.shared
let delegate = AppDelegate()
app.delegate = delegate
app.run()
