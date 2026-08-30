{ stdenv
# , m32r-toolchain
, pkgsCross
}:

stdenv.mkDerivation {
  name = "test-suite-rom";
  src = ../test-suite;
  buildInputs = [ pkgsCross.m32r-elf.buildPackages.gcc ];
  installPhase = ''
    mkdir $out
    cp build/*.bin $out/
  '';
}
