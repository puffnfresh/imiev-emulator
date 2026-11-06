//! Simulated hardware wired to the BMU and EV-ECU nodes.

mod airbag;
mod battery;
mod charger;
mod driver;
mod eeprom;
mod hv;
mod ic2;
mod inverter;
mod vacuum;

pub use battery::Cmu;
pub use charger::Charger;
pub use driver::{DriverControls, Gear};
pub use eeprom::Eeprom;
pub use hv::{dclink_connected, AcRelay, Condenser, Contactor};
pub use ic2::Ic2Companion;
pub use inverter::{Inverter, Vehicle, VEHICLE_DT};
pub use vacuum::VacuumPump;
pub use airbag::AirbagEcu;
