//! Tabla de interrupciones (IDT): excepciones de la CPU y IRQ de hardware.
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use super::{gdt, pic, pit};
use crate::crash;
use crate::drivers::input;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // excepciones
        idt.divide_error.set_handler_fn(divide_error);
        idt.invalid_opcode.set_handler_fn(invalid_opcode);
        idt.breakpoint.set_handler_fn(breakpoint);
        idt.general_protection_fault.set_handler_fn(general_protection);
        idt.page_fault.set_handler_fn(page_fault);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // hardware
        idt[pic::TIMER_VECTOR].set_handler_fn(timer);
        idt[pic::KEYBOARD_VECTOR].set_handler_fn(keyboard);
        idt[pic::MOUSE_VECTOR].set_handler_fn(mouse);
        idt
    };
}

pub fn init() {
    IDT.load();
}

fn read_cr2() -> u64 {
    let value: u64;
    unsafe {
        core::arch::asm!("mov {}, cr2", out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

// ───────── excepciones ─────────

extern "x86-interrupt" fn divide_error(frame: InterruptStackFrame) {
    crash::show("DIVISION POR CERO", format_args!("{:#?}", frame))
}

extern "x86-interrupt" fn invalid_opcode(frame: InterruptStackFrame) {
    crash::show("INSTRUCCION INVALIDA", format_args!("{:#?}", frame))
}

extern "x86-interrupt" fn breakpoint(frame: InterruptStackFrame) {
    crate::log!("breakpoint: {:?}", frame);
}

extern "x86-interrupt" fn general_protection(frame: InterruptStackFrame, code: u64) {
    crash::show(
        "FALLO DE PROTECCION GENERAL",
        format_args!("codigo: {}\n{:#?}", code, frame),
    )
}

extern "x86-interrupt" fn page_fault(frame: InterruptStackFrame, code: PageFaultErrorCode) {
    crash::show(
        "FALLO DE PAGINA",
        format_args!("direccion: {:#x}\ncodigo: {:?}\n{:#?}", read_cr2(), code, frame),
    )
}

extern "x86-interrupt" fn double_fault(frame: InterruptStackFrame, code: u64) -> ! {
    crash::show("DOBLE FALLO", format_args!("codigo: {}\n{:#?}", code, frame))
}

// ───────── hardware ─────────

extern "x86-interrupt" fn timer(_frame: InterruptStackFrame) {
    pit::on_tick();
    pic::eoi(0);
}

extern "x86-interrupt" fn keyboard(_frame: InterruptStackFrame) {
    let byte = unsafe { Port::<u8>::new(0x60).read() };
    input::push_keyboard(byte);
    pic::eoi(1);
}

extern "x86-interrupt" fn mouse(_frame: InterruptStackFrame) {
    let byte = unsafe { Port::<u8>::new(0x60).read() };
    input::push_mouse(byte);
    pic::eoi(12);
}
