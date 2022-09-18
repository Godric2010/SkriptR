use std::iter;
use std::mem::ManuallyDrop;
use gfx_hal::adapter::Adapter;
use gfx_hal::device::Device;
use gfx_hal::format::{ChannelType, Format};
use gfx_hal::image::Layout;
use gfx_hal::pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDesc};
use gfx_hal::window::Surface;

pub struct RenderPass<B: gfx_hal::Backend> {
    pub pass: ManuallyDrop<B::RenderPass>,
    pub color_format: Format,
}

impl<B: gfx_hal::Backend> RenderPass<B> {
    pub fn new(adapter: &Adapter<B>, device: &B::Device, surface: &B::Surface) -> Option<Self> {
        let supported_formats = surface.supported_formats(&adapter.physical_device)
            .unwrap_or(vec![]);
        let default_format = *supported_formats.get(0).unwrap_or(&Format::Rgb8Srgb);
        let surface_color_format = supported_formats
            .into_iter()
            .find(|format| format.base_format().1 == ChannelType::Srgb)
            .unwrap_or(default_format);

        let color_attachment = Attachment {
            format: Some(surface_color_format),
            samples: 1,
            ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
            stencil_ops: AttachmentOps::DONT_CARE,
            layouts: Layout::Undefined..Layout::Present,
        };

        let subpass = SubpassDesc {
            colors: &[(0, Layout::ColorAttachmentOptimal)],
            depth_stencil: None,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };

        let render_pass_result = unsafe {
            device.create_render_pass(iter::once(color_attachment), iter::once(subpass), iter::empty())
        };
        if render_pass_result.is_err() {
            println!("Could not create render pass! Out of memory!");
            return None;
        }
        let render_pass = render_pass_result.unwrap();

        Some(Self {
            pass: ManuallyDrop::new(render_pass as B::RenderPass),
            color_format: surface_color_format,
        })
    }

    pub unsafe fn destroy(mut self, device: &B::Device) {
        let render_pass = ManuallyDrop::take(&mut self.pass);
        device.destroy_render_pass(render_pass);
    }
}

