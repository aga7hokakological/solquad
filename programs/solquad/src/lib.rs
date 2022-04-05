use anchor_lang::prelude::*;
use anchor_spl::token::{self, SetAuthority, TokenAccount, Transfer, Token};
use spl_token::instruction::AuthorityType;
// use anchor_lang::system_program::Transfer;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod solquad {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    // pub fn create_project(ctx: Context<CreateProject>, project: ProjectAccount) -> Result<()> {
    //     ctx.accounts.project_account.project_owner_key = *ctx.accounts.project_owner.key;

    //     Ok(())
    // }

    pub fn create_pool(ctx: Context<CreatePool>, amount: u64, start: i64, end: i64) -> Result<()> {
        ctx.accounts.pool_account.pool_owner_key = *ctx.accounts.pool_owner.key;
        ctx.accounts
            .pool_account
            .pool_token = *ctx
                .accounts
                .pool_token_account
                .to_account_info()
                .key;
        ctx.accounts.pool_account.pool_amount = amount;
        ctx.accounts.pool_account.start_time = start;
        ctx.accounts.pool_account.end_time = end;
        Ok(())
    }

    pub fn add_projects_to_pool(ctx: Context<AddProjectsToPoolAccount>, name: String) -> Result<()> {
        let mut id = 1;

        let pool_account = &mut ctx.accounts.pool_account;
        pool_account.pool_owner_key = *ctx.accounts.pool_owner.key;
        // ctx.accounts.pool_account.projects.project_owner_key = &ctx.accounts.project_owner.key;

        // ctx.accounts.pool_account.project_owner

        let project = ProjectAccount {
            project_id: id,
            project_owner_key: *ctx.accounts.project_owner.key,
            project_name: name,
            votes_count: 0,
            vote_amount: 0,
        };

        pool_account.projects.push(project);
        pool_account.total_projects += 1;
        id += 1;

        Ok(())
    }

    pub fn vote_for_project(ctx: Context<VoteForProject>, vote_for: u32, amount: u64) -> Result<()> {
        let pool_account = &mut ctx.accounts.pool_account;
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.voter = *ctx.accounts.voter.key;

        if pool_account.start_time > Clock::get().unwrap().unix_timestamp && pool_account.end_time > Clock::get().unwrap().unix_timestamp {
            let index = pool_account.projects.iter().position(|x| x.project_id == vote_for).unwrap();
            pool_account.projects[index].votes_count += 1;

            // let ix = anchor_lang::solana_program::system_instruction::transfer(
            //     &vote_account.voter.key(),
            //     &pool_account.projects[index].project_owner_key.key(),
            //     amount,
            // );

            // anchor_lang::solana_program::program::invoke(
            //     &ix,
            //     &[
            //         vote_account.voter.to_account_info(),
            //         pool_account.projects[index].project_owner_key.to_account_info(),
            //     ],
            // );
            pool_account.projects[index].vote_amount += amount;
        }

        Ok(())
    }

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts
            .escrow_account
            .initializer_deposit_token_account = *ctx
            .accounts
            .initializer_deposit_token_account
            .to_account_info()
            .key;
        ctx.accounts
            .escrow_account
            .initializer_receive_token_account = *ctx
            .accounts
            .initializer_receive_token_account
            .to_account_info()
            .key;
        ctx.accounts.escrow_account.initializer_amount = initializer_amount;
        ctx.accounts.escrow_account.taker_amount = taker_amount;

        let (pda, _bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        token::set_authority(ctx.accounts.into(), AuthorityType::AccountOwner, Some(pda))?;
        Ok(())
    }

    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        let (_pda, bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let seeds = &[&ESCROW_PDA_SEED[..], &[bump_seed]];

        token::set_authority(
            ctx.accounts
                .into_set_authority_context()
                .with_signer(&[&seeds[..]]),
            AuthorityType::AccountOwner,
            Some(ctx.accounts.escrow_account.initializer_key),
        )?;

        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        // Transferring from initializer to taker
        let (_pda, bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let seeds = &[&ESCROW_PDA_SEED[..], &[bump_seed]];

        token::transfer(
            ctx.accounts
                .into_transfer_to_taker_context()
                .with_signer(&[&seeds[..]]),
            ctx.accounts.escrow_account.initializer_amount,
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_initializer_context(),
            ctx.accounts.escrow_account.taker_amount,
        )?;

        token::set_authority(
            ctx.accounts
                .into_set_authority_context()
                .with_signer(&[&seeds[..]]),
            AuthorityType::AccountOwner,
            Some(ctx.accounts.escrow_account.initializer_key),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(initializer_amount: u64)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        mut,
        constraint = initializer_deposit_token_account.amount >= initializer_amount
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(init, payer = initializer, space = 8 + EscrowAccount::LEN)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(signer)]
    pub taker: AccountInfo<'info>,
    #[account(mut)]
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pda_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_main_account: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.taker_amount <= taker_deposit_token_account.amount,
        constraint = escrow_account.initializer_deposit_token_account == *pda_deposit_token_account.to_account_info().key,
        constraint = escrow_account.initializer_receive_token_account == *initializer_receive_token_account.to_account_info().key,
        constraint = escrow_account.initializer_key == *initializer_main_account.key,
        close = initializer_main_account
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub pda_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    pub initializer: AccountInfo<'info>,
    #[account(mut)]
    pub pda_deposit_token_account: Account<'info, TokenAccount>,
    pub pda_account: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.initializer_key == *initializer.key,
        constraint = escrow_account.initializer_deposit_token_account == *pda_deposit_token_account.to_account_info().key,
        close = initializer
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub token_program: Program<'info, Token>,
}

// #[derive(Accounts)]
// pub struct CreateProject<'info> {
//     #[account(mut)]
//     pub project_owner: Signer<'info>,
//     #[account(
//         mut,
//         constraint = project_account.project_owner_key == *project_owner.key,
//         // close = project_owner
//     )]
//     pub project_account: Account<'info, ProjectAccount>,
//     pub system_program: Program<'info, System>,
// }

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub pool_owner: Signer<'info>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = pool_account.pool_owner_key == *pool_owner.key,
        // close = project_owner
    )]
    pub pool_account: Account<'info, PoolAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AddProjectsToPoolAccount<'info> {
    #[account(mut)]
    pub pool_owner: Signer<'info>,
    #[account(mut)]
    pub project_owner: Signer<'info>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = pool_account.pool_owner_key == *project_owner.key,
        // close = project_owner
    )]
    pub pool_account: Account<'info, PoolAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct VoteForProject<'info> {
    #[account(mut)]
    pub voter: AccountInfo<'info>,
    pub pool_account: Account<'info, PoolAccount>,
    pub vote_account: Account<'info, VoterAccount>,
}

