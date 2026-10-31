- Smart contract holds USDC in a vault.
- State lifecycle management:
	- Creation (waiting for funds).
	- Holding (USDC locked).
	- Delivery (condition met).
	- Release (funds sent to seller).
	- Refund (funds returned to the buyer).

Before any funds move or transition to `Delivery`, a signed verification issued by an issuer must be cryptographically verified.

On initialization, should lock the buyer's USDC into a PDA, recording the buyer, seller, amount, deadline, and condition for release.
