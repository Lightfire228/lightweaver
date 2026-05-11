

use log::debug;
use vulkanalia::vk::{self, DeviceV1_0, Handle, HasBuilder, KhrSwapchainExtensionDeviceCommands};

use std::{fs::File, ptr::copy_nonoverlapping as memcpy, rc::Rc, result::Result::Ok};
use anyhow::{anyhow, Result};
use winit::window::Window;

use crate::{app::{begin_single_time_commands, buffer::Buffer, command_pool::CommandPool, device::Device, end_single_time_commands, image::{Image, ImageOpts}, image_view::ImageView, instance::Instance}, script::vm::debug};

use super::device;

pub struct TextureImage {
    device:   Rc<Device>,

    image:   Image,
    view:    ImageView,
    sampler: vk::Sampler,
}

impl TextureImage {
    pub fn new(
        device:         Rc<device::Device>,
        instance:       &Instance,

        command_pool:   &CommandPool,
        graphics_queue: vk::Queue
    )
        -> Result<Self>
    {
        let d = device.device();

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

        let staging_buffer = Buffer::create_buffer(
            device.clone(),
            &instance,
            size,
            vk::BufferUsageFlags   ::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
            "",
        )?;

        let image = unsafe {

            let memory = d.map_memory(
                staging_buffer.memory(),
                0,
                size,
                vk::MemoryMapFlags::empty(),
            )?;

            memcpy(pixels.as_ptr(), memory.cast(), pixels.len());

            d.unmap_memory(staging_buffer.memory());

            let image = Image::new(
                device.clone(),
                &instance,
                ImageOpts {
                    width,
                    height,
                    format:     vk::Format             ::R8G8B8A8_SRGB,
                    tiling:     vk::ImageTiling        ::OPTIMAL,
                    usage:      vk::ImageUsageFlags    ::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
                    properties: vk::MemoryPropertyFlags::DEVICE_LOCAL,
                }
            )?;

            transition_image_layout(
                &device,
                command_pool.command_pool(),
                graphics_queue,
                image.image(),
                vk::Format::R8G8B8A8_SRGB,
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            )?;

            copy_buffer_to_image(
                &device,
                command_pool.command_pool(),
                graphics_queue,
                staging_buffer.buffer(),
                image.image(),
                width,
                height,
            )?;

            transition_image_layout(
                &device,
                command_pool.command_pool(),
                graphics_queue,
                image.image(),
                vk::Format     ::R8G8B8A8_SRGB,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            )?;

            image
        };

        let view    = ImageView::new(
            device.clone(),
            image.image(),
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageAspectFlags::COLOR,
        )?;

        let sampler = unsafe { create_texture_sampler(&device)? };

        Ok(Self {
            device,
            image,
            view,
            sampler,
        })
    }

    pub fn view(&self) -> &ImageView {
        &self.view
    }

    pub fn sampler(&self) -> vk::Sampler {
        self.sampler
    }
}

impl Drop for TextureImage {
    fn drop(&mut self) {
        debug!("Dropping Texture Image");

        unsafe {
            self.device.device().destroy_sampler(self.sampler, None);
        }

        debug!("/Dropping Texture Image");
    }
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

    Ok(device.device().create_sampler(&info, None)?)
}


unsafe fn transition_image_layout(
    device:         &Device,
    command_pool:   vk::CommandPool,
    graphics_queue: vk::Queue,
    image:          vk::Image,
    _format:        vk::Format,
    old_layout:     vk::ImageLayout,
    new_layout:     vk::ImageLayout,

)
    -> Result<()>
{

    let (
        src_access_mask,
        dst_access_mask,
        src_stage_mask,
        dst_stage_mask,
    ) = match (old_layout, new_layout) {
        (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => (
            vk::AccessFlags       ::empty(),
            vk::AccessFlags       ::TRANSFER_WRITE,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
        ),
        (vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) => (
            vk::AccessFlags       ::TRANSFER_WRITE,
            vk::AccessFlags       ::SHADER_READ,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::FRAGMENT_SHADER,
        ),
        _ => return Err(anyhow!("Unsupported image layout transition!")),
    };


    let command_buffer = begin_single_time_commands(device, command_pool)?;

    let subresource = vk::ImageSubresourceRange::builder()
        .aspect_mask     (vk::ImageAspectFlags::COLOR)
        .base_mip_level  (0)
        .level_count     (1)
        .base_array_layer(0)
        .layer_count     (1)
    ;


    let barrier = vk::ImageMemoryBarrier::builder()
        .old_layout            (old_layout)
        .new_layout            (new_layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image                 (image)
        .subresource_range     (subresource)
        .src_access_mask       (src_access_mask)
        .dst_access_mask       (dst_access_mask)
    ;

    device.device().cmd_pipeline_barrier(
        command_buffer,
        src_stage_mask,
        dst_stage_mask,
        vk::DependencyFlags   ::empty(),
        &[] as &[vk::MemoryBarrier],
        &[] as &[vk::BufferMemoryBarrier],
        &[barrier],
    );


    end_single_time_commands(device, command_pool, command_buffer, graphics_queue)?;

    Ok(())
}



unsafe fn copy_buffer_to_image(
    device:         &Device,
    command_pool:   vk::CommandPool,
    graphics_queue: vk::Queue,
    buffer:         vk::Buffer,
    image:          vk::Image,
    width:          u32,
    height:         u32,
)
    -> Result<()>
{
    let command_buffer = begin_single_time_commands(device, command_pool)?;

    let subresource = vk::ImageSubresourceLayers::builder()
        .aspect_mask     (vk::ImageAspectFlags::COLOR)
        .mip_level       (0)
        .base_array_layer(0)
        .layer_count     (1)
    ;

    let region = vk::BufferImageCopy::builder()
        .buffer_offset      (0)
        .buffer_row_length  (0)
        .buffer_image_height(0)
        .image_subresource  (subresource)
        .image_offset       (vk::Offset3D { x: 0,  y: 0,   z: 0 })
        .image_extent       (vk::Extent3D { width, height, depth: 1 })
    ;

    device.device().cmd_copy_buffer_to_image(
        command_buffer,
        buffer,
        image,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        &[region],
    );


    end_single_time_commands(device, command_pool, command_buffer, graphics_queue)?;

    Ok(())
}
