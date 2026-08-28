import ghidra.app.script.GhidraScript;
import ghidra.app.cmd.function.ApplyFunctionSignatureCmd;
import ghidra.app.util.parser.FunctionSignatureParser;
import ghidra.program.model.address.Address;
import ghidra.program.model.data.*;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.SourceType;
import ghidra.program.model.symbol.Symbol;
import java.io.BufferedReader;
import java.io.FileReader;

// Apply a version-controlled symbol map onto a freshly analyzed program.
//
//   ghidra-analyzeHeadless <proj> <name> -process <bin> \
//       -scriptPath emulator/scripts -postScript ApplySymbols.java <path-to-map>
//
// Map file format (one entry per line; # = comment, blank lines ignored):
//   <addr> <kind> <name> [type] [; free-text comment]
//     addr  : hex, e.g. 0x00020070  (the function ENTRY point, or a label address)
//     kind  : FUN  -> create/rename a function here
//             LBL  -> create/rename a plain code label here
//             DAT  -> name a data global (renames DAT_xxxx in the decompiler);
//                     optional 4th token defines its type/size:
//                     b=byte h=half(2) w=word(4) f=float(4) d=double(8) q=long(8)
//             SIG  -> apply a C function prototype (sets param/return types + name):
//                     0x00009464  SIG  ushort celltbl_get(ushort id, ushort * out)
//     name  : the symbol name (no spaces)  [for SIG the whole prototype follows]
//     ;...  : optional; becomes the plate comment (FUN) / EOL comment (LBL/DAT)
//
// Idempotent: re-running renames in place and refreshes comments. Names are marked
// USER_DEFINED so they survive and so ExportSymbols round-trips them.
public class ApplySymbols extends GhidraScript {
    public void run() throws Exception {
        String[] a = getScriptArgs();
        String path = (a.length > 0) ? a[0] : "ghidra/symbols/bmu_symbols.txt";
        int applied = 0, created = 0, skipped = 0, ln = 0;

        BufferedReader br = new BufferedReader(new FileReader(path));
        String line;
        while ((line = br.readLine()) != null) {
            ln++;
            String raw = line.trim();
            if (raw.isEmpty() || raw.startsWith("#")) continue;

            String comment = null;
            int sc = raw.indexOf(';');
            if (sc >= 0) { comment = raw.substring(sc + 1).trim(); raw = raw.substring(0, sc).trim(); }

            String[] tok = raw.split("\\s+");
            if (tok.length < 3) { println("line " + ln + ": malformed: " + line); skipped++; continue; }

            long off;
            try { off = Long.decode(tok[0]); }
            catch (Exception e) { println("line " + ln + ": bad addr: " + tok[0]); skipped++; continue; }
            Address addr = toAddr(off);
            String kind = tok[1].toUpperCase();
            String name = tok[2];

            if (kind.equals("FUN")) {
                Function f = getFunctionAt(addr);
                if (f == null) {
                    // Many BMU functions are reached indirectly and never auto-defined.
                    disassemble(addr);
                    f = createFunction(addr, name);
                    if (f != null) created++;
                }
                if (f == null) {
                    Function c = getFunctionContaining(addr);
                    if (c != null && !c.getEntryPoint().equals(addr)) {
                        println("line " + ln + ": " + tok[0] + " is INSIDE " + c.getName()
                                + " @" + c.getEntryPoint() + " (not an entry) -> skipped");
                    } else {
                        println("line " + ln + ": could not create function @ " + tok[0]);
                    }
                    skipped++; continue;
                }
                f.setName(name, SourceType.USER_DEFINED);
                if (comment != null) f.setComment(comment);
                applied++;
            } else if (kind.equals("SIG")) {
                // rest of line after the kind token = a C prototype
                String afterAddr = raw.substring(raw.indexOf(tok[0]) + tok[0].length()).trim();
                String sig = afterAddr.substring(afterAddr.indexOf(tok[1]) + tok[1].length()).trim();
                Function f = getFunctionAt(addr);
                if (f == null) { disassemble(addr); f = createFunction(addr, null); }
                if (f == null) { println("line " + ln + ": SIG no function @ " + tok[0]); skipped++; continue; }
                try {
                    FunctionSignatureParser p = new FunctionSignatureParser(currentProgram.getDataTypeManager(), null);
                    FunctionDefinitionDataType def = p.parse(f.getSignature(), sig);
                    ApplyFunctionSignatureCmd cmd = new ApplyFunctionSignatureCmd(addr, def, SourceType.USER_DEFINED);
                    if (!cmd.applyTo(currentProgram, monitor)) {
                        println("line " + ln + ": SIG apply failed @ " + tok[0] + ": " + cmd.getStatusMsg());
                        skipped++; continue;
                    }
                    if (comment != null) f.setComment(comment);
                    applied++;
                } catch (Exception e) {
                    println("line " + ln + ": SIG parse error @ " + tok[0] + ": " + e.getMessage());
                    skipped++;
                }
            } else if (kind.equals("LBL") || kind.equals("DAT")) {
                Symbol s = createLabel(addr, name, true, SourceType.USER_DEFINED);
                if (s == null) { println("line " + ln + ": could not label @ " + tok[0]); skipped++; continue; }
                if (kind.equals("DAT") && tok.length >= 4) {
                    DataType dt = dtFor(tok[3]);
                    if (dt != null) {
                        try { clearListing(addr, addr.add(dt.getLength() - 1)); createData(addr, dt); }
                        catch (Exception e) { println("line " + ln + ": type '" + tok[3]
                                + "' not applied @ " + tok[0] + " (" + e.getMessage() + ") — label kept"); }
                    }
                }
                if (comment != null) setEOLComment(addr, comment);
                applied++;
            } else {
                println("line " + ln + ": unknown kind '" + kind + "'"); skipped++;
            }
        }
        br.close();
        println("ApplySymbols: applied=" + applied + " (functions created=" + created
                + "), skipped=" + skipped + ", from " + path);
    }

    private DataType dtFor(String t) {
        switch (t.toLowerCase()) {
            case "b": return new ByteDataType();
            case "h": return new ShortDataType();     // 2 bytes
            case "w": return new IntegerDataType();    // 4 bytes
            case "f": return new FloatDataType();      // 4 bytes
            case "d": return new DoubleDataType();     // 8 bytes
            case "q": return new LongLongDataType();   // 8 bytes
            default:  return null;
        }
    }
}
