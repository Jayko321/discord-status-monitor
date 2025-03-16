use std::collections::{HashMap, VecDeque};

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
    Comparison,
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
    UnexpectedTokenKind(Token),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum LeftDenotationHandlerTypes {
    Default,
    Assignment,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum NullDenotationHandlerTypes {
    Default,
    Unary,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum StatementHandlerTypes {
    Default,
    Variable,
}

pub struct Parser {
    tokens: VecDeque<Token>,
    left_denotation_lookup: HashMap<TokenKind, LeftDenotationHandlerTypes>,
    null_denotation_lookup: HashMap<TokenKind, NullDenotationHandlerTypes>,
    statement_lookup: HashMap<TokenKind, StatementHandlerTypes>,
    binding_power_lookup: HashMap<TokenKind, BindingPower>,
}

impl Parser {
    fn parse_primary_expression(&mut self) -> Result<Box<dyn Expression>, ParserErrors> {
        use super::token::TokenKind::*;
        let next_token = self.next_token()?;
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
                panic!("Should never happen")
            }
        }
    }

    fn try_parse_null_denotaion(&mut self, kind: TokenKind) -> Result<Box<dyn Expression>, ParserErrors> {
        let handler_type = self
            .null_denotation_lookup
            .get(&kind)
            .ok_or(ParserErrors::NoFunctionHandler(
                self.current_token().clone(),
            ))?;

        match handler_type {
            NullDenotationHandlerTypes::Default => return self.parse_primary_expression(),
            NullDenotationHandlerTypes::Unary => todo!(),
        }
    }

    fn try_parse_left_denotation(&mut self, left: Box<dyn Expression>, new_power: BindingPower, kind: TokenKind)  -> Result<Box<dyn Expression>, ParserErrors> {
        let function_type = self.left_denotation_lookup.get(&kind).ok_or(
            ParserErrors::UnexpectedExpressionType(self.current_token().clone()),
        )?;

        return match function_type {
            LeftDenotationHandlerTypes::Default => {
                self.parse_binary_expression(left, new_power.clone())
            }
            LeftDenotationHandlerTypes::Assignment => todo!(),
        };
    }

    fn parse_expression(
        &mut self,
        power: BindingPower,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        let mut kind = self.current_token().kind.clone();

        self.try_parse_null_denotaion(kind)?;

        let mut left = self.parse_primary_expression()?;

        while *self
            .binding_power_lookup
            .get(&self.current_token().kind)
            .unwrap_or(&BindingPower::None)
            > power
        {
            kind = self.current_token().kind.clone();

            let new_power = self
                .binding_power_lookup
                .get(&self.current_token().kind)
                .ok_or(ParserErrors::BindingPowerError)?;

            left = self.try_parse_left_denotation(left, new_power.clone(), kind)?;
        }

        Ok(left)
    }

    fn parse_binary_expression(
        &mut self,
        left: Box<dyn Expression>,
        power: BindingPower,
    ) -> Result<Box<dyn Expression>, ParserErrors> {
        let op_token = self.next_token()?;
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
        return &self.tokens.front().unwrap();
    }

    fn next_token(&mut self) -> Result<Token, ParserErrors> {
        self.tokens
            .pop_front()
            .ok_or(ParserErrors::NextTokenNotFound)
    }

    fn new(tokens: Vec<Token>) -> Self {
        let mut res = Self {
            tokens: tokens.into(),
            left_denotation_lookup: HashMap::new(),
            null_denotation_lookup: HashMap::new(),
            statement_lookup: HashMap::new(),
            binding_power_lookup: HashMap::new(),
        };
        res.create_lookup_table();

        res
    }

    fn expect_token_and_skip(&mut self, kind: TokenKind) -> Result<Token, ParserErrors> {
        if kind != self.current_token().kind {
            return Err(ParserErrors::UnexpectedTokenKind(
                self.current_token().clone(),
            ));
        }
        let token = self.next_token()?;

        return Ok(token);
    }

    pub(self) fn create_lookup_table(&mut self) {
        use super::token::TokenKind::*;
        use BindingPower::*;

        let mut add_new =
            |kind: TokenKind, power: BindingPower, h_type: LeftDenotationHandlerTypes| {
                self.binding_power_lookup.insert(kind.clone(), power);
                self.left_denotation_lookup.insert(kind, h_type);
            };

        self.null_denotation_lookup
            .insert(Number, NullDenotationHandlerTypes::Default);
        self.null_denotation_lookup
            .insert(String, NullDenotationHandlerTypes::Default);
        self.null_denotation_lookup
            .insert(Identifier, NullDenotationHandlerTypes::Default);

        //Logical
        add_new(And, Logical, LeftDenotationHandlerTypes::Default);
        add_new(Or, Logical, LeftDenotationHandlerTypes::Default);
        add_new(DotDot, Logical, LeftDenotationHandlerTypes::Default);

        //Comparison
        add_new(Less, Comparison, LeftDenotationHandlerTypes::Default);
        add_new(LessEquals, Comparison, LeftDenotationHandlerTypes::Default);
        add_new(Greater, Comparison, LeftDenotationHandlerTypes::Default);
        add_new(
            GreaterEquals,
            Comparison,
            LeftDenotationHandlerTypes::Default,
        );
        add_new(Equals, Comparison, LeftDenotationHandlerTypes::Default);
        add_new(NotEquals, Comparison, LeftDenotationHandlerTypes::Default);

        //Math
        add_new(Plus, Additive, LeftDenotationHandlerTypes::Default);
        add_new(Minus, Additive, LeftDenotationHandlerTypes::Default);

        add_new(Star, Multiplicative, LeftDenotationHandlerTypes::Default);
        add_new(Divide, Multiplicative, LeftDenotationHandlerTypes::Default);
        add_new(Percent, Multiplicative, LeftDenotationHandlerTypes::Default);

        //Statements
        self.statement_lookup
            .insert(Let, StatementHandlerTypes::Variable);
        self.statement_lookup
            .insert(Const, StatementHandlerTypes::Variable);
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParserErrors> {
        let statement_handler_type = self.statement_lookup.get(&self.current_token().kind);
        if let Some(statement_handler_type) = statement_handler_type {
            match *statement_handler_type {
                StatementHandlerTypes::Default => return self.parse_statement(),
                StatementHandlerTypes::Variable => return self.parse_variable_statement(),
            };
        }

        let expression = self.parse_expression(BindingPower::None)?;
        self.expect_token_and_skip(TokenKind::SemiColon)?;

        return Ok(Box::new(ExpressionStatement { expression }));
    }

    fn parse_variable_statement(&mut self) -> Result<Box<dyn Statement>, ParserErrors> {
        let let_token = self.next_token()?;
        let is_const = match let_token.kind {
            TokenKind::Const => true,
            TokenKind::Let => false,
            _ => {
                return Err(ParserErrors::UnexpectedTokenKind(let_token));
            }
        };

        let name_token = self.expect_token_and_skip(TokenKind::Identifier)?;
        let after_name = self.next_token()?;
        let has_explicit_type = match after_name.kind {
            TokenKind::Colon => true,
            TokenKind::Assignment => false,
            _ => return Err(ParserErrors::UnexpectedTokenKind(after_name)),
        };
        let mut explicit_type_val = None;
        if has_explicit_type {
            let explicit_type = self.next_token()?;
            if explicit_type.kind != TokenKind::Identifier {
                return Err(ParserErrors::UnexpectedTokenKind(explicit_type));
            }
            explicit_type_val = Some(explicit_type.value);
            self.expect_token_and_skip(TokenKind::Assignment)?;
        }

        //self.expect_token_and_skip(TokenKind::Assignment)?;

        let expr = self.parse_expression(BindingPower::None)?;

        self.expect_token_and_skip(TokenKind::SemiColon)?;

        return Ok(Box::new(VariableStatement {
            is_const,
            variable_name: name_token.value,
            explicit_type: explicit_type_val,
            assignment: expr,
        }));
    }
}
