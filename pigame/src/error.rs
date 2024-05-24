use std::{ffi::CStr, io, result};

use libc::strerror;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("out of bounds of screen size")]
    OutOfBounds,
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error(
        "error while calling ioctl: {:?}",
        unsafe {
            #[allow(clippy::cast_possible_truncation)]
            CStr::from_ptr(strerror(*.0)).to_str()
        }
    )]
    Ioctl(i32),
    #[error("error while loading font: {0}")]
    Font(&'static str),
}

pub type Result<T> = result::Result<T, Error>;
