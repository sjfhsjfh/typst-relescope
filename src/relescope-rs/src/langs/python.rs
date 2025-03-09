use core::range::Range;

use crate::{ExtractItem, GetName, ItemInfo, ItemParser, LocateItem};
use rustpython_parser::{
    ast::{Ranged, Stmt},
    Parse,
};

impl GetName<Stmt> for ItemParser {
    fn get_name(&self, item: &Stmt) -> Option<String> {
        match item {
            Stmt::FunctionDef(f) => Some(f.name.to_string()),
            Stmt::ClassDef(c) => Some(c.name.to_string()),
            Stmt::Assign(a) => {
                for tar in &a.targets {
                    if let rustpython_parser::ast::Expr::Name(n) = tar {
                        return Some(n.id.to_string());
                    }
                }
                None
            }
            _ => None,
        }
    }
}

impl LocateItem<Stmt> for ItemParser {
    fn locate(&self, item: &Stmt) -> Range<usize> {
        let range = item.range();
        Range {
            start: range.start().to_usize(),
            end: range.end().to_usize(),
        }
    }
}

pub(crate) fn pick(p: ItemParser, name: impl AsRef<str>) -> Result<ItemInfo, String> {
    let ast =
        rustpython_parser::ast::Suite::parse(&p.src, "<embedded>").map_err(|e| e.to_string())?;
    let item = ast.iter().find_map(|stmt| {
        p.get_name(stmt).and_then(|n| {
            if n == name.as_ref() {
                Some(p.extract(stmt))
            } else {
                None
            }
        })
    });
    if let Some(item) = item {
        return Ok(item);
    }
    Err(format!("Item not found: {}", name.as_ref()))
}

pub(crate) fn named_list(p: ItemParser) -> Result<Vec<ItemInfo>, String> {
    let ast =
        rustpython_parser::ast::Suite::parse(&p.src, "<embedded>").map_err(|e| e.to_string())?;
    let items = ast
        .iter()
        .filter_map(|stmt| {
            let item = p.extract(stmt);
            if item.name.is_some() {
                Some(item)
            } else {
                None
            }
        })
        .collect::<Vec<ItemInfo>>();
    Ok(items)
}
