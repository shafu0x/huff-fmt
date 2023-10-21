pub struct CLexer {
    input: String,
    position: usize,
}

impl CLexer {
    pub fn new(input: String) -> CLexer {
        CLexer { input, position: 0 }
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let current_char = self.input.chars().nth(self.position);
        self.position += 1;
        Some(Token::new(TokenKind::Tag, current_char.unwrap().to_string()))
    }
}

impl Iterator for CLexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    //     while let Some(token) = self.next_token() {
    //         match token {
    //             Ok(token) => return Some(Ok(token)),
    //             Err(err) => return Some(Err(err)),
    //         }
    //     }
    }
}

pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

impl Token {
    pub fn new(kind: TokenKind, text: String) -> Token {
        Token { kind, text }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Tag,
    TagName,
    Comment,
    Whitespace,
    Newline,
}
