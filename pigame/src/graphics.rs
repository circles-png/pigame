/// Colour abstractions and functions.
pub mod colour;
/// Text rendering functions.
pub mod text;

use crate::context::get;
use crate::error::{Error, Result};
use libc::__errno_location;
use libc::ioctl;
use log::info;
use memmap::MmapOptions;
use std::fs::File;
use std::mem::zeroed;
use std::time::{Duration, Instant};

use memmap::MmapMut;
use std::fs::OpenOptions;
use std::os::fd::AsRawFd;

use self::colour::Colour;

pub(crate) struct FrameBuffer {
    pub(crate) file: File,
    pub(crate) buffer: Vec<u8>,
    pub(crate) map: MmapMut,
    pub(crate) fixed_info: FixScreeninfo,
    pub(crate) variable_info: VarScreeninfo,
}

#[repr(u64)]
pub(crate) enum IoctlRequest {
    FbiogetVscreeninfo = 0x4600,
    FbiogetFscreeninfo = 0x4602,
    FbioWaitforvsync = 0x4004_4620,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub(crate) struct Bitfield {
    pub(crate) offset: u32,
    pub(crate) length: u32,
    pub(crate) msb_right: u32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub(crate) struct VarScreeninfo {
    pub(crate) xres: u32,
    pub(crate) yres: u32,
    pub(crate) xres_virtual: u32,
    pub(crate) yres_virtual: u32,
    pub(crate) xoffset: u32,
    pub(crate) yoffset: u32,
    pub(crate) bits_per_pixel: u32,
    pub(crate) grayscale: u32,
    pub(crate) red: Bitfield,
    pub(crate) green: Bitfield,
    pub(crate) blue: Bitfield,
    pub(crate) transp: Bitfield,
    pub(crate) nonstd: u32,
    pub(crate) activate: u32,
    pub(crate) height: u32,
    pub(crate) width: u32,
    pub(crate) accel_flags: u32,
    pub(crate) pixclock: u32,
    pub(crate) left_margin: u32,
    pub(crate) right_margin: u32,
    pub(crate) upper_margin: u32,
    pub(crate) lower_margin: u32,
    pub(crate) hsync_len: u32,
    pub(crate) vsync_len: u32,
    pub(crate) sync: u32,
    pub(crate) vmode: u32,
    pub(crate) rotate: u32,
    pub(crate) colorspace: u32,
    pub(crate) reserved: [u32; 4],
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct FixScreeninfo {
    pub(crate) id: [u8; 16],
    pub(crate) smem_start: usize,
    pub(crate) smem_len: u32,
    pub(crate) r#type: u32,
    pub(crate) type_aux: u32,
    pub(crate) visual: u32,
    pub(crate) xpanstep: u16,
    pub(crate) ypanstep: u16,
    pub(crate) ywrapstep: u16,
    pub(crate) line_length: u32,
    pub(crate) mmio_start: usize,
    pub(crate) mmio_len: u32,
    pub(crate) accel: u32,
    pub(crate) capabilities: u16,
    pub(crate) reserved: [u16; 2],
}

impl FrameBuffer {
    pub(crate) fn new() -> Result<Self> {
        info!("opening framebuffer device");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("/dev/fb0")?;
        info!("getting framebuffer information (fixed)");
        let mut fixed_info: FixScreeninfo = unsafe { zeroed() };
        if unsafe {
            ioctl(
                file.as_raw_fd(),
                IoctlRequest::FbiogetFscreeninfo as _,
                &mut fixed_info,
            )
        } == -1
        {
            return Err(Error::Ioctl(unsafe { *__errno_location() }));
        };
        info!("getting framebuffer information (variable)");
        let mut variable_info: VarScreeninfo = unsafe { zeroed() };
        if unsafe {
            ioctl(
                file.as_raw_fd(),
                IoctlRequest::FbiogetVscreeninfo as _,
                &mut variable_info,
            )
        } == -1
        {
            return Err(Error::Ioctl(unsafe { *__errno_location() }));
        }
        info!("\n{:#?}\n{:#?}", fixed_info, variable_info);
        info!("mapping framebuffer");
        let map = unsafe {
            MmapOptions::new()
                .len(fixed_info.smem_len as usize)
                .map_mut(&file)?
        };
        Ok(Self {
            file,
            buffer: vec![0; fixed_info.smem_len as usize],
            map,
            fixed_info,
            variable_info,
        })
    }

    pub(crate) fn draw_bitmap(&mut self, bitmap: &[[u8; 3]]) {
        assert_eq!(bitmap.len(), (self.fixed_info.smem_len / 4) as usize);
        let bitmap: Vec<_> = bitmap
            .iter()
            .flat_map(|[red, green, blue]| [*blue, *green, *red, 0].into_iter())
            .collect();
        self.buffer.copy_from_slice(&bitmap);
    }

    #[must_use]
    pub(crate) const fn screen_size(&self) -> (u32, u32) {
        (self.variable_info.xres, self.variable_info.yres)
    }

    pub(crate) fn wait_until_vsync(&self) -> Result<()> {
        let mut dummy = 0;
        if unsafe {
            ioctl(
                self.file.as_raw_fd(),
                IoctlRequest::FbioWaitforvsync as _,
                &mut dummy,
            )
        } == -1
        {
            return Err(Error::Ioctl(unsafe { *__errno_location() }));
        }
        Ok(())
    }
}

/// Get the width of the screen.
#[must_use]
pub fn screen_width() -> u32 {
    get().frame_buffer.screen_size().0
}

/// Get the height of the screen.
#[must_use]
pub fn screen_height() -> u32 {
    get().frame_buffer.screen_size().1
}

/// Draw a rectangle on the screen.
pub fn draw_rectangle(x: u32, y: u32, w: u32, h: u32, colour: Colour) {
    let frame_buffer = &mut get().frame_buffer;
    for x in x..x + w {
        for y in y..y + h {
            let start = (y * (frame_buffer.variable_info.xres) + x) as usize * 4;
            let Some(slice) = frame_buffer.buffer.get_mut(start..start + 4) else {
                break;
            };
            slice.copy_from_slice(&colour.to_bgra_bytes());
        }
    }
}

/// Clear the screen to a colour.
pub fn clear_background(colour: Colour) {
    let frame_buffer = &mut get().frame_buffer;
    let src = &vec![colour.to_bgra_bytes(); frame_buffer.buffer.len() / 4].into_flattened();
    frame_buffer.buffer.copy_from_slice(src);
}

/// Get the time since the program started.
#[must_use]
pub fn get_time() -> f64 {
    get().start_time.elapsed().as_secs_f64()
}

/// Wait until the next frame and update the screen.
///
/// # Errors
///
/// If the `ioctl` call fails when waiting for the next frame, an error is returned.
pub fn next_frame() -> Result<()> {
    let context = get();
    context.last_frame = Instant::now();
    let frame_buffer = &mut context.frame_buffer;
    frame_buffer.wait_until_vsync()?;
    frame_buffer.map.copy_from_slice(&frame_buffer.buffer);
    Ok(())
}

/// Get the time since the last frame.
#[must_use]
pub fn get_frame_time() -> Duration {
    get().last_frame.elapsed()
}
