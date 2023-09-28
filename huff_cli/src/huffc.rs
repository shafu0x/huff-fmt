use huff_lexer::Lexer;
use huff_utils::prelude::*;

// fn format_fn_header() {

// }

fn main() {
    // Instantiate a new lexer
    let source = r#"
        #define macro __NON_PAYABLE_SELECTOR_CHECK() = takes(2) returns(0) {
            55
            43
            add

            complete_withdrawFrom:
                7
        }
    "#;
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.source);


    // lexer iterator into a list
    let mut tokens = lexer.collect::<Vec<_>>();

    let mut formatted = String::new();

    for token in tokens {
        let token = token.unwrap();
        // print if not whitespace
         if token.kind != TokenKind::Whitespace {
            println!("{:?}", token);
         }
        
    }
}
