use pinocchio::{
    ProgramResult, account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};
use pinocchio_token::ID;

pub fn validate_signer(account: &AccountInfo) -> ProgramResult {
    if !account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

pub fn validate_token_program(token_program: &AccountInfo) -> ProgramResult {
    if token_program.key() != &ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

pub fn validate_instruction_length(instruction: &[u8], expected_len: usize) -> ProgramResult {
    if instruction.len() != expected_len {
        return Err(ProgramError::InvalidInstructionData);
    }
    Ok(())
}

pub fn validate_pubkey_match(actual: &Pubkey, expected: &Pubkey) -> ProgramResult {
    if actual != expected {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}

pub fn validate_non_zero(amount: u64) -> ProgramResult {
    if amount == 0 {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}
