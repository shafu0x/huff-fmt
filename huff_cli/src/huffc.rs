use huff_lexer::Lexer;
use huff_utils::prelude::*;

mod generator;
use generator::Generator;

mod evm;
use evm::OpcodeFormatted;

fn main() {
    // Instantiate a new lexer
    let source = r#"
        #define constant S_SHIFT = 12
        #define macro _NAME_OF_FUNCTION() = takes(2) returns(0) {
           40
           40
           call          // fsdfdsf
           calldatacopy
        }
    "#;

    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);

    let tokens: Vec<Token> = lexer.collect::<Result<_, _>>().unwrap();

    // remove the whitespace tokens
    let tokens: Vec<Token> =
        tokens.into_iter().filter(|token| !matches!(token.kind, TokenKind::Whitespace)).collect();

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
        println!("{:?}", token);

        check_new_line(token.line_number, &mut formatted);
        if token.kind == TokenKind::Define {
            // constant
            if generator.peeks(0).unwrap().kind == TokenKind::Constant {
                if let TokenKind::Ident(ident) = &generator.peeks(1).unwrap().kind {
                    if let TokenKind::Num(num) = &generator.peeks(3).unwrap().kind {
                        formatted
                            .push_str(&format!("{} constant {} = {}", &token.kind, ident, num));
                        generator.increment_index(4);
                        println!("found");
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
