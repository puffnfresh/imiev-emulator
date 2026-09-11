{ runCommand, imiev-wasm }:

runCommand "imiev-wasm-site" { } ''
  mkdir -p "$out/pkg"
  install -m 0644 ${../imiev-wasm/index.html} "$out/index.html"
  install -m 0644 \
    ${imiev-wasm}/imiev_wasm.js \
    ${imiev-wasm}/imiev_wasm_bg.wasm \
    "$out/pkg/"
''
