use std::iter;
use std::mem::ManuallyDrop;
use gfx_hal::device::Device;
use gfx_hal::format::Format;
use gfx_hal::pass::Subpass;
use gfx_hal::pso::{AttributeDesc, BlendState, ColorBlendDesc, ColorMask, Element, EntryPoint, Face, GraphicsPipelineDesc, InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, Specialization};
use shaderc::ShaderKind;

pub struct GraphicsPipeline<B: gfx_hal::Backend> {
    pipeline_layout: ManuallyDrop<B::PipelineLayout>,
    pipeline: ManuallyDrop<B::GraphicsPipeline>,
}

impl<B: gfx_hal::Backend> GraphicsPipeline<B> {
    pub fn new(device: &B::Device, render_pass: &mut crate::rendering::pass::RenderPass<B>) -> Option<Self> {
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

        let rasterizer = Rasterizer {
            cull_face: Face::BACK,
            ..Rasterizer::FILL
        };


        let subpass = Subpass { index: 1, main_pass: &*render_pass.pass };

        let mut pipeline_desc = GraphicsPipelineDesc::new(primitive_assembler,
                                                          rasterizer,
                                                          Some(fs_entry),
                                                          &pipeline_layout,
                                                          subpass);
        pipeline_desc.blender.targets.push(ColorBlendDesc {
            mask: ColorMask::ALL,
            blend: Some(BlendState::ALPHA),
        });

        println!("Pipeline horray!");

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

    pub unsafe fn destroy(&mut self, device: &B::Device){
        let pipeline = ManuallyDrop::take(&mut self.pipeline);
        device.destroy_graphics_pipeline(pipeline);

        let layout = ManuallyDrop::take(&mut self.pipeline_layout);
        device.destroy_pipeline_layout(layout);
    }
}

#[allow(duplicate_macro_attributes)]
fn compile_shader(glsl: &str, shader_kind: ShaderKind) -> Vec<u32> {
    let mut compiler = shaderc::Compiler::new().unwrap();

    let compiled_shader = compiler.compile_into_spirv(glsl, shader_kind, "unnamed", "main", None).expect("Failed to compile shader");
    compiled_shader.as_binary().to_vec()
}
