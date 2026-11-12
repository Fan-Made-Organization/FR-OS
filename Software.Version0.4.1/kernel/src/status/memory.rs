//! Memoria: uso real del heap del kernel.
use alloc::format;

use super::{Indicator, Level};
use crate::memory::heap;

pub fn probe() -> Indicator {
    let (used, total) = heap::stats();
    let mib = 1024 * 1024;
    Indicator {
        label: "Memoria",
        level: Level::Ok,
        detail: format!("{} MiB libres de {} MiB (heap)", (total - used) / mib, total / mib),
    }
}
