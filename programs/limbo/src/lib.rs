#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::cast_possible_truncation)]
#![deny(clippy::cast_possible_wrap)]

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;

// - needs a verification mechanism that will validate signed verdict token
// - needs to check an ed25519 signature instruction passed in the transaction or verify state proof against a program stored pk representing the signing authority
// - needs the remaining lifecycle instruction suite (attest, release)
// - - attest: an instruction callable only by the designated attestator (either an agent or role) that transitions the state from `Funded` to `Delivered`
// - - release: instruction to move funds from the vault token account to the seller's token account once condition/attestation or verification verdict is met
// - arbiter & dispute pipeline
// -

declare_id!("2Jq4CX5Eez8cPJfLeKDwFdpvuuyjqZhzo1p1TL3tUbbR");

type Result<T = ()> = anchor_lang::Result<T>;


#[error_code]
enum Error {
	#[msg("escrow has already been completed or resolved")]
	AlreadyComplete,

	#[msg("deadline must be in the future")]
	IllegalDeadline,

	#[msg("escrow amount must be greater than zero")]
	IllegalAmount,

	#[msg("deadline has not been reached yet")]
	Pending,

	#[msg("unauthorized")]
	Unauthorized
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, InitSpace, AnchorSerialize, AnchorDeserialize)]
enum Lifecycle {
	#[default]
	Idle,
	Funded,
	Delivered,
	Released,
	Refunded
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, InitSpace, AnchorSerialize, AnchorDeserialize)]
enum ArbitrationPolicy {
	#[default]
	Direct,
	Uncontested {
		arbiter: Pubkey
	},
	Disputed {
		arbiter: Pubkey
	}
}

#[account]
#[derive(InitSpace)]
struct Escrow {
	pub buyer: Pubkey,
	pub seller: Pubkey,
	pub attestator: Option<Pubkey>,
	pub amount: u64,
	pub deadline: i64,
	pub arbitration_policy: ArbitrationPolicy,
	pub lifecycle: Lifecycle,
	pub bump: u8
}


#[derive(Accounts)]
#[instruction(seed: u64, amount: u64, deadline: i64, attestator: Option<Pubkey>, arbiter: Option<Pubkey>)]
struct New<'info> {
	pub system_program: Program<'info, System>,
	pub token_program: Program<'info, Token>,
	pub associated_token_program: Program<'info, AssociatedToken>,
	pub rent: Sysvar<'info, Rent>,
	pub usdc: Account<'info, Mint>,

	/// CHECK: We are only transferring tokens/sol to this account and validating it via business logic
	pub seller: UncheckedAccount<'info>,

	#[account(mut)]
	pub buyer: Signer<'info>,

	#[account(mut)]
	#[account(associated_token::mint = usdc)]
	#[account(associated_token::authority = buyer)]
	pub buyer_token_account: Account<'info, TokenAccount>,

	#[account(init)]
	#[account(payer = buyer)]
	#[account(associated_token::mint = usdc)]
	#[account(associated_token::authority = escrow)]
	pub vault_token_account: Account<'info, TokenAccount>,

	#[account(init)]
	#[account(payer = buyer)]
	#[account(space = 8 + Escrow::INIT_SPACE)]
	#[account(seeds = [b"escrow", buyer.key().as_ref(), seller.key().as_ref(), seed.to_le_bytes().as_ref()])]
	#[account(bump)]
	pub escrow: Account<'info, Escrow>
}

#[derive(Accounts)]
#[instruction(seed: u64)]
struct Attest<'info> {
	pub token_program: Program<'info, Token>,
	pub usdc: Account<'info, Mint>,

	/// CHECK: used for destination transfer and seed derivation
	pub seller: UncheckedAccount<'info>,

	#[account(mut)]
	#[account(associated_token::mint = usdc)]
	#[account(associated_token::authority = seller)]
	pub seller_token_account: Account<'info, TokenAccount>,

	/// CHECK: used for PDE seed derivation
	pub buyer: UncheckedAccount<'info>,

	#[account(mut)]
	pub attestator: Signer<'info>,

	#[account(mut)]
	#[account(associated_token::mint = usdc)]
	#[account(associated_token::authority = buyer)]
	pub buyer_token_account: Account<'info, TokenAccount>,

	#[account(associated_token::mint = usdc)]
	#[account(associated_token::authority = escrow)]
	pub vault_token_account: Account<'info, TokenAccount>,

	#[account(mut)]
	#[account(seeds = [b"escrow", buyer.key().as_ref(), seller.key().as_ref(), seed.to_le_bytes().as_ref()])]
	#[account(bump)]
	pub escrow: Account<'info, Escrow>
}

#[derive(Accounts)]
#[instruction(seed: u64)]
struct Refund<'info> {
	pub system_program: Program<'info, System>,
	pub token_program: Program<'info, Token>,
	pub associated_token_program: Program<'info, AssociatedToken>,
	pub rent: Sysvar<'info, Rent>,
	pub usdc: Account<'info, Mint>,

	/// CHECK: Only used for seeds derivation
	pub seller: UncheckedAccount<'info>,

	#[account(mut)]
	pub buyer: Signer<'info>,

	#[account(mut)]
	#[account(associated_token::mint = usdc)]
	#[account(associated_token::authority = buyer)]
	pub buyer_token_account: Account<'info, TokenAccount>,

	#[account(associated_token::mint = usdc)]
	#[account(associated_token::authority = escrow)]
	pub vault_token_account: Account<'info, TokenAccount>,

	#[account(mut)]
	#[account(seeds = [b"escrow", buyer.key().as_ref(), seller.key().as_ref(), seed.to_le_bytes().as_ref()])]
	#[account(bump)]
	pub escrow: Account<'info, Escrow>
}

