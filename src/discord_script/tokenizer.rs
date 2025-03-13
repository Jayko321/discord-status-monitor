use std::fmt::Display;

use crate::discord_script::token::*;
use lazy_regex::{regex, Lazy, Regex};

pub struct Lexer {
    tokens: Vec<Token>,
    input: String,
    pos: usize,
    patterns: Vec<RegexPatterns>,
}


pub struct RegexPatterns {
    regex: regex::Regex,
    handler: Box<dyn Fn(&Regex) -> (usize, Option<Token>)>,
}

#[derive(Debug)]
pub enum TokenizerError {
    InvalidToken,
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TokenizerError {}

fn default_handler(kind: TokenKind, value: String) -> impl Fn(&Regex) -> (usize, Option<Token>) {
    let res = move |_: &Regex| -> (usize, Option<Token>) {
        return (value.len(), Some(Token::new(kind.clone(), value.clone())));
    };
    res
}

impl Lexer {
    fn new(source: String) -> Self {
        Self {
            tokens: vec![],
            input: source,
            pos: 0,
            patterns: vec![
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\["#)).clone(),
                    handler: Box::new(default_handler(TokenKind::OpenBracket, "[".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\]"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::CloseBracket, "]".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\{"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::OpenCurly, "{".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\}"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::CloseCurly, "}".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\("#)).clone(),
                    handler: Box::new(default_handler(TokenKind::OpenParen, "(".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\)"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::CloseParen, ")".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"=="#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Equals, "==".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"!="#)).clone(),
                    handler: Box::new(default_handler(TokenKind::NotEquals, "!=".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"="#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Assignment, "=".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"!"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Not, "!".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"<="#)).clone(),
                    handler: Box::new(default_handler(TokenKind::LessEquals, "<=".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"<"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Less, "<".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#">="#)).clone(),
                    handler: Box::new(default_handler(TokenKind::GreaterEquals, ">=".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#">"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Greater, ">".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\|\|"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Or, "||".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"&&"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::And, "&&".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\.\."#)).clone(),
                    handler: Box::new(default_handler(TokenKind::DotDot, "..".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\."#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Dot, ".".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#";"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::SemiColon, ";".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#":"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Colon, ":".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\?"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Question, "?".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#","#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Comma, ",".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\+\+"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::PlusPlus, "++".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"--"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::MinusMinus, "--".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\+="#)).clone(),
                    handler: Box::new(default_handler(TokenKind::PlusEquals, "+=".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"-="#)).clone(),
                    handler: Box::new(default_handler(TokenKind::MinusEquals, "-=".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\+"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Plus, "+".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"-"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Dash, "-".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"/"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Slash, "/".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"\*"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Star, "*".to_string())),
                },
                RegexPatterns {
                    regex: Lazy::<Regex>::force(regex!(r#"%"#)).clone(),
                    handler: Box::new(default_handler(TokenKind::Percent, "%".to_string())),
                },
            ],
        }
    }

    fn remainder(&self) -> String {
        self.input[self.pos..].to_string()
    }

    fn at_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn tokenize(source: String) -> Result<Vec<Token>, TokenizerError> {
        let mut lexer = Lexer::new(source);

        while !lexer.at_eof() {
            let mut matched = false;

            let remainder = lexer.remainder().to_owned();
            let mut advance = 0;
            let mut token = None;
            for pattern in &mut lexer.patterns {
                if let Some(location) = pattern.regex.find(&remainder) {
                    if location.start() == 0 {
                        (advance, token) = (pattern.handler)(&pattern.regex);
                        matched = true;
                        break;
                    }
                }
            }

            if !matched {
                return Err(TokenizerError::InvalidToken);
            }

            if let Some(token) = token {
                lexer.tokens.push(token);
            }
            lexer.pos += advance;
        }

        Ok(lexer.tokens)
    }
}
