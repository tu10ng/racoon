use crate::compiler::ir::values::Ty;

#[derive(Debug)]
pub enum Error {
    TypeMismatch { expected: Ty, found: Ty },
    UnknownName(String),
    DuplicateName(String),
    WrongParamLength { expected: usize, found: usize },
    ExpectedFunction(String),
    None
}