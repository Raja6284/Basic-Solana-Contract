use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{AccountInfo,next_account_info},
    entrypoint::ProgramResult,entrypoint,
    pubkey::Pubkey,
    msg,
};


#[derive(BorshDeserialize, BorshSerialize)]
enum InstructionType{
    Increment(u32),
    Decrement(u32),
    Reset,
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter{
    count:u32,
}


entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
)->ProgramResult{
    let acc: &AccountInfo<'_> = next_account_info(&mut accounts.iter())?;

    let instruction_type:InstructionType = InstructionType::try_from_slice(instruction_data)?;

    let mut counter_data:Counter = Counter::try_from_slice(&acc.data.borrow())?;
    

    match instruction_type{
       InstructionType::Increment(value)=>{
           msg!("Incrementing Counter");
           counter_data.count += value;
           msg!("Counter incremented by {}", value);
        },
        InstructionType::Decrement(value)=>{
           msg!("Decrementing Counter");
           counter_data.count -= value;
           msg!("Counter decremented by {}", value);
        },
        InstructionType::Reset=>{
            msg!("Resetting Counter");
            counter_data.count = 0;
            msg!("Counter reset to 0");
        }
    }
    
    //  counter_data.serialize(writer: &mut *acc.data.borrow_mut());
    counter_data.serialize(&mut &mut acc.data.borrow_mut()[..])?;

    msg!("contract executed successfully");
    Ok(())
} 