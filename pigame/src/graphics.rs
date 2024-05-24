pub mod colour;
pub mod text;

use crate::context::get;
use crate::error::{Error, Result};
use libc::__errno_location;
use libc::ioctl;
use memmap::MmapOptions;
use std::ffi::c_uint;
use std::fs::File;
use std::mem::zeroed;
use std::time::{Duration, Instant};

use memmap::MmapMut;
use std::fs::OpenOptions;
use std::os::fd::AsRawFd;
use std::os::fd::IntoRawFd;

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
    Kdsetmode = 0x4B3A,
}

#[repr(u8)]
pub(crate) enum KdMode {
    KdGraphics = 0x01,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Bitfield {
    pub offset: u32,
    pub length: u32,
    pub msb_right: u32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VarScreeninfo {
    pub xres: u32,
    pub yres: u32,
    pub xres_virtual: u32,
    pub yres_virtual: u32,
    pub xoffset: u32,
    pub yoffset: u32,
    pub bits_per_pixel: u32,
    pub grayscale: u32,
    pub red: Bitfield,
    pub green: Bitfield,
    pub blue: Bitfield,
    pub transp: Bitfield,
    pub nonstd: u32,
    pub activate: u32,
    pub height: u32,
    pub width: u32,
    pub accel_flags: u32,
    pub pixclock: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub upper_margin: u32,
    pub lower_margin: u32,
    pub hsync_len: u32,
    pub vsync_len: u32,
    pub sync: u32,
    pub vmode: u32,
    pub rotate: u32,
    pub colorspace: u32,
    pub reserved: [u32; 4],
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
    pub fn new() -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("/dev/fb0")?;
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
        if unsafe {
            ioctl(
                File::create("/dev/tty1")?.into_raw_fd(),
                IoctlRequest::Kdsetmode as _,
                KdMode::KdGraphics as c_uint,
            )
        } == -1
        {
            return Err(Error::Ioctl(unsafe { *__errno_location() }));
        }
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

    pub fn draw_bitmap(&mut self, bitmap: &[[u8; 3]]) {
        assert_eq!(bitmap.len(), (self.fixed_info.smem_len / 4) as usize);
        let bitmap: Vec<_> = bitmap
            .iter()
            .flat_map(|[red, green, blue]| [*blue, *green, *red, 0].into_iter())
            .collect();
        self.buffer.copy_from_slice(&bitmap);
    }

    #[must_use]
    pub const fn screen_size(&self) -> (u32, u32) {
        (self.variable_info.xres, self.variable_info.yres)
    }

    pub fn wait_until_vsync(&self) -> Result<()> {
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

#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn screen_width() -> u32 {
    get().frame_buffer.screen_size().0
}

#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn screen_height() -> u32 {
    get().frame_buffer.screen_size().1
}

pub fn draw_rectangle(x: u32, y: u32, w: u32, h: u32, colour: Colour) {
    let frame_buffer = &mut get().frame_buffer;
    for x in x..x + w {
        for y in y..y + h {
            let start = (y * (frame_buffer.variable_info.xres + 10) + x) as usize * 4;
            let Some(slice) = frame_buffer.buffer.get_mut(start..start + 4) else {
                break;
            };
            slice.copy_from_slice(&colour.to_bgra_bytes());
        }
    }
}

pub fn clear_background(colour: Colour) {
    let frame_buffer = &mut get().frame_buffer;
    frame_buffer
        .draw_bitmap(&[colour.into()].repeat((frame_buffer.fixed_info.smem_len / 4) as usize));
}

#[must_use]
pub fn get_time() -> f64 {
    get().start_time.elapsed().as_secs_f64()
}

#[allow(clippy::significant_drop_tightening)]
pub fn next_frame() -> Result<()> {
    let context = get();
    context.last_frame = Instant::now();
    let frame_buffer = &mut context.frame_buffer;
    frame_buffer.wait_until_vsync()?;
    frame_buffer.map.copy_from_slice(&frame_buffer.buffer);
    Ok(())
}

#[must_use]
pub fn get_frame_time() -> Duration {
    get().last_frame.elapsed()
}
