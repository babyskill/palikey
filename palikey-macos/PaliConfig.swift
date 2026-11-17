//
//  PaliConfig.swift
//  PaliKey
//
//  Manages PaliKey configuration and persistence to config.json.
//

import Foundation

public final class PaliConfig {
    public static let shared = PaliConfig()

    public var version: String = "0.1.0"
    public var scheme: UInt32 = 0        // 0: Velthuis, 1: PaliTelex, 2: SIL
    public var niggahita: UInt32 = 0     // 0: DotBelow (ṃ), 1: DotAbove (ṁ)
    public var extendedSet: Bool = false
    public var escapeChar: Character = "\\"
    public var undoByBackspace: Bool = true
    public var repeatKeyEscapes: Bool = true

    private let fileManager = FileManager.default

    private var configURL: URL {
        let appSupport = fileManager.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
        let dir = appSupport.appendingPathComponent("PaliKey")
        try? fileManager.createDirectory(at: dir, withIntermediateDirectories: true)
        return dir.appendingPathComponent("config.json")
    }

    public init() {
        load()
    }

    public func load() {
        let url = configURL
        guard fileManager.fileExists(atPath: url.path),
              let data = try? Data(contentsOf: url),
              let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            return
        }

        if let schemeStr = json["scheme"] as? String {
            switch schemeStr.lowercased() {
            case "velthuis": self.scheme = 0
            case "palitelex", "telex": self.scheme = 1
            case "sil", "keyman": self.scheme = 2
            default: break
            }
        } else if let schemeNum = json["scheme_id"] as? UInt32 {
            self.scheme = schemeNum
        }

        if let nigStr = json["niggahita"] as? String {
            switch nigStr.lowercased() {
            case "dot_above": self.niggahita = 1
            default: self.niggahita = 0
            }
        } else if let nigNum = json["niggahita_variant"] as? UInt32 {
            self.niggahita = nigNum
        }

        if let ext = json["extended_set"] as? Bool {
            self.extendedSet = ext
        }

        if let escDict = json["escape"] as? [String: Any] {
            if let undo = escDict["undo_by_backspace"] as? Bool {
                self.undoByBackspace = undo
            }
            if let rep = escDict["repeat_key_escapes"] as? Bool {
                self.repeatKeyEscapes = rep
            }
        }
    }

    public func save() {
        let schemeStr: String
        switch scheme {
        case 1: schemeStr = "palitelex"
        case 2: schemeStr = "sil"
        default: schemeStr = "velthuis"
        }

        let nigStr = (niggahita == 1) ? "dot_above" : "dot_below"

        let dict: [String: Any] = [
            "$schema": "./config.schema.json",
            "version": version,
            "scheme": schemeStr,
            "niggahita": nigStr,
            "extended_set": extendedSet,
            "escape": [
                "enabled": true,
                "escape_char": "\\",
                "undo_by_backspace": undoByBackspace,
                "repeat_key_escapes": repeatKeyEscapes
            ]
        ]

        if let data = try? JSONSerialization.data(withJSONObject: dict, options: [.prettyPrinted]) {
            try? data.write(to: configURL, options: .atomic)
        }
    }
}
