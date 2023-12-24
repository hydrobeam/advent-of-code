#![no_std]
#![no_main]
#![allow(internal_features)]
#![allow(unused_imports)]
#![feature(error_in_core)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(lazy_cell)]
#![feature(core_intrinsics)]
#![feature(iter_array_chunks)]
#![feature(slice_partition_dedup)]

pub mod custom_alloc;
mod days;
mod solutions;
pub mod syscall;
mod utils;

use crate::syscall::FileDescriptor;
use alloc::format;
use core::arch::asm;
use core::ffi::CStr;
use days::*;
use solutions::Solution;

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

macro_rules! output_sol {
    ($day:ident, $v1:ident, $v2:ident) => {
        $crate::println!(
            r#"
Day {}
    Part 1: {}
    Part 2: {}
"#,
            $day,
            $v1,
            $v2
        )
    };
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
        2 => (Day02::solve_p1(input), Day02::solve_p2(input)),
        3 => (Day03::solve_p1(input), Day03::solve_p2(input)),
        4 => (Day04::solve_p1(input), Day04::solve_p2(input)),
        5 => (Day05::solve_p1(input), Day05::solve_p2(input)),
        6 => (Day06::solve_p1(input), Day06::solve_p2(input)),
        7 => (Day07::solve_p1(input), Day07::solve_p2(input)),
        8 => (Day08::solve_p1(input), Day08::solve_p2(input)),
        9 => (Day09::solve_p1(input), Day09::solve_p2(input)),
        10 => (Day10::solve_p1(input), Day10::solve_p2(input)),
        11 => (Day11::solve_p1(input), Day11::solve_p2(input)),
        12 => (Day12::solve_p1(input), Day12::solve_p2(input)),
        13 => (Day13::solve_p1(input), Day13::solve_p2(input)),
        14 => (Day14::solve_p1(input), Day14::solve_p2(input)),
        15 => (Day15::solve_p1(input), Day15::solve_p2(input)),
        16 => (Day16::solve_p1(input), Day16::solve_p2(input)),
        17 => (Day17::solve_p1(input), Day17::solve_p2(input)),
        18 => (Day18::solve_p1(input), Day18::solve_p2(input)),
        19 => (Day19::solve_p1(input), Day19::solve_p2(input)),
        20 => (Day20::solve_p1(input), Day20::solve_p2(input)),
        21 => (Day21::solve_p1(input), Day21::solve_p2(input)),
        22 => (Day22::solve_p1(input), Day22::solve_p2(input)),
        23 => (Day23::solve_p1(input), Day23::solve_p2(input)),
        24 => (Day24::solve_p1(input), Day24::solve_p2(input)),
        25 => (Day25::solve_p1(input), Day25::solve_p2(input)),
        _ => todo!(),
    };

    match (p1_sol, p2_sol) {
        (Ok(s1), Ok(s2)) => output_sol!(day, s1, s2),
        (Ok(s1), Err(e)) => {
            output_sol!(day, s1, e)
        }
        (Err(e), Ok(s2)) => {
            output_sol!(day, e, s2)
        }
        (Err(e1), Err(e2)) => {
            output_sol!(day, e1, e2)
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::println!("{info}");
    core::intrinsics::abort();
}
