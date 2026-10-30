{ ... }: {
	perSystem = { pkgs, config, ... }: {
		packages.anchor =
		(let
			pname = "anchor";
			version = "0.30.1";
			rust_platform = (pkgs.makeRustPlatform {
				rustc = config.rust-project.toolchain;
				cargo = config.rust-project.toolchain;
			});
		in (rust_platform.buildRustPackage {
			inherit pname;
			inherit version;

			src = (pkgs.fetchFromGitHub {
				owner = "coral-xyz";
				repo = "anchor";
				tag = "v${version}";
				hash = pkgs.lib.fakeHash;
			});

			cargoHash = pkgs.lib.fakeHash;
			cargoBuildFlags = [
				"--package" "anchor-cli"
			];

			nativeBuildInputs = [
				pkgs.pkg-config
			];

			buildInputs = [
				pkgs.openssl
				pkgs.udev
			];

			doCheck = false;
		}));
	};
}
