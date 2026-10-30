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

	GenerateKeypair
}

fn main() -> Result {
	tracing_subscriber::fmt().init();

	match Command::parse() {
		Command::SystemHook(SystemHook::Welcome) => Ok(()),
		Command::GenerateKeypair => {
			let keypair: solana_sdk::signature::Keypair = solana_sdk::signature::Keypair::new();
			let public_key: solana_sdk::pubkey::Pubkey = keypair.pubkey();
			let public_key_line: String = format!("SOLANA_PUBLIC_KEY={}", public_key);
			let public_key_line: &[_] = public_key_line.as_bytes();
			let secret_key: Vec<_> = keypair.to_bytes().to_vec();
			let secret_key: String = serde_json::to_string(&secret_key)?;
			let secret_key_line: String = format!("SOLANA_SECRET_KEY={}", secret_key);
			let secret_key_line: &[_] = secret_key_line.as_bytes();

			let mut file: std::fs::File = std::fs::OpenOptions::new()
				.create(true)
				.append(true)
				.open(".env")?;

			file.write_all(public_key_line)?;
			file.write_all(b"\n")?;
			file.write_all(secret_key_line)?;
			file.write_all(b"\n")?;

			tracing::info!("keypair generation complete > `.env`");

			Ok(())
		}
	}
}
