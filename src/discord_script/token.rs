use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    True,
    False,
    Number,
    String,
    Identifier,

    // Grouping & Braces
    Pipe,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,
    OpenParen,
    CloseParen,

    // Equivilance
    Equals,
    NotEquals,
    Not,
    Assignment,

    // Conditional
    Less,
    LessEquals,
    Greater,
    GreaterEquals,

    // Logical
    Or,
    And,

    // Symbols
    DotDot,
    Dot,
    SemiColon,
    DoubleColon,
    Colon,
    Question,
    Comma,

    // Shorthand
    PlusPlus,
    MinusMinus,
    PlusEquals,
    MinusEquals,

    DivideEquals,
    MultiplyEquals,
    ModEquals,

    //Maths
    Plus,
    Dash,
    Slash,
    Star,
    Percent,

    // Reserved Keywords
    Let,
    Const,
    Struct,
    Import,
    Fn,
    If,
    Else,
    While,
    For,
    In,
    Match,
    Pub,
    Return,
    Continue,
    Break,
}

impl TokenKind {
    pub fn is_keyword(input: &str) -> Option<TokenKind> {
        match input {
            "let" => Some(TokenKind::Let),
            "const" => Some(TokenKind::Const),
            "struct" => Some(TokenKind::Struct),
            "import" => Some(TokenKind::Import),
            "fn" => Some(TokenKind::Fn),
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "while" => Some(TokenKind::While),
            "for" => Some(TokenKind::For),
            "in" => Some(TokenKind::In),
            "match" => Some(TokenKind::Match),
            "pub" => Some(TokenKind::Pub),
            "return" => Some(TokenKind::Return),
            "continue" => Some(TokenKind::Continue),
            "break" => Some(TokenKind::Break),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    kind: TokenKind,
    value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        Token { kind, value }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TokenKind::Identifier | TokenKind::String | TokenKind::Number => {
                write!(f, "{:?}({})", self.kind, self.value)
            }
            _ => write!(f, "{:?}", self.kind),
        }
    }
}
