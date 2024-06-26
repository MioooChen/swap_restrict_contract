use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
    program_error::ProgramError,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let clock_sysvar = next_account_info(account_info_iter)?;
    let current_time = Clock::from_account_info(clock_sysvar)?.unix_timestamp;

    
    let restriction_period: i64 = 8 * 60 * 60; // 8小时的秒数

   
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let start_time = i64::from_le_bytes(instruction_data[0..8].try_into().unwrap());

    
    let user_pubkey = next_account_info(account_info_iter)?;
    let owner_pubkey = next_account_info(account_info_iter)?;

    
    if current_time < start_time + restriction_period && user_pubkey.key != owner_pubkey.key {
        msg!("8小时内禁止除合约所有者外的所有交易");
        return Err(ProgramError::Custom(0));
    }

    
    msg!("执行合约允许的交易逻辑");
    Ok(())
}
