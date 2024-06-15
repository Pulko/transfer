use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct TransferInstructionArgs {
    pub value: u64,
}

pub enum TransferInstruction {
    Transfer(TransferInstructionArgs),
}

impl TransferInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, value) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => Ok(Self::Transfer(TransferInstructionArgs::try_from_slice(
                value,
            )?)),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
