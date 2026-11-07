import init, { SplitEcu } from './pkg/imiev_wasm.js';
import { FRAME_BYTES, H, PUMP_CYCLES, handOver, views, waitPast } from './link.js';

const STATUS_MS = 50;
const WATCHED = [0x285, 0x288, 0x298, 0x373, 0x374, 0x375];

self.onmessage = async ({ data: { sab } }) => {
  await init();
  const car = new SplitEcu();
  const { hdr, bmu, ecu } = views(sab);
  const limit = Atomics.load(hdr, H.LIMIT);
  let gear = 0;
  let pedal = 0;
  let pumps = 0;
  let last = 0;

  const status = () => ({
    cycles: pumps * PUMP_CYCLES,
    mode: car.ecu_op_mode(),
    modeCode: car.ecu_mode_code(),
    torque: car.ecu_torque_request(),
    frames: Object.fromEntries(WATCHED.map((id) => [id, car.bus_last(id)])),
    ecuDtcs: car.ecu_dtcs(),
  });

  for (;;) {
    const g = Atomics.load(hdr, H.GEAR);
    if (g !== gear) car.set_gear((gear = g));
    const p = Atomics.load(hdr, H.PEDAL);
    if (p !== pedal) car.set_pedal((pedal = p));

    car.run_to_pump();
    waitPast(hdr, H.BMU_SEQ, pumps);
    const fromBmu = bmu.slice(0, Atomics.load(hdr, H.BMU_LEN) * FRAME_BYTES);
    handOver(hdr, ecu, car.pump(fromBmu), H.ECU_SEQ, H.ECU_LEN);
    pumps += 1;

    const now = performance.now();
    if (now - last >= STATUS_MS || pumps === limit) {
      last = now;
      self.postMessage(status());
    }
    if (pumps === limit) break;
  }
};
