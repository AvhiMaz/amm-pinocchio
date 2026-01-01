#![allow(unexpected_cfgs)]

use pinocchio::{
    ProgramResult, account_info::AccountInfo, default_panic_handler, no_allocator,
    program_entrypoint, program_error::ProgramError, pubkey::Pubkey,
};

use crate::instructions::{add_liquidity::process_add_liquidity, initializer::process_initialize};

program_entrypoint!(process_instruction);

no_allocator!();

default_panic_handler!();

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((0, rest)) => process_initialize(program_id, accounts, rest),
        Some((1, rest)) => process_add_liquidity(program_id, accounts, rest),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
