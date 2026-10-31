{ ... }: {
	perSystem = { pkgs, config, system, ... }: {
		packages.solana =
		(let
			version = "1.18.18";
			asset."x86_64-linux".name = "x86_64-unknown-linux-gnu";
			asset."x86_64-linux".hash = "sha256-7EGCsk6cqxThXu2SqGRIwbjvRtatbZMnXK08Nta9yMw=";

			configuration = (asset.${system} or (throw "unsupported (system=${system})"));
		in (pkgs.stdenv.mkDerivation {
			inherit version;

			pname = "solana";

			src = (pkgs.fetchurl {
				url = "https://github.com/anza-xyz/agave/releases/download/v${version}/solana-release-${configuration.name}.tar.bz2";
				sha256 = "sha256-7EGCsk6cqxThXu2SqGRIwbjvRtatbZMnXK08Nta9yMw=";
			});

			nativeBuildInputs = [
				pkgs.autoPatchelfHook
				pkgs.makeWrapper
			];

			buildInputs = [
				pkgs.stdenv.cc.cc.lib
				pkgs.zlib
				pkgs.zlib-ng
				pkgs.openssl
				pkgs.udev

				config.packages.solana_platform_tool
			];

			installPhase = ''
				mkdir -p $out

				cp -r . $out/
				rm -rf $out/bin/perf-libs

				mkdir -p $out/bin/sdk/sbf/dependencies

				ln -sfn ${config.packages.solana_platform_tool}/env-vars $out/bin/sdk/sbf/env-vars
				ln -sfn ${config.packages.solana_platform_tool}/rust $out/bin/sdk/sbf/rust
				ln -sfn ${config.packages.solana_platform_tool}/llvm $out/bin/sdk/sbf/llvm
				ln -sfn ${config.packages.solana_platform_tool}/version.md $out/bin/sdk/version.md

				wrapProgram $out/bin/cargo-build-sbf \
					--set SOLANA_SDK_PATH "$out/bin/sdk/sbf" \
					--prefix PATH : "$out/bin" \
					--prefix PATH : "${config.packages.solana_platform_tool}/bin" \
					--prefix PATH : "${config.packages.solana_platform_tool}/rust/bin" \
					--prefix PATH : "${config.packages.solana_platform_tool}/llvm/bin"
			'';

			meta.mainProgram = "solana";
		}));
	};
}
