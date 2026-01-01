use pinocchio::{ProgramResult, account_info::AccountInfo, pubkey::Pubkey};

pub fn process_add_liquidity(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction: &[u8],
) -> ProgramResult {
    Ok(())
}
