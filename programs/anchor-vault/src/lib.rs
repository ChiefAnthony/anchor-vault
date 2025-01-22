use anchor_lang::prelude::*;

declare_id!("6vErrQ5TcgvAHW3AzM7JwX3QCh79TbhQw7VPPhhfnsr6");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
