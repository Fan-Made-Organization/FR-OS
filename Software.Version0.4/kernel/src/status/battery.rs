//! Bateria: se lee por ACPI (necesita un interprete de AML). Aun no existe.
use alloc::string::String;

use super::{Indicator, Level};

pub fn probe() -> Indicator {
    Indicator {
        label: "Bateria",
        level: Level::Off,
        detail: String::from("Sin driver ACPI: no se puede leer todavia"),
    }
}
