//
//  PaliKeyApp.swift
//  PaliKey
//
//  iOS Container App for PaliKey.
//

import SwiftUI

@main
struct PaliKeyApp: App {
    @StateObject private var preferences = PaliPreferences.shared

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(preferences)
        }
    }
}
