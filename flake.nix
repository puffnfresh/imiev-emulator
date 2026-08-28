{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    imiev-hacking-tools = {
      url = "github:bonybrown/imiev-hacking-tools";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, imiev-hacking-tools }: {
    packages = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (system:
      let pkgs = nixpkgs.legacyPackages."${system}"; in rec {
        default = imiev-hacking-tools.outPath;

        ghidra-m32r = pkgs.ghidra.overrideAttrs (drv: {
          pname = "ghidra-m32r";
          postInstall = (drv.postInstall or "") + ''
            proc="$out/lib/ghidra/Ghidra/Processors/m32r"
            mkdir -p "$proc"
            cp -r ${imiev-hacking-tools}/Ghidra/Processors/m32r/data "$proc/"
            : > "$proc/Module.manifest"
            chmod -R u+w "$proc"
          '';
        });

        decompilation = pkgs.runCommand "imiev-decompilation" { buildInputs = [ ghidra-m32r ]; } ''
          cp ${./firmware}/* .
          mkdir tmp-project

          mkdir -p $out
          decompile() {
            ghidra-analyzeHeadless \
              tmp-project \
              $1 \
              -import $1.bin \
              -processor m32r:2:default \
              -scriptPath ${./ghidra/scripts} \
              -postScript ApplySymbols.java \
              ${./ghidra/symbols}/$1.txt

            ghidra-analyzeHeadless \
              tmp-project \
              $1 \
              -process $1.bin \
              -noanalysis \
              -scriptPath ${./ghidra/scripts} \
              -postScript DumpAll.java \
              $out/$1.txt
          }

          decompile bmu
          decompile ev-ecu
        '';
      }
    );
  };
}
