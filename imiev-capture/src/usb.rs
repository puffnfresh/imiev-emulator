//! Finds, opens and sends commands to Tactrix OpenPort 2.0 clones.

use std::time::{Duration, Instant};

use rusb::{Context, DeviceHandle, Direction, TransferType, UsbContext};

use crate::can;
use crate::openport::{Command, Parser, Reply, PRODUCT_ID, VENDOR_ID};

const REPLY_TIMEOUT: Duration = Duration::from_secs(2);

pub struct OpenPort {
    handle: DeviceHandle<Context>,
    iface: u8,
    ep_in: u8,
    ep_out: u8,
    parser: Parser,
}

impl OpenPort {
    pub fn open() -> Result<OpenPort, String> {
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
            format!("opening adapter: {e} (needs a udev rule for {VENDOR_ID:04x}:{PRODUCT_ID:04x})")
        })?;
        for i in [0, iface] {
            if handle.kernel_driver_active(i).unwrap_or(false) {
                handle.detach_kernel_driver(i).map_err(|e| format!("detaching kernel driver: {e}"))?;
            }
        }
        handle.claim_interface(iface).map_err(|e| format!("claiming interface {iface}: {e}"))?;
        Ok(OpenPort { handle, iface, ep_in, ep_out, parser: Parser::default() })
    }

    pub fn command(&mut self, cmd: Command) -> Result<String, String> {
        self.write(cmd)?;
        let deadline = Instant::now() + REPLY_TIMEOUT;
        while Instant::now() < deadline {
            for reply in self.read(Duration::from_millis(100))? {
                if let Reply::Text(line) = reply {
                    if line.starts_with("are") {
                        return Err(format!("{cmd:?}: adapter error `{line}`"));
                    }
                    if line.starts_with(cmd.ack()) {
                        return Ok(line);
                    }
                }
            }
        }
        Err(format!("{cmd:?}: no `{}` reply within {REPLY_TIMEOUT:?}", cmd.ack()))
    }

    pub fn transmit(&mut self, frame: can::Frame) -> Result<(), String> {
        self.write(Command::Transmit(frame))
    }

    fn write(&mut self, cmd: Command) -> Result<(), String> {
        self.handle
            .write_bulk(self.ep_out, &cmd.bytes(), REPLY_TIMEOUT)
            .map(|_| ())
            .map_err(|e| format!("{cmd:?}: write failed: {e}"))
    }

    pub fn read(&mut self, timeout: Duration) -> Result<Vec<Reply>, String> {
        let mut buf = [0u8; 4096];
        match self.handle.read_bulk(self.ep_in, &mut buf, timeout) {
            Ok(n) => self.parser.push(&buf[..n]),
            Err(rusb::Error::Timeout) => {}
            Err(e) => return Err(format!("USB read failed: {e}")),
        }
        Ok(std::iter::from_fn(|| self.parser.next_reply()).collect())
    }

    pub fn skipped_bytes(&self) -> u64 {
        self.parser.skipped
    }
}

impl Drop for OpenPort {
    fn drop(&mut self) {
        let _ = self.handle.release_interface(self.iface);
        let _ = self.handle.attach_kernel_driver(0);
    }
}
