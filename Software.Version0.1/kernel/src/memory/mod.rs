//! Memoria. Por ahora solo leemos el mapa que nos da el bootloader.
//! Despues: frames, paginacion y heap.
use bootloader_api::info::{MemoryRegionKind, MemoryRegions};

pub fn usable_kib(regions: &MemoryRegions) -> u64 {
    regions
        .iter()
        .filter(|r| r.kind == MemoryRegionKind::Usable)
        .map(|r| (r.end - r.start) / 1024)
        .sum()
}
