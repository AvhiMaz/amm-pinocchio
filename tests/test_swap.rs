use amm_pinocchio::{
    constants::{LP_MINT_SEED, POOL_SEED},
    states::Pool,
};

use mollusk_svm::{Mollusk, program};
use solana_sdk::{
    account::{Account, WritableAccount},
    instruction::{AccountMeta, Instruction},
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::Mint;

#[test]
fn test_swap_success() {
    let program_id = Pubkey::new_unique();
    let mut mollusk = Mollusk::new(&program_id, "tests/elfs/amm_pinocchio");

    mollusk.add_program(&spl_token::ID, "tests/elfs/spl_token-3.5.0");

    let (token_program, token_account) = (
        spl_token::ID,
        program::create_program_account_loader_v3(&spl_token::ID),
    );

    let user = Pubkey::new_unique();

    let token_a = Pubkey::new_from_array([0x03; 32]);
    let mut mint_a_account = Account::new(
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
        mint_a_account.data_as_mut_slice(),
    )
    .unwrap();

    let token_b = Pubkey::new_from_array([0x02; 32]);
    let mut mint_b_account = Account::new(
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
        mint_b_account.data_as_mut_slice(),
    )
    .unwrap();

    let (pool_pda, pool_bump) = Pubkey::find_program_address(
        &[POOL_SEED.as_bytes(), token_a.as_ref(), token_b.as_ref()],
        &program_id,
    );

    let (lp_mint, lp_mint_bump) =
        Pubkey::find_program_address(&[LP_MINT_SEED.as_bytes(), pool_pda.as_ref()], &program_id);

    let vault_a = Pubkey::new_from_array([0x05; 32]);
    let mut vault_a_acc = Account::new(
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
            amount: 100_000,
            delegate: COption::None,
            is_native: COption::None,
            close_authority: COption::None,
            delegated_amount: 0,
            state: spl_token::state::AccountState::Initialized,
        },
        vault_a_acc.data_as_mut_slice(),
    )
    .unwrap();

    let vault_b = Pubkey::new_from_array([0x09; 32]);
    let mut vault_b_acc = Account::new(
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
            amount: 100_000,
            delegate: COption::None,
            is_native: COption::None,
            close_authority: COption::None,
            delegated_amount: 0,
            state: spl_token::state::AccountState::Initialized,
        },
        vault_b_acc.data_as_mut_slice(),
    )
    .unwrap();
    let mut pool_account = Account::new(
        mollusk.sysvars.rent.minimum_balance(Pool::LEN),
        Pool::LEN,
        &program_id,
    );

    let pool_state = Pool {
        authority: pool_pda.to_bytes(),
        token_a: token_a.to_bytes(),
        token_b: token_b.to_bytes(),
        lp_mint: lp_mint.to_bytes(),
        vault_a: vault_a.to_bytes(),
        vault_b: vault_b.to_bytes(),
        reserve_a: 100_000,
        reserve_b: 100_000,
        fee_rate: 30,
        bump: pool_bump,
        lp_mint_bump,
        _padding: [0; 4],
    };
    pool_account
        .data_as_mut_slice()
        .copy_from_slice(bytemuck::bytes_of(&pool_state));

    let mut lp_mint_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    Pack::pack(
        Mint {
            mint_authority: COption::Some(pool_pda),
            supply: 100_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        lp_mint_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_token_a = Pubkey::new_from_array([0x07; 32]);
    let mut user_token_a_account = Account::new(
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
            amount: 900_000,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_token_a_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_token_b = Pubkey::new_from_array([0x08; 32]);
    let mut user_token_b_account = Account::new(
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
            amount: 900_000,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_token_b_account.data_as_mut_slice(),
    )
    .unwrap();

    let (system_program, _system_account) = program::keyed_account_for_system_program();
    let user_account = Account::new(1_000_000_000, 0, &system_program);

    let amount_in: u64 = 10_000;
    let min_amount_out: u64 = 9_000;

    let mut data = vec![2u8];
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&min_amount_out.to_le_bytes());

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(pool_pda, false),
            AccountMeta::new_readonly(token_a, false),
            AccountMeta::new_readonly(token_b, false),
            AccountMeta::new(vault_a, false),
            AccountMeta::new(vault_b, false),
            AccountMeta::new(user_token_a, false),
            AccountMeta::new(user_token_b, false),
            AccountMeta::new_readonly(token_program, false),
        ],
        data,
    };

    let result = mollusk.process_and_validate_instruction(
        &ix,
        &[
            (user, user_account),
            (pool_pda, pool_account),
            (token_a, mint_a_account),
            (token_b, mint_b_account),
            (vault_a, vault_a_acc),
            (vault_b, vault_b_acc),
            (user_token_a, user_token_a_account),
            (user_token_b, user_token_b_account),
            (token_program, token_account),
        ],
        &[mollusk_svm::result::Check::success()],
    );

    assert!(!result.program_result.is_err());
}
