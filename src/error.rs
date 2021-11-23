use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Clone, Copy)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,

    /// Lamport balance below rent-exempt threshold.
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
