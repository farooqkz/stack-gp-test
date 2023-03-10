#[derive(Clone, Debug, Copy)]
pub enum Instruction {
    Multiply,
    Sum,
    Neg,
    Integer(i32),
}
