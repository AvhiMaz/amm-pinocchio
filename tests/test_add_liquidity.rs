use {
    mollusk_svm::Mollusk,
    solana_sdk::{
        account::{Account, WritableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
    },
};

use amm_pinocchio::constants::{LP_MINT_SEED, POOL_SEED};
use amm_pinocchio::states::Pool;
use mollusk_svm::program;
use spl_token::state::Mint;

#[test]
fn test_add_liquidity_success() {
    let program_id = Pubkey::new_unique();
    let mut mollusk = Mollusk::new(&program_id, "tests/elfs/amm_pinocchio");

    mollusk.add_program(&spl_token::ID, "tests/elfs/spl_token-3.5.0");

    let (token_program, token_account) = (
        spl_token::ID,
        program::create_program_account_loader_v3(&spl_token::ID),
    );

    let user = Pubkey::new_unique();

    // Create mints
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

    // Calculate PDAs
    let (pool_pda, pool_bump) = Pubkey::find_program_address(
        &[POOL_SEED.as_bytes(), token_a.as_ref(), token_b.as_ref()],
        &program_id,
    );

    let (lp_mint, lp_mint_bump) =
        Pubkey::find_program_address(&[LP_MINT_SEED.as_bytes(), pool_pda.as_ref()], &program_id);

    // Create vaults
    let vault_a = Pubkey::new_from_array([0x05; 32]);
    let mut vault_a_account = Account::new(
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
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_a_account.data_as_mut_slice(),
    )
    .unwrap();

    let vault_b = Pubkey::new_from_array([0x09; 32]);
    let mut vault_b_account = Account::new(
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
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_b_account.data_as_mut_slice(),
    )
    .unwrap();

    // Create initialized pool account
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
        reserve_a: 0,
        reserve_b: 0,
        fee_rate: 30,
        bump: pool_bump,
        lp_mint_bump,
        _padding: [0; 4],
    };
    pool_account
        .data_as_mut_slice()
        .copy_from_slice(bytemuck::bytes_of(&pool_state));

    // Create initialized LP mint
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
            supply: 0,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        lp_mint_account.data_as_mut_slice(),
    )
    .unwrap();

    // Create user token accounts
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
            amount: 100_000,
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
            amount: 100_000,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_token_b_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_lp_token = Pubkey::new_from_array([0x0A; 32]);
    let mut user_lp_token_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    Pack::pack(
        spl_token::state::Account {
            mint: lp_mint,
            owner: user,
            amount: 0,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_lp_token_account.data_as_mut_slice(),
    )
    .unwrap();

    let (system_program, _system_account) = program::keyed_account_for_system_program();
    let user_account = Account::new(1_000_000_000, 0, &system_program);

    // Build add_liquidity instruction
    let amount_a: u64 = 50_000;
    let amount_b: u64 = 50_000;
    let min_lp_amount: u64 = 0;

    let mut data = vec![1u8];
    data.extend_from_slice(&amount_a.to_le_bytes());
    data.extend_from_slice(&amount_b.to_le_bytes());
    data.extend_from_slice(&min_lp_amount.to_le_bytes());

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(pool_pda, false),
            AccountMeta::new(lp_mint, false),
            AccountMeta::new(vault_a, false),
            AccountMeta::new(vault_b, false),
            AccountMeta::new(user_token_a, false),
            AccountMeta::new(user_token_b, false),
            AccountMeta::new(user_lp_token, false),
            AccountMeta::new_readonly(token_program, false),
        ],
        data,
    };

    let result = mollusk.process_and_validate_instruction(
        &ix,
        &[
            (user, user_account),
            (pool_pda, pool_account),
            (lp_mint, lp_mint_account),
            (vault_a, vault_a_account),
            (vault_b, vault_b_account),
            (user_token_a, user_token_a_account),
            (user_token_b, user_token_b_account),
            (user_lp_token, user_lp_token_account),
            (token_a, mint_a_account),
            (token_b, mint_b_account),
            (token_program, token_account),
        ],
        &[mollusk_svm::result::Check::success()],
    );

    // Verify the results
    let user_token_a_after: spl_token::state::Account =
        Pack::unpack(&result.get_account(&user_token_a).unwrap().data).unwrap();
    let user_token_b_after: spl_token::state::Account =
        Pack::unpack(&result.get_account(&user_token_b).unwrap().data).unwrap();
    let user_lp_token_after: spl_token::state::Account =
        Pack::unpack(&result.get_account(&user_lp_token).unwrap().data).unwrap();
    let vault_a_after: spl_token::state::Account =
        Pack::unpack(&result.get_account(&vault_a).unwrap().data).unwrap();
    let vault_b_after: spl_token::state::Account =
        Pack::unpack(&result.get_account(&vault_b).unwrap().data).unwrap();
    let lp_mint_after: spl_token::state::Mint =
        Pack::unpack(&result.get_account(&lp_mint).unwrap().data).unwrap();

    // User should have 50,000 tokens left in each account
    assert_eq!(user_token_a_after.amount, 50_000);
    assert_eq!(user_token_b_after.amount, 50_000);

    // Vaults should have 50,000 tokens each
    assert_eq!(vault_a_after.amount, 50_000);
    assert_eq!(vault_b_after.amount, 50_000);

    // LP tokens minted should be sqrt(50_000 * 50_000) = 50_000
    assert_eq!(user_lp_token_after.amount, 50_000);
    assert_eq!(lp_mint_after.supply, 50_000);

    // Verify pool state was updated
    let pool_after = bytemuck::from_bytes::<Pool>(&result.get_account(&pool_pda).unwrap().data);
    assert_eq!(pool_after.reserve_a, 50_000);
    assert_eq!(pool_after.reserve_b, 50_000);
}
