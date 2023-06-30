use std::cell::RefCell;
use std::io::Cursor;
use image::{Rgba, RgbaImage};
use std::rc::Rc;
use gfx_hal::adapter::MemoryType;
use gfx_hal::{Backend, Limits};
use gfx_hal::buffer::Usage;
use gfx_hal::device::Device;
use gfx_hal::pool::CommandPoolCreateFlags;
use gfx_hal::pso::{DescriptorPoolCreateFlags, DescriptorRangeDesc, DescriptorSetLayoutBinding, DescriptorType, ImageDescriptorType, ShaderStageFlags};
use crate::core::CoreDevice;
use crate::descriptors::{DescSet, DescSetLayout};
use crate::image_buffer::ImageBuffer;
use crate::material::TextureFormat;

/**
  Define the state a texture can have.
  - Raw means that the texture is not written to a buffer yet
  - Initialized means that the texture is already loaded into a buffer an can be used in the rendering process
  */
enum TextureState<B: Backend> {
    Raw(TextureRawData),
    Initialized(TextureBufferData<B>),
}

/**
  The texture buffer object (TBO) reference contains the index of the initialized image buffer.
  */
#[derive(Copy, Clone, Hash)]
pub struct TBORef(usize);

/**
  Contains the raw texture data as byte vector and the name of the texture.
  */
#[derive(Clone)]
struct TextureRawData {
    name: String,
    data: Vec<u8>,
    format: TextureFormat,
}

/**
  Contains the image buffer of the texture, as well as its name and the reference to the pool this texture is allocated in.
  */
struct TextureBufferData<B: Backend> {
    name: String,
    buffer_pool_index: usize,
    image: ImageBuffer<B>,
}

/**
  Contains the pool descriptor and the pool capacity. The current fill field indicates
  how many textures have been registered to this pool so far.
  */
struct TextureBufferPool<B: Backend> {
    image_descriptor: B::DescriptorPool,
    capacity: u32,
    current_fill: u32,
}

pub struct TextureLibrary<B: Backend> {
    device_ptr: Rc<RefCell<CoreDevice<B>>>,
    memory_types: Vec<MemoryType>,
    adapter_limits: Limits,
    buffer_pools: Vec<TextureBufferPool<B>>,
    entries: Vec<TextureState<B>>,
}

impl<B: Backend> TextureLibrary<B> {
    pub fn new(device_ptr: Rc<RefCell<CoreDevice<B>>>, memory_types: Vec<MemoryType>, adapter_limits: Limits) -> Self {
        let mut instance = TextureLibrary {
            device_ptr,
            memory_types,
            adapter_limits,
            buffer_pools: vec![],
            entries: vec![],
        };

        instance
    }

    pub fn add_texture(&mut self, name: &str, data: Vec<u8>, format: TextureFormat) {
        let raw_texture = TextureRawData {
            name: name.to_string(),
            data,
            format,
        };
        self.entries.push(TextureState::Raw(raw_texture))
    }

    pub fn get_texture_buffer(&self, ref_obj: &TBORef) -> &ImageBuffer<B> {
        let texture_state = &self.entries[ref_obj.0];
        let buffer_data = match texture_state {
            TextureState::Raw(_) => { panic!("This should not be possible, given the fact that a TBORef exists of an uninitialized texture!") }
            TextureState::Initialized(buffer_data) => buffer_data,
        };
        &buffer_data.image
    }

    pub fn get_texture_from_name(&mut self, name: &str) -> Option<TBORef> {

        let (raw_texture, index) = self.get_raw_texture_data_from_name(name)?;

        let pool_index = self.find_or_create_new_buffer_pool();
        let texture_buffer = self.create_buffer_from_raw_texture(&raw_texture, index)?;
        self.entries[index] = TextureState::Initialized(texture_buffer);
        let tbo_ref = TBORef(index);
        Some(tbo_ref)
    }

    pub fn update_texture_with_name(&mut self, name: &str, new_data: Vec<u8>, format: TextureFormat) {
        todo!()
    }

