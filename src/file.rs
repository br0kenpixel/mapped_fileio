use nix::{
    fcntl::{open, OFlag},
    sys::{
        mman::{mmap, munmap, MapFlags, ProtFlags},
        stat::{fstat, Mode},
    },
    unistd::close,
};
use std::{
    ffi::c_void,
    io::{Error, ErrorKind, Read, Result, Seek, SeekFrom},
    num::NonZeroUsize,
    path::Path,
};

#[derive(Debug)]
pub struct MappedFile {
    fd: i32,
    mem: *mut u8,
    pos: usize,
    size: usize,
}

impl MappedFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let fd = open(
            path.to_str().unwrap(),
            OFlag::O_RDONLY,
            Mode::S_IRUSR | Mode::S_IWUSR,
        )?;
        let stat = fstat(fd)?;
        let file_size = unsafe { NonZeroUsize::new_unchecked(stat.st_size as usize) };

        let mem = (unsafe {
            mmap(
                None,
                file_size,
                ProtFlags::PROT_READ,
                MapFlags::MAP_PRIVATE,
                fd,
                0,
            )
        }?)
        .cast::<u8>();

        Ok(Self {
            fd,
            mem,
            pos: 0,
            size: stat.st_size as usize,
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

/* impl Write for MappedFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.pos + buf.len() > self.size {
            return Err(Error::new(ErrorKind::OutOfMemory, "no space left"));
        }

        let start = unsafe { self.mem.offset(self.pos as isize) };
        unsafe { start.copy_from_nonoverlapping(buf.as_ptr(), buf.len()) }

        self.pos += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        fsync(self.fd).map_err(|err| Error::new(ErrorKind::Other, err.desc()))
    }
} */

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

impl Drop for MappedFile {
    fn drop(&mut self) {
        unsafe { munmap(self.mem.cast::<c_void>(), self.size).unwrap() };
        close(self.fd).unwrap();
    }
}
