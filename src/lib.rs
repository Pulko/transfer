mod instruction;

use {
    instruction::TransferInstruction,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke_signed,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
    },
    spl_token::{
        instruction::transfer_checked,
        state::{Account, Mint},
    },
};

solana_program::entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let instruction = instruction::TransferInstruction::unpack(instruction_data)
        .unwrap_or(Err(ProgramError::InvalidInstructionData)?);

    let source_info = next_account_info(accounts_iter)?; // The sender
    let destination_info = next_account_info(accounts_iter)?; // The receiver
    let mint_info = next_account_info(accounts_iter)?; // The info about the token we transfer
    let authority_info = next_account_info(accounts_iter)?; // Do we have an authority to transfer a token
    let token_program_info = next_account_info(accounts_iter)?; // The token program

    let (expected_authority, bump_seed) = Pubkey::find_program_address(&[b"authority"], program_id);

    if expected_authority != *authority_info.key {
        msg!("Invalid authority");
        return Err(ProgramError::InvalidSeeds);
    }

    let source_account = Account::unpack(&source_info.try_borrow_data()?)?;
    let mint = Mint::unpack(&mint_info.try_borrow_data()?)?;
    let decimals = mint.decimals;

    // calculate amount from instruction

    let amount = match instruction {
        TransferInstruction::Transfer(args) => args.value,
        _ => source_account.amount,
    };

    // invoke transfer program spl_token

    invoke_signed(
        &transfer_checked(
            token_program_info.key,
            source_info.key,
            mint_info.key,
            destination_info.key,
            authority_info.key,
            &[],
            amount,
            decimals,
        )
        .unwrap(),
        &[
            source_info.clone(),
            mint_info.clone(),
            destination_info.clone(),
            authority_info.clone(),
            token_program_info.clone(),
        ],
        &[&[b"authority", &[bump_seed]]],
    )?;

    Ok(())
}
