//! Configuration management for PaliKey Windows shell.
//!
//! Conforms to `shared/config.schema.json` and manages persistence to
//! `%APPDATA%/PaliKey/config.json`.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use palikey_engine::chars::NiggahitaVariant;
use palikey_engine::engine::EngineConfig;
use palikey_engine::schemes::SchemeId;
use serde::{Deserialize, Serialize};

/// Supported transliteration schemes in configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemeConfig {
    Velthuis,
    Palitelex,
    Sil,
}

impl Default for SchemeConfig {
    fn default() -> Self {
        Self::Velthuis
    }
}

impl From<SchemeConfig> for SchemeId {
    fn from(s: SchemeConfig) -> Self {
        match s {
            SchemeConfig::Velthuis => SchemeId::Velthuis,
            SchemeConfig::Palitelex => SchemeId::PaliTelex,
            SchemeConfig::Sil => SchemeId::Sil,
        }
    }
}

impl From<SchemeId> for SchemeConfig {
    fn from(id: SchemeId) -> Self {
        match id {
            SchemeId::Velthuis => SchemeConfig::Velthuis,
            SchemeId::PaliTelex => SchemeConfig::Palitelex,
            SchemeId::Sil => SchemeConfig::Sil,
        }
    }
}

/// Niggahīta orthographic variants in configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NiggahitaConfig {
    DotBelow,
    DotAbove,
}

impl Default for NiggahitaConfig {
    fn default() -> Self {
        Self::DotBelow
    }
}

impl From<NiggahitaConfig> for NiggahitaVariant {
    fn from(n: NiggahitaConfig) -> Self {
        match n {
            NiggahitaConfig::DotBelow => NiggahitaVariant::DotBelow,
            NiggahitaConfig::DotAbove => NiggahitaVariant::DotAbove,
        }
    }
}

impl From<NiggahitaVariant> for NiggahitaConfig {
    fn from(v: NiggahitaVariant) -> Self {
        match v {
            NiggahitaVariant::DotBelow => NiggahitaConfig::DotBelow,
            NiggahitaVariant::DotAbove => NiggahitaConfig::DotAbove,
        }
    }
}

/// Escape and undo behavior settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscapeConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_escape_char")]
    pub escape_char: String,
    #[serde(default = "default_true")]
    pub undo_by_backspace: bool,
    #[serde(default = "default_true")]
    pub repeat_key_escapes: bool,
}

fn default_true() -> bool {
    true
}

fn default_escape_char() -> String {
    "\\".to_string()
}

impl Default for EscapeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            escape_char: default_escape_char(),
            undo_by_backspace: true,
            repeat_key_escapes: true,
        }
    }
}

/// Root configuration matching `shared/config.schema.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub scheme: SchemeConfig,
    #[serde(default)]
    pub niggahita: NiggahitaConfig,
    #[serde(default)]
    pub extended_set: bool,
    #[serde(default)]
    pub escape: EscapeConfig,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            schema: Some("./config.schema.json".to_string()),
            version: default_version(),
            scheme: SchemeConfig::Velthuis,
            niggahita: NiggahitaConfig::DotBelow,
            extended_set: false,
            escape: EscapeConfig::default(),
        }
    }
}

