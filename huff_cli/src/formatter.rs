use crate::evm::OpcodeFormatted;
use crate::generator::Generator;
use crate::lexer::CLexer;
use huff_utils::prelude::*;

pub struct Formatter<'a> {
    current_line_number: usize,
    generator: &'a mut Generator,
    pub output: String,
}

impl Formatter<'_> {
    pub fn new(generator: &mut Generator) -> Formatter {
        Formatter { current_line_number: 0, generator, output: String::new() }
    }

    pub fn fmt(&mut self) {
        while let Some(token) = self.generator.next() {
            match &token.kind {
                TokenKind::Include => self.fmt_include(),
                TokenKind::Define => match &self.generator.peeks(0).unwrap().kind {
                    TokenKind::Constant => self.fmt_constant(&token),
                    TokenKind::CodeTable => self.fmt_code_table(&token),
                    TokenKind::Macro => self.fmt_macro(),
                    _ => (),
                },
                TokenKind::Comment(comment) => self.fmt_comment(comment),
                _ => (),
            }

            self.output.push('\n');
        }
    }

    fn fmt_comment(&mut self, comment: &str) {
        self.output.push_str(&format!("{}", comment));
        let lexer = CLexer::new(comment.to_string());
        println!("{:?}", lexer.peek().unwrap());
    }

    fn fmt_include(&mut self) {
        if let TokenKind::Str(string) = &self.generator.peeks(0).unwrap().kind {
            self.output.push_str(&format!("#include \"{}\"", &string));
        }
    }

    fn fmt_constant(&mut self, token: &Token) {
        if let TokenKind::Ident(ident) = &self.generator.peeks(1).unwrap().kind {
            if let TokenKind::Num(num) = &self.generator.peeks(3).unwrap().kind {
                self.output.push_str(&format!("{} constant {} = {}", token.kind, ident, num));
                self.generator.increment_index(4);
            }
        }
    }

    fn fmt_code_table(&mut self, token: &Token) {
        if let TokenKind::Ident(ident) = &self.generator.peeks(1).unwrap().kind {
            if let TokenKind::Ident(ident2) = &self.generator.peeks(3).unwrap().kind {
                self.output
                    .push_str(&format!("{} table {} {{ \n  {}\n}}", &token.kind, ident, ident2));
                self.generator.increment_index(4);
            }
        }
    }

    fn fmt_macro(&mut self) {
        // start of macro
        if let TokenKind::Ident(ident) = &self.generator.peeks(1).unwrap().kind {
            let takes = &self.generator.peeks(7).unwrap().kind;
            let returns = &self.generator.peeks(11).unwrap().kind;

            self.output.push_str(&format!(
                "#define macro {}() = takes({}) returns({}) {{",
                ident, takes, returns,
            ));

            self.generator.increment_index(14);
        }

        // in macro
        while self.generator.peeks(0).unwrap().kind != TokenKind::CloseBrace {
            let token = self.generator.next().unwrap();
            self.is_new_line(&token);
            match &token.kind {
                TokenKind::Opcode(opcode) => {
                    self.output.push_str(&format!("    {}", opcode.format()));
                }
                TokenKind::Comment(comment) => {
                    self.output.push_str(&format!("    {}", comment));
                }
                _ => {
                    self.output.push_str(&format!("    {}", token.kind.to_string()));
                }
            }
        }

        // end of macro
        self.output.push_str("\n}");
    }

    fn is_new_line(&mut self, token: &Token) {
        if token.line_number > self.current_line_number {
            self.current_line_number = token.line_number;
            self.output.push('\n');
        }
    }
}
