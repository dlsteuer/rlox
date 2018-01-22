use std::fmt::{Display, Formatter, Result};

use token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i64,
    literal: Box<Literal>,
}

impl Token {
    pub fn new<T: Literal + 'static>(token_type: TokenType, lexeme: String, line: i64, literal: T) -> Self {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            line: line,
            literal: Box::new(literal),
        }
    }
}

impl<'a> Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let b = self.literal.clone();
        write!(f, "{} {} {}", self.token_type, self.lexeme, b.to_string())
    }
}

pub trait Literal {
    fn box_clone(&self) -> Box<Literal>;
    fn to_string(self: Box<Self>) -> String;
}

impl Clone for Box<Literal> {
    fn clone(&self) -> Box<Literal> {
        self.box_clone()
    }
}

pub struct NullLiteral {}

impl NullLiteral {
    pub fn new() -> Self {
        NullLiteral {}
    }
}
impl Literal for NullLiteral {
    fn to_string(self: Box<Self>) -> String {
        String::from("")
    }
    fn box_clone(&self) -> Box<Literal> {
        Box::new(NullLiteral {})
    }
}

pub struct StringLiteral {
    val: String,
}

impl StringLiteral {
    pub fn new(val: String) -> Self {
        StringLiteral { val: val }
    }
}
impl Literal for StringLiteral {
    fn to_string(self: Box<Self>) -> String {
        self.val
    }
    fn box_clone(&self) -> Box<Literal> {
        Box::new(StringLiteral {
            val: self.val.clone(),
        })
    }
}
