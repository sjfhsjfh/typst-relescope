use crate::{ExtractItem, GetName, ItemInfo, ItemParser};
use syn::{parse_str, spanned::Spanned, File, Item};

impl GetName<Item> for ItemParser {
    fn get_name(&self, item: &Item) -> Option<String> {
        Some(match item {
            Item::Fn(f) => f.sig.ident.to_string(),
            Item::Struct(s) => s.ident.to_string(),
            Item::Enum(e) => e.ident.to_string(),
            Item::Const(c) => c.ident.to_string(),
            Item::Type(t) => t.ident.to_string(),
            Item::Static(s) => s.ident.to_string(),
            Item::Macro(m) => match m.ident.as_ref() {
                Some(ident) => ident.to_string(),
                None => return None,
            },
            _ => return None,
        })
    }
}

impl ExtractItem<Item> for ItemParser {
    fn extract(&self, item: &Item) -> ItemInfo {
        let span = item.span();
        ItemInfo {
            name: self.get_name(item),
            first_line: span.start().line,
            src: span.source_text().unwrap().to_string(),
        }
    }
}

/// A function to pick specific function/struct/enum from the Rust source code
pub(crate) fn pick(p: ItemParser, name: impl AsRef<str>) -> Result<ItemInfo, String> {
    let file: File = parse_str(&p.src).map_err(|e| e.to_string())?;
    let item = file.items.iter().find_map(|item| {
        if p.get_name(item).is_some_and(|n| n == name.as_ref()) {
            Some(p.extract(item))
        } else {
            None
        }
    });
    if let Some(item) = item {
        return Ok(item);
    }
    Err(format!("Item not found: {}", name.as_ref()))
}

/// A function to list all the exsiting functions/structs/enums in the Rust source code
pub(crate) fn named_list(p: ItemParser) -> Result<Vec<ItemInfo>, String> {
    let file: File = parse_str(&p.src).map_err(|e| e.to_string())?;
    let items = file
        .items
        .iter()
        .map(|item| p.extract(item))
        .filter(|i| i.name.is_some())
        .collect::<Vec<ItemInfo>>();
    Ok(items)
}
