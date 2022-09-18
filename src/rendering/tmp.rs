use std::borrow::Borrow;
use std::iter;
use std::mem::ManuallyDrop;
use gfx_hal::{Instance};
use gfx_hal::adapter::Adapter;
use gfx_hal::command::Level;
use gfx_hal::device::Device;
use gfx_hal::format::{ChannelType, Format};
use gfx_hal::image::{Extent, Layout};
use gfx_hal::pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDesc};
use gfx_hal::pool::{CommandPool, CommandPoolCreateFlags};
use gfx_hal::prelude::PhysicalDevice;
use gfx_hal::pso::{BlendState, ColorBlendDesc, ColorMask, EntryPoint, Face, GraphicsPipelineDesc, InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, Rect, Specialization, Viewport};
use gfx_hal::queue::QueueFamily;
use gfx_hal::window::{Extent2D, PresentationSurface, Surface, SwapchainConfig};
use shaderc::ShaderKind;
use crate::window::Window;

pub struct Renderer<B: gfx_hal::Backend> {
    instance: B::Instance,
    surface: B::Surface,
    surface_extent: Extent2D,
    surface_color_format: Format,
    adapter: Adapter<B>,
    device: B::Device,
    render_passes: Vec<B::RenderPass>,
    pipeline_layouts: Vec<B::PipelineLayout>,
    pipelines: Vec<B::GraphicsPipeline>,
    command_pool: B::CommandPool,
    submission_complete_fence: B::Fence,
    rendering_complete_semaphore: B::Semaphore,
}

pub struct RendererInstance<B: gfx_hal::Backend>(ManuallyDrop<Renderer<B>>);


impl<B: gfx_hal::Backend> RendererInstance<B> {
    pub fn new(window: &Window) -> Option<Self> {
        let renderer = Renderer::new(window);
        if renderer.is_none() {
            return None;
        }

        Some(RendererInstance(ManuallyDrop::new(renderer.unwrap())))
    }

    pub fn render(&mut self, should_configure_swapchain: &mut bool) {
        //     let res = &self.0;
        //     let render_pass = &res.render_passes[0];
        //     let pipeline = &res.pipelines[0];
        //
        //     let mut reconfigure_swapchain = *should_configure_swapchain;
        //
        //     let render_timeout_ms = 1_000_000_000;
        //
        //     unsafe {
        //         res.device.wait_for_fence(&res.submission_complete_fence, render_timeout_ms).expect
        //         ("Out of mem or device lost!");
        //
        //         res.device.reset_fence(&mut res.submission_complete_fence).expect("Out of memory!");
        //
        //         res.command_pool.reset(false);
        //     }
        //
        //     if reconfigure_swapchain {
        //         let caps = res.surface.capabilities(&res.adapter.physical_device);
        //         let mut swapchain_config = SwapchainConfig::from_caps(&caps, res.surface_color_format
        //                                                               , res.surface_extent);
        //         if caps.image_count.contains(&3){
        //             swapchain_config.image_count = 3;
        //         }
        //
        //         unsafe{
        //             res.surface.configure_swapchain(&res.device, swapchain_config).expect("failed to \
        //             configure swapchain");
        //         }
        //     }
        //
        //     let surface_image = unsafe {
        //         let aquire_timeout_ns = 1_000_000_000;
        //         match res.surface.acquire_image(aquire_timeout_ns) {
        //             Ok((image, _)) => image,
        //             Err(_) => {
        //                 reconfigure_swapchain = true;
        //                 return;
        //             }
        //         }
        //     };
        //
        //     let framebuffer = unsafe {
        //         res.device.create_framebuffer(render_pass, vec![surface_image.borrow()],
        //                                       Extent {
        //                                           width: res.surface_extent.width,
        //                                           height: res.surface_extent.height,
        //                                           depth: 1,
        //                                       }, ).unwrap()
        //     };
        //
        //     let viewport = Viewport {
        //         rect: Rect {
        //             x: 0,
        //             y: 0,
        //             w: res.surface_extent.width as i16,
        //             h: res.surface_extent.height as i16,
        //         },
        //         depth: 0.0..1.0,
        //     };
    }
}


impl<B: gfx_hal::Backend> Drop for RendererInstance<B> {
    fn drop(&mut self) {
        unsafe {
            let Renderer {
                instance,
                mut surface,
                surface_extent,
                surface_color_format,
                adapter,
                device,
                command_pool,
                render_passes,
                pipeline_layouts,
                pipelines,
                submission_complete_fence,
                rendering_complete_semaphore,
            } = ManuallyDrop::take(&mut self.0);

            device.destroy_semaphore(rendering_complete_semaphore);
            device.destroy_fence(submission_complete_fence);
            for pipeline in pipelines {
                device.destroy_graphics_pipeline(pipeline);
            }
            for pipeline_layout in pipeline_layouts {
                device.destroy_pipeline_layout(pipeline_layout);
            }
            for render_pass in render_passes {
                device.destroy_render_pass(render_pass);
            }
            device.destroy_command_pool(command_pool);
            surface.unconfigure_swapchain(&device);
            instance.destroy_surface(surface)
        }
    }
}


