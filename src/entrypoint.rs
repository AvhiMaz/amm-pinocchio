#![allow(unexpected_cfgs)]

use pinocchio::{ProgramResult, default_panic_handler, no_allocator, program_entrypoint};

program_entrypoint!(process_instruction);

no_allocator!();

default_panic_handler!();

fn process_instruction(
    _program_id: &pinocchio::pubkey::Pubkey,
    _accounts: &[pinocchio::account_info::AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
