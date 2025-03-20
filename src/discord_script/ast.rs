use super::token::Token;

#[derive(Debug)]
pub enum StatementExecutionError {}

#[derive(Debug)]
pub enum ExepressionExecutionError {
    ScaryError,
}

pub trait Expression: std::fmt::Debug {
    fn evaluate(&self) -> Result<ReturnType, ExepressionExecutionError>;
    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        MemoryAllocationInfo { count: 0 }
    }
}

pub struct MemoryAllocationInfo {
    pub count: usize, //How much to allocate in bytes
}

pub trait Statement: std::fmt::Debug {
    fn evaluate(&self, buffer: Box<&mut [u8]>) -> Result<(), StatementExecutionError>;
    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo;
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum PrimitiveTypes {
    Int,
    Float,
    String,
    Pointer,
}

pub struct ReturnType {
    pub memory: Box<[u8]>,
    pub primitive_type: PrimitiveTypes,
}

//EXPRESSIONS

//LITERAL
#[derive(Debug)]
pub struct NumberExpression {
    pub value: f64,
}
impl Expression for NumberExpression {
    fn evaluate(&self) -> Result<ReturnType, ExepressionExecutionError> {
        self.value;
        Ok(ReturnType {
            memory: Box::new(self.value.to_be_bytes()),
            primitive_type: PrimitiveTypes::Float,
        })
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        MemoryAllocationInfo { count: 8 }
    }
}

#[derive(Debug)]
pub struct StringExpression {
    pub value: String,
}
impl Expression for StringExpression {
    fn evaluate(&self) -> Result<ReturnType, ExepressionExecutionError> {
        todo!()
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        MemoryAllocationInfo {
            count: self.value.len(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolExpression {
    pub value: String,
}
impl Expression for SymbolExpression {
    fn evaluate(&self) -> Result<ReturnType, ExepressionExecutionError> {
        todo!()
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        MemoryAllocationInfo { count: 0 }
    }
}

//COMPLEX
#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}
impl Expression for BinaryExpression {
    fn evaluate(&self) -> Result<ReturnType, ExepressionExecutionError> {
        let left_res = self.left.evaluate()?;
        let right_res = self.right.evaluate()?;
        if left_res.primitive_type != right_res.primitive_type {
            return Err(ExepressionExecutionError::ScaryError);
        }
        match left_res.primitive_type {
            PrimitiveTypes::Int => {}
            PrimitiveTypes::Float => {
                let left = f64::from_be_bytes(left_res.memory[0..8].try_into().unwrap());
                let right = f64::from_be_bytes(right_res.memory[0..8].try_into().unwrap());

                use super::token::TokenKind::*;
                let ret_value: f64 = match self.operator.kind {
                    Plus => left + right,
                    Minus => left - right,
                    Star => left * right,
                    Divide => left / right,
                    _ => {
                        panic!("Wrong operator assigned to a binary expression {:#?}", self.operator)
                    }
                };
                return Ok(ReturnType {
                    memory: Box::new(ret_value.to_be_bytes()),
                    primitive_type: PrimitiveTypes::Float,
                });
            }
            PrimitiveTypes::String => {}
            PrimitiveTypes::Pointer => {}
        };

        Err(ExepressionExecutionError::ScaryError)
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        MemoryAllocationInfo {
            count: self.left.get_memory_allocation_info().count
                + self.right.get_memory_allocation_info().count,
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Token,
    pub expression: Box<dyn Expression>,
}
impl Expression for UnaryExpression {
    fn evaluate(&self) -> Result<ReturnType, ExepressionExecutionError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct GrouppingExpression {
    pub inner: Box<dyn Expression>,
}
impl Expression for GrouppingExpression {
    fn evaluate(&self) -> Result<ReturnType, ExepressionExecutionError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct AssignmentExpression {
    pub assigne: Box<dyn Expression>,
    pub operator: Token,
    pub value: Box<dyn Expression>,
}
impl Expression for AssignmentExpression {
    fn evaluate(&self) -> Result<ReturnType, ExepressionExecutionError> {
        todo!()
    }
}

//STATEMENTS
#[derive(Debug)]
pub struct BlockStatement {
    pub body: Vec<Box<dyn Statement>>,
}
impl Statement for BlockStatement {
    fn evaluate(&self, buffer: Box<&mut [u8]>) -> Result<(), StatementExecutionError> {
        let mut offset = 0;
        for statement in &self.body {
            let mai = statement.get_memory_allocation_info();
            let buffer_slice = &mut buffer[offset..mai.count];
            statement.evaluate(Box::new(buffer_slice))?;
            offset += mai.count;
        }
        Ok(())
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        let mut count = 0;
        for statement in &self.body {
            count += statement.get_memory_allocation_info().count;
        }
        MemoryAllocationInfo { count }
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
}
impl Statement for ExpressionStatement {
    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        self.expression.get_memory_allocation_info()
    }

    fn evaluate(&self, buffer: Box<&mut [u8]>) -> Result<(), StatementExecutionError> {
        let r = self.expression.evaluate().unwrap();
        for (index, byte) in r.memory.iter().enumerate() {
            (*buffer)[index] = *byte;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct VariableStatement {
    pub is_const: bool,
    pub variable_name: String,
    pub explicit_type: Option<String>,
    pub assignment: Box<dyn Expression>,
}
impl Statement for VariableStatement {
    fn evaluate(&self, _buffer: Box<&mut [u8]>) -> Result<(), StatementExecutionError> {
        todo!()
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        todo!()
    }
}
