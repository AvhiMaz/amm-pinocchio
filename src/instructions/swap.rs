use bytemuck::{Pod, Zeroable};
use pinocchio::{
    ProgramResult, account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use super::{
    utils::{create_pool_seed, create_pool_signer},
    validators::{
        validate_instruction_length, validate_non_zero, validate_signer, validate_token_program,
    },
};
use crate::states::Pool;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SwapInstructionData {
    pub amount_in: u64,
    pub min_amount_out: u64,
}

impl SwapInstructionData {
    pub const LEN: usize = core::mem::size_of::<Self>();
}

pub fn process_swap(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions: &[u8],
) -> ProgramResult {
    let [
        user,
        pool,
        input_mint,
        output_mint,
        input_vault,
        output_vault,
        user_input_account,
        user_output_account,
        token_program,
        _remaining @ ..,
    ] = accounts
    else {
        return Err(ProgramError::InvalidAccountData);
    };

    validate_signer(user)?;
    validate_token_program(token_program)?;
    validate_instruction_length(instructions, SwapInstructionData::LEN)?;

    let data: SwapInstructionData = bytemuck::checked::pod_read_unaligned(instructions);

    validate_non_zero(data.amount_in)?;

    let (amount_out, is_a_to_b, token_a, token_b, pool_bump) = {
        let pool_state = pool.try_borrow_data()?;
        let pool = Pool::load(&pool_state)?;

        let user_input_acc = TokenAccount::from_account_info(user_input_account)?;
        let user_output_acc = TokenAccount::from_account_info(user_output_account)?;

        let (reserve_in, reserve_out, is_a_to_b) =
            if input_mint.key() == &pool.token_a && output_mint.key() == &pool.token_b {
                if input_vault.key() != &pool.vault_a {
                    return Err(ProgramError::InvalidAccountData);
                }

                if output_vault.key() != &pool.vault_b {
                    return Err(ProgramError::InvalidAccountData);
                }
                (pool.reserve_a, pool.reserve_b, true)
            } else if input_mint.key() == &pool.token_b && output_mint.key() == &pool.token_a {
                if input_vault.key() != &pool.vault_b {
                    return Err(ProgramError::InvalidAccountData);
                }

                if output_vault.key() != &pool.vault_a {
                    return Err(ProgramError::InvalidAccountData);
                }
                (pool.reserve_b, pool.reserve_a, false)
            } else {
                return Err(ProgramError::IllegalOwner);
            };

        if user_input_acc.mint() != input_mint.key() {
            return Err(ProgramError::InvalidAccountData);
        }
        if user_output_acc.mint() != output_mint.key() {
            return Err(ProgramError::InvalidAccountData);
        }

        if user_input_acc.owner() != user.key() {
            return Err(ProgramError::InvalidAccountData);
        }
        if user_output_acc.owner() != user.key() {
            return Err(ProgramError::InvalidAccountData);
        }
        let amount_in_with_fee = data
            .amount_in
            .checked_mul(
                10000_u64
                    .checked_sub(pool.fee_rate as u64)
                    .ok_or(ProgramError::ArithmeticOverflow)?,
            )
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(10000)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let amount_out = reserve_out
            .checked_mul(amount_in_with_fee)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(
                reserve_in
                    .checked_add(amount_in_with_fee)
                    .ok_or(ProgramError::ArithmeticOverflow)?,
            )
            .ok_or(ProgramError::ArithmeticOverflow)?;

        if amount_out < data.min_amount_out {
            return Err(ProgramError::InsufficientFunds);
        }

        (amount_out, is_a_to_b, pool.token_a, pool.token_b, pool.bump)
    };

    Transfer {
        from: user_input_account,
        to: input_vault,
        amount: data.amount_in,
        authority: user,
    }
    .invoke()?;

    let binding = [pool_bump];
    let pool_seed = create_pool_seed(&binding, &token_a, &token_b);
    Transfer {
        from: output_vault,
        to: user_output_account,
        amount: amount_out,
        authority: pool,
    }
    .invoke_signed(&[create_pool_signer(&pool_seed)])?;

    let mut pool_data = pool.try_borrow_mut_data()?;
    let pool_state = Pool::load_mut(&mut pool_data)?;

    if is_a_to_b {
        pool_state.reserve_a = pool_state
            .reserve_a
            .checked_add(data.amount_in)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        pool_state.reserve_b = pool_state
            .reserve_b
            .checked_sub(amount_out)
            .ok_or(ProgramError::ArithmeticOverflow)?;
    } else {
        pool_state.reserve_b = pool_state
            .reserve_b
            .checked_add(data.amount_in)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        pool_state.reserve_a = pool_state
            .reserve_a
            .checked_sub(amount_out)
            .ok_or(ProgramError::ArithmeticOverflow)?;
    }

    Ok(())
}
