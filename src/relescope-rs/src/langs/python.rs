use rustpython_parser::{
    ast::{Ranged, Stmt},
    Parse,
};

use crate::{MyString, PickResult};

pub(crate) fn pick(src: MyString, name: MyString) -> Result<PickResult, String> {
    let ast =
        rustpython_parser::ast::Suite::parse(&src.0, "<embedded>").map_err(|e| e.to_string())?;
    let mut target: Option<&Stmt> = None;
    for stmt in &ast {
        match stmt {
            Stmt::FunctionDef(f) => {
                if f.name == name.0 {
                    target = Some(stmt);
                    break;
                }
            }
            Stmt::ClassDef(c) => {
                if c.name == name.0 {
                    target = Some(stmt);
                    break;
                }
            }
            Stmt::Assign(a) => {
                for tar in &a.targets {
                    if let rustpython_parser::ast::Expr::Name(n) = tar {
                        if n.id == name.0 {
                            target = Some(stmt);
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    let target = target.ok_or(format!("Item not found: {}", name.0))?;
    let range = target.range();
    let start = range.start().to_usize();
    let end = range.end().to_usize();
    let mut first_line: usize = 0;
    for (i, c) in src.0.chars().enumerate() {
        if i >= start {
            break;
        }
        if c == '\n' {
            first_line += 1;
        }
    }
    Ok(PickResult {
        first_line,
        src: src.0[start..end].to_string(),
    })
}
