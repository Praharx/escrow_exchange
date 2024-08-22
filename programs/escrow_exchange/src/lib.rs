use anchor_lang::prelude::*;

declare_id!("BL6rR3d7HiDaXu4vgxGVnEAD7ntZJpfGvuJregU83Mtf");

#[program]
pub mod escrow_exchange {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
