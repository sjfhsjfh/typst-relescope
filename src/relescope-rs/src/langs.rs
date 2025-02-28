pub(crate) mod rust;
pub(crate) mod python;

use std::str::FromStr;

pub enum SupportedLang {
    Rust,
    Python
}

impl FromStr for SupportedLang {
    type Err = String;

    fn from_str(lang: &str) -> Result<Self, Self::Err> {
        match lang.to_lowercase().as_str() {
            "rust" | "rs" => Ok(Self::Rust),
            "python" | "py" => Ok(Self::Python),
            l => Err(format!("Unsupported language: {}", l)),
        }
    }
}
