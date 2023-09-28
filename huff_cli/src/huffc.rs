use huff_lexer::Lexer;
use huff_utils::prelude::*;

mod generator;
use generator::Generator;

fn main() {
    // Instantiate a new lexer
    let source = r#"
        #define macro _NAME_OF_FUNCTION() = takes(2) returns(0) {
        }
    "#;

    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);

    let tokens: Vec<Token> = lexer.collect::<Result<_, _>>().unwrap();

    // remove the whitespace tokens
    let tokens: Vec<Token> = tokens
        .into_iter()
        .filter(|token| match token.kind {
            TokenKind::Whitespace => false,
            _ => true,
        })
        .collect();

    let mut generator = Generator::new(tokens);

    let mut formatted = String::new();

    while let Some(token) = generator.next() {
        // println!("{:?}", token);

        if token.kind == TokenKind::Define {
            if generator.peeks(0).unwrap().kind == TokenKind::Macro {
                 if let TokenKind::Ident(ident) = &generator.peeks(1).unwrap().kind {
                    println!("Found a macro with name: {}", ident);
                }
                

            }
        }
    }

    // println!("{}", formatted);

    // for token in generator {
    //     if token.kind == TokenKind::Define {
    // if generator.peek().unwrap().kind == TokenKind::Macro {
    //     // if generator.peek().unwrap().kind == TokenKind::Ident {
    //     //     println!("Found macro definition");
    //     // }
    // }
    // }
    // }
}
