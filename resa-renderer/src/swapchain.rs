use gfx_hal::Backend;
use gfx_hal::format::{ChannelType, Format};
use gfx_hal::image::{Extent, FramebufferAttachment};
use gfx_hal::prelude::{PresentationSurface, Surface};
use gfx_hal::pso::{Rect, Viewport};
use gfx_hal::window::{Extent2D, SwapchainConfig};
use crate::core::CoreDevice;

pub struct Swapchain {
    extent: Extent,
    format: Format,
    frame_index: u32,
    frame_queue_size: u32,
    framebuffer_attachment: FramebufferAttachment,
}

impl Swapchain {
    pub fn new<B: Backend>(surface: &mut B::Surface, core_device: &CoreDevice<B>, dimensions: Extent2D) -> Self {
        let capabilities = surface.capabilities(&core_device.physical_device);
        let formats = surface.supported_formats(&core_device.physical_device);

        let format = formats.map_or(Format::Rgba8Srgb, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .map(|format| *format)
                .unwrap_or(formats[0])
        });

        let swap_config = SwapchainConfig::from_caps(&capabilities, format, dimensions);
        let framebuffer_attachment = swap_config.framebuffer_attachment();
        let extent = swap_config.extent.to_extent();
        let frame_queue_size = swap_config.image_count;

        unsafe {
            surface.configure_swapchain(&core_device.device, swap_config).expect("Cannot create swapchain!");
        }

        Swapchain {
            extent,
            format,
            frame_index: 0,
            frame_queue_size,
            framebuffer_attachment,
        }
    }

    pub fn make_viewport(&self) -> Viewport{
        Viewport{
            rect: Rect{
                x: 0,
                y: 0,
                h: self.extent.height as i16,
                w: self.extent.width as i16,
            },
            depth: 0.0..1.0,
        }
    }
}