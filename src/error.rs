use thiserror::Error;

pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid InvalidInstruction")]
    InvalidInstruction,
}
