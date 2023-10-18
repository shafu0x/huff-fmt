use clap::Parser;
use huff_lexer::Lexer;
use huff_utils::prelude::*;

mod generator;
use generator::Generator;

mod evm;
use crate::opts::Opts;

mod opts;

mod formatter;
use formatter::Formatter;

fn main() {
    // parsing args
    let opts = Opts::parse();

    // Check if input is specified as path or passed directly as a string
    let inputs = opts.inputs;
    let source = if let Some(path) = inputs.path {
        std::fs::read_to_string(&path).unwrap()
    } else {
        inputs.input.unwrap()
    };

    let flattened_source = FullFileSource { source: &source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);

    // remove the whitespace tokens
    let tokens: Vec<Token> = lexer
        .into_iter()
        .map(|res| res.unwrap())
        .filter(|token| !matches!(token.kind, TokenKind::Whitespace))
        .collect();

    let mut generator = Generator::new(tokens);

    let mut formatter = Formatter::new(&mut generator);
    formatter.fmt();
    println!("{}", formatter.output);
}
