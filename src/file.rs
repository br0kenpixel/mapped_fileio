use crate::OpenOptions;
use nix::{
    fcntl::open,
    sys::{
        mman::{mmap, msync, munmap, MsFlags},
        stat::fstat,
    },
    unistd::{close, fsync},
};
use std::{
    ffi::c_void,
    io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write},
    num::NonZeroUsize,
    path::Path,
};

/// A memory mapped file. Similar to [`File`](std::fs::File).
#[derive(Debug)]
pub struct MappedFile {
    fd: i32,
    mem: *mut u8,
    pos: usize,
    size: usize,
    mode: OpenOptions,
}

impl MappedFile {
    /// Opens a file mapping it into memory.
    /// Similar to [`File::open()`](std::fs::File::open).
    pub fn open<P: AsRef<Path>>(path: P, mode: OpenOptions) -> Result<Self> {
        let path = path.as_ref();

        let fd = open(path.to_str().unwrap(), mode.into(), mode.into())?;
        let stat = fstat(fd)?;
        let file_size;
        let mem;

        unsafe {
            file_size = NonZeroUsize::new(stat.st_size as usize)
                .ok_or_else(|| Error::new(ErrorKind::Unsupported, "cannot open empty file"))?;
            mem = mmap(None, file_size, mode.into(), mode.into(), fd, 0)?;
        }

        let mem = mem.cast::<u8>();

        Ok(Self {
            fd,
            mem,
            pos: 0,
            size: stat.st_size as usize,
            mode,
        })
    }
}

impl Read for MappedFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let start = unsafe { self.mem.offset(self.pos as isize) };
        let read = buf.len().clamp(0, self.size - self.pos);

        unsafe { start.copy_to_nonoverlapping(buf.as_mut_ptr(), read) };
        self.pos += read;

        Ok(read)
    }
}

impl Write for MappedFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.mode != OpenOptions::ReadWrite {
            return Err(Error::new(ErrorKind::Unsupported, "write not enabled"));
        }

        if self.pos + buf.len() > self.size {
            return Err(Error::new(ErrorKind::OutOfMemory, "no space left"));
        }

        let start = unsafe { self.mem.offset(self.pos as isize) };
        unsafe { start.copy_from_nonoverlapping(buf.as_ptr(), buf.len()) }

        self.pos += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        unsafe { msync(self.mem.cast::<c_void>(), self.size, MsFlags::MS_SYNC) }?;
        fsync(self.fd).map_err(|err| Error::new(ErrorKind::Other, err.desc()))
    }
}

impl Seek for MappedFile {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let new_pos = match pos {
            SeekFrom::Current(offset) => self.pos + offset as usize,
            SeekFrom::Start(offset) => offset as usize,
            SeekFrom::End(offset) => self.size - offset as usize,
        };

        if !(0..self.size).contains(&new_pos) {
            return Err(Error::new(ErrorKind::Unsupported, "seek beyond limits"));
        }

        self.pos = new_pos;
        Ok(new_pos as u64)
    }
}

impl Iterator for MappedFile {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.size {
            return None;
        }

        let c = unsafe { self.mem.offset(self.pos as isize).read() };
        self.pos += 1;
        Some(c)
    }
}

impl Drop for MappedFile {
    fn drop(&mut self) {
        self.flush().unwrap();
        unsafe { munmap(self.mem.cast::<c_void>(), self.size).unwrap() };
        close(self.fd).unwrap();
    }
}
