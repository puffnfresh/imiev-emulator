//! Simulated hardware wired to the BMU and EV-ECU nodes.

mod airbag;
mod battery;
mod charger;
mod driver;
mod eeprom;
mod etacs;
mod hv;
mod ic2;
mod inverter;
mod lamps;
mod leak;
mod steering;
mod vacuum;

pub use battery::Cmu;
pub use charger::Charger;
pub use driver::{DriverControls, Gear};
pub use eeprom::Eeprom;
pub use etacs::{immobiliser_key, Etacs};
pub use hv::{dclink_connected, AcRelay, Condenser, Contactor};
pub use ic2::Ic2Companion;
pub use inverter::{Inverter, Vehicle, VEHICLE_DT};
pub use lamps::BacklampRelay;
pub use leak::LeakDetector;
pub use steering::SteeringAngleSensor;
pub use vacuum::VacuumPump;
pub use airbag::AirbagEcu;
