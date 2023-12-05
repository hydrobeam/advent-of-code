#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(error_in_core)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(lazy_cell)]
#![feature(core_intrinsics)]

pub mod custom_alloc;
mod days;
use alloc::format;
use days::*;
pub mod syscall;
mod utils;

use crate::syscall::FileDescriptor;
use core::arch::asm;
use core::ffi::CStr;

extern crate alloc;

// my custom allocator
use custom_alloc::BumpAllocator;

#[global_allocator]
static mut ALLOCATOR: BumpAllocator = BumpAllocator::empty();

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() -> ! {
    asm!("mov rdi, rsp", "call pre_main", options(noreturn))
}

#[no_mangle]
pub unsafe fn pre_main(stack_top: *const u8) {
    // mmap to obtain memory we can work with to initialize the allocator
    let addr = 0;
    let len = 1 << 30;
    // PROT_WRITE
    let prot = 0x3;

    const MAP_ANONYMOUS: u32 = 0x20;
    const MAP_PRIVATE: u32 = 0x2;
    let flags = MAP_ANONYMOUS | MAP_PRIVATE;

    let fd = FileDescriptor::NO_FILE;
    let off = 0;

    let pa = syscall::mmap(addr, len, prot, flags, fd, off);
    ALLOCATOR.init(pa, len);

    let argc = *stack_top as usize;
    let argv = stack_top.add(8) as *const *const u8;

    let args = core::slice::from_raw_parts(argv, argc).iter().map(|&arg| {
        CStr::from_ptr(arg as *const i8)
            .to_str()
            .expect("invalid utf8 argv")
    });

    main(args);
    syscall::exit(0);
}

fn main(mut args: impl Iterator<Item = &'static str>) {
    let day = args
        .nth(1)
        .expect("pass in a day to run")
        .parse::<u8>()
        .expect("invalid integer");

    // const PATH_MAX: usize = 4096;
    // let mut path_buf: [u8; 4096] = [0; PATH_MAX];
    // unsafe { syscall::getcwd(&mut path_buf, PATH_MAX) };
    // let path = CStr::from_bytes_until_nul(&path_buf).expect("not null terminated");

    let filename = format!("inputs/day{day}.txt\0");

    let fd: FileDescriptor = unsafe {
        // SAFETY: we explicitly define the terminating null byte \0 in the string,
        // so it will succeed. only 1 allocation 😎
        syscall::open(
            CStr::from_bytes_with_nul_unchecked(filename.as_bytes()),
            0,
            0,
        )
    };

    // extract file size so we know how much to mmap
    let mut stat = syscall::Stat::default();
    unsafe { syscall::fstat(fd, &mut stat) }

    // 0x01 => PROT_READ
    // 0x02  => MAP_PRIVATE
    let buf = unsafe { syscall::mmap(0, stat.st_size as usize, 0x01, 0x02, fd, 0) };

    // SAFETY:
    // from_utf8_unchecked: input files are always valid utf8
    // from_raw_parts: assume mmap gives us a proper address
    let input = unsafe {
        core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, stat.st_size as usize))
    };

    let (p1_sol, p2_sol) = match day {
        1 => (Day01::solve_p1(input), Day01::solve_p2(input)),
        _ => todo!(),
    };

    println!(
        r#"Day {day}
  Part 1: {p1_sol}
  Part 2: {p2_sol}
"#
    )
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::println!("{info}");
    core::intrinsics::abort();
}

pub trait Solution {
    fn solve_p1(_input: &str) -> impl core::fmt::Debug + core::fmt::Display;
    fn solve_p2(_input: &str) -> impl core::fmt::Debug + core::fmt::Display;
}
