use bytemuck::{Pod, Zeroable, checked::try_from_bytes};
use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use pinocchio_token::{
    ID,
    instructions::{MintTo, Transfer},
    state::{Mint, TokenAccount},
};

use crate::{constants::LP_MINT_SEED, helper::integer_sqrt, states::Pool};

#[repr(C)]
#[derive(Clone, Debug, Copy, PartialEq, Pod, Zeroable)]
pub struct AddLiquidityInstructionData {
    pub amount_a: u64,
    pub amount_b: u64,
    pub min_lp_amount: u64,
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

    let data = try_from_bytes::<AddLiquidityInstructionData>(instruction)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    if data.amount_a == 0 || data.amount_b == 0 {
        return Err(ProgramError::InvalidAccountData);
    };

    if token_program.key() != &ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut pool_data = pool.try_borrow_mut_data()?;
    let pool_state = Pool::load_mut(&mut pool_data)?;

    let lp_mint_acc = Mint::from_account_info(lp_mint)?;
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

    let lp_tokens_to_mint = if pool_state.reserve_a == 0 && pool_state.reserve_b == 0 {
        integer_sqrt(
            data.amount_a
                .checked_mul(data.amount_b)
                .ok_or(ProgramError::ArithmeticOverflow)?,
        )
    } else {
        let a = data
            .amount_a
            .checked_mul(total_lp_supply)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(pool_state.reserve_a)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let b = data
            .amount_b
            .checked_mul(total_lp_supply)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(pool_state.reserve_b)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        a.min(b)
    };

    if lp_tokens_to_mint < data.min_lp_amount {
        return Err(ProgramError::InsufficientFunds);
    }

    Transfer {
        from: user_token_a,
        to: vault_a,
        authority: user,
        amount: data.amount_a,
    }
    .invoke()?;

    Transfer {
        from: user_token_b,
        to: vault_b,
        authority: user,
        amount: data.amount_b,
    }
    .invoke()?;

    let binding = [pool_state.lp_mint_bump];
    let lp_mint_seed = [
        Seed::from(LP_MINT_SEED.as_bytes()),
        Seed::from(pool.key().as_ref()),
        Seed::from(&binding),
    ];

    MintTo {
        mint: lp_mint,
        mint_authority: pool,
        account: user_lp_token,
        amount: lp_tokens_to_mint,
    }
    .invoke_signed(&[Signer::from(&lp_mint_seed[..])])?;

    pool_state.reserve_a = pool_state
        .reserve_a
        .checked_add(data.amount_a)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    pool_state.reserve_b = pool_state
        .reserve_b
        .checked_add(data.amount_b)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    Ok(())
}
