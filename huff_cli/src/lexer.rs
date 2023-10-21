pub struct CLexer {
    text: String,
    position: usize,
}

impl CLexer {
    pub fn new(text: String) -> CLexer {
        CLexer { text, position: 0 }
    }

    pub fn peek(&self) -> Option<char> {
        self.text.chars().nth(self.position)
    }

    pub fn peeks(&self, n: usize) -> Option<char> {
        self.text.chars().nth(self.position + n)
    }

    fn remove_whitespace(&mut self) {
        while let Some(current_char) = self.peek() {
            if current_char.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn get_line(&mut self) -> String {
        let mut line = String::new();
        while let Some(current_char) = self.peek() {
            if current_char == '\n' {
                break;
            } else {
                line.push(current_char);
                self.position += 1;
            }
        }
        line
    }

    fn get_word(&mut self) -> String {
        let mut word = String::new();
        while let Some(current_char) = self.peek() {
            if current_char.is_whitespace() {
                break;
            } else {
                word.push(current_char);
                self.position += 1;
            }
        }
        word
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.remove_whitespace();
        if let Some(current_char) = self.text.chars().nth(self.position) {
            if current_char == '/' {
                if self.peeks(1).unwrap() == '*' {
                    if self.peeks(2).unwrap() == '*' {
                        self.position += 3;
                        return Some(Token::new(TokenKind::MutliLineStart, "/**".to_string()));
                    }
                }
            }
            if current_char == '*' {
                if self.peeks(1).unwrap() == '/' {
                    self.position += 2;
                    return Some(Token::new(TokenKind::MutliLineEnd, "*/".to_string()));
                } else {
                    self.position += 1;
                    self.remove_whitespace();

                    let word = self.get_word();
                    println!("word: {}", word);

                    // if word starts with @
                    if word.chars().nth(0).unwrap() == '@' {
                        return Some(Token::new(TokenKind::Tag, word));
                    } else {
                        return Some(Token::new(TokenKind::Text, word));
                    }

                    let line = self.get_line(); 
                    return Some(Token::new(TokenKind::TagName, line));
                }
            }
            None
        } else {
            None
        }
    }
}

impl Iterator for CLexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[derive(Debug, PartialEq)]
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
    Text, 
    TagName,
    Comment,
    Whitespace,
    Newline,
    MutliLineStart, 
    MutliLineEnd, 
}
