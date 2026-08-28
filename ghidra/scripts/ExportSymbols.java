import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.SourceType;
import ghidra.program.model.symbol.Symbol;
import ghidra.program.model.symbol.SymbolTable;
import ghidra.program.model.symbol.SymbolType;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

// Dump every user-defined name (functions + labels) back into the symbol-map
// format ApplySymbols consumes. Round-trips names made interactively in the GUI
// so they can be committed to the name library.
//
//   ghidra-analyzeHeadless <proj> <name> -process <bin> -noanalysis \
//       -scriptPath emulator/scripts -postScript ExportSymbols.java <out-path>
public class ExportSymbols extends GhidraScript {
    public void run() throws Exception {
        String[] a = getScriptArgs();
        String out = (a.length > 0) ? a[0] : "ghidra/symbols/bmu_symbols.export.txt";

        List<String> lines = new ArrayList<>();

        for (Function f : currentProgram.getFunctionManager().getFunctions(true)) {
            Symbol s = f.getSymbol();
            if (s == null || s.getSource() == SourceType.DEFAULT) continue;
            if (f.getName().startsWith("FUN_")) continue;
            String c = f.getComment();
            lines.add(fmt(f.getEntryPoint(), "FUN", f.getName(), c));
        }

        SymbolTable st = currentProgram.getSymbolTable();
        for (Symbol s : st.getAllSymbols(false)) {
            if (s.getSource() == SourceType.DEFAULT) continue;
            if (s.getSymbolType() != SymbolType.LABEL) continue;
            String c = getEOLComment(s.getAddress());
            // A label sitting on defined data -> DAT (with a type token); else a code LBL.
            ghidra.program.model.listing.Data d = getDataAt(s.getAddress());
            if (d != null && d.isDefined()) {
                String tok = typeToken(d.getLength(), d.getDataType().getName());
                lines.add(fmt(s.getAddress(), "DAT", s.getName() + (tok != null ? "  " + tok : ""), c));
            } else {
                lines.add(fmt(s.getAddress(), "LBL", s.getName(), c));
            }
        }

        Collections.sort(lines);
        PrintWriter pw = new PrintWriter(out);
        pw.println("# BMU symbol map (exported by ExportSymbols.java)");
        pw.println("# format: <addr> <FUN|LBL> <name> [; comment]");
        for (String l : lines) pw.println(l);
        pw.close();
        println("ExportSymbols: wrote " + lines.size() + " symbols to " + out);
    }

    private String typeToken(int len, String name) {
        String n = name.toLowerCase();
        if (n.startsWith("float")) return "f";
        if (n.startsWith("double")) return "d";
        switch (len) { case 1: return "b"; case 2: return "h"; case 4: return "w"; case 8: return "q"; }
        return null;
    }

    private String fmt(Address addr, String kind, String name, String comment) {
        // zero-padded addr so lexical sort == address sort
        String h = String.format("0x%08x", addr.getOffset());
        String base = h + "  " + kind + "  " + name;
        if (comment != null && !comment.isEmpty()) {
            String first = comment.split("\\R", 2)[0].trim();
            base = base + "  ; " + first;
        }
        return base;
    }
}
