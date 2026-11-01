use std::io::Write as _;
use clap::Parser as _;
use solana_sdk::signature::Signer as _;

type Result<T = ()> = eyre::Result<T>;

#[derive(Debug, clap::Parser)]
enum Command {
	New {
		#[arg(long)]
		seed: u64,

		#[arg(long)]
		amount: u64,

		#[arg(long)]
		deadline: i64,

		#[arg(long, default_value = "http://127.0.0.1:8899")]
		rpc_url: String,

		#[arg(long)]
		buyer_keypair: std::path::PathBuf,

		#[arg(long)]
		buyer_ata: solana_sdk::pubkey::Pubkey,

		#[arg(long)]
		seller_public_key: solana_sdk::pubkey::Pubkey,

		#[arg(long)]
		token_mint: solana_sdk::pubkey::Pubkey,

		#[arg(long)]
		program_public_key: solana_sdk::pubkey::Pubkey
	}
}

fn main() -> Result {
	tracing_subscriber::fmt().init();

	match Command::parse() {
		Command::New {
			seed,
			amount,
			deadline,
			rpc_url,
			buyer_keypair,
			buyer_ata,
			seller_public_key,
			token_mint,
			program_public_key
		} => {
			let buyer_keypair: solana_sdk::signature::Keypair = solana_sdk::signature::read_keypair_file(&buyer_keypair).unwrap();
			let buyer_public_key: solana_sdk::pubkey::Pubkey = buyer_keypair.pubkey();
			let buyer_public_key_bytes: &[_] = buyer_public_key.as_ref();

			let seller_public_key_bytes: &[_] = seller_public_key.as_ref();

			let (escrow_pda, bump) = solana_sdk::pubkey::Pubkey::find_program_address(&[b"escrow", buyer_public_key_bytes, seller_public_key_bytes, &seed.to_le_bytes()], &program_public_key);

			let vault_ata = spl_associated_token_account::get_associated_token_address_with_program_id(&escrow_pda, &token_mint, &spl_associated_token_account::id());

			let pre_image = "global:new";
			let mut discriminator = [0u8; 8];
			discriminator.copy_from_slice(&solana_sdk::hash::hash(pre_image.as_bytes()).to_bytes()[..8]);

			let mut payload: Vec<_> = Vec::default();
			payload.extend_from_slice(&discriminator);
			payload.extend_from_slice(&seed.to_le_bytes());
			payload.extend_from_slice(&amount.to_le_bytes());
			payload.extend_from_slice(&deadline.to_le_bytes());

			// no arbitration
			payload.push(0);

			let accounts = vec![
				solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::pubkey!("11111111111111111111111111111111"), false),
				solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::pubkey::Pubkey::new_from_array(spl_token::id().to_bytes()), false),
				solana_sdk::instruction::AccountMeta::new_readonly(spl_associated_token_account::id(), false),
				solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
				solana_sdk::instruction::AccountMeta::new_readonly(token_mint, false),
				solana_sdk::instruction::AccountMeta::new_readonly(seller_public_key, false),
				solana_sdk::instruction::AccountMeta::new(buyer_public_key, true),
				solana_sdk::instruction::AccountMeta::new(buyer_ata, false),
				solana_sdk::instruction::AccountMeta::new(vault_ata, false),
				solana_sdk::instruction::AccountMeta::new(escrow_pda, false)
			];

			let task = solana_sdk::instruction::Instruction::new_with_bytes(program_public_key, &payload, accounts);

			let rpc_client = solana_client::rpc_client::RpcClient::new(rpc_url);
			let recent_block_hash = rpc_client.get_latest_blockhash()?;

			let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(&[task], Some(&buyer_public_key), &[&buyer_keypair], recent_block_hash);

			let signature: solana_sdk::signature::Signature = rpc_client.send_and_confirm_transaction(&transaction)?;

			tracing::info!("escrow initialized (signature={})", signature);

			Ok(())
		}
	}
}
