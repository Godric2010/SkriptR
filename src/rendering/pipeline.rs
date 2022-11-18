use std::iter;
use std::mem::ManuallyDrop;
use gfx_hal::device::Device;
use gfx_hal::format::Format;
use gfx_hal::pass::Subpass;
use gfx_hal::pso::{AttributeDesc, BlendState, ColorBlendDesc, ColorMask, Element, EntryPoint, Face, GraphicsPipelineDesc, InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, ShaderStageFlags, Specialization, VertexBufferDesc, VertexInputRate};
use shaderc::ShaderKind;
use crate::rendering::mesh::Vertex;
use crate::rendering::push_constants::PushConstants;

pub struct GraphicsPipeline<B: gfx_hal::Backend> {
    pub pipeline_layout: ManuallyDrop<B::PipelineLayout>,
    pub pipeline: ManuallyDrop<B::GraphicsPipeline>,
}

impl<B: gfx_hal::Backend> GraphicsPipeline<B> {
    pub fn new(device: &B::Device, render_pass: &mut crate::rendering::pass::RenderPass<B>) -> Option<Self> {
        let push_constant_bytes = std::mem::size_of::<PushConstants>() as u32;

        let pipeline_layout_result = unsafe {
            device.create_pipeline_layout(iter::empty(), iter::once((ShaderStageFlags::VERTEX, 0..push_constant_bytes)))
        };
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
            buffers: &[VertexBufferDesc{
                binding: 0,
                stride: std::mem::size_of::<Vertex>() as u32,
                rate: VertexInputRate::Vertex,
            }],
            attributes: &[
                AttributeDesc{
                    location: 0,
                    binding: 0,
                    element: Element{
                        format: Format::Rg32Sfloat,
                        offset: 0
                    }
                },
                AttributeDesc{
                    location: 1,
                    binding:0,
                    element: Element{
                        format: Format::Rgba32Sfloat,
                        offset: 12
                    },
                },
            ],
            input_assembler: InputAssemblerDesc::new(Primitive::TriangleList),
            vertex: vs_entry,
            tessellation: None,
            geometry: None,
        };

        let rasterizer = Rasterizer {
            cull_face: Face::BACK,
            ..Rasterizer::FILL
        };


        let subpass = Subpass { index: 0, main_pass: &*render_pass.pass };

        let mut pipeline_desc = GraphicsPipelineDesc::new(primitive_assembler,
                                                          rasterizer,
                                                          Some(fs_entry),
                                                          &pipeline_layout,
                                                          subpass);
        pipeline_desc.blender.targets.push(ColorBlendDesc {
            mask: ColorMask::ALL,
            blend: Some(BlendState::ALPHA),
        });

        let pipeline_result = unsafe {
            device.create_graphics_pipeline(&pipeline_desc, None)
        };


        if pipeline_result.is_err() {
            println!("Failed to create pipeline! Out of memory!");
            return None;
        }
        let pipeline = pipeline_result.unwrap();

        unsafe {
            device.destroy_shader_module(vertex_shader_module);
            device.destroy_shader_module(fragment_shader_module);
        }

        Some(Self {
            pipeline: ManuallyDrop::new(pipeline as B::GraphicsPipeline),
            pipeline_layout: ManuallyDrop::new(pipeline_layout as B::PipelineLayout),
        })
    }

    pub unsafe fn destroy(&mut self, device: &B::Device) {
        let pipeline = ManuallyDrop::take(&mut self.pipeline);
        device.destroy_graphics_pipeline(pipeline);

        let layout = ManuallyDrop::take(&mut self.pipeline_layout);
        device.destroy_pipeline_layout(layout);
    }
}

#[allow(duplicate_macro_attributes)]
fn compile_shader(glsl: &str, shader_kind: ShaderKind) -> Vec<u32> {
    let compiler = shaderc::Compiler::new().unwrap();

    let compiled_shader = compiler.compile_into_spirv(glsl, shader_kind, "unnamed", "main", None).expect("Failed to compile shader");
    compiled_shader.as_binary().to_vec()
}
