//! Export scheme tables from schemes.rs into a machine-readable JSON format.
//!
//! Run with:
//!   cargo run -p palikey-engine --example export_schemes
//! Or specify an output path:
//!   cargo run -p palikey-engine --example export_schemes -- shared/schemes.json

use std::env;
use std::fs;
use std::path::Path;

use palikey_engine::chars::*;
use palikey_engine::schemes::*;

fn escape_json(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn generate_schemes_json() -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"version\": \"0.1.0\",\n");
    json.push_str("  \"schemes\": {\n");

    let schemes = all_schemes();
    for (s_idx, scheme) in schemes.iter().enumerate() {
        json.push_str(&format!("    \"{}\": {{\n", scheme.name));
        json.push_str(&format!("      \"id\": \"{}\",\n", scheme.name));
        json.push_str(&format!("      \"display_name\": \"{}\",\n", escape_json(scheme.display_name)));
        json.push_str(&format!("      \"description\": \"{}\",\n", escape_json(scheme.description)));
        json.push_str("      \"mappings\": [\n");

        for (m_idx, rule) in scheme.mappings.iter().enumerate() {
            let (target_char, char_dot_above, target_type) = match rule.target {
                TargetChar::Fixed(c) => (c, None, "fixed"),
                TargetChar::Niggahita(is_upper) => {
                    let below = NiggahitaVariant::DotBelow.char_for(is_upper);
                    let above = NiggahitaVariant::DotAbove.char_for(is_upper);
                    (below, Some(above), "niggahita")
                }
                TargetChar::Extended(c) => (c, None, "extended"),
            };

            let unicode_hex = format!("U+{:04X}", target_char as u32);

            json.push_str("        {\n");
            json.push_str(&format!("          \"sequence\": \"{}\",\n", escape_json(rule.sequence)));
            json.push_str(&format!("          \"char\": \"{}\",\n", escape_json(&target_char.to_string())));
            if let Some(above) = char_dot_above {
                json.push_str(&format!("          \"char_dot_above\": \"{}\",\n", escape_json(&above.to_string())));
            } else {
                json.push_str("          \"char_dot_above\": null,\n");
            }
            json.push_str(&format!("          \"unicode\": \"{}\",\n", unicode_hex));
            json.push_str(&format!("          \"target_type\": \"{}\",\n", target_type));
            json.push_str(&format!("          \"is_extended\": {}\n", rule.is_extended));

            if m_idx + 1 < scheme.mappings.len() {
                json.push_str("        },\n");
            } else {
                json.push_str("        }\n");
            }
        }

        json.push_str("      ]\n");
        if s_idx + 1 < schemes.len() {
            json.push_str("    },\n");
        } else {
            json.push_str("    }\n");
        }
    }

    json.push_str("  }\n");
    json.push_str("}\n");
    json
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_content = generate_schemes_json();

    if args.contains(&"--stdout".to_string()) {
        print!("{}", json_content);
        return;
    }

    // Determine target path
    let target_path = if args.len() > 1 && !args[1].starts_with('-') {
        args[1].clone()
    } else {
        // Try relative to workspace root or current directory
        if Path::new("shared").exists() {
            "shared/schemes.json".to_string()
        } else if Path::new("../shared").exists() {
            "../shared/schemes.json".to_string()
        } else {
            "shared/schemes.json".to_string()
        }
    };

    if let Some(parent) = Path::new(&target_path).parent() {
        let _ = fs::create_dir_all(parent);
    }

    fs::write(&target_path, &json_content).expect("Failed to write schemes.json");
    println!("Exported schemes to {}", target_path);
}
