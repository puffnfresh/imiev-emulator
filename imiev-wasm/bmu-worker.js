import init, { SplitBmu } from './pkg/imiev_wasm.js';
import { FRAME_BYTES, H, handOver, views, waitPast } from './link.js';

const STATUS_MS = 250;

self.onmessage = async ({ data: { sab } }) => {
  await init();
  const chip = new SplitBmu();
  const { hdr, bmu, ecu } = views(sab);
  const limit = Atomics.load(hdr, H.LIMIT);
  let pumps = 0;
  let last = 0;

  for (;;) {
    chip.run_to_pump();
    handOver(hdr, bmu, chip.drain_tx(), H.BMU_SEQ, H.BMU_LEN);
    waitPast(hdr, H.ECU_SEQ, pumps);
    chip.receive(ecu.slice(0, Atomics.load(hdr, H.ECU_LEN) * FRAME_BYTES));
    pumps += 1;

    const now = performance.now();
    if (now - last >= STATUS_MS || pumps === limit) {
      last = now;
      self.postMessage({ bmuDtcs: chip.bmu_dtcs() });
    }
    if (pumps === limit) break;
  }
};
