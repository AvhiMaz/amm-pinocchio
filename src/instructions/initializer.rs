use pinocchio::{ProgramResult, account_info::AccountInfo, pubkey::Pubkey};

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
        remaining @ ..,
    ] = accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    Ok(())
}
