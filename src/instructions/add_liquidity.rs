use bytemuck::{Pod, Zeroable};
use pinocchio::{
    ProgramResult, account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};

#[repr(C)]
#[derive(Clone, Debug, Copy, PartialEq, Pod, Zeroable)]
pub struct AddLiquidityInstructionData {
    pub amount_a: u64,      // amount of token a
    pub amount_b: u64,      // amount of token b
    pub min_lp_amount: u64, // slippage
}

impl AddLiquidityInstructionData {
    pub const LEN: usize = core::mem::size_of::<Self>();
}

pub fn process_add_liquidity(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction: &[u8],
) -> ProgramResult {
    let [
        _user,
        _pool,
        _lp_mint,
        _vault_a,
        _vault_b,
        _user_token_a,
        _user_token_b,
        _user_lp_token,
        _token_program,
        _remaining @ ..,
    ] = accounts
    else {
        return Err(ProgramError::InvalidAccountData);
    };

    Ok(())
}
