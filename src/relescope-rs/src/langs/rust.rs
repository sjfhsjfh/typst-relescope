use syn::{parse_str, spanned::Spanned, File, Item};

use crate::{MyString, PickResult};

/// A function to pick specific function/struct/enum from the Rust source code
pub(crate) fn pick(src: MyString, name: MyString) -> Result<PickResult, String> {
    let file: File = parse_str(&src.0).map_err(|e| e.to_string())?;
    let mut target: Option<&Item> = None;
    // TODO: Better way to find the target item
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
    return Ok(PickResult {
        first_line: target.span().start().line,
        src: source,
    });
}
