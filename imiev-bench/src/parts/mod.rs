//! Simulated hardware wired to the BMU and EV-ECU nodes.

mod battery;
mod driver;
mod eeprom;
mod hv;
mod ic2;
mod inverter;

pub use battery::Cmu;
pub use driver::{DriverControls, Gear};
pub use eeprom::Eeprom;
pub use hv::{AcRelay, Condenser, Contactor};
pub use ic2::{Can0RxIsr, Ic2Companion};
pub use inverter::{Inverter, Vehicle, VEHICLE_DT};
