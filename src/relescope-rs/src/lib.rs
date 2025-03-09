#![feature(new_range_api)]

pub mod langs;

use core::range::Range;
use langs::SupportedLang;
use serde::Serialize;
use std::collections::HashMap;
use typst_wasm_protocol::wasm_export;

pub struct MyString(pub String);

impl From<&[u8]> for MyString {
    fn from(s: &[u8]) -> Self {
        MyString(String::from_utf8_lossy(s).to_string())
    }
}

impl AsRef<str> for MyString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize)]
struct ItemInfo {
    pub name: Option<String>,
    /// **1-based** line number
    pub first_line: usize,
    pub src: String,
}

struct ItemParser {
    /// Raw source code
    pub src: String,
}

impl GetSrc for ItemParser {
    fn get_src(&self, range: Range<usize>) -> String {
        self.src[range].to_string()
    }
}

impl ItemParser {
    fn new(src: MyString) -> Self {
        Self { src: src.0 }
    }
}

pub(crate) trait GetSrc {
    fn get_src(&self, range: Range<usize>) -> String;
}

pub(crate) trait GetName<T> {
    fn get_name(&self, item: &T) -> Option<String>;
}

pub(crate) trait LocateItem<T> {
    fn locate(&self, item: &T) -> Range<usize>;
}

pub(crate) trait ExtractItem<T> {
    fn extract(&self, item: &T) -> ItemInfo;
}

impl<T, P> ExtractItem<T> for P
where
    P: LocateItem<T> + GetName<T> + GetSrc,
{
    fn extract(&self, item: &T) -> ItemInfo {
        let range = self.locate(&item);
        return ItemInfo {
            name: self.get_name(&item),
            first_line: 0,
            src: self.get_src(range).to_string(),
        };
    }
}

#[wasm_export]
#[allow(private_interfaces)]
pub fn pick(src: MyString, name: MyString, lang: MyString) -> Result<String, String> {
    let lang: SupportedLang = lang.0.parse()?;
    let src = ItemParser::new(src);
    serde_json::to_string(&match lang {
        SupportedLang::Rust => langs::rust::pick(src, name)?,
        SupportedLang::Python => langs::python::pick(src, name)?,
    })
    .map_err(|e| e.to_string())
}

#[wasm_export(export_rename = "scope")]
pub fn list(src: MyString, lang: MyString) -> Result<String, String> {
    let lang: SupportedLang = lang.0.parse()?;
    let src = ItemParser::new(src);
    let named_list = match lang {
        SupportedLang::Rust => langs::rust::named_list(src)?,
        SupportedLang::Python => langs::python::named_list(src)?,
    };
    serde_json::to_string(
        &named_list
            .iter()
            .map(|i| (i.name.clone(), i))
            .collect::<HashMap<_, _>>(),
    )
    .map_err(|e| e.to_string())
}
