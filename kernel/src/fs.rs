// SPDX-License-Identifier: GPL-3.0-or-later

use core::{mem, str};
use cstr_core::CStr;
use x86_64::VirtAddr;

pub(super) fn list_names() {
    for c in iter() {
        info!(
            "Name: {}, file size: {}, name size: {}",
            c.name(),
            c.header().file_size(),
            c.header().name_size()
        );
    }
}

fn iter() -> impl Iterator<Item = CpioArchievedFile> {
    Iter::default()
}

fn initrd_addr() -> VirtAddr {
    extern "C" {
        static initrd: usize;
    }

    VirtAddr::new(unsafe { &initrd as *const _ as _ })
}

struct CpioArchievedFile {
    ptr: VirtAddr,
}
impl CpioArchievedFile {
    unsafe fn new(ptr: VirtAddr) -> Self {
        assert_eq!(
            &ptr.as_ptr::<[u8; 6]>().read(),
            b"070707",
            "Invalid signature."
        );

        Self { ptr }
    }

    fn header(&self) -> CpioHeader {
        // SAFETY: The caller of the `new` method ensures that `self.header` is the correct
        // address.
        unsafe { self.ptr.as_ptr::<CpioHeader>().read() }
    }

    fn name(&self) -> &str {
        let name_ptr = self.ptr + mem::size_of::<CpioHeader>();

        unsafe {
            let s = CStr::from_ptr(name_ptr.as_ptr()).to_str();
            s.expect("Failed to get the name of a file.")
        }
    }
}

struct Iter {
    ptr: VirtAddr,
}
impl Iterator for Iter {
    type Item = CpioArchievedFile;

    fn next(&mut self) -> Option<Self::Item> {
        let f = unsafe { CpioArchievedFile::new(self.ptr) };

        if f.name() == "TRAILER!!!" {
            None
        } else {
            self.ptr += mem::size_of::<CpioHeader>()
                + usize::from(f.header().name_size() + f.header().file_size());
            Some(f)
        }
    }
}
impl Default for Iter {
    fn default() -> Self {
        Self { ptr: initrd_addr() }
    }
}

#[repr(C, packed)]
struct CpioHeader {
    magic: [u8; 6],
    dev: [u8; 6],
    ino: [u8; 6],
    mode: [u8; 6],
    uid: [u8; 6],
    gid: [u8; 6],
    nlink: [u8; 6],
    rdev: [u8; 6],
    mtime: [u8; 11],
    namesize: [u8; 6],
    filesize: [u8; 11],
}
impl CpioHeader {
    fn name_size(&self) -> u16 {
        byte_array_to_str(&self.namesize)
    }

    fn file_size(&self) -> u16 {
        byte_array_to_str(&self.filesize)
    }
}

fn byte_array_to_str(b: &[u8]) -> u16 {
    let s = str::from_utf8(b).expect("Not the valid UTF-8");
    u16::from_str_radix(s, 8).expect("Radix is out of range.")
}