#[program]
mod contract {
	use super::*;

	pub fn new(context: Context<New>, seed: u64, amount: u64, deadline: i64, attestator: Option<Pubkey>, arbiter: Option<Pubkey>) -> Result {
		require!(amount > 0, Error::IllegalAmount);

		let now: Clock = Clock::get()?;

		require!(deadline > now.unix_timestamp, Error::IllegalDeadline);

		let escrow: &mut Account<_> = &mut context.accounts.escrow;
		escrow.buyer = context.accounts.buyer.key();
		escrow.seller = context.accounts.seller.key();
		escrow.attestator = attestator;
		escrow.amount = amount;
		escrow.deadline = deadline;
		escrow.bump = context.bumps.escrow;
		escrow.lifecycle = Lifecycle::Funded;

		if let Some(arbiter) = arbiter {
			escrow.arbitration_policy = ArbitrationPolicy::Uncontested {
				arbiter
			};
		}

		let buyer_account_info: AccountInfo = context.accounts.buyer.to_account_info();
		let buyer_token_account_info: AccountInfo = context.accounts.buyer_token_account.to_account_info();
		let vault_token_account_info: AccountInfo = context.accounts.vault_token_account.to_account_info();

		let token_transfer_task: Transfer = Transfer {
			from: buyer_token_account_info,
			to: vault_token_account_info,
			authority: buyer_account_info
		};

		let token_program_address = context.accounts.token_program.key();
		let token_deposit_cpi_context: CpiContext<_> = CpiContext::new(token_program_address, token_transfer_task);

		anchor_spl::token::transfer(token_deposit_cpi_context, amount)?;

		Ok(())
	}

	pub fn attest(context: Context<Attest>, seed: u64) -> Result {
		// a signature or cryptographic proof can be checked onchain instead of a specific attestator being required (for this implementation requiring a single attestator should be enough)
		//
		// attest can automatically send funds to the seller and complete the lifecycle but perhaps its wiser to keep these as two separate instructions to leave room for a dispute or challenge window

		let escrow: &mut Account<_> = &mut context.accounts.escrow;

		require!(escrow.lifecycle == Lifecycle::Funded, Error::AlreadyComplete);

		let Some(required_attestator_addr) = escrow.attestator else {
			todo!("no branch for zero attestator handling")
		};

		let attestator_addr = context.accounts.attestator.key();

		require!(attestator_addr == required_attestator_addr, Error::Unauthorized);

		escrow.lifecycle = Lifecycle::Delivered;

		let buyer_addr = escrow.buyer;
		let buyer_addr_bytes: &[_] = buyer_addr.as_ref();
		let seller_addr = context.accounts.seller.key();
		let seller_addr_bytes: &[_] = seller_addr.as_ref();
		let bump: u8 = escrow.bump;
		let seed_bytes: [_; _] = seed.to_le_bytes();
		let seeds = &[b"escrow", buyer_addr_bytes, seller_addr_bytes, &seed_bytes, &[bump]];
		let signer_seeds = &[&seeds[..]];

		let escrow_account_info: AccountInfo = escrow.to_account_info();
		let seller_token_account_info: AccountInfo = context.accounts.seller_token_account.to_account_info();
		let vault_token_account_info: AccountInfo = context.accounts.vault_token_account.to_account_info();

		let token_transfer_task: Transfer = Transfer {
			from: vault_token_account_info,
			to: seller_token_account_info,
			authority: escrow_account_info
		};

		let token_program_addr = context.accounts.token_program.key();
		let token_withdrawal_cpi_context: CpiContext<_> = CpiContext::new_with_signer(token_program_addr, token_transfer_task, signer_seeds);

		anchor_spl::token::transfer(token_withdrawal_cpi_context, escrow.amount)?;

		escrow.lifecycle = Lifecycle::Released;

		Ok(())
	}

	pub fn refund(context: Context<Refund>, seed: u64) -> Result {
		let escrow: &mut Account<_> = &mut context.accounts.escrow;

		require!(escrow.lifecycle == Lifecycle::Funded, Error::AlreadyComplete);

		let now: Clock = Clock::get()?;

		require!(escrow.deadline < now.unix_timestamp, Error::Pending);

		escrow.lifecycle = Lifecycle::Refunded;

		let amount: u64 = escrow.amount;
		let bump: u8 = escrow.bump;

		let seller: &UncheckedAccount = &context.accounts.seller;
		let seller_address = seller.key();
		let seller_address_bytes: &[_] = seller_address.as_ref();

		let buyer: &Signer = &context.accounts.buyer;
		let buyer_address = buyer.key();
		let buyer_address_bytes: &[_] = buyer_address.as_ref();
		let buyer_token_account_info: AccountInfo = context.accounts.buyer_token_account.to_account_info();

		let vault_token_account_info: AccountInfo = context.accounts.vault_token_account.to_account_info();

		let escrow_account_info: AccountInfo = context.accounts.escrow.to_account_info();

		let seed_bytes: [_; _] = seed.to_le_bytes();
		let seeds = &[b"escrow", buyer_address_bytes, seller_address_bytes, &seed_bytes, &[bump]];

		let signer_seeds = &[&seeds[..]];

		let token_transfer_task: Transfer = Transfer {
			from: vault_token_account_info,
			to: buyer_token_account_info,
			authority: escrow_account_info
		};

		let token_program_address = context.accounts.token_program.key();
		let token_withdrawal_cpi_context: CpiContext<_> = CpiContext::new_with_signer(token_program_address, token_transfer_task, signer_seeds);

		anchor_spl::token::transfer(token_withdrawal_cpi_context, amount)?;

		Ok(())
	}
}
