{ ... }: {
	perSystem = { pkgs, system, ... }: {
		packages.solana_platform_tool =
		(let
			pname = "solana_platform_tool";
			version = "1.41";

			asset."x86_64-linux".hash = "sha256-1GQinOMfo2qvEYRQVwGi2lx7tI4p00AE9bIFXf2fsLg=";

			configuration = (asset.${system} or (throw "unsupported (system=${system})"));
		in (pkgs.stdenv.mkDerivation {
			inherit pname;
			inherit version;

			sourceRoot = ".";
			src = (pkgs.fetchurl {
				url = "https://github.com/anza-xyz/platform-tools/releases/download/v${version}/platform-tools-linux-x86_64.tar.bz2";
				hash = configuration.hash;
			});

			nativeBuildInputs = [
				pkgs.autoPatchelfHook
			];

			buildInputs = [
				pkgs.stdenv.cc.cc.lib
				pkgs.zlib
				pkgs.zlib-ng
				pkgs.openssl
				pkgs.ncurses
				pkgs.libxml2
				pkgs.xz
			];

			autoPatchelfIgnoreMissingDeps = [
				"libpython3.8.so.1.0"
				"libxml2.so.2"
			];

			dontCheckSignificantSymlinks = true;

			installPhase = ''
				mkdir -p $out

				cp -r . $out/
				rm -rf $out/llvm/lib/python3.8
			'';
		}));
	};
}
