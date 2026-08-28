import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.*;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.SourceType;
import java.io.PrintWriter;

// Decompile every function to a single text file, each prefixed with an
// address header, so a naming pass can read the whole corpus at once:
//
//   ghidra-analyzeHeadless <proj> <name> -process <bin> -noanalysis \
//       -scriptPath emulator/scripts -postScript DumpAll.java <out-path>
public class DumpAll extends GhidraScript {
    public void run() throws Exception {
        String[] a = getScriptArgs();
        String out = (a.length > 0) ? a[0] : "emulator/ghidra/bmu_decomp_all.txt";
        DecompInterface di = new DecompInterface();
        di.openProgram(currentProgram);
        PrintWriter pw = new PrintWriter(out);
        int n = 0;
        for (Function f : currentProgram.getFunctionManager().getFunctions(true)) {
            // skip spurious "functions" in RAM/SFR space (>= 0x800000) — not real code
            if (f.getEntryPoint().getOffset() >= 0x800000L) continue;
            String tag = (f.getSymbol() != null && f.getSymbol().getSource() != SourceType.DEFAULT
                    && !f.getName().startsWith("FUN_")) ? " [NAMED]" : "";
            pw.println("//=== 0x" + String.format("%08x", f.getEntryPoint().getOffset())
                    + " " + f.getName() + tag + " (calls=" + f.getCalledFunctions(monitor).size() + ")");
            DecompileResults r = di.decompileFunction(f, 45, monitor);
            if (r != null && r.decompileCompleted()) pw.println(r.getDecompiledFunction().getC());
            else pw.println("// <decompile failed>");
            n++;
        }
        pw.close();
        println("DumpAll: wrote " + n + " functions to " + out);
    }
}
