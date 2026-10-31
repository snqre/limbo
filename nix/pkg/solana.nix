{ ... }: {
	perSystem = { pkgs, config, system, ... }: {
		packages.solana =
		(let
			version = "1.18.18";
			asset."x86_64-linux".name = "x86_64-unknown-linux-gnu";
			asset."x86_64-linux".hash = "sha256-7EGCsk6cqxThXu2SqGRIwbjvRtatbZMnXK08Nta9yMw=";

			configuration = asset.${system} or (throw "unsupported (system=${system})");
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
			];

			installPhase = ''
				mkdir -p $out

				cp -r . $out/
				rm -rf $out/bin/perf-libs

				mkdir -p $out/bin/sdk/sbf/dependencies

				ln -s ${config.packages.solana_platform_tool} $out/bin/sdk/sbf/dependencies/platform-tools
			'';

			meta.mainProgram = "solana";
		}));
	};
}
