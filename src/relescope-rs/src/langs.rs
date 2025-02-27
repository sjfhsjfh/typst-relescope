pub mod rust;

use std::str::FromStr;

pub enum SupportedLang {
    Rust,
}

impl FromStr for SupportedLang {
    type Err = String;

    fn from_str(lang: &str) -> Result<Self, Self::Err> {
        match lang.to_lowercase().as_str() {
            "rust" | "rs" => Ok(Self::Rust),
            l => Err(format!("Unsupported language: {}", l)),
        }
    }
}
