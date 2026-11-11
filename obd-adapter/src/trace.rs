//! Transport that shows every byte going each way

use crate::Transport;

pub type Show = Box<dyn FnMut(&str)>;

pub struct Trace<T> {
    inner: T,
    show: Option<Show>,
    start_us: u64,
}

impl<T: Transport> Trace<T> {
    pub fn new(inner: T, show: Option<Show>) -> Trace<T> {
        let start_us = inner.now_us();
        Trace { inner, show, start_us }
    }

    fn line(&mut self, arrow: &str, bytes: &[u8]) {
        let ms = self.inner.now_us().saturating_sub(self.start_us) as f64 / 1000.0;
        if let Some(show) = &mut self.show {
            show(&format!("{ms:9.1} ms {arrow} {}", bytes.escape_ascii()));
        }
    }
}

impl<T: Transport> Transport for Trace<T> {
    async fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.line(">", bytes);
        self.inner.write(bytes).await
    }

    async fn read(&mut self, timeout_ms: u32) -> Result<Vec<u8>, String> {
        let bytes = self.inner.read(timeout_ms).await?;
        if !bytes.is_empty() {
            self.line("<", &bytes);
        }
        Ok(bytes)
    }

    fn now_us(&self) -> u64 {
        self.inner.now_us()
    }
}
