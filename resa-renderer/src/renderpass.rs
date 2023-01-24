use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::device::Device;
use gfx_hal::image::Layout;
use gfx_hal::pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDesc};
use crate::core::CoreDevice;
use crate::swapchain::Swapchain;

pub struct RenderPass<B: Backend> {
    pub render_pass: Option<B::RenderPass>,
    device: Rc<RefCell<CoreDevice<B>>>,
}

// This can be optimized for multiple render pass usage. A render pass builder of some sort would be nice.
impl<B: Backend> RenderPass<B> {
    pub fn new(swapchain: &Swapchain, device: Rc<RefCell<CoreDevice<B>>>) -> Self {
        let mut render_pass = {
            let attachment = Attachment {
                format: Some(swapchain.format.clone()),
                samples: 1,
                ops: AttachmentOps::new(
                    AttachmentLoadOp::Clear,
                    AttachmentStoreOp::Store,
                ),
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

            let rp = unsafe {
                device.borrow().device.create_render_pass(
                    iter::once(attachment),
                    iter::once(subpass),
                    iter::empty())
            }
                .ok();
            rp
        };

        if let Some(ref mut rp) = render_pass {
            unsafe { device.borrow().device.set_render_pass_name(rp, "main pass") };
        }

        RenderPass {
            render_pass,
            device,
        }
    }
}


impl<B: Backend> Drop for RenderPass<B> {
    fn drop(&mut self) {
       let device = &self.device.borrow().device;
        unsafe {
            device.destroy_render_pass(self.render_pass.take().unwrap());
        }
    }
}