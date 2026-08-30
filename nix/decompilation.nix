{ runCommand
, ghidra-m32r
}:

runCommand "imiev-decompilation" { buildInputs = [ ghidra-m32r ]; } ''
  cp ${../firmware}/* .
  mkdir tmp-project

  mkdir -p $out
  decompile() {
    ghidra-analyzeHeadless \
      tmp-project \
      $1 \
      -import $1.bin \
      -processor m32r:2:default \
      -scriptPath ${../ghidra/scripts} \
      -postScript ApplySymbols.java \
      ${../ghidra/symbols}/$1.txt

    ghidra-analyzeHeadless \
      tmp-project \
      $1 \
      -process $1.bin \
      -noanalysis \
      -scriptPath ${../ghidra/scripts} \
      -postScript DumpAll.java \
      $out/$1.c
  }

  decompile bmu
  decompile ev-ecu
''
