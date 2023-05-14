use nix::{
    fcntl::OFlag,
    sys::{
        mman::{MapFlags, ProtFlags},
        stat::Mode,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenOptions {
    ReadOnly,
    ReadWrite,
}

impl From<OpenOptions> for OFlag {
    fn from(value: OpenOptions) -> Self {
        match value {
            OpenOptions::ReadOnly => OFlag::O_RDONLY,
            OpenOptions::ReadWrite => OFlag::O_RDWR,
        }
    }
}

impl From<OpenOptions> for Mode {
    fn from(_: OpenOptions) -> Self {
        Mode::S_IRUSR | Mode::S_IWUSR
    }
}

impl From<OpenOptions> for ProtFlags {
    fn from(value: OpenOptions) -> Self {
        match value {
            OpenOptions::ReadOnly => ProtFlags::PROT_READ,
            OpenOptions::ReadWrite => ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
        }
    }
}

impl From<OpenOptions> for MapFlags {
    fn from(value: OpenOptions) -> Self {
        match value {
            OpenOptions::ReadOnly => MapFlags::MAP_PRIVATE,
            OpenOptions::ReadWrite => MapFlags::MAP_SHARED,
        }
    }
}
