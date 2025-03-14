use std::collections::HashMap;

use crate::discord_script::ast::*;

use super::ast::{Expression, Statement};
use super::token::{Token, TokenKind};

#[derive(Debug, PartialEq, Hash, PartialOrd, Clone)]
#[repr(i32)]
pub enum BindingPower {
    None,
    Comma,
    Assignment,
    Logical,
    Relational,
    Additive,
    Multiplicative,
    Unary,
    Call,
    Member,
    Primary,
}

#[derive(Debug)]
pub enum ParserErrors {
    NextTokenNotFound,
    NumberIsNotANumber,
    BindingPowerError,
}

type LeftDenotationHandler = dyn Fn(
    &mut Parser,
    Box<dyn Expression>,
    BindingPower,
) -> Result<Box<dyn Expression>, ParserErrors>;
type NullDenotationHandler = dyn Fn(TokenKind, Token) -> Result<Box<dyn Expression>, ParserErrors>;
type StatementHandler = dyn Fn(&mut Parser) -> Result<Box<dyn Statement>, ParserErrors>;

pub struct Parser {
    tokens: Vec<Token>,
    left_denotation_lookup: HashMap<TokenKind, Box<LeftDenotationHandler>>,
    null_denotation_lookup: HashMap<TokenKind, Box<NullDenotationHandler>>,
    statement_lookup: HashMap<TokenKind, Box<StatementHandler>>,
    binding_power_lookup: HashMap<TokenKind, BindingPower>,
}

impl Parser {
    fn parse_primary_expression(
        current_token: TokenKind,
        next_token: Token,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        use super::token::TokenKind::*;
        //let next_token = self.next_token().ok_or(ParserErrors::NextTokenNotFound)?;
        match current_token {
            Number => {
                if let Ok(number) = next_token.value.parse::<f64>() {
                    return Ok(Box::new(NumberExpression { value: number }));
                }
                return Err(ParserErrors::NumberIsNotANumber);
            }
            String => Ok(Box::new(StringExpression {
                value: next_token.value,
            })),
            Identifier => Ok(Box::new(SymbolExpression {
                value: next_token.value,
            })),
            _ => {
                panic!("TODO: ")
            }
        }
    }

    fn parse_expression(
        &mut self,
        power: BindingPower,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        let mut kind = self.current_token().kind.clone();
        println!("{kind:?}");
        let next = self.next_token().unwrap();

        let null_denotation_func = self
            .null_denotation_lookup
            .get_mut(&kind)
            .ok_or(ParserErrors::NextTokenNotFound)?;
        let mut left = (null_denotation_func)(kind.clone(), next)?;

        kind = self.current_token().kind.clone();
        println!("{kind:?}");
        while *self
            .binding_power_lookup
            .get(&kind)
            .unwrap_or(&BindingPower::None)
            > power
        {

            left = self.parse_binary_expression(left, power.clone()).unwrap();
            kind = self.current_token().kind.clone();
            println!("{kind:?}");
        }

        Ok(left)
    }

    fn parse_binary_expression(
        &mut self,
        left: Box<dyn Expression>,
        power: BindingPower,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        let op_token = self.next_token().ok_or(ParserErrors::NextTokenNotFound)?;
        let right = self.parse_expression(power)?;

        Ok(Box::new(BinaryExpression {
            left,
            operator: op_token.clone(),
            right,
        }))
    }

    fn has_tokens(&self) -> bool {
        self.current_token().kind != TokenKind::Eof 
    }

    pub fn parse(tokens: Vec<Token>) -> Result<Box<BlockStatement>, ParserErrors> {
        let mut body: Vec<Box<dyn Statement>> = Vec::new();
        let mut parser = Parser::new(tokens);

        while parser.has_tokens() {
            body.push(parser.parse_statement()?)
        }

        Ok(Box::new(BlockStatement { body }))
    }

    fn current_token(&self) -> &Token {
        return &self.tokens.first().unwrap();
    }

    fn next_token(&mut self) -> Option<Token> {
        Some(self.tokens.remove(0))
    }

    fn new(tokens: Vec<Token>) -> Self {
        let mut res = Self {
            tokens,
            left_denotation_lookup:  HashMap::new(),
            null_denotation_lookup: HashMap::new(),
            statement_lookup: HashMap::new(),
            binding_power_lookup: HashMap::new(),
        };
        use super::token::TokenKind::*;
        res.left_denotation(Plus, BindingPower::Additive, Box::new(self::Parser::parse_binary_expression));
        res.left_denotation(Star, BindingPower::Multiplicative, Box::new(self::Parser::parse_binary_expression));
        //res.statement(SemiColon, BindingPower::Call, Box::new(self::Parser::parse_statement));
        //res.left_denotation_lookup(And, BindingPower::Logical, );
        res.null_denotation(
            Number,
            BindingPower::Primary,
            Box::new(self::Parser::parse_primary_expression),
        );
        res.null_denotation(
            String,
            BindingPower::Primary,
            Box::new(self::Parser::parse_primary_expression),
        );
        res.null_denotation(
            Identifier,
            BindingPower::Primary,
            Box::new(self::Parser::parse_primary_expression),
        );

        res
    }

    pub fn left_denotation(
        &mut self,
        kind: TokenKind,
        power: BindingPower,
        left_denotation_func: Box<LeftDenotationHandler>,
    ) {
        self.binding_power_lookup.insert(kind.clone(), power);

        self.left_denotation_lookup
            .insert(kind, left_denotation_func);
    }

    pub fn null_denotation(
        &mut self,
        kind: TokenKind,
        power: BindingPower,
        null_denotation_func: Box<NullDenotationHandler>,
    ) {
        self.binding_power_lookup.insert(kind.clone(), power);
        self.null_denotation_lookup
            .insert(kind, null_denotation_func);
    }
    pub fn statement(
        &mut self,
        kind: TokenKind,
        power: BindingPower,
        statement_handler: Box<StatementHandler>,
    ) {
        self.binding_power_lookup.insert(kind.clone(), power);
        self.statement_lookup.insert(kind, statement_handler);
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParserErrors> {
        let stat = self.statement_lookup.get(&self.current_token().kind);
        if let Some(_) = stat {
            return self.parse_statement();
        }

        let expression = self.parse_expression(BindingPower::None)?;
        self.next_token();

        return Ok(Box::new(ExpressionStatement { expression }));
    }
}
