{ runCommand, imiev-wasm }:

runCommand "imiev-wasm-site" { } ''
  mkdir -p "$out/pkg"
  install -m 0644 ${../imiev-wasm/index.html} "$out/index.html"
  install -m 0644 ${../imiev-wasm/engine.js} "$out/engine.js"
  install -m 0644 ${../imiev-wasm/link.js} "$out/link.js"
  install -m 0644 ${../imiev-wasm/ecu-worker.js} "$out/ecu-worker.js"
  install -m 0644 ${../imiev-wasm/bmu-worker.js} "$out/bmu-worker.js"
  install -m 0644 ${../imiev-wasm/coi-sw.js} "$out/coi-sw.js"
  install -m 0644 \
    ${imiev-wasm}/imiev_wasm.js \
    ${imiev-wasm}/imiev_wasm_bg.wasm \
    "$out/pkg/"
''
