{ ... }: {
	perSystem = { pkgs, system, ... }: {
		packages.anchor =
		(let
			pname = "anchor";
			version = "1.2.1";
			asset."x86_64-linux".name = "anchor-${version}-x86_64-unknown-linux-gnu";
			asset."x86_64-linux".hash = "sha256-XImTecvZqEP6SnMKN7p6thNXVx/8U1A2qa+gjlzQuZs=";
			asset."x86_64-darwin".name = "anchor-${version}-x86_64-apple-darwin";
			asset."x86_64-darwin".hash = pkgs.lib.fakeHash;
			asset."aarch64-darwin".name = "anchor-${version}-aarch64-apple-darwin";
			asset."aarch64-darwin".hash = pkgs.lib.fakeHash;
			configuration = (asset.${system} or (throw "unsupported (system=${system})"));
		in (pkgs.stdenv.mkDerivation {
			inherit pname;
			inherit version;

			src = (pkgs.fetchurl {
				url = "https://github.com/otter-sec/anchor/releases/download/v${version}/${configuration.name}";
				sha256 = configuration.hash;
			});

			dontUnpack = true;

			nativeBuildInputs = pkgs.lib.optional pkgs.stdenv.isLinux pkgs.autoPatchelfHook;

			buildInputs = [
				pkgs.openssl
				pkgs.udev
				pkgs.zlib
			] ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux [
				pkgs.stdenv.cc.cc.lib
			];

			installPhase = ''
				install -Dm755 $src $out/bin/anchor
			'';

			meta.description = "Solana Program Framework CLI";
			meta.license = pkgs.lib.licenses.asl20;
			meta.platforms = [
				"x86_64-linux"
				"x86_64-darwin"
				"aarch64-darwin"
			];
		}));
	};
}
