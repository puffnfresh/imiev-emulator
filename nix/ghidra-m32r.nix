{ ghidra
, imiev-hacking-tools
}:

ghidra.overrideAttrs (drv: {
  pname = "ghidra-m32r";
  postInstall = (drv.postInstall or "") + ''
    proc="$out/lib/ghidra/Ghidra/Processors/m32r"
    mkdir -p "$proc"
    cp -r ${imiev-hacking-tools}/Ghidra/Processors/m32r/data "$proc/"
    : > "$proc/Module.manifest"
    chmod -R u+w "$proc"
  '';
})

