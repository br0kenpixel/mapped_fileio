use crate::OpenOptions;
use nix::{
    fcntl::open,
    sys::{
        mman::{mmap, munmap},
        stat::fstat,
    },
    unistd::close,
};
use std::{
    ffi::c_void,
    io::{Error, ErrorKind, Read, Result, Write},
    num::NonZeroUsize,
    path::Path,
    slice::from_raw_parts_mut,
};

/// A memory mapped file. Similar to [`File`](std::fs::File).
#[derive(Debug)]
pub struct MappedFile<'f> {
    fd: i32,
    mem: &'f mut [u8],
    mode: OpenOptions,
}

impl<'f> MappedFile<'f> {
    /// Opens a file mapping it into memory.
    /// Similar to [`File::open()`](std::fs::File::open).
    pub fn open<P: AsRef<Path>>(path: P, mode: OpenOptions) -> Result<Self> {
        let path = path.as_ref();

        let fd = open(path.to_string_lossy().as_ref(), mode.into(), mode.into())?;
        let file_size = NonZeroUsize::new(fstat(fd)?.st_size as usize)
            .ok_or_else(|| Error::new(ErrorKind::Unsupported, "cannot open empty file"))?;

        let mem = unsafe { mmap(None, file_size, mode.into(), mode.into(), fd, 0) }?;
        let slice = unsafe { from_raw_parts_mut(mem.cast::<u8>(), file_size.get()) };

        Ok(Self {
            fd,
            mem: slice,
            mode,
        })
    }
}

impl<'f> Read for MappedFile<'f> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.mem.as_ref().read(buf)
    }
}

impl<'f> Write for MappedFile<'f> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.mode != OpenOptions::ReadWrite {
            return Err(Error::new(ErrorKind::Unsupported, "write not enabled"));
        }

        self.mem.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.mem.flush()
    }
}

impl<'f> Drop for MappedFile<'f> {
    fn drop(&mut self) {
        self.flush().unwrap();
        unsafe { munmap(self.mem.as_mut_ptr().cast::<c_void>(), self.mem.len()).unwrap() };
        close(self.fd).unwrap();
    }
}
