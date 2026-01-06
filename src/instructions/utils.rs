use crate::{constants::POOL_SEED, states::Pool};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn create_pool_seed<'a>(
    pool_bump: &'a [u8; 1],
    token_a: &'a Pubkey,
    token_b: &'a Pubkey,
) -> [Seed<'a>; 4] {
    [
        Seed::from(POOL_SEED.as_bytes()),
        Seed::from(token_a.as_ref()),
        Seed::from(token_b.as_ref()),
        Seed::from(pool_bump.as_ref()),
    ]
}

pub fn load_pool_data(pool: &AccountInfo) -> Result<(u8, Pubkey, Pubkey), ProgramError> {
    let pool_data = pool.try_borrow_data()?;
    let pool_state = Pool::load(&pool_data)?;
    Ok((
        pool_state.bump,
        Pubkey::from(pool_state.token_a),
        Pubkey::from(pool_state.token_b),
    ))
}

pub fn create_pool_signer<'a, 'b>(pool_seed: &'a [Seed<'b>; 4]) -> Signer<'a, 'b> {
    Signer::from(&pool_seed[..])
}
