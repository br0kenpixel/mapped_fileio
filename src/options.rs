use nix::{
    fcntl::OFlag,
    sys::{
        mman::{MapFlags, ProtFlags},
        stat::Mode,
    },
};

/// Options and flags which can be used to configure how a file is opened. Similar to [`OpenOptions`](std::fs::OpenOptions).  
/// Unlike [`OpenOptions`](std::fs::OpenOptions), this variant does not have a write-only mode. You can either choose Read-Only
/// or Read/Write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenOptions {
    /// This option indicates that the file should be read-only if opened.
    ReadOnly,
    /// This option indicates that the file should be readable and writable if opened.  
    /// __Unlike [`OpenOptions::write()`](std::fs::OpenOptions::write) this will not
    /// create a file if it does not exist!__
    ReadWrite,
}

impl From<OpenOptions> for OFlag {
    fn from(value: OpenOptions) -> Self {
        match value {
            OpenOptions::ReadOnly => Self::O_RDONLY,
            OpenOptions::ReadWrite => Self::O_RDWR,
        }
    }
}

impl From<OpenOptions> for Mode {
    fn from(_: OpenOptions) -> Self {
        Self::S_IRUSR | Self::S_IWUSR
    }
}

impl From<OpenOptions> for ProtFlags {
    fn from(value: OpenOptions) -> Self {
        match value {
            OpenOptions::ReadOnly => Self::PROT_READ,
            OpenOptions::ReadWrite => Self::PROT_READ | Self::PROT_WRITE,
        }
    }
}

impl From<OpenOptions> for MapFlags {
    fn from(value: OpenOptions) -> Self {
        match value {
            OpenOptions::ReadOnly => Self::MAP_PRIVATE,
            OpenOptions::ReadWrite => Self::MAP_SHARED,
        }
    }
}
