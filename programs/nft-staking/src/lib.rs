use anchor_lang::prelude::*;

declare_id!("5ZsUk5GEfARGJVjcR5UpwW9DTK4fMAx7Gsw1vAN9JmCX");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
