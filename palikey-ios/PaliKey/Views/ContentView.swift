//
//  ContentView.swift
//  PaliKey
//
//  Main settings, setup instructions, and interactive test area for PaliKey iOS.
//

import SwiftUI

struct ContentView: View {
    @EnvironmentObject var preferences: PaliPreferences
    @State private var testInputText: String = ""
    @State private var showingInfoAlert: Bool = false

    var body: some View {
        NavigationView {
            Form {
                // Section 1: Activation & Quick Setup
                Section(header: Label("Setup & Activation", systemImage: "gearshape.fill")) {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("To use PaliKey across iOS apps:")
                            .font(.subheadline)
                            .foregroundColor(.secondary)

                        SetupStepRow(number: "1", text: "Open iOS Settings")
                        SetupStepRow(number: "2", text: "Go to General → Keyboard → Keyboards")
                        SetupStepRow(number: "3", text: "Tap 'Add New Keyboard...'")
                        SetupStepRow(number: "4", text: "Select 'PaliKey' under Third-Party Keyboards")
                    }
                    .padding(.vertical, 4)

                    Button(action: openSettings) {
                        HStack {
                            Image(systemName: "arrow.up.forward.app")
                            Text("Open iOS Settings")
                            Spacer()
                        }
                    }
                }

                // Section 2: Input Method Configuration
                Section(header: Label("Transliteration Scheme", systemImage: "character.book.closed.fill")) {
                    Picker("Active Scheme", selection: $preferences.activeScheme) {
                        ForEach(Scheme.allCases) { scheme in
                            Text(scheme.displayName).tag(scheme)
                        }
                    }
                    .pickerStyle(.segmented)
                    .padding(.vertical, 2)

                    Text(preferences.activeScheme.description)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                // Section 3: Orthographic Options
                Section(header: Label("Orthography & Mode", systemImage: "slider.horizontal.3")) {
                    Picker("Niggahīta Variant", selection: $preferences.activeNiggahita) {
                        ForEach(NiggahitaVariant.allCases) { variant in
                            Text(variant.displayName).tag(variant)
                        }
                    }

                    Picker("Default Keyboard Mode", selection: $preferences.keyboardMode) {
                        ForEach(KeyboardMode.allCases) { mode in
                            Text(mode.displayName).tag(mode)
                        }
                    }

                    Toggle("Extended Sanskrit / IAST Set", isOn: $preferences.extendedSet)
                    if preferences.extendedSet {
                        Text("Enables Sanskrit vowels (ṛ, ṝ, ḷ, ḹ) and sibilants (ś, ṣ).")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }

                // Section 4: Live Test Field
                Section(header: Label("Live Test Area", systemImage: "keyboard")) {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Tap below to test the PaliKey keyboard extension:")
                            .font(.caption)
                            .foregroundColor(.secondary)

                        TextField("Type Pāḷi words here (e.g. sammadi.t.thi)...", text: $testInputText)
                            .textFieldStyle(.roundedBorder)
                            .autocapitalization(.none)
                            .disableAutocorrection(true)

                        if !testInputText.isEmpty {
                            HStack {
                                Text("Length: \(testInputText.count) characters")
                                    .font(.caption2)
                                    .foregroundColor(.secondary)
                                Spacer()
                                Button("Clear") {
                                    testInputText = ""
                                }
                                .font(.caption2)
                            }
                        }
                    }
                    .padding(.vertical, 4)
                }

                // Section 5: Privacy & Security (NO Full Access)
                Section(header: Label("Privacy & Security", systemImage: "lock.shield.fill")) {
                    VStack(alignment: .leading, spacing: 6) {
                        HStack {
                            Image(systemName: "checkmark.shield.fill")
                                .foregroundColor(.green)
                            Text("No Full Access Required")
                                .font(.subheadline)
                                .fontWeight(.semibold)
                        }

                        Text("PaliKey operates under iOS's strict sandboxed keyboard mode. It has no network access (`INTERNET` permission), cannot access the system clipboard, and does not record or transmit typed text anywhere. All transliteration is performed 100% locally on-device by the embedded Rust engine.")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    .padding(.vertical, 4)
                }

                // Section 6: About & Engine Version
                Section(header: Label("About", systemImage: "info.circle")) {
                    HStack {
                        Text("App Version")
                        Spacer()
                        Text("0.1.0")
                            .foregroundColor(.secondary)
                    }
                    HStack {
                        Text("Rust Engine Core")
                        Spacer()
                        Text("v\(PaliEngine.version)")
                            .foregroundColor(.secondary)
                    }
                    HStack {
                        Text("License")
                        Spacer()
                        Text("MIT (Clean-room)")
                            .foregroundColor(.secondary)
                    }
                    HStack {
                        Text("Repository")
                        Spacer()
                        Text("babyskill/palikey")
                            .foregroundColor(.secondary)
                    }
                }
            }
            .navigationTitle("PaliKey")
        }
        .navigationViewStyle(.stack)
    }

    private func openSettings() {
        guard let url = URL(string: UIApplication.openSettingsURLString) else { return }
        if UIApplication.shared.canOpenURL(url) {
            UIApplication.shared.open(url)
        }
    }
}

private struct SetupStepRow: View {
    let number: String
    let text: String

    var body: some View {
        HStack(alignment: .top, spacing: 8) {
            Text(number)
                .font(.caption)
                .fontWeight(.bold)
                .foregroundColor(.white)
                .frame(width: 18, height: 18)
                .background(Circle().fill(Color.accentColor))
            Text(text)
                .font(.footnote)
                .foregroundColor(.primary)
        }
    }
}
