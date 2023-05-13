#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::module_name_repetitions,
    clippy::ptr_offset_with_cast
)]
mod file;
mod options;

pub use file::MappedFile;
pub use options::OpenOptions;
