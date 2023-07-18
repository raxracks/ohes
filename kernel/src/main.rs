#![no_std]
#![no_main]

use core::arch::asm;

extern crate alloc;
extern crate font8x8;

use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;
use font8x8::{UnicodeFonts, BASIC_FONTS};
use klogger::{sprint, sprintln};
use limine::{FramebufferRequest, MemmapRequest, MemoryMapEntryType};

#[global_allocator]
static ALLOCATOR: LockedHeap<64> = LockedHeap::empty();
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new(0);
static MEMMAP_REQUEST: MemmapRequest = MemmapRequest::new(0);

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        if framebuffer_response.framebuffer_count < 1 {
            hcf();
        }

        let framebuffer = &framebuffer_response.framebuffers()[0];

        let mut x_pos: usize = 0;
        let mut y_pos: usize = 0;
        let mut x_off: usize = 0;
        let mut y_off: usize = 0;
        if let Some(glyph) = BASIC_FONTS.get('h') {
            for x in &glyph {
                for bit in 0..8 {
                    match *x & 1 << bit {
                        0 => x_off += 1,
                        _ => {
                            for x in x_off..x_off + 1_usize {
                                for y in y_off..y_off + 1_usize {
                                    let pixel_offset =
                                        (y_pos + y) * framebuffer.pitch as usize + (x_pos + x) * 4;
                                    unsafe {
                                        *(framebuffer
                                            .address
                                            .as_ptr()
                                            .unwrap()
                                            .offset(pixel_offset as isize)
                                            as *mut u32) = 0xFFFFFFFF;
                                    }
                                }
                            }
                            x_off += 1;
                        }
                    }
                }

                if x_off > x_pos {
                    x_pos = x_off;
                }
                x_off = 0;
                y_off += 1;
            }
            if y_off > y_pos {
                y_pos = y_off;
            }
            x_off = 0;
            y_off = 0;
        }

        if let Some(glyph) = BASIC_FONTS.get('i') {
            for x in &glyph {
                for bit in 0..8 {
                    match *x & 1 << bit {
                        0 => x_off += 1,
                        _ => {
                            for x in x_off..x_off + 1_usize {
                                for y in y_off..y_off + 1_usize {
                                    let pixel_offset =
                                        (y_pos + y) * framebuffer.pitch as usize + (x_pos + x) * 4;
                                    unsafe {
                                        *(framebuffer
                                            .address
                                            .as_ptr()
                                            .unwrap()
                                            .offset(pixel_offset as isize)
                                            as *mut u32) = 0xFFFFFFFF;
                                    }
                                }
                            }
                            x_off += 1;
                        }
                    }
                }

                if x_off > x_pos {
                    x_pos = x_off;
                }
                x_off = 0;
                y_off += 1;
            }
            if y_off > y_pos {
                y_pos = y_off;
            }
            x_off = 0;
            y_off = 0;
        }
    }

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
    }

    hcf();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    sprintln!("{}", info);
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
