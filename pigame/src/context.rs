use std::time::Instant;

use fontdue::Font;
use once_cell::sync::Lazy;

use crate::{
    error::Result,
    graphics::{colour::BLACK, FrameBuffer},
};

static mut CONTEXT: Lazy<Context> = Lazy::new(|| Context::new().unwrap());

pub(crate) struct Context {
    pub frame_buffer: FrameBuffer,
    pub start_time: Instant,
    pub fonts: Vec<Font>,
    pub last_frame: Instant,
}

impl Context {
    pub fn new() -> Result<Self> {
        Ok(Self {
            frame_buffer: FrameBuffer::new()?,
            start_time: Instant::now(),
            fonts: Vec::new(),
            last_frame: Instant::now(),
        })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        self.frame_buffer.draw_bitmap(
            &[BLACK.into()].repeat((self.frame_buffer.fixed_info.smem_len / 4) as usize),
        );
    }
}

#[must_use]
pub(crate) fn get() -> &'static mut Context {
    #[allow(static_mut_refs)]
    unsafe {
        &mut CONTEXT
    }
}
