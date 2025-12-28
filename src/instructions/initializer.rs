use pinocchio::{
    ProgramResult, account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};

use crate::constants::SYSTEM_PROGRAM_ID;
use pinocchio_token::{ID, state::TokenAccount};

#[repr(C)]
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct InitializeInstructionData {
    pub fee_rate: u16,
}

impl InitializeInstructionData {
    pub const LEN: usize = core::mem::size_of::<Self>();
}

pub fn process_initialize(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction: &[u8],
) -> ProgramResult {
    let [
        authority,
        pool,
        token_a,
        token_b,
        lp_mint,
        vault_a,
        vault_b,
        system_program,
        token_program,
        _remaining @ ..,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !pool.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if token_a.key() == token_b.key() {
        return Err(ProgramError::InvalidArgument);
    }

    if !lp_mint.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if system_program.key() != &SYSTEM_PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    if token_program.key() != &ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let vault_a_account = TokenAccount::from_account_info(vault_a)?;

    let vault_b_account = TokenAccount::from_account_info(vault_b)?;

    if vault_a_account.mint() != token_a.key() {
        return Err(ProgramError::InvalidAccountData);
    }
    if vault_a_account.amount() != 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if vault_b_account.mint() != token_b.key() {
        return Err(ProgramError::InvalidAccountData);
    }
    if vault_b_account.amount() != 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    Ok(())
}
