{ self, ... }: {
	perSystem = { pkgs, ... }: {
		packages.limbo =
		(let
			pname = "limbo";
			version = "0.1.0";
			src = self;
			arch = "v2";
		in (pkgs.lib.mkForce (pkgs.buildAnchorProgram {
			inherit pname;
			inherit version;
			inherit src;
			inherit arch;

			cargoLock.lockFile = "${self}/Cargo.lock";
		})));
	};
}
