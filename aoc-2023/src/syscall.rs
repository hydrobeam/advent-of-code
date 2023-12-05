use core::{arch::asm, ffi::CStr};

use crate::dbg;

pub unsafe fn exit(code: i32) -> ! {
    let syscall_number: u64 = 60;
    asm!(
        "syscall",
        in("rax") syscall_number,
        in("rdi") code,
        options(noreturn)
    )
}

pub const STDOUT_FILENO: u32 = 1;
pub const STDERR_FILENO: u32 = 2;

pub unsafe fn write(fd: u32, buf: *const u8, count: usize) -> usize {
    let syscall_number: usize = 1;
    let rax: usize;
    asm!(
        "syscall",
        inout("rax") syscall_number => rax,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        lateout("rcx") _, lateout("r11") _,
        options(nostack)
    );
    rax
}

#[derive(Debug, Copy, Clone)]
pub struct FileDescriptor(i32);

impl FileDescriptor {
    pub const NO_FILE: FileDescriptor = FileDescriptor(-1);
    fn inner(&self) -> i32 {
        self.0
    }
}

impl From<i32> for FileDescriptor {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

pub unsafe fn mmap(
    addr: usize,
    len: usize,
    prot: i32,
    flags: u32,
    fd: FileDescriptor,
    offset: i32,
) -> *mut u8 {
    let syscall_number = 9;
    let mut rax: usize = syscall_number;

    asm!(
        "syscall",
        inout("rax") rax,
        in("rdi") addr,
        in("rsi") len,
        in("rdx") prot,
        in("r10") flags,
        in("r8") fd.inner(),
        in("r9") offset,
        lateout("rcx") _, lateout("r11") _,
        options(nostack)
    );

    if rax as isize <= -1 && rax as isize >= -4096 {
        panic!("mmap call failed with error code: {}", (rax as isize).abs())
    }

    rax as *mut u8
}

pub unsafe fn open(filename: &CStr, flags: u32, mode: i32) -> FileDescriptor {
    let syscall_number = 2;
    let mut rax: i32 = syscall_number;

    // let null_terminated = CString::new(filename).expect("file name contains a 0 byte");
    asm!(
        "syscall",
        inout("rax") rax,
        in("rdi") filename.as_ptr(),
        in("rsi") flags,
        in("rdx") mode,
        lateout("rcx") _, lateout("r11") _,
        options(nostack)
    );

    rax.into()
}

pub unsafe fn read(fd: FileDescriptor, buf: *mut u8, count: usize) -> FileDescriptor {
    let syscall_number = 0;
    let mut rax: i32 = syscall_number;

    asm!(
        "syscall",
        inout("rax") rax,
        in("rdi") fd.inner(),
        in("rsi") buf,
        in("rdx") count,
        lateout("rcx") _, lateout("r11") _,
        options(nostack)
    );

    rax.into()
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
    __unused: [i64; 3],
}

// pub unsafe extern "C" fn fstat(fildes: c_int, buf: *mut stat) -> c_int;
pub unsafe fn fstat(fd: FileDescriptor, buf: &mut Stat) {
    let syscall_number = 5;
    // let mut rax: i32 = syscall_number;

    asm!(
        "syscall",
        inout("rax") syscall_number => _,
        in("rdi") fd.inner(),
        in("rsi") buf as *mut Stat,
        lateout("rcx") _, lateout("r11") _,
        options(nostack)
    );
}

pub unsafe fn getcwd(buf: &mut [u8], size: usize) {
    let syscall_number = 79;
    // let mut rax: i32 = syscall_number;

    asm!(
        "syscall",
        inout("rax") syscall_number => _,
        in("rdi") buf.as_ptr(),
        in("rsi") size,
        lateout("rcx") _, lateout("r11") _,
        options(nostack)
    );
}
