{ lib, rustPlatform }:

rustPlatform.buildRustPackage {
  name = "imiev-emulator";

  src = lib.cleanSourceWith {
    src = ../.;
    filter = path: type:
      let base = baseNameOf path; in
      base != "result"
      && base != "target"
      && base != "flake.nix"
      && base != "flake.lock";
  };

  cargoLock.lockFile = ../Cargo.lock;
}
