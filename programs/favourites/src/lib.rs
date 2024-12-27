use anchor_lang::prelude::*;

declare_id!("CZtQLmgP3KFVjhEGjAGN34TyycQp8nGsZuyRhrgLEyDQ");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; // bytes

#[program]
pub mod favourites {
    use super::*; // Importing everything from the parent module

    pub fn set_favourite(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        // Instructions called on transactions
        msg!("Greeting from {}", context.accounts.user.key());

        let user_public_key = context.accounts.user.key();
        msg!(
            "User {}'s favorite number is {}, favorite color is {}, and their hobbies are {:?}",
            user_public_key,
            number,
            color,
            hobbies
        );

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)] // Restricts string length to 50
    pub color: String,

    #[max_len(5, 50)] // Restricts vector to 5 elements, each with max length 50
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + 8 + 4 + 50 + 4 + (5 * 4 + 50),
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}