    pub fn update_texture_with_tbo(&mut self, tbo_ref: &TBORef, new_data: Vec<u8>, format: TextureFormat) {
        todo!()
    }

    fn get_raw_texture_data_from_name(&self, name: &str) -> Option<(TextureRawData, usize)>{
        for (index, entry) in self.entries.iter().enumerate() {
            if let TextureState::Raw(raw_data) = entry{
                if raw_data.name == name.to_string(){
                    return Some((raw_data.clone(), index));
                }
            }
        }
        None
    }

    fn create_buffer_from_raw_texture(&mut self , raw_data: &TextureRawData, entry_index: usize) -> Option<TextureBufferData<B>>{

        let image_desc = self.create_descriptor(entry_index);
        let rgba_image = self.build_rgba_image(raw_data.data.clone(), raw_data.format.clone());

        let mut staging_pool = unsafe {
            self.device_ptr.borrow().device.create_command_pool(
                self.device_ptr.borrow().queues.family,
                CommandPoolCreateFlags::empty(),
                )
        }.expect("Cannot create staging command pool");

        let image_buffer = ImageBuffer::new(
            image_desc,
            &rgba_image,
            &self.adapter_limits,
            &self.memory_types,
            Usage::TRANSFER_SRC,
            Rc::clone(&self.device_ptr),
            &mut staging_pool,
            );

        image_buffer.wait_for_transfer_completion();


        let texture_buffer_data = TextureBufferData{
            name: raw_data.name.to_string(),
            buffer_pool_index: entry_index,
            image: image_buffer,
        };

       Some(texture_buffer_data)
    }


    fn count_all_raw_texture_references(&mut self) -> usize{
        self.entries.iter().filter(|texture_state| match texture_state { TextureState::Raw(_) => true, _ => false}).count()
    }

    fn find_or_create_new_buffer_pool(&mut self) -> usize{

        let  result = self.buffer_pools.iter().enumerate().find(|(idx, pool)| pool.current_fill < pool.capacity);
        if result.is_some() {
            return result.unwrap().0;
        }

        let capacity = self.count_all_raw_texture_references();

        let image_desc_pool = unsafe {
            self.device_ptr.borrow().device.create_descriptor_pool(
                capacity,
                vec![DescriptorRangeDesc {
                    ty: DescriptorType::Image {
                        ty: ImageDescriptorType::Sampled {
                            with_sampler: false,
                        },
                    },
                    count: capacity,
                },
                DescriptorRangeDesc {
                    ty: DescriptorType::Sampler,
                    count: capacity,
                },
                ].into_iter(),
                DescriptorPoolCreateFlags::empty(),
                )
        }.ok();

        let buffer_pool = TextureBufferPool{
            image_descriptor: image_desc_pool.unwrap(),
            capacity: capacity as u32,
            current_fill: 0,
        };


        let pool_index = self.buffer_pools.len();
        self.buffer_pools.push(buffer_pool);
        pool_index
    }

    fn create_descriptor(&mut self, entry_index: usize) -> DescSet<B> {
        let image_desc = DescSetLayout::new(
            Rc::clone(&self.device_ptr),
            vec![
            DescriptorSetLayoutBinding {
                binding: 0,
                ty: DescriptorType::Image {
                    ty: ImageDescriptorType::Sampled {
                        with_sampler: false,
                    },
                },
                count: 1,
                stage_flags: ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            },
            DescriptorSetLayoutBinding {
                binding: 1,
                ty: DescriptorType::Sampler,
                count: 1,
                stage_flags: ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            },
            ],
            );
            let image_desc = image_desc.create_desc_set(
                &mut self.buffer_pools[entry_index].image_descriptor,
                "image",
                Rc::clone(&self.device_ptr),
                );
            image_desc
    }

    fn build_rgba_image(&self, image_data: Vec<u8>, format: TextureFormat) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
        return match format {
            TextureFormat::Custom((width, height)) => RgbaImage::from_raw(width, height, image_data).unwrap(),
            TextureFormat::Png => image::load(Cursor::new(&image_data[..]), image::ImageFormat::Png).unwrap().to_rgba8(),
        }
    }
}
