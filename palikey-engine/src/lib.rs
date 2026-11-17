//! PaliKey Engine — Clean-room cross-platform Pāḷi input method engine.
//!
//! Licensed under the MIT License. Clean-room implementation per DESIGN.md.

pub mod chars;
pub mod engine;
pub mod ffi;
pub mod schemes;
pub mod vi_telex;

pub use chars::{NiggahitaVariant, CHAR_A_MACRON_LOWER, CHAR_M_DOT_BELOW_LOWER};
pub use engine::{Engine, EngineConfig, KeyAction, ProcessResult};
pub use ffi::{PalikeyEngine, PalikeyProcessResult};
pub use schemes::{get_scheme, SchemeDefinition, SchemeId};
pub use vi_telex::VietnameseTelexComposer;
