use token::{Literal, NullLiteral, Token};
use token_type::TokenType;
use util::StringUtils;

pub struct Scanner {
    source: String,
    start: i64,
    current: i64,
    line: i64,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= (self.source.len() as i64)
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            Some('(') => self.add_token(TokenType::LeftParen),
            Some(')') => self.add_token(TokenType::RightParen),
            Some('{') => self.add_token(TokenType::LeftBrace),
            Some('}') => self.add_token(TokenType::RightBrace),
            Some(',') => self.add_token(TokenType::Comma),
            Some('.') => self.add_token(TokenType::Dot),
            Some('-') => self.add_token(TokenType::Minus),
            Some('+') => self.add_token(TokenType::Plus),
            Some(';') => self.add_token(TokenType::Semicolon),
            Some('*') => self.add_token(TokenType::Star),
            Some('!') => if self.match_token('=') {
                self.add_token(TokenType::BangEqual)
            } else {
                self.add_token(TokenType::Bang)
            },
            Some('=') => if self.match_token('=') {
                self.add_token(TokenType::EqualEqual)
            } else {
                self.add_token(TokenType::Equal)
            },
            Some('<') => if self.match_token('=') {
                self.add_token(TokenType::LessEqual)
            } else {
                self.add_token(TokenType::Less)
            },
            Some('>') => if self.match_token('=') {
                self.add_token(TokenType::GreaterEqual)
            } else {
                self.add_token(TokenType::Greater)
            },
            Some('/') => {
                if self.match_token('/') {
                    while self.peek_token() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            Some(' ') => (),
            Some('\r') => (),
            Some('\t') => (),
            Some('\n') => self.line += 1,
            Some(val) => println!("Unknown character: {} ({})", val, self.line),
            None => println!("invalid input"),
        }
    }

    fn peek_token(&mut self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }
        return self.source.chars().nth(self.current as usize);
    }

    fn match_token(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        };

        if self.source.chars().nth(self.current as usize) != Some(c) {
            return false;
        };

        self.current += 1;
        return true;
    }

    fn advance(&mut self) -> Option<char> {
        let val = self.source.chars().nth(self.current as usize);
        self.current += 1;
        val
    }

    fn add_token(&mut self, token_type: TokenType) {
        let literal = NullLiteral::new();
        self.add_token_with_literal(token_type, literal);
    }

    fn add_token_with_literal<T: Literal + 'static>(&mut self, token_type: TokenType, literal: T) {
        let s = self.start as usize;
        let c = self.current as usize;
        let text = self.source.substring(s, c - s);
        match token_type {
            TokenType::EOF => self.tokens.push(Token::new(
                token_type,
                String::from("EOF"),
                self.line,
                literal,
            )),
            _ => self.tokens
                .push(Token::new(token_type, text, self.line, literal)),
        }
    }
}

impl<'a> Scanner {
    pub fn scan_tokens(&'a mut self) -> &'a [Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::EOF);
        self.tokens.as_slice()
    }
}
