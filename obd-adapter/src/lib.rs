//! Car diagnostic adapters

pub mod can;
pub mod elm327;
pub mod openport;
#[cfg(all(feature = "serial", target_os = "linux"))]
pub mod serial;
pub mod trace;
#[cfg(feature = "usb")]
pub mod usb;

pub trait Transport {
    fn write(&mut self, bytes: &[u8]) -> impl Future<Output = Result<(), String>>;
    fn read(&mut self, timeout_ms: u32) -> impl Future<Output = Result<Vec<u8>, String>>;
    fn now_us(&self) -> u64;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Frame { time_us: u64, frame: can::Frame },
    Warning(String),
    Other,
}

pub trait CanAdapter {
    fn send(&mut self, frame: can::Frame) -> impl Future<Output = Result<(), String>>;
    fn recv(&mut self, timeout_ms: u32) -> impl Future<Output = Result<Vec<Event>, String>>;
    fn filter(&mut self, _id: Option<u32>) {}
    fn now_us(&self) -> u64;
    fn keeps_up_with_bursts(&self) -> bool;
    fn skipped_bytes(&self) -> u64 {
        0
    }
    fn close(&mut self) -> impl Future<Output = ()>;
}
