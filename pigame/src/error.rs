use std::{ffi::CStr, io, result};

use libc::strerror;
use rppal::gpio;
use thiserror::Error;

/// Main error type.
#[derive(Error, Debug)]
pub enum Error {
    /// Out of bounds of screen size.
    #[error("out of bounds of screen size")]
    OutOfBounds,
    /// IO error.
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    /// Error while calling ioctl.
    #[error(
        "error while calling ioctl: {:?}",
        unsafe {
            CStr::from_ptr(strerror(*.0)).to_str()
        }
    )]
    Ioctl(i32),
    /// Error while loading font.
    #[error("error while loading font: {0}")]
    Font(&'static str),
    /// Error from the `rppal` crate.
    #[error("error from rppal: {0}")]
    Rppal(#[from] gpio::Error),
}

/// Main result type.
pub type Result<T> = result::Result<T, Error>;
