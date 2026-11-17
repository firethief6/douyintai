//! 跨平台宿主文件元数据与定位读写。
//!
//! guest 只关心模式位、大小和时间，Windows 用现有字段近似，不追求与 Unix 完全一致。

use std::io;

#[cfg(unix)]
pub(crate) use std::os::unix::fs::MetadataExt;

#[cfg(windows)]
pub(crate) trait MetadataExt {
    fn mode(&self) -> u32;
    fn ino(&self) -> u64;
    fn dev(&self) -> u64;
    fn nlink(&self) -> u64;
    fn uid(&self) -> u32;
    fn gid(&self) -> u32;
    fn rdev(&self) -> u64;
    fn size(&self) -> u64;
    fn blocks(&self) -> u64;
    fn atime(&self) -> i64;
    fn atime_nsec(&self) -> i64;
    fn mtime(&self) -> i64;
    fn mtime_nsec(&self) -> i64;
    fn ctime(&self) -> i64;
    fn ctime_nsec(&self) -> i64;
}

#[cfg(windows)]
impl MetadataExt for std::fs::Metadata {
    fn mode(&self) -> u32 {
        use std::os::windows::fs::MetadataExt as _;
        if self.is_dir() {
            0o040755
        } else if self.file_attributes() & 0x400 != 0 {
            0o120777
        } else {
            0o100644
        }
    }
    fn ino(&self) -> u64 {
        0
    }
    fn dev(&self) -> u64 {
        0
    }
    fn nlink(&self) -> u64 {
        1
    }
    fn uid(&self) -> u32 {
        0
    }
    fn gid(&self) -> u32 {
        0
    }
    fn rdev(&self) -> u64 {
        0
    }
    fn size(&self) -> u64 {
        use std::os::windows::fs::MetadataExt as _;
        if self.is_dir() { 0 } else { self.file_size() }
    }
    fn blocks(&self) -> u64 {
        self.size().div_ceil(512)
    }
    fn atime(&self) -> i64 {
        use std::os::windows::fs::MetadataExt as _;
        filetime(self.last_access_time()).0
    }
    fn atime_nsec(&self) -> i64 {
        use std::os::windows::fs::MetadataExt as _;
        filetime(self.last_access_time()).1
    }
    fn mtime(&self) -> i64 {
        use std::os::windows::fs::MetadataExt as _;
        filetime(self.last_write_time()).0
    }
    fn mtime_nsec(&self) -> i64 {
        use std::os::windows::fs::MetadataExt as _;
        filetime(self.last_write_time()).1
    }
    fn ctime(&self) -> i64 {
        use std::os::windows::fs::MetadataExt as _;
        filetime(self.creation_time()).0
    }
    fn ctime_nsec(&self) -> i64 {
        use std::os::windows::fs::MetadataExt as _;
        filetime(self.creation_time()).1
    }
}

/// FILETIME（1601 起 100ns）转 Unix 秒与纳秒。
#[cfg(windows)]
fn filetime(value: u64) -> (i64, i64) {
    const OFFSET: u64 = 116_444_736_000_000_000;
    let value = value.saturating_sub(OFFSET);
    ((value / 10_000_000) as i64, ((value % 10_000_000) * 100) as i64)
}

pub(crate) trait FileExt {
    fn read_at(&self, buffer: &mut [u8], offset: u64) -> io::Result<usize>;
}

#[cfg(unix)]
impl FileExt for std::fs::File {
    fn read_at(&self, buffer: &mut [u8], offset: u64) -> io::Result<usize> {
        std::os::unix::fs::FileExt::read_at(self, buffer, offset)
    }
}

#[cfg(windows)]
impl FileExt for std::fs::File {
    fn read_at(&self, buffer: &mut [u8], offset: u64) -> io::Result<usize> {
        std::os::windows::fs::FileExt::seek_read(self, buffer, offset)
    }
}
