//! Informacion del procesador via CPUID.
use core::arch::x86_64::__cpuid;

pub struct CpuInfo {
    vendor: [u8; 12],
    brand: [u8; 48],
}

impl CpuInfo {
    pub fn read() -> Self {
        // SAFETY: CPUID existe en todo x86_64.
        #[allow(unused_unsafe)]
        let (leaf0, brand) = unsafe {
            let leaf0 = __cpuid(0);
            let mut brand = [0u8; 48];
            for i in 0..3u32 {
                let r = __cpuid(0x8000_0002 + i);
                let o = i as usize * 16;
                brand[o..o + 4].copy_from_slice(&r.eax.to_le_bytes());
                brand[o + 4..o + 8].copy_from_slice(&r.ebx.to_le_bytes());
                brand[o + 8..o + 12].copy_from_slice(&r.ecx.to_le_bytes());
                brand[o + 12..o + 16].copy_from_slice(&r.edx.to_le_bytes());
            }
            (leaf0, brand)
        };

        let mut vendor = [0u8; 12];
        vendor[0..4].copy_from_slice(&leaf0.ebx.to_le_bytes());
        vendor[4..8].copy_from_slice(&leaf0.edx.to_le_bytes());
        vendor[8..12].copy_from_slice(&leaf0.ecx.to_le_bytes());

        Self { vendor, brand }
    }

    pub fn vendor(&self) -> &str {
        core::str::from_utf8(&self.vendor).unwrap_or("?")
    }

    pub fn brand(&self) -> &str {
        core::str::from_utf8(&self.brand)
            .unwrap_or("?")
            .trim_matches(|c| c == '\0' || c == ' ')
    }
}
