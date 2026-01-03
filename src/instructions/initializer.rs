use bytemuck::{Pod, Zeroable};
use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_system::instructions::CreateAccount;

use crate::{
    constants::{LP_MINT_SEED, POOL_SEED, SYSTEM_PROGRAM_ID},
    states::Pool,
};
use pinocchio_token::{
    ID,
    instructions::InitializeMint2,
    state::{Mint, TokenAccount},
};

#[repr(C)]
#[derive(Clone, Debug, Copy, PartialEq, Pod, Zeroable)]
pub struct InitializeInstructionData {
    pub fee_rate: u16,
    pub pool_bump: u8,
    pub lp_mint_bump: u8,
}

impl InitializeInstructionData {
    pub const LEN: usize = core::mem::size_of::<Self>();
}

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction: &[u8],
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

    if instruction.len() != InitializeInstructionData::LEN {
        return Err(ProgramError::InvalidInstructionData);
    }

    let data = bytemuck::checked::pod_read_unaligned::<InitializeInstructionData>(instruction);

    //  - 1 basis point = 0.01%
    //  - 10000 basis points = 100%
    if data.fee_rate > 10000 {
        return Err(ProgramError::InvalidArgument);
    }

    let pool_pda = pinocchio::pubkey::create_program_address(
        &[
            POOL_SEED.as_bytes(),
            token_a.key().as_ref(),
            token_b.key().as_ref(),
            &[data.pool_bump],
        ],
        program_id,
    )?;

    if pool.key() != &pool_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    let lp_mint_pda = pinocchio::pubkey::create_program_address(
        &[
            LP_MINT_SEED.as_bytes(),
            pool.key().as_ref(),
            &[data.lp_mint_bump],
        ],
        program_id,
    )?;

    if lp_mint.key() != &lp_mint_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    let rent = Rent::get()?;

    let binding = [data.pool_bump];
    let pool_seed = [
        Seed::from(POOL_SEED.as_bytes()),
        Seed::from(token_a.key().as_ref()),
        Seed::from(token_b.key().as_ref()),
        Seed::from(&binding),
    ];

    let pool_seed_signer = Signer::from(&pool_seed[..]);

    (CreateAccount {
        from: authority,
        to: pool,
        space: Pool::LEN as u64,
        lamports: rent.minimum_balance(Pool::LEN),
        owner: program_id,
    })
    .invoke_signed(&[pool_seed_signer])?;

    let mut pool_data = pool.try_borrow_mut_data()?;
    let pool_state = Pool::load_mut(&mut pool_data)?;

    pool_state.set_inner_full(Pool {
        authority: *pool.key(),
        token_a: *token_a.key(),
        token_b: *token_b.key(),
        lp_mint: *lp_mint.key(),
        vault_a: *vault_a.key(),
        vault_b: *vault_b.key(),
        reserve_a: 0,
        reserve_b: 0,
        fee_rate: data.fee_rate,
        bump: data.pool_bump,
        lp_mint_bump: data.lp_mint_bump,
        _padding: [0; 4],
    });

    let binding = [data.lp_mint_bump];
    let lp_mint_seed = [
        Seed::from(LP_MINT_SEED.as_bytes()),
        Seed::from(pool.key().as_ref()),
        Seed::from(&binding),
    ];

    (CreateAccount {
        from: authority,
        to: lp_mint,
        space: Mint::LEN as u64,
        lamports: rent.minimum_balance(Mint::LEN),
        owner: &ID,
    })
    .invoke_signed(&[Signer::from(&lp_mint_seed[..])])?;

    InitializeMint2 {
        mint: lp_mint,
        decimals: 6,
        mint_authority: pool.key(),
        freeze_authority: None,
    }
    .invoke_signed(&[Signer::from(&lp_mint_seed[..])])?;

    Ok(())
}
