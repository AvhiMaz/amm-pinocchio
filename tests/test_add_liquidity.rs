use amm_pinocchio::constants::POOL_SEED;
use mollusk_svm::{Mollusk, program};
use solana_sdk::{
    account::{Account, WritableAccount},
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::Mint;

#[test]
fn test_add_liquidity_success() {
    let program_id = Pubkey::new_unique();
    let mut mollusk = Mollusk::new(&program_id, "tests/elfs/amm_pinocchio");
    let (_system_program, _system_account) = program::keyed_account_for_system_program();

    mollusk.add_program(&spl_token::ID, "tests/elfs/spl_token-3.5.0");

    let user = Pubkey::new_unique();

    let (token_program, _token_account) = (
        spl_token::ID,
        program::create_program_account_loader_v3(&spl_token::ID),
    );

    let token_a = Pubkey::new_from_array([0x03; 32]);
    let mut token_a_acc = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    Pack::pack(
        Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        token_a_acc.data_as_mut_slice(),
    )
    .unwrap();

    let token_b = Pubkey::new_from_array([0x02; 32]);
    let mut token_b_acc = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    Pack::pack(
        Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        token_b_acc.data_as_mut_slice(),
    )
    .unwrap();

    let _user_token_a = Pubkey::new_from_array([0x07; 32]);
    let mut user_token_a_acc = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );

    Pack::pack(
        spl_token::state::Account {
            mint: token_a,
            owner: user,
            amount: 100_000,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            close_authority: COption::None,
            delegated_amount: 0,
        },
        user_token_a_acc.data_as_mut_slice(),
    )
    .unwrap();
    let _user_token_b = Pubkey::new_from_array([0x08; 32]);
    let mut user_token_b_acc = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );

    Pack::pack(
        spl_token::state::Account {
            mint: token_b,
            owner: user,
            amount: 100_000_000,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            close_authority: COption::None,
            delegated_amount: 0,
        },
        user_token_b_acc.data_as_mut_slice(),
    )
    .unwrap();

    let (pool_pda, pool_bump) = Pubkey::find_program_address(
        &[POOL_SEED.as_bytes(), token_a.as_ref(), token_b.as_ref()],
        &program_id,
    );

    let vault_a = Pubkey::new_from_array([0x05; 32]);

    let mut vault_a_token = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );

    Pack::pack(
        spl_token::state::Account {
            mint: token_a,
            owner: pool_pda,
            amount: 0,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            close_authority: COption::None,
            delegated_amount: 0,
        },
        vault_a_token.data_as_mut_slice(),
    )
    .unwrap();

    let vault_b = Pubkey::new_from_array([0x09; 32]);

    let mut vault_b_token = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );

    Pack::pack(
        spl_token::state::Account {
            mint: token_b,
            owner: pool_pda,
            amount: 0,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            close_authority: COption::None,
            delegated_amount: 0,
        },
        vault_b_token.data_as_mut_slice(),
    )
    .unwrap();
}
