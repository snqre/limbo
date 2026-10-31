{
	inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
	inputs.flake-parts.url = "github:hercules-ci/flake-parts";
	inputs.git_hook.url = "github:cachix/git-hooks.nix";
	inputs.git_hook.inputs.nixpkgs.follows = "nixpkgs";
	inputs.rust_flake.url = "github:juspay/rust-flake";
	inputs.rust_flake.inputs.nixpkgs.follows = "nixpkgs";
	inputs.fenix.url = "github:nix-community/fenix";
	inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";
	inputs.anchor.url = "github:vaporif/anchor-overlay";
	inputs.anchor.inputs.nixpkgs.follows = "nixpkgs";

	outputs = inputs @ { nixpkgs, flake-parts, ... }:
	(flake-parts.lib.mkFlake {
		inherit inputs;
	} {
		systems = nixpkgs.lib.systems.flakeExposed;

		imports = [
			./nix/check/launch_limbo.nix
			./nix/dev
			./nix/pkg/limbo.nix
		];

		perSystem = ({ ... }: {
			nixpkgs.overlays = [
				inputs.anchor.overlays.default
			];
		});
	});
}
