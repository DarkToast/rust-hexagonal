use std::fmt;

#[derive(Debug)]
pub enum Kind {
    Temperature,
    Humidity,
    Pressure,
    ElectricPower
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Temperature => write!(f, "Temperature"),
            Kind::Humidity => write!(f, "Humidity"),
            Kind::Pressure => write!(f, "Pressure"),
            Kind::ElectricPower => write!(f, "ElectricPower")
        }
    }
}