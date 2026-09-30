use crate::{
    diagnostic::Span,
    lexer::Token,
    source_map::{SourceFile, SourceMap},
};

use anyhow::bail;
use logos::Logos;
use std::fs;

mod diagnostic;
mod lexer;
mod source_map;

fn main() -> anyhow::Result<()> {
    let path = "examples/vars.tl";
    let source = fs::read_to_string(path)?;

    let mut source_map = SourceMap::new();
    let source_file = SourceFile::new(path, source);
    let input_id = source_map.add(source_file);

    let file = source_map.file(input_id);
    let lexer = Token::lexer(&file.text);

    for (result, span) in lexer.spanned() {
        match result {
            Ok(token) => println!("{:?}", token),
            Err(e) => {
                let diag = e.to_diagnostic(Span::new(input_id, span.into()));
                diag.render(&source_map)?;

                bail!("error encountered during lexing");
            }
        }
    }

    Ok(())
}
