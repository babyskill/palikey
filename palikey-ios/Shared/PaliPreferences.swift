//
//  PaliPreferences.swift
//  PaliKey
//
//  Shared configuration storage between the PaliKey container app and
//  the keyboard extension via App Groups UserDefaults.
//

import Foundation
import Combine

/// Manages persistent user preferences with App Group sharing and safe local fallback.
public final class PaliPreferences: ObservableObject {
    public static let shared = PaliPreferences()

    public static let appGroupIdentifier = "group.org.giacngo.palikey"

    private enum Keys {
        static let schemeId = "scheme_id"
        static let niggahitaVariant = "niggahita_variant"
        static let extendedSet = "extended_set"
        static let keyboardMode = "keyboard_mode"
    }

    private let userDefaults: UserDefaults

    public init(suiteName: String? = appGroupIdentifier) {
        if let suiteName = suiteName, let groupDefaults = UserDefaults(suiteName: suiteName) {
            self.userDefaults = groupDefaults
        } else {
            self.userDefaults = .standard
        }

        self.schemeId = UInt32(userDefaults.object(forKey: Keys.schemeId) as? Int ?? 0)
        self.niggahitaVariant = UInt32(userDefaults.object(forKey: Keys.niggahitaVariant) as? Int ?? 0)
        self.extendedSet = userDefaults.bool(forKey: Keys.extendedSet)
        self.keyboardMode = KeyboardMode(rawValue: userDefaults.object(forKey: Keys.keyboardMode) as? Int ?? 0) ?? .transducer
    }

    @Published public var schemeId: UInt32 {
        didSet {
            userDefaults.set(Int(schemeId), forKey: Keys.schemeId)
        }
    }

    @Published public var niggahitaVariant: UInt32 {
        didSet {
            userDefaults.set(Int(niggahitaVariant), forKey: Keys.niggahitaVariant)
        }
    }

    @Published public var extendedSet: Bool {
        didSet {
            userDefaults.set(extendedSet, forKey: Keys.extendedSet)
        }
    }

    @Published public var keyboardMode: KeyboardMode {
        didSet {
            userDefaults.set(keyboardMode.rawValue, forKey: Keys.keyboardMode)
        }
    }

    /// Active Scheme enum
    public var activeScheme: Scheme {
        get { Scheme(rawValue: schemeId) ?? .velthuis }
        set { schemeId = newValue.rawValue }
    }

    /// Active Niggahīta variant enum
    public var activeNiggahita: NiggahitaVariant {
        get { NiggahitaVariant(rawValue: niggahitaVariant) ?? .dotBelow }
        set { niggahitaVariant = newValue.rawValue }
    }

    /// Character used for Niggahīta
    public var niggahitaCharacter: Character {
        activeNiggahita.character
    }

    /// Capital character used for Niggahīta
    public var capitalNiggahitaCharacter: Character {
        activeNiggahita.capitalCharacter
    }

    /// Re-reads values from underlying UserDefaults (useful when extension opens)
    public func reload() {
        let newScheme = UInt32(userDefaults.object(forKey: Keys.schemeId) as? Int ?? 0)
        let newNiggahita = UInt32(userDefaults.object(forKey: Keys.niggahitaVariant) as? Int ?? 0)
        let newExtended = userDefaults.bool(forKey: Keys.extendedSet)
        let newMode = KeyboardMode(rawValue: userDefaults.object(forKey: Keys.keyboardMode) as? Int ?? 0) ?? .transducer

        if schemeId != newScheme { schemeId = newScheme }
        if niggahitaVariant != newNiggahita { niggahitaVariant = newNiggahita }
        if extendedSet != newExtended { extendedSet = newExtended }
        if keyboardMode != newMode { keyboardMode = newMode }
    }
}
