use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod notes {
    use super::*;

    // function to create a note
    pub fn create_note(ctx: Context<CreateNote>, content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let user = &mut ctx.accounts.user;

        note.content = content;
        note.user = *user.key; //public key of the user

        Ok(())
    }

    pub fn delete_note(ctx: Context<DeleteNote>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateNote<'info> {
    #[account(
        init, //to initialize
        payer = user, // the payer to store data
        space = 2000 // in bytes(size)

    )]
    pub note : Account<'info, Note>,

    #[account(mut)]
    pub user : Signer<'info>,

    pub system_program : Program<'info, System> // need to be passed in case of account initialization

}

#[derive(Accounts)]
pub struct DeleteNote<'info> {
    #[account(
        mut,
        has_one = user, //has_one is to specify that Account user == Signer user
        close = user // basically signifies close from the user
    )]
    pub note : Account<'info, Note>,

    #[account(mut)]
    pub user : Signer<'info>,

}

#[account]
pub struct Note {
    pub content: String,
    pub user: Pubkey
}


//Note: In Solana everything is an account, the note we create, the signer wallet etc.,