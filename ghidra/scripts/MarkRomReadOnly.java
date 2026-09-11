import ghidra.app.script.GhidraScript;
import ghidra.program.model.mem.MemoryBlock;

public class MarkRomReadOnly extends GhidraScript {
    public void run() throws Exception {
        for (MemoryBlock b : currentProgram.getMemory().getBlocks()) {
            if (b.isInitialized() && b.getStart().getOffset() < 0x800000L && b.isWrite()) {
                b.setWrite(false);
            }
        }
    }
}
