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
    NoFunctionHandler(Token),
    UnexpectedExpressionType(Token),
    NextTokenNotFound,
    NumberIsNotANumber(Token),
    BindingPowerError,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum HandlerTypes {
    LeftDenotation,
    NullDenotation,
    Statement,
}

pub struct Parser {
    tokens: Vec<Token>,
    function_lookup_table: HashMap<TokenKind, HandlerTypes>,
    binding_power_lookup: HashMap<TokenKind, BindingPower>,
}

impl Parser {
    fn parse_primary_expression(&mut self) -> Result<Box<dyn Expression>, ParserErrors> {
        use super::token::TokenKind::*;
        let next_token = self.next_token().ok_or(ParserErrors::NextTokenNotFound)?;
        match next_token.kind {
            Number => {
                if let Ok(number) = next_token.value.parse::<f64>() {
                    return Ok(Box::new(NumberExpression { value: number }));
                }
                return Err(ParserErrors::NumberIsNotANumber(next_token));
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

        if *self
            .function_lookup_table
            .get(&kind)
            .ok_or(ParserErrors::NoFunctionHandler(self.current_token().clone()))?
            != HandlerTypes::NullDenotation
        {
            return Err(ParserErrors::UnexpectedExpressionType(self.current_token().clone()));
        }

        let mut left = self.parse_primary_expression()?;

        while *self
            .binding_power_lookup
            .get(&self.current_token().kind)
            .unwrap_or(&BindingPower::None)
            > power
        {
            kind = self.current_token().kind.clone();
            let function_type = self
                .function_lookup_table
                .get(&kind)
                .ok_or(ParserErrors::UnexpectedExpressionType(self.current_token().clone()))?;

            match function_type {
                HandlerTypes::LeftDenotation => {
                    left = self.parse_binary_expression(left, power.clone())?
                }
                HandlerTypes::NullDenotation | HandlerTypes::Statement => {
                    return Err(ParserErrors::UnexpectedExpressionType(self.current_token().clone()))
                }
            };
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
            function_lookup_table: HashMap::new(),
            binding_power_lookup: HashMap::new(),
        };
        res.create_lookup_table();

        res
    }

    pub(self) fn create_lookup_table(&mut self) {
        use super::token::TokenKind::*;
        use BindingPower::*;
        let mut add_new = |kind: TokenKind, power: BindingPower, h_type: HandlerTypes| {
            self.binding_power_lookup.insert(kind.clone(), power);
            self.function_lookup_table.insert(kind, h_type);
        };
        add_new(Number, BindingPower::Primary, HandlerTypes::NullDenotation);
        add_new(String, BindingPower::Primary, HandlerTypes::NullDenotation);
        add_new(
            Identifier,
            BindingPower::Primary,
            HandlerTypes::NullDenotation,
        );

        add_new(Plus, Additive, HandlerTypes::LeftDenotation);
        add_new(Star, Multiplicative, HandlerTypes::LeftDenotation);
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParserErrors> {
        if *self
            .function_lookup_table
            .get(&self.current_token().kind)
            .ok_or(ParserErrors::UnexpectedExpressionType(self.current_token().clone()))?
            == HandlerTypes::Statement
        {
            return self.parse_statement();
        }

        let expression = self.parse_expression(BindingPower::None)?;
        self.next_token();

        return Ok(Box::new(ExpressionStatement { expression }));
    }
}
