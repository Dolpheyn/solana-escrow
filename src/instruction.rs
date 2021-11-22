use solana_program::program_error::ProgramError;
use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {
    /// Starts the trade by creating and populating an escrow account and transferring ownership of
    /// the given temp token account to the PDA
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction
    ///    and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go
    ///    through
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    ///
    /// * `Initializer` is party A(aka the signer) - the person that initialize the escrow.
    InitEscrow {
        /// The amount party A expects to receive of token Y.
        amount: u64,
    }
}

impl EscrowInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction);
        Ok
    }
}