#[account]
pub struct ProjectAccount {
    pub project_id: u32,
    pub project_owner_key: Pubkey,
    pub project_name: String,
    pub votes_count: u64,
    pub vote_amount: u64,
}

#[account]
pub struct PoolAccount {
    pub pool_id: u32,
    pub pool_owner_key: Pubkey,
    pub pool_token: Pubkey,
    pub pool_amount: u64,
    pub projects: Vec<ProjectAccount>,
    pub total_projects: u64,
    pub start_time: i64,
    pub end_time: i64,
}

#[account]
pub struct VoterAccount {
    pub voter: Pubkey,
    pub voted_to: Pubkey,
}

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
}

impl EscrowAccount {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8;
}

impl<'info> From<&mut InitializeEscrow<'info>>
    for CpiContext<'_, '_, '_, 'info, SetAuthority<'info>>
{
    fn from(accounts: &mut InitializeEscrow<'info>) -> Self {
        let cpi_accounts = SetAuthority {
            account_or_mint: accounts
                .initializer_deposit_token_account
                .to_account_info()
                .clone(),
            current_authority: accounts.initializer.to_account_info().clone(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> CancelEscrow<'info> {
    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.pda_deposit_token_account.to_account_info().clone(),
            current_authority: self.pda_account.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> Exchange<'info> {
    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.pda_deposit_token_account.to_account_info().clone(),
            current_authority: self.pda_account.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> Exchange<'info> {
    fn into_transfer_to_taker_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.pda_deposit_token_account.to_account_info().clone(),
            to: self.taker_receive_token_account.to_account_info().clone(),
            authority: self.pda_account.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> Exchange<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.taker_deposit_token_account.to_account_info().clone(),
            to: self
                .initializer_receive_token_account
                .to_account_info()
                .clone(),
            authority: self.taker.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}