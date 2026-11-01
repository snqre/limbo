use litesvm::*;
use litesvm_token::*;
use anchor_lang::*;
use solana_sdk::instruction::*;
use solana_sdk::pubkey::*;
use solana_sdk::message::*;
use solana_sdk::signature::*;
use solana_sdk::transaction::*;
use solana_address::*;
use sha2::*;

#[derive(AnchorSerialize)]
struct New {
	seed: u64,
	amount: u64,
	deadline: i64,
	attestator: Option<Pubkey>,
	arbiter: Option<Pubkey>
}

#[test]
fn new() -> eyre::Result {
	let local_root_directory: std::path::PathBuf = env!("CARGO_MANIFEST_DIR").to_owned().into();

	let workspace_root_directory: &std::path::Path = local_root_directory
		.parent()
		.ok_or(eyre::eyre!("no parent"))?
		.parent()
		.ok_or(eyre::eyre!("no parent"))?;

	let limbo_so_path: std::path::PathBuf = workspace_root_directory
    	.join("target")
     	.join("sbpfv3-solana-solana")
      	.join("release")
       	.join("limbo.so");

	let exit: std::process::ExitStatus = std::process::Command::new("anchor").args([
		"build",
		"--",
		"--package", "limbo"
	])
	.current_dir(workspace_root_directory)
	.stdout(std::process::Stdio::inherit())
	.stderr(std::process::Stdio::inherit())
	.status()?;

	assert!(exit.success(), "anchor build failed");

	let mut svm: LiteSVM = LiteSVM::new();

	let program_pk: Pubkey = "2Jq4CX5Eez8cPJfLeKDwFdpvuuyjqZhzo1p1TL3tUbbR".parse().expect("valid program ID");
	let program_pk_bytes: [_; _] = program_pk.to_bytes();
	let program_addr: Address = program_pk_bytes.into();

	let buyer_keypair: Keypair = Keypair::new();
	let buyer_pk: Pubkey = buyer_keypair.pubkey();
	let buyer_pk_bytes: [_; _] = buyer_pk.to_bytes();
	let buyer_addr: Address = buyer_pk_bytes.into();

	let seller_keypair: Keypair = Keypair::new();
	let seller_pk: Pubkey = seller_keypair.pubkey();
	let seller_pk_bytes: [_; _] = seller_pk.to_bytes();
	let seller_addr: Address = seller_pk_bytes.into();

	// loading the so file and supplying the buyer with sufficient sol
	svm.add_program_from_file(program_addr, limbo_so_path)?;
	svm.airdrop(&buyer_pk, 10_000_000_000).expect("airdrop");

	// usdc placeholder
	let mint_addr = CreateMint::new(&mut svm, &buyer_keypair).decimals(6).send().unwrap();

	let seed: u64 = 67;
	let seed_bytes: [_; _] = seed.to_le_bytes();

	let (escrow_pda, bump) = Pubkey::find_program_address(&[b"escrow", &buyer_pk_bytes, &seller_pk_bytes, &seed_bytes], &program_pk);

	let buyer_token_account_addr: Address = CreateAssociatedTokenAccount::new(&mut svm, &buyer_keypair, &mint_addr).owner(&buyer_pk).send().unwrap();
	let buyer_token_account_addr_bytes: [_; _] = buyer_token_account_addr.to_bytes();
	let buyer_token_account_pk: Pubkey = buyer_token_account_addr_bytes.into();

	let vault_token_account_addr = spl_associated_token_account::get_associated_token_address(&escrow_pda, &mint_addr);
	let vault_token_account_pk: Pubkey = vault_token_account_addr.to_bytes().into();

	let token_program_pk = spl_token::id();
	let token_program_pk_bytes: [_; _] = token_program_pk.to_bytes();
	let token_program_pk: Pubkey = token_program_pk_bytes.into();

	// $1000 (1000000) to the buyer
	MintTo::new(&mut svm, &buyer_keypair, &mint_addr, &buyer_token_account_addr, 1_000_000_000_000).send().unwrap();

	// 500 USDC (1000000) to be locked up in escrow
	let amount: u64 = 500_000_000;
	let deadline: i64 = 1_800_000_000;

	let mut hasher: Sha256 = Sha256::new();
	hasher.update(b"global:new");

	let discriminator = hasher.finalize();

	let mut task_payload: Vec<_> = Vec::new();
	task_payload.extend_from_slice(&discriminator[..8]);

	let task: New = New {
		seed,
		amount,
		deadline,
		attestator: None,
		arbiter: None
	};

	task.serialize(&mut task_payload)?;

	let accounts: Vec<_> = vec![
		AccountMeta::new_readonly(anchor_lang::solana_program::system_program::id(), false),
		AccountMeta::new_readonly(token_program_pk, false),
		AccountMeta::new_readonly(spl_associated_token_account::id(), false),
		AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
		AccountMeta::new_readonly(mint_addr, false),
		AccountMeta::new_readonly(seller_pk, false),
		AccountMeta::new(buyer_pk, true),
		AccountMeta::new(buyer_token_account_addr, false),
		AccountMeta::new(vault_token_account_addr, false),
		AccountMeta::new(escrow_pda, false)
	];

	let payload: Instruction = Instruction {
		accounts,
		program_id: program_pk,
		data: task_payload
	};

	let transaction: Transaction = Transaction::new_signed_with_payer(&[payload], Some(&buyer_pk), &[&buyer_keypair], svm.latest_blockhash());

	svm.send_transaction(transaction).unwrap();

	Ok(())
}
