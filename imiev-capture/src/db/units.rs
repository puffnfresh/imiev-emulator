//! The units of the values in our database.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    None,
    AmpHours,
    Amps,
    Celsius,
    KiloAmpHours,
    KilowattHours,
    Kilowatts,
    Kilohms,
    Km,
    KmPerHour,
    MegaPascals,
    MetresPerSecondSquared,
    Millimetres,
    Milliohms,
    Minutes,
    Months,
    NewtonMetres,
    Percent,
    Rpm,
    Seconds,
    Volts,
}

impl Unit {
    pub fn symbol(self) -> &'static str {
        match self {
            Unit::None => "",
            Unit::AmpHours => "Ah",
            Unit::Amps => "A",
            Unit::Celsius => "degC",
            Unit::KiloAmpHours => "kAh",
            Unit::KilowattHours => "kWh",
            Unit::Kilowatts => "kW",
            Unit::Kilohms => "kohm",
            Unit::Km => "km",
            Unit::KmPerHour => "km/h",
            Unit::MegaPascals => "MPa",
            Unit::MetresPerSecondSquared => "m/s^2",
            Unit::Millimetres => "mm",
            Unit::Milliohms => "mohm",
            Unit::Minutes => "min",
            Unit::Months => "months",
            Unit::NewtonMetres => "Nm",
            Unit::Percent => "%",
            Unit::Rpm => "r/min",
            Unit::Seconds => "s",
            Unit::Volts => "V",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AmpHours(pub f64);

impl fmt::Display for AmpHours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::AmpHours.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Amps(pub f64);

impl fmt::Display for Amps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Amps.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f64);

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Celsius.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KiloAmpHours(pub f64);

impl fmt::Display for KiloAmpHours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::KiloAmpHours.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KilowattHours(pub f64);

impl fmt::Display for KilowattHours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::KilowattHours.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kilowatts(pub f64);

impl fmt::Display for Kilowatts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Kilowatts.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kilohms(pub f64);

impl fmt::Display for Kilohms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Kilohms.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Km(pub f64);

impl fmt::Display for Km {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Km.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KmPerHour(pub f64);

impl fmt::Display for KmPerHour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::KmPerHour.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MegaPascals(pub f64);

impl fmt::Display for MegaPascals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::MegaPascals.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MetresPerSecondSquared(pub f64);

impl fmt::Display for MetresPerSecondSquared {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::MetresPerSecondSquared.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Millimetres(pub f64);

impl fmt::Display for Millimetres {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Millimetres.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Milliohms(pub f64);

impl fmt::Display for Milliohms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Milliohms.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Minutes(pub f64);

impl fmt::Display for Minutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Minutes.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Months(pub f64);

impl fmt::Display for Months {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Months.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NewtonMetres(pub f64);

impl fmt::Display for NewtonMetres {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::NewtonMetres.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percent(pub f64);

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Percent.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rpm(pub f64);

impl fmt::Display for Rpm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Rpm.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Seconds(pub f64);

impl fmt::Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Seconds.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Volts(pub f64);

impl fmt::Display for Volts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, Unit::Volts.symbol())
    }
}
