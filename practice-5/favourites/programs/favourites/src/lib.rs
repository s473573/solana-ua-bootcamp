use anchor_lang::prelude::*;

declare_id!("7wKuKurRWfEcesYa63om4zfZa2sA89BXejgezwWDEYn2");

// Anchor programs always use
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[account]
#[derive(InitSpace)]
pub struct Favourites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,
}

// When people call the set_favourites instruction, they will need to provide the accounts that will
// be modified. This keeps Solana fast!
#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favourites::INIT_SPACE,
        seeds = [b"favourites", user.key().as_ref()],
        bump,
    )]
    pub favourites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>,
}


#[program]
pub mod favourites {
    use super::*;

    // Our instruction handler! It sets the user's favourite number and color
    pub fn set_favourites(context: Context<SetFavourites>, number: u64, color: String) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);
        msg!(
            "User {}'s favourite number is {} and favourite color is: {}",
            user_public_key,
            number,
            color
        );

        context
            .accounts
            .favourites
            .set_inner(Favourites { number, color });
        Ok(())
    }

    // We can also add a get_favourites instruction to get the user's favourite number and color


    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
