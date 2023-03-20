#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Instruction {
    Multiply,
    Sum,
    Neg,
    Duplicate,
    Integer(i32),
}
