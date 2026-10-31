{ self, ... }: {
	perSystem = { pkgs, config, ... }: {
		packages.limbo =
		(let
			pname = "limbo";
			version = "0.1.0";
			src = self;
		in (pkgs.lib.mkForce (pkgs.stdenv.mkDerivation {
			inherit pname;
			inherit version;
			inherit src;

			__noChroot = true;

			SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";

			RUST_BACKTRACE = "full";

			nativeBuildInputs = [
				pkgs.nushell
				pkgs.autoPatchelfHook
				pkgs.makeWrapper

				config.rust-project.toolchain
				config.packages.anchor
				config.packages.solana
			];

			buildInputs = [
				pkgs.zlib
				pkgs.zlib-ng
				pkgs.stdenv.cc.cc
			];

			buildPhase = '' nu -c '
				$env.HOME = ($env.TMPDIR | path join "home")

				let cache_dir = ($env.HOME | path join ".cache" "solana" "v1.41" "platform-tools")

				^mkdir -p $cache_dir

				^cp -r ("${config.packages.solana_platform_tool}" + "/.") $cache_dir
				^chmod -R +w $cache_dir

				$env.SOLANA_SDK_PATH = ("${config.packages.solana}/bin/sdk/sbf")
				$env.SBF_SDK_PATH = ("${config.packages.solana}/bin/sdk/sbf")

				^mkdir -p target/deploy

				^solana-keygen new --no-passphrase --outfile target/deploy/limbo-keypair.json --force

				^anchor build --arch sbfv2 --ignore-keys
			' '';

			installPhase = '' nu -c '
				mkdir -p $out/deploy

				cp -r target/deploy/* $out/deploy/
			' '';
		})));
	};
}
