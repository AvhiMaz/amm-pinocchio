use amm_pinocchio::instructions::initializer::InitializeInstructionData;
use {
    mollusk_svm::Mollusk,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
};
#[cfg(test)]
mod tests {
    use std::vec;

    use amm_pinocchio::constants::{LP_MINT_SEED, POOL_SEED};
    use mollusk_svm::program;
    use solana_sdk::{
        account::{Account, WritableAccount},
        program_option::COption,
        program_pack::Pack,
    };
    use spl_token::state::Mint;

    use super::*;

    fn bytemuck_helper(data: &[u8]) -> Result<&InitializeInstructionData, bytemuck::PodCastError> {
        let ix_data: &InitializeInstructionData =
            bytemuck::try_from_bytes::<InitializeInstructionData>(data)?;

        Ok(ix_data)
    }

    #[test]
    fn test_instruction_data_parsing() {
        let cases = [
            ([100, 0, 254, 255], 100u16, 254u8, 255u8),
            ([244, 1, 253, 252], 500u16, 253u8, 252u8),
            ([16, 39, 251, 250], 10_000u16, 251u8, 250u8),
        ];

        for (input, expected_fee, expected_pool_bump, expected_lp_bump) in cases {
            let ix_data = bytemuck_helper(&input).unwrap();
            assert_eq!(ix_data.fee_rate, expected_fee);
            assert_eq!(ix_data.pool_bump, expected_pool_bump);
            assert_eq!(ix_data.lp_mint_bump, expected_lp_bump);
        }
    }

    #[test]
    fn test_instruction_data_parsing_with_insufficient_data() {
        let data = [30];
        let result = bytemuck_helper(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parsing_with_extra_bytes() {
        let data = [50, 0, 253, 254, 255, 255];
        let result = bytemuck_helper(&data[..4]);
        assert!(result.is_ok());
        let ix_data = result.unwrap();
        assert_eq!(ix_data.fee_rate, 50);
        assert_eq!(ix_data.pool_bump, 253);
        assert_eq!(ix_data.lp_mint_bump, 254);
    }
    #[test]
    fn test_initialize_success() {
        let program_id = Pubkey::new_unique();
        let mut mollusk = Mollusk::new(&program_id, "tests/elfs/amm_pinocchio");
        let (system_program, system_account) = program::keyed_account_for_system_program();

        mollusk.add_program(&spl_token::ID, "tests/elfs/spl_token-3.5.0");

        let authority = Pubkey::new_unique();

        let (token_program, token_account) = (
            spl_token::ID,
            program::create_program_account_loader_v3(&spl_token::ID),
        );
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
                supply: 200_000_000,
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

        let vault_a = Pubkey::new_from_array([0x05; 32]);
        let mut vault_a_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
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

        let vault_b = Pubkey::new_from_array([0x06; 32]);
        let mut vault_b_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
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

        let (lp_mint, lp_mint_bump) = Pubkey::find_program_address(
            &[LP_MINT_SEED.as_bytes(), pool_pda.as_ref()],
            &program_id,
        );

        let fee_rate: u16 = 30;

        let mut data = vec![0u8];
        data.extend_from_slice(&fee_rate.to_le_bytes());
        data.push(pool_bump);
        data.push(lp_mint_bump);

        let ix = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(authority, true),
                AccountMeta::new(pool_pda, false),
                AccountMeta::new(token_a, false),
                AccountMeta::new(token_b, false),
                AccountMeta::new(lp_mint, false),
                AccountMeta::new(vault_a, false),
                AccountMeta::new(vault_b, false),
                AccountMeta::new_readonly(system_program, false),
                AccountMeta::new_readonly(token_program, false),
            ],
            data,
        };

        let authority_account = Account::new(1_000_000_000, 0, &system_program);
        let pool_account = Account::new(0, 0, &system_program);
        let lp_mint_account = Account::new(0, 0, &system_program);

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (authority, authority_account),
                (pool_pda, pool_account),
                (token_a, mint_a_account),
                (token_b, mint_b_account),
                (lp_mint, lp_mint_account),
                (vault_a, vault_a_account),
                (vault_b, vault_b_account),
                (system_program, system_account),
                (token_program, token_account),
            ],
            &[mollusk_svm::result::Check::success()],
        );

        assert!(!result.program_result.is_err());
    }
}
