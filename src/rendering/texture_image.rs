
use log::trace;
use vulkanalia::vk::{self, DeviceV1_0, Handle, HasBuilder, KhrSwapchainExtensionDeviceCommands};
use crate::rendering::{ALLOCATOR, Buffer, QueueFamilyIndices, SwapchainSupport, copy_buffer_to_image, create_buffer, create_image, device::Device, instance::Instance, swapchain::{create_image_view, depth_objects::DepthImage, pipeline::Pipeline, render_pass::RenderPass}, transition_image_layout };

use std::{fs::File, ptr::copy_nonoverlapping as memcpy, rc::Rc, result::Result::Ok};
use anyhow::{Result};
use winit::window::Window;

use super::device;

#[derive(Debug)]
pub struct TextureImage {
    instance: Rc<Instance>,
    device:   Rc<device::Device>,

    pub image:   vk::Image,
    pub memory:  vk::DeviceMemory,
    pub view:    vk::ImageView,
    pub sampler: vk::Sampler,
}

impl TextureImage {
    pub unsafe fn new(
        device:         Rc<device::Device>,
        instance:       Rc<Instance>,

        command_pool:   vk::CommandPool,
        graphics_queue: vk::Queue
    )
        -> Result<Self>
    {
        // let image  = File::open("resources/texture.png")?;
        let image  = File::open("resources/viking_room.png")?;

        let     decoder = png::Decoder::new(image);
        let mut reader  = decoder.read_info()?;

        let mut pixels = vec![0; reader.info().raw_bytes()];
        reader.next_frame(&mut pixels)?;

        let size = reader.info().raw_bytes() as u64;
        let (width, height) = reader.info().size();

        if width != 1024 || height != 1024 || reader.info().color_type != png::ColorType::Rgba {
            panic!("Invalid texture image.");
        }

        let staging_buffer = create_buffer(
            &instance,
            &device,
            size,
            vk::BufferUsageFlags   ::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        let memory = device.map_memory(
            staging_buffer.memory,
            0,
            size,
            vk::MemoryMapFlags::empty(),
        )?;

        memcpy(pixels.as_ptr(), memory.cast(), pixels.len());

        device.unmap_memory(staging_buffer.memory);

        let (image, memory) = create_image(
            &instance,
            &device,
            width,
            height,
            vk::Format             ::R8G8B8A8_SRGB,
            vk::ImageTiling        ::OPTIMAL,
            vk::ImageUsageFlags    ::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        transition_image_layout(
            &device,
            command_pool,
            graphics_queue,
            image,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        )?;

        copy_buffer_to_image(
            &device,
            command_pool,
            graphics_queue,
            staging_buffer.buffer,
            image,
            width,
            height,
        )?;

        transition_image_layout(
            &device,
            command_pool,
            graphics_queue,
            image,
            vk::Format     ::R8G8B8A8_SRGB,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )?;

        device.destroy_buffer(staging_buffer.buffer, ALLOCATOR);
        device.free_memory   (staging_buffer.memory, ALLOCATOR);


        let view    = create_texture_image_view(&device, image)?;
        let sampler = create_texture_sampler   (&device)?;

        Ok(Self {
            instance,
            device,
            image,
            memory,
            view,
            sampler,
        })
    }

}


unsafe fn create_texture_image_view(device: &device::Device, texture_image: vk::Image) -> Result<vk::ImageView> {
    create_image_view(
        device,
        texture_image,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageAspectFlags::COLOR,
    )
}




unsafe fn create_texture_sampler(device: &Device) -> Result<vk::Sampler> {

    let info = vk::SamplerCreateInfo::builder()
        .mag_filter              (vk::Filter            ::LINEAR)
        .min_filter              (vk::Filter            ::LINEAR)
        .address_mode_u          (vk::SamplerAddressMode::REPEAT)
        .address_mode_v          (vk::SamplerAddressMode::REPEAT)
        .address_mode_w          (vk::SamplerAddressMode::REPEAT)
        .anisotropy_enable       (true)
        .max_anisotropy          (16.0)
        .border_color            (vk::BorderColor::INT_OPAQUE_BLACK)
        .unnormalized_coordinates(false)
        .compare_enable          (false)
        .compare_op              (vk::CompareOp        ::ALWAYS)
        .mipmap_mode             (vk::SamplerMipmapMode::LINEAR)
        .mip_lod_bias            (0.0)
        .min_lod                 (0.0)
        .max_lod                 (0.0)
    ;

    Ok(device.create_sampler(&info, ALLOCATOR)?)
}
