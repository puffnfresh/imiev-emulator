//! Serial lines to ELM327-style adapters

use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};
use std::os::unix::fs::OpenOptionsExt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::Transport;

const BTPROTO_RFCOMM: libc::c_int = 3;
const RFCOMM_CHANNEL: u8 = 1;

pub struct Serial {
    file: File,
}

impl Serial {
    pub fn open(path: &str, baud: u32) -> Result<Serial, String> {
        let speed = speed(baud).ok_or(format!("unsupported serial speed {baud}"))?;
        let file = OpenOptions::new().read(true).write(true).custom_flags(libc::O_NOCTTY).open(path).map_err(|e| {
            let hint = if e.kind() == ErrorKind::PermissionDenied { " (is the user in the dialout group?)" } else { "" };
            format!("{path}: {e}{hint}")
        })?;
        let fd = file.as_raw_fd();
        unsafe {
            let mut t: libc::termios = std::mem::zeroed();
            if libc::tcgetattr(fd, &mut t) != 0 {
                return Err(format!("{path}: not a serial device: {}", io::Error::last_os_error()));
            }
            libc::cfmakeraw(&mut t);
            t.c_cflag |= libc::CLOCAL | libc::CREAD;
            t.c_cc[libc::VMIN] = 0;
            t.c_cc[libc::VTIME] = 0;
            libc::cfsetispeed(&mut t, speed);
            libc::cfsetospeed(&mut t, speed);
            if libc::tcsetattr(fd, libc::TCSANOW, &t) != 0 {
                return Err(format!("{path}: setting {baud} baud: {}", io::Error::last_os_error()));
            }
            libc::tcflush(fd, libc::TCIOFLUSH);
        }
        Ok(Serial { file })
    }

    pub fn tcp(address: &str) -> Result<Serial, String> {
        let stream = TcpStream::connect(address).map_err(|e| format!("connecting to {address}: {e}"))?;
        stream.set_nodelay(true).map_err(|e| format!("{address}: {e}"))?;
        Ok(Serial { file: unsafe { File::from_raw_fd(stream.into_raw_fd()) } })
    }

    pub fn bluetooth(address: &str) -> Result<Serial, String> {
        let bdaddr = bluetooth_address(address).ok_or(format!("`{address}` is not a Bluetooth address"))?;
        #[repr(C)]
        struct SockaddrRc {
            family: libc::sa_family_t,
            bdaddr: [u8; 6],
            channel: u8,
        }
        let addr = SockaddrRc { family: libc::AF_BLUETOOTH as _, bdaddr, channel: RFCOMM_CHANNEL };
        unsafe {
            let fd = libc::socket(libc::AF_BLUETOOTH, libc::SOCK_STREAM | libc::SOCK_CLOEXEC, BTPROTO_RFCOMM);
            if fd < 0 {
                let e = io::Error::last_os_error();
                let hint = if e.raw_os_error() == Some(libc::EAFNOSUPPORT) { " (no Bluetooth support loaded?)" } else { "" };
                return Err(format!("Bluetooth socket: {e}{hint}"));
            }
            let file = File::from_raw_fd(fd);
            let len = size_of::<SockaddrRc>() as libc::socklen_t;
            if libc::connect(fd, (&raw const addr).cast(), len) != 0 {
                let e = io::Error::last_os_error();
                return Err(format!("connecting to {address}: {e} (is it on, and paired with bluetoothctl?)"));
            }
            Ok(Serial { file })
        }
    }
}

pub fn bluetooth_address(text: &str) -> Option<[u8; 6]> {
    let bytes: Vec<u8> =
        text.split(':').map(|b| if b.len() == 2 { u8::from_str_radix(b, 16).ok() } else { None }).collect::<Option<_>>()?;
    let mut addr: [u8; 6] = bytes.try_into().ok()?;
    addr.reverse();
    Some(addr)
}

fn speed(baud: u32) -> Option<libc::speed_t> {
    Some(match baud {
        9600 => libc::B9600,
        19200 => libc::B19200,
        38400 => libc::B38400,
        57600 => libc::B57600,
        115200 => libc::B115200,
        230400 => libc::B230400,
        460800 => libc::B460800,
        500000 => libc::B500000,
        921600 => libc::B921600,
        1000000 => libc::B1000000,
        2000000 => libc::B2000000,
        _ => return None,
    })
}

impl Transport for Serial {
    async fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.file.write_all(bytes).map_err(|e| e.to_string())
    }

    async fn read(&mut self, timeout_ms: u32) -> Result<Vec<u8>, String> {
        let mut poll = libc::pollfd { fd: self.file.as_raw_fd(), events: libc::POLLIN, revents: 0 };
        match unsafe { libc::poll(&mut poll, 1, timeout_ms.min(i32::MAX as u32) as i32) } {
            0 => return Ok(Vec::new()),
            n if n < 0 => {
                let e = io::Error::last_os_error();
                return if e.kind() == ErrorKind::Interrupted { Ok(Vec::new()) } else { Err(format!("poll: {e}")) };
            }
            _ => {}
        }
        let mut buf = [0u8; 4096];
        match self.file.read(&mut buf) {
            Ok(0) => Err("the adapter went away".into()),
            Ok(n) => Ok(buf[..n].to_vec()),
            Err(e) if matches!(e.kind(), ErrorKind::Interrupted | ErrorKind::WouldBlock) => Ok(Vec::new()),
            Err(e) => Err(format!("read failed: {e}")),
        }
    }

    fn now_us(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_micros() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bluetooth_addresses() {
        assert_eq!(bluetooth_address("00:1D:A5:68:98:8A"), Some([0x8a, 0x98, 0x68, 0xa5, 0x1d, 0x00]));
        assert_eq!(bluetooth_address("/dev/rfcomm0"), None);
        assert_eq!(bluetooth_address("00:1D:A5:68:98"), None);
        assert_eq!(bluetooth_address("00:1D:A5:68:98:8A:00"), None);
    }
}