impl Config {
    /// Resolves the default configuration file path (%APPDATA%/PaliKey/config.json).
    pub fn default_path() -> PathBuf {
        if let Ok(appdata) = std::env::var("APPDATA") {
            PathBuf::from(appdata).join("PaliKey").join("config.json")
        } else if let Ok(userprofile) = std::env::var("USERPROFILE") {
            PathBuf::from(userprofile)
                .join("AppData")
                .join("Roaming")
                .join("PaliKey")
                .join("config.json")
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home)
                .join(".config")
                .join("PaliKey")
                .join("config.json")
        } else {
            PathBuf::from(".palikey").join("config.json")
        }
    }

    /// Loads configuration from the default path, falling back to default if absent or invalid.
    pub fn load() -> Self {
        Self::load_from_path(&Self::default_path()).unwrap_or_default()
    }

    /// Loads configuration from a specific path.
    pub fn load_from_path(path: &Path) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let cfg: Self = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(cfg)
    }

    /// Saves configuration to the default path.
    pub fn save(&self) -> io::Result<()> {
        Self::save_to_path(self, &Self::default_path())
    }

    /// Saves configuration to a specific path.
    pub fn save_to_path(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json_str = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, json_str)?;
        Ok(())
    }

    /// Converts configuration to the engine's native `EngineConfig`.
    pub fn to_engine_config(&self) -> EngineConfig {
        EngineConfig {
            scheme: self.scheme.into(),
            niggahita: self.niggahita.into(),
            extended_set: self.extended_set,
            escape_char: if self.escape.enabled {
                self.escape.escape_char.chars().next()
            } else {
                None
            },
            undo_by_backspace: self.escape.undo_by_backspace,
            repeat_key_escapes: self.escape.repeat_key_escapes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_matches_schema() {
        let cfg = Config::default();
        let json_val = serde_json::to_value(&cfg).expect("serialize default config");

        assert_eq!(json_val["version"], "0.1.0");
        assert_eq!(json_val["scheme"], "velthuis");
        assert_eq!(json_val["niggahita"], "dot_below");
        assert_eq!(json_val["extended_set"], false);
        assert_eq!(json_val["escape"]["enabled"], true);
        assert_eq!(json_val["escape"]["escape_char"], "\\");
        assert_eq!(json_val["escape"]["undo_by_backspace"], true);
        assert_eq!(json_val["escape"]["repeat_key_escapes"], true);
    }

    #[test]
    fn test_parse_shared_config_json() {
        let shared_json = r#"{
  "$schema": "./config.schema.json",
  "version": "0.1.0",
  "scheme": "velthuis",
  "niggahita": "dot_below",
  "extended_set": false,
  "escape": {
    "enabled": true,
    "escape_char": "\\",
    "undo_by_backspace": true,
    "repeat_key_escapes": true
  }
}"#;
        let cfg: Config = serde_json::from_str(shared_json).expect("parse shared config");
        assert_eq!(cfg.scheme, SchemeConfig::Velthuis);
        assert_eq!(cfg.niggahita, NiggahitaConfig::DotBelow);
        assert_eq!(cfg.extended_set, false);
        assert_eq!(cfg.escape.escape_char, "\\");

        let eng_cfg = cfg.to_engine_config();
        assert_eq!(eng_cfg.scheme, SchemeId::Velthuis);
        assert_eq!(eng_cfg.niggahita, NiggahitaVariant::DotBelow);
        assert_eq!(eng_cfg.extended_set, false);
        assert_eq!(eng_cfg.escape_char, Some('\\'));
    }

    #[test]
    fn test_scheme_and_niggahita_variants() {
        let telex_above = r#"{
  "scheme": "palitelex",
  "niggahita": "dot_above",
  "extended_set": true
}"#;
        let cfg: Config = serde_json::from_str(telex_above).expect("parse telex config");
        assert_eq!(cfg.scheme, SchemeConfig::Palitelex);
        assert_eq!(cfg.niggahita, NiggahitaConfig::DotAbove);
        assert_eq!(cfg.extended_set, true);

        let eng_cfg = cfg.to_engine_config();
        assert_eq!(eng_cfg.scheme, SchemeId::PaliTelex);
        assert_eq!(eng_cfg.niggahita, NiggahitaVariant::DotAbove);
        assert_eq!(eng_cfg.extended_set, true);

        let sil_json = r#"{
  "scheme": "sil",
  "niggahita": "dot_below",
  "extended_set": false
}"#;
        let cfg_sil: Config = serde_json::from_str(sil_json).expect("parse sil config");
        assert_eq!(cfg_sil.scheme, SchemeConfig::Sil);
        assert_eq!(cfg_sil.to_engine_config().scheme, SchemeId::Sil);
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let temp_dir = std::env::temp_dir().join("palikey_test_config");
        let temp_file = temp_dir.join("config.json");

        let mut cfg = Config::default();
        cfg.scheme = SchemeConfig::Palitelex;
        cfg.niggahita = NiggahitaConfig::DotAbove;
        cfg.extended_set = true;

        cfg.save_to_path(&temp_file).expect("save config");
        let loaded = Config::load_from_path(&temp_file).expect("load config");

        assert_eq!(loaded.scheme, SchemeConfig::Palitelex);
        assert_eq!(loaded.niggahita, NiggahitaConfig::DotAbove);
        assert_eq!(loaded.extended_set, true);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
