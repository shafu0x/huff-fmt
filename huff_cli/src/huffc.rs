use huff_lexer::Lexer;
use huff_utils::prelude::*;

mod generator;
use generator::Generator;

fn main() {
    // Instantiate a new lexer
    let source = r#"
        #define macro _NAME_OF_FUNCTION() = takes(2) returns(0) {
           40
           40
           call
           calldatacopy
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
        println!("{:?}", token);

        if token.kind == TokenKind::Define {
            if generator.peeks(0).unwrap().kind == TokenKind::Macro {
                // start of macro
                if let TokenKind::Ident(ident) = &generator.peeks(1).unwrap().kind {
                    let takes = &generator.peeks(7).unwrap().kind;
                    let returns = &generator.peeks(11).unwrap().kind;

                    formatted.push_str(&format!(
                        "#define macro {} = takes({}) returns({}) {{\n",
                        ident,
                        takes,
                        returns,
                    ));

                    generator.increment_index(14);
                }

                // in macro
                while generator.peeks(0).unwrap().kind != TokenKind::CloseBrace {
                    let token = generator.next().unwrap();
                    // TODO: map opcode hex to name
                    println!("{:?}", token);
                    formatted.push_str(&format!("    {}\n", token.kind));
                }


                // end of macro
                formatted.push_str(&format!( "}}\n",));
            }
        }
    }

    println!("{}", formatted);

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
