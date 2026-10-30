use std::io::Write as _;
use clap::Parser as _;
use solana_sdk::signature::Signer as _;

type Result<T = ()> = eyre::Result<T>;

#[derive(Debug, clap::Subcommand)]
enum SystemHook {
	Welcome
}

#[derive(Debug, clap::Parser)]
enum Command {
	#[clap(subcommand, hide = true)]
	SystemHook(SystemHook),

	GenerateKeyPair
}

fn main() -> Result {
	tracing_subscriber::fmt().init();

	match Command::parse() {
		Command::SystemHook(SystemHook::Welcome) => Ok(()),
		Command::GenerateKeyPair => {
			let key_pair: solana_sdk::signature::Keypair = solana_sdk::signature::Keypair::new();
			let public_key: solana_sdk::pubkey::Pubkey = key_pair.pubkey();
			let secret_key: Vec<_> = key_pair.to_bytes().to_vec();
			let secret_key: String = serde_json::to_string(&secret_key)?;

			let line: String = format!("SOLANA_PRIVATE_KEY={}", secret_key);
			let line: &[_] = line.as_bytes();

			let mut file: std::fs::File = std::fs::OpenOptions::new()
				.create(true)
				.append(true)
				.open(".env")?;

			file.write_all(line)?;

			tracing::info!("successful keypair generation");
			tracing::info!("public key: {}", public_key);
			tracing::info!("secret key injected to `.env`");

			Ok(())
		}
	}
}
