#[derive(Clone, Debug, Copy)]
pub enum Instruction {
    Duplicate,
    Multiply,
    Sum,
    Neg,
    Swap,
    Integer(i32),
}
