export const FRAME_BYTES = 11;

export const H = {
  BMU_SEQ: 0,
  BMU_LEN: 1,
  ECU_SEQ: 2,
  ECU_LEN: 3,
  GEAR: 4,
  PEDAL: 5,
  LIMIT: 6,
};

const HEADER_BYTES = 64;
const BMU_FRAMES = 256;
const ECU_FRAMES = 2048;
const BMU_OFF = HEADER_BYTES;
const ECU_OFF = BMU_OFF + BMU_FRAMES * FRAME_BYTES;
const TOTAL = ECU_OFF + ECU_FRAMES * FRAME_BYTES;

export const PUMP_CYCLES = 10_000;

export function newLink() {
  return new SharedArrayBuffer(TOTAL);
}

export function views(sab) {
  return {
    hdr: new Int32Array(sab, 0, HEADER_BYTES / 4),
    bmu: new Uint8Array(sab, BMU_OFF, BMU_FRAMES * FRAME_BYTES),
    ecu: new Uint8Array(sab, ECU_OFF, ECU_FRAMES * FRAME_BYTES),
  };
}

export function waitPast(hdr, slot, seen) {
  while (Atomics.load(hdr, slot) === seen) {
    Atomics.wait(hdr, slot, seen);
  }
}

export function handOver(hdr, area, bytes, seq, len) {
  if (bytes.length > area.length) {
    throw new Error(`${bytes.length / FRAME_BYTES} frames in one pump: more than the link holds`);
  }
  area.set(bytes);
  Atomics.store(hdr, len, bytes.length / FRAME_BYTES);
  Atomics.add(hdr, seq, 1);
  Atomics.notify(hdr, seq);
}
