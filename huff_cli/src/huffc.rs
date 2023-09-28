use huff_lexer::Lexer;
use huff_utils::prelude::*;

mod generator;
use generator::Generator;

fn main() {
    // Instantiate a new lexer
    let source = r#"
        #define macro _NAME_OF_FUNCTION() = takes(2) returns(0) {
            55
            43
            add
        }
    "#;

    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.source);

    let tokens: Vec<Token> = lexer.collect::<Result<_, _>>().unwrap();
    let mut generator = Generator::new(tokens);

    for token in generator {
        println!("{:?}", token);
    }
}
