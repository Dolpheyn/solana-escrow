use crate::error::EscrowError::InvalidInstruction;
use solana_program::program_error::ProgramError;
use std::convert::TryInto;

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
    },
}

impl EscrowInstruction {
    /// The entrypoint of `instruction`.
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        // Get the first 8 bytes (because u64 is 8 bytes long) and cast it into u64.
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
