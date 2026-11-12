//! Heap simple: un bloque fijo dentro de la imagen del kernel.
//! Permite usar Vec, Box y String. Mas adelante se reemplaza por
//! un heap sobre paginas reales.
use core::ptr::addr_of_mut;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_SIZE: usize = 32 * 1024 * 1024;

#[repr(align(4096))]
struct HeapSpace([u8; HEAP_SIZE]);

static mut HEAP_SPACE: HeapSpace = HeapSpace([0; HEAP_SIZE]);

pub fn init() {
    unsafe {
        let start = addr_of_mut!(HEAP_SPACE) as *mut u8;
        ALLOCATOR.lock().init(start, HEAP_SIZE);
    }
}

/// (bytes usados, bytes totales) del heap.
pub fn stats() -> (usize, usize) {
    let heap = ALLOCATOR.lock();
    (heap.used(), heap.size())
}
