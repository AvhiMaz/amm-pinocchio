use core::cmp::min;

use bytemuck::{Pod, Zeroable};
use pinocchio::{
    ProgramResult, account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};
use pinocchio_token::{
    ID,
    state::{Mint, TokenAccount},
};

use crate::{helper::integer_sqrt, states::Pool};

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
    instruction: &[u8],
) -> ProgramResult {
    let [
        user,
        pool,
        lp_mint,
        vault_a,
        vault_b,
        user_token_a,
        user_token_b,
        user_lp_token,
        token_program,
        _remaining @ ..,
    ] = accounts
    else {
        return Err(ProgramError::InvalidAccountData);
    };

    if !user.is_signer() {
        return Err(ProgramError::IncorrectAuthority);
    }

    if instruction.len() != AddLiquidityInstructionData::LEN {
        return Err(ProgramError::InvalidInstructionData);
    }

    let amount_a = u64::from_le_bytes(
        instruction[0..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    let amount_b = u64::from_le_bytes(
        instruction[8..16]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    let min_lp_amount = u64::from_le_bytes(
        instruction[16..24]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    if amount_a == 0 || amount_b == 0 {
        return Err(ProgramError::InvalidAccountData);
    };

    if token_program.key() != &ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut pool_data = pool.try_borrow_mut_data()?;
    let pool_state = Pool::load_mut(&mut pool_data)?;

    let lp_mint_acc = Mint::from_account_info(lp_mint)?;
    let vault_a_acc = TokenAccount::from_account_info(vault_a)?;
    let vault_b_acc = TokenAccount::from_account_info(vault_b)?;
    let user_token_a_acc = TokenAccount::from_account_info(user_token_a)?;
    let user_token_b_acc = TokenAccount::from_account_info(user_token_b)?;
    let user_lp_token_acc = TokenAccount::from_account_info(user_lp_token)?;

    if lp_mint.key() != &pool_state.lp_mint {
        return Err(ProgramError::InvalidAccountData);
    }

    if vault_a.key() != &pool_state.vault_a {
        return Err(ProgramError::InvalidAccountData);
    }

    if vault_b.key() != &pool_state.vault_b {
        return Err(ProgramError::InvalidAccountData);
    }

    if user_token_a_acc.mint() != &pool_state.token_a {
        return Err(ProgramError::InvalidInstructionData);
    }

    if user_token_b_acc.mint() != &pool_state.token_b {
        return Err(ProgramError::InvalidInstructionData);
    }

    if user_lp_token_acc.mint() != &pool_state.lp_mint {
        return Err(ProgramError::InvalidInstructionData);
    }

    let total_lp_supply = lp_mint_acc.supply();

    let lp_token_mint = if pool_state.reserve_a == 0 && pool_state.reserve_b == 0 {
        integer_sqrt(amount_a * amount_b)
    } else {
        let a = (amount_a * total_lp_supply) / pool_state.reserve_a;
        let b = (amount_b * total_lp_supply) / pool_state.reserve_b;
        min(a, b)
    };

    Ok(())
}
