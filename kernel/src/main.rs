#![no_std]
#![no_main]

use core::arch::asm;

extern crate alloc;

use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;
use klogger::sprintln;
use limine::{MemmapRequest, MemoryMapEntryType};

#[global_allocator]
static ALLOCATOR: LockedHeap<64> = LockedHeap::empty();

static MEMMAP_REQUEST: MemmapRequest = MemmapRequest::new(0);

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    if let Some(memmap_response) = MEMMAP_REQUEST.get_response().get() {
        if memmap_response.entry_count < 1 {
            hcf();
        }

        sprintln!("memory map:");
        for entry in (&memmap_response.memmap()).into_iter() {
            if entry.typ == MemoryMapEntryType::Usable {
                unsafe {
                    ALLOCATOR
                        .lock()
                        .add_to_heap(entry.base as usize, (entry.base + entry.len) as usize);
                }
            }

            sprintln!(
                "{:016p} - {:016p} ({:>16}) {:?}",
                &entry.base,
                &(entry.base + entry.len),
                entry.len,
                entry.typ
            );
        }

        let mut vec = Vec::<u64>::new();
        for i in 0..50_000_000 {
            vec.push(i);
        }

        sprintln!("created 50,000,000 items");
    }

    hcf();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    sprintln!("{}", info);
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
