use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self,system_program, sysvar::rent::Rent,};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod basic_crud {
    use super::*;

    pub fn setdata(ctx: Context<DataContext>, _integer : u64, _string : String) -> Result<()> {


        let data_account = &mut ctx.accounts.data_account;

        data_account.integerdata = _integer;
        data_account.stringdata = _string;
        data_account.keydata = ctx.accounts.authority.key();
        data_account.bump = *ctx.bumps.get("data_account").unwrap();
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DataContext<'info>  {
    #[account(
        mut,
    )]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + (4 + 50) + 8 + 32 + 1,
        seeds = [b"data".as_ref(),authority.key().as_ref()], 
        bump
    )]
    pub data_account: Box<Account<'info, DataAccount>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}


#[account]
#[derive(Default)]
pub struct DataAccount {
    integerdata: u64,
    stringdata: String,
    keydata : Pubkey,
    bump: u8
}