use std::fmt::{self, Debug}; // Import Debug trait

pub struct Token {
    typ: TokenType,
    value: Option<u64>,
}
impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Token Type: {:?} ", self.typ)?;
        match &self.value {
            Some(value) => writeln!(f, "with value {}", value),
            None => writeln!(f, "No value"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Operators
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Pointer,        // ->
    Reference,      // &
    Modulus,        // %
    BitAnd,         // &
    BitOr,          // |
    BitXor,         // ^
    Not,            // !
    ConditionalAnd, // &&
    ConditionalOr,  // ||
    LeftRound,      // (
    RightRound,     // )
    LeftCurly,      // {
    RightCurly,     // }
    LeftSquare,     // [
    RightSquare,    // ]
    Equals,         // =
    EqualEquals,    // ==
    NotEquals,      // !=
    Greater,        // >
    GreaterEqual,   // >=
    Less,           // <
    LessEqual,      // <=
    Semicolon,      // ;
    CharOpen,       // '
    CharClose,      // '
    QuotesOpen,     // "
    QuotesClose,    // "

    // Data Types
    Int,
    Short,
    Long,
    Float,
    Double,
    Unsigned,
    Char,
    Void,

    // Keywords
    If,
    Else,
    For,
    While,
    Do,
    Break,
    Return,

    // Literals
    Data8,  // 8-bit integer literal
    Data16, // 16-bit integer literal
    Data32, // 32-bit integer literal
    Data64, // 64-bit integer literal
}
