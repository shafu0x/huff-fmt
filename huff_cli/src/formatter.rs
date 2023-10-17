use crate::generator::Generator;
use huff_utils::prelude::*;

pub struct Formatter<'a> {
    current_line_number: usize,
    generator: &'a Generator,
    output: String,
}

impl Formatter<'_> {
    pub fn new(generator: &Generator) -> Formatter {
        Formatter { current_line_number: 0, generator, output: String::new() }
    }

    pub fn fmt(&mut self) {
        // while let Some(token) = generator.next() {

        // }
    }

    fn fmt_include(&mut self) {
        if let TokenKind::Str(string) = &self.generator.peeks(0).unwrap().kind {
            self.output.push_str(&format!("#include \"{}\"", &string));
        }
    }

    fn is_new_line(&mut self, token: &Token) {
        if token.line_number > self.current_line_number {
            self.current_line_number = token.line_number;
            self.output.push('\n');
        }
    }
}
