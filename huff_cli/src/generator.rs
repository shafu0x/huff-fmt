use huff_lexer::Lexer;
use huff_utils::prelude::*;

pub struct Generator {
    tokens: Vec<Token>,
    index: usize,
}

impl Generator {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn peeks(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.index + n)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.index).cloned();
        self.index += 1;
        token
    }

    pub fn increment_index(&mut self, n: usize) {
        self.index += n;
    }
}

impl Iterator for Generator {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
