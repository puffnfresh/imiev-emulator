{
  inputs = {
    nixpkgs.url = "github:puffnfresh/nixpkgs/m32r";
    imiev-hacking-tools = {
      url = "github:bonybrown/imiev-hacking-tools";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, imiev-hacking-tools }: {
    packages = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (system:
      let pkgs = nixpkgs.legacyPackages."${system}"; in rec {
        default = imiev-hacking-tools.outPath;

        ghidra-m32r = pkgs.callPackage ./nix/ghidra-m32r.nix {
          inherit imiev-hacking-tools;
        };
        decompilation = pkgs.callPackage ./nix/decompilation.nix {
          inherit ghidra-m32r;
        };
        test-suite-rom = pkgs.callPackage ./nix/test-suite-rom.nix { };

        copy-artifacts = pkgs.writeShellScriptBin "copy-artifacts.sh" ''
          install -m 0644 ${decompilation}/* decompilation/

          TESTDATA=m32r-emulator/src/testdata/
          mkdir -p "$TESTDATA"
          install -m 0644 ${test-suite-rom}/* "$TESTDATA"
        '';
      }
    );
  };
}
