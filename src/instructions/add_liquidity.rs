use bytemuck::{Pod, Zeroable};
use pinocchio::{
    ProgramResult, account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};
use pinocchio_token::{
    instructions::{MintTo, Transfer},
    state::{Mint, TokenAccount},
};

use super::{
    utils::{create_pool_seed, create_pool_signer, load_pool_data},
    validators::{
        validate_instruction_length, validate_non_zero, validate_pubkey_match, validate_signer,
        validate_token_program,
    },
};
use crate::{helper::integer_sqrt, states::Pool};

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

    validate_signer(user)?;
    validate_token_program(token_program)?;
    validate_instruction_length(instruction, AddLiquidityInstructionData::LEN)?;

    let data = bytemuck::checked::pod_read_unaligned::<AddLiquidityInstructionData>(instruction);

    validate_non_zero(data.amount_a)?;
    validate_non_zero(data.amount_b)?;

    let lp_tokens_to_mint = {
        let mut pool_data = pool.try_borrow_mut_data()?;
        let pool_state = Pool::load_mut(&mut pool_data)?;

        let lp_mint_acc = Mint::from_account_info(lp_mint)?;
        let user_token_a_acc = TokenAccount::from_account_info(user_token_a)?;
        let user_token_b_acc = TokenAccount::from_account_info(user_token_b)?;
        let user_lp_token_acc = TokenAccount::from_account_info(user_lp_token)?;

        validate_pubkey_match(lp_mint.key(), &pool_state.lp_mint)?;
        validate_pubkey_match(vault_a.key(), &pool_state.vault_a)?;
        validate_pubkey_match(vault_b.key(), &pool_state.vault_b)?;
        validate_pubkey_match(user_token_a_acc.mint(), &pool_state.token_a)?;
        validate_pubkey_match(user_token_b_acc.mint(), &pool_state.token_b)?;
        validate_pubkey_match(user_lp_token_acc.mint(), &pool_state.lp_mint)?;

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

        lp_tokens_to_mint
    };

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

    let (pool_bump, token_a, token_b) = load_pool_data(pool)?;
    let binding = [pool_bump];
    let pool_seed = create_pool_seed(&binding, &token_a, &token_b);

    MintTo {
        mint: lp_mint,
        mint_authority: pool,
        account: user_lp_token,
        amount: lp_tokens_to_mint,
    }
    .invoke_signed(&[create_pool_signer(&pool_seed)])?;

    let mut pool_data = pool.try_borrow_mut_data()?;
    let pool_state = Pool::load_mut(&mut pool_data)?;

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