impl<B: gfx_hal::Backend> Renderer<B> {
    pub fn new(window: &Window) -> Option<Self> {
        let instance_result = Instance::create(&window.name, 1);

        // Use this kind of error handling, because otherwise the instance memory would not be accessible for other things.
        if instance_result.is_err() {
            println!("Creating an instance failed due to unsupported backend!");
            return None;
        }
        let instance: B::Instance = instance_result.unwrap();


        let surface_result = unsafe { instance.create_surface(&window.instance) };

        if surface_result.is_err() {
            println!("Creating surface failed!");
            return None;
        }
        let surface = surface_result.unwrap();
        let surface_extent = Extent2D {
            width: window.physical_size.width,
            height: window.physical_size.height,
        };


        let mut adapters = instance.enumerate_adapters();//.remove(0);
        let adapter = adapters.remove(0);


        let queue_family_result = adapter.queue_families.iter().find(|family| {
            surface.supports_queue_family(family) && family.queue_type().supports_graphics()
        });

        if queue_family_result.is_none() {
            println!("No compatible queue found!");
            return None;
        }
        let queue_family = queue_family_result.unwrap();

        let gpu_result = unsafe {
            adapter.physical_device.open(&[(queue_family, &[1.0])], gfx_hal::Features::empty())
        };

        if gpu_result.is_err() {
            println!("Failed to open device!");
            return None;
        }
        let mut gpu = gpu_result.unwrap();

        let device: B::Device = gpu.device;
        let queue_group_result = gpu.queue_groups.pop();
        if queue_group_result.is_none() {
            println!("Failed to receive queue group!");
            return None;
        }
        let queue_group = queue_group_result.unwrap();

        // Command buffer shit probably moved to another file and struct?
        let command_pool_result = unsafe { device.create_command_pool(queue_group.family, CommandPoolCreateFlags::empty()) };
        if command_pool_result.is_err() {
            println!("Out of memory!");
            return None;
        }
        let mut command_pool = command_pool_result.unwrap();

        let command_buffer = unsafe { command_pool.allocate_one(Level::Primary) };

        // Render pass stuff -> make this a seperate file!
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

        let render_pass_result = unsafe { device.create_render_pass(iter::once(color_attachment), iter::once(subpass), iter::empty()) };
        if render_pass_result.is_err() {
            println!("Out of memory!");
            return None;
        }
        let render_pass = render_pass_result.unwrap();

        // Pipeline shit also in a seperate file!
        let pipeline_layout_result = unsafe { device.create_pipeline_layout(iter::empty(), iter::empty()) };
        if pipeline_layout_result.is_err() {
            println!("Pipeline layout; Out of memory");
            return None;
        }
        let pipeline_layout = pipeline_layout_result.unwrap();

        let vertex_shader = include_str!("shaders/base.vert");
        let fragment_shader = include_str!("shaders/base.frag");

        let vertex_shader_module = unsafe {
            device.create_shader_module(&compile_shader
                (vertex_shader, ShaderKind::Vertex)).expect("Failed to create vertex shader module")
        };
        let fragment_shader_module = unsafe {
            device.create_shader_module(&compile_shader
                (fragment_shader, ShaderKind::Fragment)).expect("Failed to create frag shader module")
        };

        let vs_entry = EntryPoint {
            entry: "main",
            module: &vertex_shader_module,
            specialization: Specialization::default(),
        };

        let fs_entry = EntryPoint {
            entry: "main",
            module: &fragment_shader_module,
            specialization: Specialization::default(),
        };

        let primitive_assembler = PrimitiveAssemblerDesc::Vertex {
            buffers: &[],
            attributes: &[],
            input_assembler: InputAssemblerDesc::new(Primitive::TriangleList),
            vertex: vs_entry,
            tessellation: None,
            geometry: None,
        };

        let mut pipeline_desc = GraphicsPipelineDesc::new(primitive_assembler,
                                                          Rasterizer {
                                                              cull_face: Face::BACK,
                                                              ..Rasterizer::FILL
                                                          },
                                                          Some(fs_entry),
                                                          &pipeline_layout,
                                                          Subpass {
                                                              index: 0,
                                                              main_pass: &render_pass,
                                                          }, );
        pipeline_desc.blender.targets.push(ColorBlendDesc {
            mask: ColorMask::ALL,
            blend: Some(BlendState::ALPHA),
        });

        let pipeline = unsafe {
            device.create_graphics_pipeline(&pipeline_desc, None).expect("Failed \
        to create graphics pipeline")
        };
        unsafe {
            device.destroy_shader_module(vertex_shader_module);
            device.destroy_shader_module(fragment_shader_module);
        }

        let submission_complete_fence = unsafe { device.create_fence(true).expect("out of memory!") };
        let rendering_complete_semaphore = unsafe { device.create_semaphore().expect("out of memory") };

        let renderer = Renderer {
            instance: instance as B::Instance,
            surface: surface as B::Surface,
            surface_extent,
            surface_color_format,
            device: device as B::Device,
            adapter: adapter as Adapter<B>,
            command_pool: command_pool as B::CommandPool,
            render_passes: vec![render_pass as B::RenderPass],
            pipeline_layouts: vec![pipeline_layout as B::PipelineLayout],
            pipelines: vec![pipeline as B::GraphicsPipeline],
            submission_complete_fence: submission_complete_fence as B::Fence,
            rendering_complete_semaphore: rendering_complete_semaphore as B::Semaphore
        };

        Some(renderer)
    }
}

fn compile_shader(glsl: &str, shader_kind: ShaderKind) -> Vec<u32> {
    let mut compiler = shaderc::Compiler::new().unwrap();

    let compiled_shader = compiler.compile_into_spirv(glsl, shader_kind, "unnamed", "main", None).expect("Failed to compile shader");
    compiled_shader.as_binary().to_vec()
}
