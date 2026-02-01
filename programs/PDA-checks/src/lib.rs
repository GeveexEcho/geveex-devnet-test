use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("EKSyZrU3ckqaufwTsh58oD5Ku326JddxaA6XygiE5FMK");

#[program]
pub mod vesting_distributor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.team_wallet = ctx.accounts.team.key();
        state.mint = ctx.accounts.mint.key();
        state.last_release_time = Clock::get()?.unix_timestamp;
        state.release_count = 0;
        
        msg!("Vesting State Initialized");
        Ok(())
    }

    pub fn distribute_initial(ctx: Context<DistributeInitial>) -> Result<()> {
        let total_supply: u64 = 100_000 * 10u64.pow(8);
        let user_share = (total_supply * 8) / 100; // 8%
        let team_share = (total_supply * 2) / 100;  // 2%

        // Distribute to 4 users (2% each for simplicity/fairness in "random" logic)
        let share_per_user = user_share / 4;
        let user_accounts = [
            &ctx.accounts.user1, &ctx.accounts.user2, 
            &ctx.accounts.user3, &ctx.accounts.user4
        ];

        let seeds = &[b"distributor".as_ref(), &[ctx.bumps.distributor_vault]];
        let signer = &[&seeds[..]];

        for user_ata in user_accounts {
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.distributor_vault.to_account_info(),
                        to: user_ata.to_account_info(),
                        authority: ctx.accounts.distributor_vault.to_account_info(),
                    },
                    signer,
                ),
                share_per_user,
            )?;
        }

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.distributor_vault.to_account_info(),
                    to: ctx.accounts.team_ata.to_account_info(),
                    authority: ctx.accounts.distributor_vault.to_account_info(),
                },
                signer,
            ),
            team_share,
        )?;

        Ok(())
    }

    pub fn release_vesting(ctx: Context<ReleaseVesting>) -> Result<()> {
        let clock = Clock::get()?;
        let state = &mut ctx.accounts.state;

        // 10 minutes = 600 seconds
        require!(clock.unix_timestamp >= state.last_release_time + 600, VestingError::TooEarly);
        require!(state.release_count < 9, VestingError::FullyVested);

        let total_supply: u64 = 100_000 * 10u64.pow(8);
        let release_amount = total_supply / 10; // 10%

        let seeds = &[b"vesting".as_ref(), &[ctx.bumps.vesting_vault]];
        let signer = &[&seeds[..]];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vesting_vault.to_account_info(),
                    to: ctx.accounts.distributor_vault.to_account_info(),
                    authority: ctx.accounts.vesting_vault.to_account_info(),
                },
                signer,
            ),
            release_amount,
        )?;

        state.last_release_time = clock.unix_timestamp;
        state.release_count += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = team, space = 8 + 32 + 32 + 8 + 8)]
    pub state: Account<'info, VestingState>,
    #[account(mut)]
    pub team: Signer<'info>,
    /// CHECK: The token mint address
    pub mint: Account<'info, token::Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeInitial<'info> {
    #[account(mut, seeds = [b"distributor"], bump)]
    pub distributor_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub team_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user1: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user2: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user3: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user4: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ReleaseVesting<'info> {
    #[account(mut)]
    pub state: Account<'info, VestingState>,
    #[account(mut, seeds = [b"vesting"], bump)]
    pub vesting_vault: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"distributor"], bump)]
    pub distributor_vault: Account<'info, TokenAccount>,
    pub team: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct VestingState {
    pub team_wallet: Pubkey,
    pub mint: Pubkey,
    pub last_release_time: i64,
    pub release_count: u8,
}

#[error_code]
pub enum VestingError {
    #[msg("Wait 10 minutes between releases.")]
    TooEarly,
    #[msg("All tokens have been released.")]
    FullyVested,
}
