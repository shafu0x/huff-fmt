use clap::Parser;
use huff_lexer::Lexer;
use huff_utils::prelude::*;

mod generator;
use generator::Generator;

mod evm;
use evm::OpcodeFormatted;

use crate::opts::Opts;

mod opts;

mod formatter;
use formatter::Formatter;

fn fmt_include(generator: &Generator, formatted: &mut String) {
    if let TokenKind::Str(string) = &generator.peeks(0).unwrap().kind {
        formatted.push_str(&format!("#include \"{}\"", &string));
    }
}

fn fmt_constant(generator: &mut Generator, formatted: &mut String, token: &Token) {
    if let TokenKind::Ident(ident) = &generator.peeks(1).unwrap().kind {
        if let TokenKind::Num(num) = &generator.peeks(3).unwrap().kind {
            formatted.push_str(&format!("{} constant {} = {}", token.kind, ident, num));
            generator.increment_index(4);
        }
    }
}

fn fmt_code_table(generator: &mut Generator, formatted: &mut String, token: &Token) {
    if let TokenKind::Ident(ident) = &generator.peeks(1).unwrap().kind {
        if let TokenKind::Ident(ident2) = &generator.peeks(3).unwrap().kind {
            formatted.push_str(&format!("{} table {} {{ \n  {}}}", &token.kind, ident, ident2));
            generator.increment_index(4);
        }
    }
}

fn fmt_macro(generator: &mut Generator, formatted: &mut String, token: &Token) {
    // start of macro
    if let TokenKind::Ident(ident) = &generator.peeks(1).unwrap().kind {
        let takes = &generator.peeks(7).unwrap().kind;
        let returns = &generator.peeks(11).unwrap().kind;

        formatted.push_str(&format!(
            "#define macro {} = takes({}) returns({}) {{",
            ident, takes, returns,
        ));

        generator.increment_index(14);
    }

    // in macro
    while generator.peeks(0).unwrap().kind != TokenKind::CloseBrace {
        let token = generator.next().unwrap();
        // check_new_line(token.line_number, &mut formatted);
        match token.kind {
            TokenKind::Opcode(opcode) => {
                formatted.push_str(&format!("    {}", opcode.format()));
            }
            _ => {
                formatted.push_str(&format!("    {}", token.kind.to_string()));
            }
        }
    }

    // end of macro
    formatted.push_str("\n}}");
}

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
