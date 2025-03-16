use super::token::Token;

pub trait Expression: std::fmt::Debug {}

pub trait Statement: std::fmt::Debug {}

//EXPRESSIONS


//LITERAL
#[derive(Debug)]
pub struct NumberExpression {
    pub value: f64,
}
impl Expression for NumberExpression {}

#[derive(Debug)]
pub struct StringExpression {
    pub value: String,
}
impl Expression for StringExpression {}

#[derive(Debug)]
pub struct SymbolExpression {
    pub value: String,
}
impl Expression for SymbolExpression {}

//COMPLEX
#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}
impl Expression for BinaryExpression {}

//STATEMENTS
#[derive(Debug)]
pub struct BlockStatement {
    pub body: Vec<Box<dyn Statement>>
}
impl Statement for BlockStatement {}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>
}
impl Statement for ExpressionStatement {}

#[derive(Debug)]
pub struct VariableStatement {
    pub is_const: bool,
    pub variable_name: String,
    pub explicit_type: Option<String>,
    pub assignment: Box<dyn Expression>
}
impl Statement for VariableStatement {}
