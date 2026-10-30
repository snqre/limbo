{ inputs, ... }: {
	imports = [
		inputs.git_hook.flakeModule
		inputs.rust_flake.flakeModules.default
		inputs.rust_flake.flakeModules.nixpkgs
	];

	perSystem = { config, pkgs, inputs', ... }: {
		devShells.default = pkgs.mkShell {
			RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
			RUST_BACKTRACE = "1";
			RUSTFLAGS = "-A warnings";

			inputsFrom = [
				config.pre-commit.devShell
				config.devShells.rust
			];

			nativeBuildInputs = [
				pkgs.bash-language-server
				pkgs.nixd
				pkgs.nixpkgs-fmt
				pkgs.git
				pkgs.git-lfs
				pkgs.cocogitto
				pkgs.pre-commit
				pkgs.clippy
				pkgs.cargo
				pkgs.rustc
				pkgs.rust-analyzer
				pkgs.pkg-config
			];

			buildInputs = [
				pkgs.openssl
				pkgs.glib
			];

			shellHook = ''
				export PATH="$HOME/.cargo/bin:$PATH"

				${pkgs.git-lfs}/bin/git-lfs install --local --skip-repo --quiet > /dev/null 2>&1

				${config.pre-commit.installationScript} > /dev/null 2>&1
			'';
		};

		rust-project.toolchain = inputs'.fenix.packages.stable.toolchain;

		pre-commit.check.enable = true;

		pre-commit.settings.hooks.check-symlinks.enable = true;
		pre-commit.settings.hooks.check-merge-conflicts.enable = true;
		pre-commit.settings.hooks.check-added-large-files.enable = true;
		pre-commit.settings.hooks.check-added-large-files.args = [
			"--maxkb=50"
			"--enforce-all"
		];

		pre-commit.settings.hooks.cog.enable = true;
		pre-commit.settings.hooks.cog.entry = "${pkgs.cocogitto}/bin/cog verify --file";
		pre-commit.settings.hooks.cog.stages = [
			"commit-msg"
		];

		pre-commit.settings.hooks.ripsecrets.enable = true;

		pre-commit.settings.hooks.check-yaml.enable = true;
		pre-commit.settings.hooks.check-json.enable = true;
		pre-commit.settings.hooks.check-toml.enable = true;

		pre-commit.settings.hooks.trim-trailing-whitespace.enable = true;

		pre-commit.settings.hooks.end-of-file-fixer.enable = true;
		pre-commit.settings.hooks.end-of-file-fixer.excludes = [
			"^target/"
			"^build/"
			"^node_modules/"
			"^vendor/"
			"^\\.git/"
			"\\.min\\.js$"
			"\\.min\\.css$"
			"\\.svg$"
			"\\.png$"
			"\\.jpg$"
			"\\.ico$"
		];
	};
}
