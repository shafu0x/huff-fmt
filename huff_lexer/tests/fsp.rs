use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn free_storage_pointer() {
    let source = "FREE_STORAGE_POINTER() ";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.source.clone());

    // The first token should be the fsp
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(
        tok,
        Token::new(TokenKind::FreeStoragePointer, Span::new(0..21, None), lexer.line_number())
    );

    // Eats the whitespace
    let _ = lexer.next();

    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Eof, Span::new(22..22, None), lexer.line_number()));

    // We should have reached EOF now
    assert!(lexer.eof);
}
