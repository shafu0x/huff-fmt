use clap::Parser;
use huff_lexer::Lexer;
use huff_utils::prelude::*;

mod generator;
use generator::Generator;

mod evm;
use evm::OpcodeFormatted;

use crate::opts::Opts;

mod opts;

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

    let mut formatted = String::new();

    let mut current_line_number = 0;

    let mut check_new_line = |token_line_number, formatted: &mut String| {
        if token_line_number > current_line_number {
            current_line_number = token_line_number;
            formatted.push('\n');
        }
    };

    while let Some(token) = generator.next() {
        check_new_line(token.line_number, &mut formatted);

        // include
        if token.kind == TokenKind::Include {
            if let TokenKind::Str(string) = &generator.peeks(0).unwrap().kind {
                formatted.push_str(&format!("#include \"{}\"", &string));
            }
        }

        if token.kind == TokenKind::Define {
            // constant
            if generator.peeks(0).unwrap().kind == TokenKind::Constant {
                if let TokenKind::Ident(ident) = &generator.peeks(1).unwrap().kind {
                    if let TokenKind::Num(num) = &generator.peeks(3).unwrap().kind {
                        formatted
                            .push_str(&format!("{} constant {} = {}", &token.kind, ident, num));
                        generator.increment_index(4);
                    }
                }
            }

            // code table
            if generator.peeks(0).unwrap().kind == TokenKind::CodeTable {
                if let TokenKind::Ident(ident) = &generator.peeks(1).unwrap().kind {
                    if let TokenKind::Ident(ident2) = &generator.peeks(3).unwrap().kind {
                        formatted.push_str(&format!(
                            "{} table {} {{ \n  {}\n}}",
                            &token.kind, ident, ident2
                        ));
                        generator.increment_index(4);
                    }
                }
            }

            // macro
            if generator.peeks(0).unwrap().kind == TokenKind::Macro {
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
                    check_new_line(token.line_number, &mut formatted);
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
                formatted.push_str("\n}}\n");
            }
        }
    }

    println!();
    println!("{}", formatted);
}
