use anchor_lang::prelude::*;

declare_id!("FH7gdsLLrJiabXW9VtJcCLAFu2AVRjx4tang6Bep7Aqd");

#[program]
pub mod solana_coinflip {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_balance: u64) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.balance = initial_balance;
        Ok(())
    }

    pub fn flip(ctx: Context<Flip>, bet: u64, guess: bool) -> Result<()> {
        let player = &mut ctx.accounts.player;

        // Simple coin flip logic
        let outcome = Clock::get()?.unix_timestamp % 2 == 0; // true for heads, false for tails

        if (outcome == guess) {
            player.balance += bet; // Win
        } else {
            player.balance -= bet; // Lose
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Flip<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,
}

#[account]
pub struct Player {
    pub balance: u64,
}
