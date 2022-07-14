use anchor_lang::prelude::*;

declare_id!("2MVTdyvCHu9kdLTAr1YGyZhMALZ7YsCc7TuNpH9pVWL3");

#[program]
pub mod gm_anchor {
    use super::*;

    pub fn execute(ctx: Context<Execute>, name: String) -> Result<()> {
        let gm_account = &mut ctx.accounts.gm_account;
        gm_account.name = name;
        msg!("Hello, {}", gm_account.name);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub gm_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GreetingAccount {
    pub name: String,
}
