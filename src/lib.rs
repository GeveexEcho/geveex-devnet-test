use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, SetAuthority, Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};

declare_id!("334XQpaLxYbB93VLLXYXkh58RUA4B8wpijPPQzZakTo3");

#[program]
pub mod nft_launcher {
    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNft>, 
        name: String, 
        symbol: String, 
        uri: String
    ) -> Result<()> {
        
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        token::mint_to(CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts), 1)?;

        let creator_key = ctx.accounts.recipient.key();
        let metadata_infos = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let mut data = vec![];
        data.push(33); 
        data.extend_from_slice(&(name.len() as u32).to_le_bytes());
        data.extend_from_slice(name.as_bytes());
        data.extend_from_slice(&(symbol.len() as u32).to_le_bytes());
        data.extend_from_slice(symbol.as_bytes());
        data.extend_from_slice(&(uri.len() as u32).to_le_bytes());
        data.extend_from_slice(uri.as_bytes());
        data.extend_from_slice(&700u16.to_le_bytes());
        data.push(1);
        data.extend_from_slice(&1u32.to_le_bytes());
        data.extend_from_slice(creator_key.as_ref());
        data.push(0);
        data.push(100);
        data.push(1);
        data.push(0);
        data.push(0);
        data.push(0);

        invoke(
            &Instruction {
                program_id: *ctx.accounts.token_metadata_program.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.metadata.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.mint.key(), false),
                    AccountMeta::new(ctx.accounts.payer.key(), true),
                    AccountMeta::new_readonly(ctx.accounts.payer.key(), true),
                    AccountMeta::new_readonly(ctx.accounts.payer.key(), true),
                    AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                ],
                data: data,
            },
            &metadata_infos,
        )?;

        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let mut me_data = vec![];
        me_data.push(17);
        me_data.extend_from_slice(&0u64.to_le_bytes());

        invoke(
             &Instruction {
                program_id: *ctx.accounts.token_metadata_program.key,
                accounts: vec![
                    AccountMeta::new(ctx.accounts.master_edition.key(), false),
                    AccountMeta::new(ctx.accounts.mint.key(), false),
                    AccountMeta::new(ctx.accounts.payer.key(), true),
                    AccountMeta::new_readonly(ctx.accounts.payer.key(), true),
                    AccountMeta::new_readonly(ctx.accounts.metadata.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
                ],
                data: me_data,
            },
            &master_edition_infos,
        )?;

        let auth_accounts = SetAuthority {
            account_or_mint: ctx.accounts.mint.to_account_info(),
            current_authority: ctx.accounts.payer.to_account_info(),
        };
        
        token::set_authority(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), auth_accounts),
            anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens,
            None,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Target wallet
    pub recipient: UncheckedAccount<'info>,
    #[account(init, payer = payer, mint::decimals = 0, mint::authority = payer, mint::freeze_authority = payer)]
    pub mint: Account<'info, Mint>,
    #[account(init_if_needed, payer = payer, associated_token::mint = mint, associated_token::authority = recipient)]
    pub destination: Account<'info, TokenAccount>,
    /// CHECK: Metaplex Metadata PDA
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: Metaplex Master Edition PDA
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: Metaplex ID
    pub token_metadata_program: UncheckedAccount<'info>,
                                              }
