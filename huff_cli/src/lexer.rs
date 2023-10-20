pub struct CommentLexer {
    input: String,
    position: usize,
}

impl CommentLexer {
    pub fn new(input: String) -> CommentLexer {
        CommentLexer {
            input: input,
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }
}

pub struct CommentToken {
    pub kind: CommentTokenKind,
    pub text: String,
}

#[derive(Debug, PartialEq)]
pub enum CommentTokenKind {
    Comment,
    Whitespace,
    Newline,
}
