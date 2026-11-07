import init, { Sim } from './pkg/imiev_wasm.js';
import { H, newLink, views } from './link.js';

const RELOADED = 'imiev-coi-reloaded';
const STEP_CHUNK = 1_000_000;
const WATCHED = [0x285, 0x288, 0x298, 0x373, 0x374, 0x375];

export async function ensureIsolation() {
  if (self.crossOriginIsolated) {
    try { sessionStorage.removeItem(RELOADED); } catch { /* storage unavailable */ }
    return true;
  }
  if (!self.isSecureContext || !('serviceWorker' in navigator)) return false;
  try {
    await navigator.serviceWorker.register(new URL('./coi-sw.js', import.meta.url));
    await navigator.serviceWorker.ready;
  } catch {
    return false;
  }
  try {
    if (sessionStorage.getItem(RELOADED)) return false;
    sessionStorage.setItem(RELOADED, '1');
  } catch {
    return false;
  }
  location.reload();
  return new Promise(() => {});
}

export function startThreaded(onStatus, { limit = 0 } = {}) {
  const sab = newLink();
  const { hdr } = views(sab);
  Atomics.store(hdr, H.LIMIT, limit);
  const worker = (file) => new Worker(new URL(file, import.meta.url), { type: 'module' });
  const ecu = worker('./ecu-worker.js');
  const bmu = worker('./bmu-worker.js');
  let bmuDtcs = [];
  bmu.onmessage = ({ data }) => { bmuDtcs = data.bmuDtcs; };
  ecu.onmessage = ({ data }) => onStatus({ ...data, bmuDtcs, threads: 2 });
  for (const w of [ecu, bmu]) {
    w.onerror = (e) => onStatus({ error: e.message ?? String(e) });
    w.postMessage({ sab });
  }
  return {
    setGear: (g) => Atomics.store(hdr, H.GEAR, g),
    setPedal: (pct) => Atomics.store(hdr, H.PEDAL, Math.round(pct)),
    stop: () => { ecu.terminate(); bmu.terminate(); },
  };
}

export async function startSingle(onStatus) {
  await init();
  const sim = new Sim();
  let cycles = 0;
  const frame = () => {
    sim.run(STEP_CHUNK);
    cycles += STEP_CHUNK;
    onStatus({
      cycles,
      mode: sim.ecu_op_mode(),
      modeCode: sim.ecu_mode_code(),
      torque: sim.ecu_torque_request(),
      frames: Object.fromEntries(WATCHED.map((id) => [id, sim.bus_last(id)])),
      ecuDtcs: sim.ecu_dtcs(),
      bmuDtcs: sim.bmu_dtcs(),
      threads: 1,
    });
    requestAnimationFrame(frame);
  };
  requestAnimationFrame(frame);
  return {
    setGear: (g) => sim.set_gear(g),
    setPedal: (pct) => sim.set_pedal(pct),
  };
}
