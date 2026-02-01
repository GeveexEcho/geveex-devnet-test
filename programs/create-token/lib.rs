use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, 
        Metadata as Metaplex,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

declare_id!("ADPKzrbAKxiztbuzDrj5pfu6oRkm3FBJrh5rQP9hzcrt");

// The specific wallet that must receive the supply
const TARGET_WALLET: &str = "PWTPrTgMX2WM1gbsFSib7RrYiXPEvVR6t13n1zWht4G";

#[program]
pub mod create_token {
    use super::*;

    pub fn initialize_token(
        ctx: Context<InitializeToken>, 
        metadata_title: String, 
        metadata_symbol: String, 
        metadata_uri: String
    ) -> Result<()> {
        
        // 1. Create Metadata Account
        let cpi_accounts = CreateMetadataAccountsV3 {
            metadata: ctx.accounts.metadata_account.to_account_info(),
            mint: ctx.accounts.mint_account.to_account_info(),
            mint_authority: ctx.accounts.payer.to_account_info(), // Payer is mint authority
            payer: ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };

        let data_v2 = DataV2 {
            name: metadata_title,
            symbol: metadata_symbol,
            uri: metadata_uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(), 
            cpi_accounts
        );
        
        create_metadata_accounts_v3(
            cpi_ctx, 
            data_v2, 
            true, // Is mutable
            true, // Update authority is signer
            None, 
        )?;

        // 2. Mint Tokens to the Target Wallet's ATA
        // Supply: 100,000 * 10^8 (Decimals)
        let amount = 100_000 * 10u64.pow(8);

        let mint_cpi_accounts = MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let mint_cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            mint_cpi_accounts
        );

        mint_to(mint_cpi_ctx, amount)?;

        msg!("Token Created and Minted 100,000 supply to {}", TARGET_WALLET);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // Init the mint with 8 decimals
    #[account(
        init,
        payer = payer,
        mint::decimals = 8,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint_account: Account<'info, Mint>,

    // Validate the recipient is exactly the wallet requested
    /// CHECK: Validated via address constraint
    #[account(address = TARGET_WALLET.parse::<Pubkey>().unwrap())]
    pub recipient: UncheckedAccount<'info>,

    // Create the Associated Token Account for the recipient if it doesn't exist
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    /// CHECK: Address validation for Metadata Account
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metaplex>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
      }

