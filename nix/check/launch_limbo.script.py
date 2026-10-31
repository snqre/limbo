import re
import typing
import hashlib
# import json
# import urllib.request

start_all()

main.wait_for_unit("multi-user.target")

main.execute("solana-test-validator --reset --quiet > /tmp/validator.log 2>&1 &")

main.wait_for_open_port(8899)

main.succeed("solana cluster-version --url http://127.0.0.1:8899")
main.succeed("solana-keygen new --no-passphrase --outfile /root/.config/solana/id.json")
main.succeed("solana airdrop 10 --url http://127.0.0.1:8899")
main.succeed("solana program deploy @LIMBO_SO@ --url http://127.0.0.1:8899")

main.succeed("solana-keygen new --no-passphrase --outfile /root/buyer.json")
main.succeed("solana-keygen new --no-passphrase --outfile /root/seller.json")

main.succeed("solana airdrop 5 --url http://127.0.0.1:8899 --keypair /root/buyer.json")
main.succeed("solana airdrop 5 --url http://127.0.0.1:8899 --keypair /root/seller.json")

main.succeed("solana config set --keypair /root/buyer.json")

token_mint: str = main.succeed("spl-token create-token --url http://127.0.0.1:8899")
token_mint: typing.Optional[typing.Match[str]] = re.search(r"Creating token\s+([A-Za-z0-9]+)", token_mint)

assert token_mint is not None, "unable to find token mint address"

token_mint: str = token_mint.group(1)

buyer_associated_token_account: str = main.succeed(f"spl-token create-account {token_mint} --url http://127.0.0.1:8899")
buyer_associated_token_account: typing.Optional[typing.Match[str]] = re.search(r"Creating account\s+([A-Za-z0-9]+)", buyer_associated_token_account)

assert buyer_associated_token_account is not None, "unable to find buyer ATA address"

buyer_associated_token_account: str = buyer_associated_token_account.group(1)

main.succeed(f"spl-token mint {token_mint} 1000 {buyer_associated_token_account} --url http://127.0.0.1:8899")

def discriminator(name: str) -> list[int]:
	return list(hashlib.sha256(f"global:{name}".encode()).digest()[:8])

new_task = discriminator("new")

print(f"Anchor 'new' instruction discriminator: {new_task}")
