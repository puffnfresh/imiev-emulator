//! Finds and opens Tactrix OpenPort 2.0 clones over libusb.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusb::{Context, DeviceHandle, Direction, TransferType, UsbContext};

use crate::openport::{Transport, PRODUCT_ID, VENDOR_ID};

const WRITE_TIMEOUT: Duration = Duration::from_secs(2);

pub struct Usb {
    handle: DeviceHandle<Context>,
    iface: u8,
    ep_in: u8,
    ep_out: u8,
}

impl Usb {
    pub fn open() -> Result<Usb, String> {
        let ctx = Context::new().map_err(|e| format!("libusb init: {e}"))?;
        let device = ctx
            .devices()
            .map_err(|e| format!("listing USB devices: {e}"))?
            .iter()
            .find(|d| {
                d.device_descriptor()
                    .is_ok_and(|desc| desc.vendor_id() == VENDOR_ID && desc.product_id() == PRODUCT_ID)
            })
            .ok_or(format!("no OpenPort 2.0 ({VENDOR_ID:04x}:{PRODUCT_ID:04x}) on USB"))?;

        let config = device.config_descriptor(0).map_err(|e| format!("reading config: {e}"))?;
        let (mut iface, mut ep_in, mut ep_out) = (None, None, None);
        for alt in config.interfaces().flat_map(|i| i.descriptors()) {
            if alt.num_endpoints() != 2 {
                continue;
            }
            for ep in alt.endpoint_descriptors().filter(|e| e.transfer_type() == TransferType::Bulk) {
                match ep.direction() {
                    Direction::In => ep_in = Some(ep.address()),
                    Direction::Out => ep_out = Some(ep.address()),
                }
            }
            iface = Some(alt.interface_number());
        }
        let (Some(iface), Some(ep_in), Some(ep_out)) = (iface, ep_in, ep_out) else {
            return Err("adapter has no interface with bulk IN/OUT endpoints".into());
        };

        let handle = device.open().map_err(|e| {
            format!("opening adapter: {e} (needs a udev rule for {VENDOR_ID:04x}:{PRODUCT_ID:04x}; see README)")
        })?;
        for i in [0, iface] {
            if handle.kernel_driver_active(i).unwrap_or(false) {
                handle.detach_kernel_driver(i).map_err(|e| format!("detaching kernel driver: {e}"))?;
            }
        }
        handle.claim_interface(iface).map_err(|e| format!("claiming interface {iface}: {e}"))?;
        Ok(Usb { handle, iface, ep_in, ep_out })
    }
}

impl Transport for Usb {
    async fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.handle.write_bulk(self.ep_out, bytes, WRITE_TIMEOUT).map(|_| ()).map_err(|e| e.to_string())
    }

    async fn read(&mut self, timeout_ms: u32) -> Result<Vec<u8>, String> {
        let mut buf = [0u8; 4096];
        match self.handle.read_bulk(self.ep_in, &mut buf, Duration::from_millis(timeout_ms.into())) {
            Ok(n) => Ok(buf[..n].to_vec()),
            Err(rusb::Error::Timeout) => Ok(Vec::new()),
            Err(e) => Err(format!("USB read failed: {e}")),
        }
    }

    fn now_us(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_micros() as u64)
    }
}

impl Drop for Usb {
    fn drop(&mut self) {
        let _ = self.handle.release_interface(self.iface);
        let _ = self.handle.attach_kernel_driver(0);
    }
}
