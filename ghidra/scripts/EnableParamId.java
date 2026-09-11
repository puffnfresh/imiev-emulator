import ghidra.app.script.GhidraScript;

public class EnableParamId extends GhidraScript {
    public void run() throws Exception {
        setAnalysisOption(currentProgram, "Decompiler Parameter ID", "true");
    }
}
