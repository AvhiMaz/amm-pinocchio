use bytemuck::{Pod, Zeroable};
use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use pinocchio_token::{
    ID,
    instructions::{Burn, Transfer},
    state::{Mint, TokenAccount},
};

use crate::states::Pool;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct WithdrawInstructionData {
    pub amount_in: u64,
    pub min_amount_a: u64,
    pub min_amount_b: u64,
}

impl WithdrawInstructionData {
    pub const LEN: usize = core::mem::size_of::<Self>();
}

pub fn process_withdraw(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions: &[u8],
) -> ProgramResult {
    let [
        user,
        pool,
        lp_mint,
        vault_a,
        vault_b,
        user_lp_token,
        user_token_a,
        user_token_b,
        token_program,
        _remaining @ ..,
    ] = accounts
    else {
        return Err(ProgramError::InvalidAccountData);
    };

    if !user.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if token_program.key() != &ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    if instructions.len() != self::WithdrawInstructionData::LEN {
        return Err(ProgramError::InvalidInstructionData);
    }

    let data: WithdrawInstructionData = bytemuck::checked::pod_read_unaligned(instructions);

    if data.amount_in == 0 {
        return Err(ProgramError::InvalidAccountData);
    }

    let (amount_a_out, amount_b_out) = {
        let pool = pool.try_borrow_data()?;
        let pool_state = Pool::load(&pool)?;

        let lp_mint_acc = Mint::from_account_info(lp_mint)?;

        let user_lp_acc = TokenAccount::from_account_info(user_lp_token)?;
        let user_token_a_acc = TokenAccount::from_account_info(user_token_a)?;
        let user_token_b_acc = TokenAccount::from_account_info(user_token_b)?;

        if user_lp_acc.mint() != &pool_state.lp_mint {
            return Err(ProgramError::InvalidAccountData);
        }

        if user_token_a_acc.mint() != &pool_state.token_a {
            return Err(ProgramError::InvalidAccountData);
        }

        if user_token_b_acc.mint() != &pool_state.token_b {
            return Err(ProgramError::InvalidAccountData);
        }

        if user_lp_acc.owner() != user.key() {
            return Err(ProgramError::InvalidAccountData);
        }
        if user_token_a_acc.owner() != user.key() {
            return Err(ProgramError::InvalidAccountData);
        }
        if user_token_b_acc.owner() != user.key() {
            return Err(ProgramError::InvalidAccountData);
        }

        if lp_mint.key() != &pool_state.lp_mint {
            return Err(ProgramError::InvalidAccountData);
        }
        if vault_a.key() != &pool_state.vault_a {
            return Err(ProgramError::InvalidAccountData);
        }
        if vault_b.key() != &pool_state.vault_b {
            return Err(ProgramError::InvalidAccountData);
        }

        let total_supply = lp_mint_acc.supply();
        if total_supply == 0 {
            return Err(ProgramError::InvalidAccountData);
        }

        if user_lp_acc.amount() < data.amount_in {
            return Err(ProgramError::InsufficientFunds);
        }

        let amount_a_out = data
            .amount_in
            .checked_mul(pool_state.reserve_a)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(total_supply)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let amount_b_out = data
            .amount_in
            .checked_mul(pool_state.reserve_b)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(total_supply)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        if amount_a_out == 0 || amount_b_out == 0 {
            return Err(ProgramError::InvalidAccountData);
        }

        if amount_a_out < data.min_amount_a {
            return Err(ProgramError::InsufficientFunds);
        }
        if amount_b_out < data.min_amount_b {
            return Err(ProgramError::InsufficientFunds);
        }

        if pool_state.reserve_a < amount_a_out {
            return Err(ProgramError::InsufficientFunds);
        }
        if pool_state.reserve_b < amount_b_out {
            return Err(ProgramError::InsufficientFunds);
        }
        (amount_a_out, amount_b_out)
    };

    Burn {
        mint: lp_mint,
        account: user_lp_token,
        authority: user,
        amount: data.amount_in,
    }
    .invoke()?;

    let (pool_bump, token_a, token_b) = {
        let pool_data = pool.try_borrow_data()?;
        let pool_state = Pool::load(&pool_data)?;
        (pool_state.bump, pool_state.token_a, pool_state.token_b)
    };

    let binding = [pool_bump];
    let pool_seed = [
        Seed::from(crate::constants::POOL_SEED.as_bytes()),
        Seed::from(token_a.as_ref()),
        Seed::from(token_b.as_ref()),
        Seed::from(&binding),
    ];

    Transfer {
        from: vault_a,
        to: user_token_a,
        authority: pool,
        amount: amount_a_out,
    }
    .invoke_signed(&[Signer::from(&pool_seed[..])])?;

    Transfer {
        from: vault_b,
        to: user_token_b,
        authority: pool,
        amount: amount_b_out,
    }
    .invoke_signed(&[Signer::from(&pool_seed[..])])?;

    let mut pool_data = pool.try_borrow_mut_data()?;
    let pool_state = Pool::load_mut(&mut pool_data)?;

    pool_state.reserve_a = pool_state
        .reserve_a
        .checked_sub(amount_a_out)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    pool_state.reserve_b = pool_state
        .reserve_b
        .checked_sub(amount_b_out)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    Ok(())
}
