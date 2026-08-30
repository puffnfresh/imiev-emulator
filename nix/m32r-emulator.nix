{ rustPlatform }:

rustPlatform.buildRustPackage {
  name = "m32r-emulator";
  src = ../m32r-emulator;
  cargoLock.lockFile = ../m32r-emulator/Cargo.lock;
}
