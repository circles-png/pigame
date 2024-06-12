use std::time::Instant;

use fontdue::Font;
use once_cell::sync::Lazy;

use crate::{
    error::Result,
    graphics::FrameBuffer,
};

#[allow(clippy::unwrap_used)]
static mut CONTEXT: Lazy<Context> = Lazy::new(|| {
    Context::new()
        .inspect_err(|error| eprintln!("{error}"))
        .unwrap()
});

pub(crate) struct Context {
    pub(crate) frame_buffer: FrameBuffer,
    pub(crate) start_time: Instant,
    pub(crate) fonts: Vec<Font>,
    pub(crate) last_frame: Instant,
}

impl Context {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            frame_buffer: FrameBuffer::new()?,
            start_time: Instant::now(),
            fonts: Vec::new(),
            last_frame: Instant::now(),
        })
    }
}

#[must_use]
pub(crate) fn get() -> &'static mut Context {
    #[allow(static_mut_refs)]
    unsafe {
        &mut CONTEXT
    }
}
