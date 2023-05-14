#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::module_name_repetitions,
    clippy::ptr_offset_with_cast
)]

//! Memory mapped file I/O
//!
//! This library allows reading files by using [`mmap()`](https://en.wikipedia.org/wiki/Mmap) under the hood.  
//! Mapping a file into memory allows reading it as if it was a simple `const char*` array (or `&[u8]` in Rust terms).
// --
mod file;
mod options;

pub use file::MappedFile;
pub use options::OpenOptions;
