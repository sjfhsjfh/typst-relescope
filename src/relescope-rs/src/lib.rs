use serde::Serialize;
use syn::{parse_str, spanned::Spanned, File, Item};
use typst_wasm_protocol::wasm_export;

#[derive(Serialize)]
struct PickResult {
    pub first_line: usize,
    pub src: String,
}

struct MyString(pub String);

impl From<&[u8]> for MyString {
    fn from(s: &[u8]) -> Self {
        MyString(String::from_utf8_lossy(s).to_string())
    }
}

#[wasm_export]
#[allow(private_interfaces)]
pub fn pick(src: MyString, name: MyString, lang: MyString) -> Result<String, String> {
    match lang.0.to_lowercase().as_str() {
        "rust" => pick_rs(src, name),
        _ => Err(format!("Unsupported language: {}", lang.0)),
    }
}

/// A function to pick specific function/struct/enum from the Rust source code
fn pick_rs(src: MyString, name: MyString) -> Result<String, String> {
    let file: File = parse_str(&src.0).map_err(|e| e.to_string())?;
    let mut target: Option<&Item> = None;
    for item in &file.items {
        match item {
            Item::Fn(f) => {
                if f.sig.ident.to_string() == name.0 {
                    target = Some(item);
                    break;
                }
            }
            Item::Struct(s) => {
                if s.ident.to_string() == name.0 {
                    target = Some(item);
                    break;
                }
            }
            Item::Enum(e) => {
                if e.ident.to_string() == name.0 {
                    target = Some(item);
                    break;
                }
            }
            Item::Const(c) => {
                if c.ident.to_string() == name.0 {
                    target = Some(item);
                    break;
                }
            }
            Item::Type(t) => {
                if t.ident.to_string() == name.0 {
                    target = Some(item);
                    break;
                }
            }
            Item::Static(s) => {
                if s.ident.to_string() == name.0 {
                    target = Some(item);
                    break;
                }
            }
            _ => {}
        }
    }
    let target = target.ok_or(format!("Item not found: {}", name.0))?;
    let source = target.span().source_text().ok_or("No source text")?;
    return Ok(serde_json::to_string(&PickResult {
        first_line: target.span().start().line,
        src: source,
    })
    .map_err(|e| e.to_string())?);
}
