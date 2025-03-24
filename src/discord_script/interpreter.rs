use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use crate::discord_script::ast::{BinaryOperations, Types};

use super::ast::{
    AbstractExpressionDescription, AbstractStatementDescription, AbstractValue, BlockStatement,
    Statement,
};

#[derive(PartialEq, Eq, PartialOrd)]
pub struct Variable {
    pub value: Box<Vec<u8>>,
    pub depth: usize,
}

impl Ord for Variable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        usize::cmp(&self.depth, &other.depth)
    }
}

pub struct Interpreter<'a> {
    pub vars: std::collections::HashMap<String, Vec<Variable>>,
    pub current_depth: usize,
    pub null_expression_out: Option<Box<dyn FnMut(AbstractValue) + 'a>>,
    pub on_error: Option<Box<dyn Fn(String) + 'a>>,
}

#[derive(Debug)]
pub enum InterpreterErrors {
    VariableAlreadyExists(String),
    SymbolNotFound(String),
    Unimplemented,
}

impl Display for InterpreterErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for InterpreterErrors {}

impl Interpreter<'_> {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            current_depth: 0,
            null_expression_out: None,
            on_error: None,
        }
    }

    pub fn execute(&mut self, ast: BlockStatement) {
        let d = ast.get_description();
        if let AbstractStatementDescription::Block(statements) = d {
            for statement in statements {
                let res = self.execute_statement(*statement);
                if let Some(e) = res.err() {
                    if let Some(cb) = &self.on_error {
                        (cb)(e.to_string())
                    }
                }
            }
        };
        //self.execute_statement(ast.get_description());
    }

    pub fn execute_statement(
        &mut self,
        description: AbstractStatementDescription,
    ) -> Result<Option<AbstractValue>, Box<dyn Error>> {
        match description {
            AbstractStatementDescription::Variable(name, _size, value) => {
                let var = Variable {
                    value: self.execute_expression(*value)?.memory,
                    depth: 0,
                };
                if let Some(vars) = self.vars.get_mut(&name) {
                    if vars.contains(&var) {
                        return Err(Box::new(InterpreterErrors::VariableAlreadyExists(name)));
                    } else {
                        vars.push(var);
                    }
                    vars.sort();
                } else {
                    self.vars.insert(name, vec![var]);
                }
                Ok(None)
            }
            AbstractStatementDescription::Block(statements) => {
                let mut value = None;
                for statement in statements {
                    value = self.execute_statement(*statement)?;
                }
                Ok(value)
            }
            AbstractStatementDescription::Expression(expr) => {
                let value = self.execute_expression(*expr)?;
                if let Some(cb) = &mut self.null_expression_out {
                    (cb)(value);
                }
                Ok(None)
            }
        }
    }

    pub fn execute_expression(
        &mut self,
        expr: AbstractExpressionDescription,
    ) -> Result<AbstractValue, Box<dyn Error>> {
        use AbstractExpressionDescription::*;
        match expr {
            Integer(value) | UnsingedInteger(value) | Float(value) | LiteralString(value) => {
                Ok(value)
            }
            AbstractExpressionDescription::Binary(left, right, binary_operations) => {
                let left_value = self.execute_expression(*left)?;
                let right_value = self.execute_expression(*right)?;

                if !left_value._type.supports(binary_operations.clone()) {
                    //TODO: Custom types
                    return Err(Box::new(InterpreterErrors::Unimplemented));
                }
                if !right_value._type.supports(binary_operations.clone()) {
                    return Err(Box::new(InterpreterErrors::Unimplemented));
                }
                let _type = left_value._type.clone();
                fn perform_operation<
                    T: std::convert::From<AbstractValue>
                        + std::ops::Add<Output = T>
                        + std::ops::Sub<Output = T>
                        + std::ops::Div<Output = T>
                        + std::ops::Mul<Output = T>
                        + num_traits::ops::checked::CheckedDiv
                        + num_traits::ops::checked::CheckedSub
                        + num_traits::ops::checked::CheckedAdd
                        + num_traits::ops::checked::CheckedMul,
                >(
                    left: AbstractValue,
                    right: AbstractValue,
                    op: BinaryOperations,
                ) -> Result<T, Box<dyn Error>> {
                    match op {
                        BinaryOperations::Add => Ok(Into::<T>::into(left)
                            .checked_add(&Into::<T>::into(right))
                            .ok_or("Overflow error")?),
                        BinaryOperations::Subtract => Ok(Into::<T>::into(left)
                            .checked_sub(&Into::<T>::into(right))
                            .ok_or("Underflow error")?),
                        BinaryOperations::Divide => Ok(Into::<T>::into(left)
                            .checked_div(&Into::<T>::into(right))
                            .ok_or("Zero division error! ")?),
                        BinaryOperations::Multiply => Ok(Into::<T>::into(left)
                            .checked_mul(&Into::<T>::into(right))
                            .ok_or("Overflow error")?),
                    }
                }

                let res: Vec<u8> = match left_value._type.clone() {
                    Types::Integer => {
                        perform_operation::<i64>(left_value, right_value, binary_operations)?
                            .to_be_bytes()
                            .to_vec()
                    }
                    Types::UnsingedInteger => {
                        perform_operation::<u64>(left_value, right_value, binary_operations)?
                            .to_be_bytes()
                            .to_vec()
                    }
                    Types::Float => {
                        let left = Into::<f64>::into(left_value);
                        let right = Into::<f64>::into(right_value);
                        let res = match binary_operations {
                            BinaryOperations::Add => left + right,
                            BinaryOperations::Subtract => left - right,
                            BinaryOperations::Divide => left / right,
                            BinaryOperations::Multiply => left * right,
                        };
                        res.to_be_bytes().to_vec()
                    }
                    Types::String => match binary_operations {
                        BinaryOperations::Add => (String::from(left_value)
                            + String::from(right_value).as_str())
                        .into_bytes(),
                        _ => return Err(Box::new(InterpreterErrors::Unimplemented)),
                    },
                    Types::Boolean => return Err(Box::new(InterpreterErrors::Unimplemented)), // TODO: add support for boolean operations,
                    _ => return Err(Box::new(InterpreterErrors::Unimplemented)),
                };

                Ok(AbstractValue::new(Box::new(res), _type))
            }
            AbstractExpressionDescription::FunctionCall(_) => todo!(),
        }
    }
}
