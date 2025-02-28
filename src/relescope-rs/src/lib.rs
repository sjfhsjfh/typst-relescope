pub mod langs;

use langs::SupportedLang;
use serde::Serialize;
use typst_wasm_protocol::wasm_export;

pub struct MyString(pub String);

impl From<&[u8]> for MyString {
    fn from(s: &[u8]) -> Self {
        MyString(String::from_utf8_lossy(s).to_string())
    }
}

#[derive(Serialize)]
struct PickResult {
    pub first_line: usize,
    pub src: String,
}
#[wasm_export]
#[allow(private_interfaces)]
pub fn pick(src: MyString, name: MyString, lang: MyString) -> Result<String, String> {
    let lang: SupportedLang = lang.0.parse()?;
    serde_json::to_string(&match lang {
        SupportedLang::Rust => langs::rust::pick(src, name)?,
        SupportedLang::Python => langs::python::pick(src, name)?,
    })
    .map_err(|e| e.to_string())
}
