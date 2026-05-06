
use log::{info, trace};
use vulkanalia::vk::{self, DeviceV1_0, Handle, HasBuilder, KhrSwapchainExtensionDeviceCommands};
use crate::rendering::{ALLOCATOR, Buffer, Index, QueueFamilyIndices, SwapchainSupport, UniformBufferObject, Vertex, create_buffer, create_command_pool, device::Device, instance::Instance, swapchain::{depth_objects::DepthImage, pipeline::Pipeline, render_pass::RenderPass}, texture_image::TextureImage };

use std::{rc::Rc, result::Result::Ok};
use anyhow::{Result};
use winit::window::Window;

use super::device;
pub mod render_pass;
pub mod pipeline;
pub mod depth_objects;

#[derive(Debug)]
pub struct Swapchain {
    instance:     Rc<Instance>,
    device:       Rc<device::Device>,
    command_pool: vk::CommandPool,

    pub swapchain:   vk::SwapchainKHR,
    pub support:     SwapchainSupport,
    pub images:      Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub format:      vk::Format,
    pub extent:      vk::Extent2D,

    pub render_pass:     RenderPass,

    pub pipeline:        Pipeline,

    pub depth_image:     DepthImage,

    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_sets: Vec<vk::DescriptorSet>,

    pub uniform_buffers: Vec<Buffer>,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub frame_buffers:   Vec<vk::Framebuffer>,
}

impl Swapchain {

    pub unsafe fn new(
        device:                Rc<device::Device>,
        instance:              Rc<Instance>,
        surface:               vk::SurfaceKHR,
        descriptor_set_layout: vk::DescriptorSetLayout,
        texture_image:         &TextureImage,
        command_pool:          vk::CommandPool,
        vertex_buffer:         &Buffer,
        index_buffer:          &Buffer,
        indices:               &[Index],
        support:               SwapchainSupport,
        extent:                vk::Extent2D,
    )
        -> Result<Self>
    {
        let indicies = QueueFamilyIndices::get(&instance, surface, device.physical_device)?;


        let surface_format = get_swapchain_surface_format(&support.formats);
        let present_mode   = get_swapchain_present_mode  (&support.present_modes);


        let mut image_count = support.capabilities.min_image_count +1;
        if
               support.capabilities.max_image_count != 0
            && image_count > support.capabilities.max_image_count
        {
            image_count = support.capabilities.max_image_count;
        }


        let mut queue_family_indices = vec![];

        let image_sharing_mode = if indicies.graphics != indicies.present {
            queue_family_indices.push(indicies.graphics);
            queue_family_indices.push(indicies.present);

            vk::SharingMode::CONCURRENT
        }
        else {
            vk::SharingMode::EXCLUSIVE
        };

        let info = vk::SwapchainCreateInfoKHR::builder()
            .surface             (surface)
            .min_image_count     (image_count)
            .image_format        (surface_format.format)
            .image_color_space   (surface_format.color_space)
            .image_extent        (extent)
            .image_array_layers  (1)
            .image_usage         (vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode  (image_sharing_mode)
            .queue_family_indices(&queue_family_indices)
            .pre_transform       (support.capabilities.current_transform)
            .composite_alpha     (vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode        (present_mode)
            .clipped             (true)
            .old_swapchain       (vk::SwapchainKHR::null())
        ;

        let swapchain = device.create_swapchain_khr(&info, ALLOCATOR)?;
        let images    = device.get_swapchain_images_khr(swapchain)?;
        let format    = surface_format.format;

        let image_views = create_swapchain_image_views(&device, &images, format)?;

        let render_pass   = RenderPass::new(device.clone(), instance.clone(), format)?;
        let pipeline      = Pipeline  ::new(device.clone(), instance.clone(), extent, descriptor_set_layout, &render_pass)?;

        let uniform_buffers = create_uniform_buffers(&device, &instance, &images)?;
        let descriptor_pool = create_descriptor_pool(&device, &images)?;
        let descriptor_sets = create_descriptor_sets(&device, &images, descriptor_set_layout, descriptor_pool, &uniform_buffers, texture_image)?;

        let depth_image = DepthImage::new(device.clone(), instance.clone(), extent)?;

        let frame_buffers   = create_framebuffers   (&device, &depth_image, &image_views, &render_pass, extent)?;
        let command_buffers = create_command_buffers(&device, &render_pass, &pipeline, &descriptor_sets, extent, command_pool, vertex_buffer, index_buffer, indices, &frame_buffers)?;

        Ok(Self {
            instance,
            device,
            command_pool,

            swapchain,
            support,
            images,
            image_views,
            format,
            extent,

            render_pass,
            pipeline,
            depth_image,

            descriptor_pool,
            descriptor_sets,
            uniform_buffers,
            command_buffers,
            frame_buffers,
        })
    }
}



impl Drop for Swapchain {
    fn drop(&mut self) {
        trace!("dropping swapchain");

        // TODO: move these to respective drops
        unsafe {
            self.device.destroy_image_view(self.depth_image.view,   ALLOCATOR);
            self.device.free_memory       (self.depth_image.memory, ALLOCATOR);
            self.device.destroy_image     (self.depth_image.image,  ALLOCATOR);

            self.device.destroy_descriptor_pool(self.descriptor_pool, ALLOCATOR);

            self.uniform_buffers.iter().for_each(|b| {
                self.device.destroy_buffer     (*&b.buffer, ALLOCATOR);
                self.device.free_memory        (*&b.memory, ALLOCATOR)
            });

            self.frame_buffers.iter().for_each(|f| self.device.destroy_framebuffer(*f, ALLOCATOR));

            self.device.free_command_buffers(self.command_pool, &self.command_buffers);

            self.device.destroy_pipeline       (self.pipeline.pipeline,       ALLOCATOR);
            self.device.destroy_pipeline_layout(self.pipeline.layout,         ALLOCATOR);
            self.device.destroy_render_pass    (self.render_pass.render_pass, ALLOCATOR);

            self.image_views.iter().for_each(|v| self.device.destroy_image_view(*v, ALLOCATOR));

            self.device.destroy_swapchain_khr(self.swapchain, ALLOCATOR);
        }

        trace!("dropped swapchain");
    }
}

// impl Clone for Swapchain {
//     fn clone(&self) -> Self {
//         Self {
//             instance:    self.instance   .clone(),
//             device:      self.device     .clone(),
//             swapchain:   self.swapchain  .clone(),
//             images:      self.images     .clone(),
//             image_views: self.image_views.clone(),
//             format:      self.format     .clone(),
//             extent:      self.extent     .clone(),

//             render_pass: self.render_pass.clone(),
//             pipeline:           todo!(),
//             depth_image:        todo!(),
//             depth_image_memory: todo!(),
//             depth_image_view:   todo!(),
//             descriptor_pool:    todo!(),
//             descriptor_sets:    todo!(),
//             uniform_buffers:    todo!(),
//             command_buffers:    todo!(),
//        }
//     }
// }


fn get_swapchain_surface_format(
    formats: &[vk::SurfaceFormatKHR],
)
    -> vk::SurfaceFormatKHR
{
    formats
        .iter          ()
        .cloned        ()
        .find          (|f| {
                  f.format      == vk::Format       ::B8G8R8A8_SRGB
               && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or_else(|| formats[0])
}

fn get_swapchain_present_mode(
    present_modes: &[vk::PresentModeKHR]
)
    -> vk::PresentModeKHR
{
    present_modes
        .iter     ()
        .cloned   ()
        .find     (|m| *m == vk::PresentModeKHR::MAILBOX)
        .unwrap_or(vk::PresentModeKHR::FIFO)
}

pub fn get_swapchain_extent(
    window:       &Window,
    capabilities: &vk::SurfaceCapabilitiesKHR,
)
    -> vk::Extent2D
{
    if capabilities.current_extent.width != u32::MAX {
        capabilities.current_extent
    }
    else {
        vk::Extent2D::builder()
            .width (window.inner_size().width .clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            ))
            .height(window.inner_size().height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            ))
            .build()
    }
}



unsafe fn create_swapchain_image_views(
    device:    &Device,
    images:    &[vk::Image],
    format:    vk::Format,
)
    -> Result<Vec<vk::ImageView>>
{
    Ok(images
        .iter()
        .map (|i| create_image_view(device, *i, format, vk::ImageAspectFlags::COLOR))
        .collect::<Result<Vec<_>, _>>()?
    )
}


pub unsafe fn create_image_view(
    device:  &Device,
    image:   vk::Image,
    format:  vk::Format,
    aspects: vk::ImageAspectFlags
)
    -> Result<vk::ImageView>
{

    let subresource_range = vk::ImageSubresourceRange::builder()
        .aspect_mask     (aspects)
        .base_mip_level  (0)
        .level_count     (1)
        .base_array_layer(0)
        .layer_count     (1)
    ;

    let info = vk::ImageViewCreateInfo::builder()
        .image            (image)
        .view_type        (vk::ImageViewType::_2D)
        .format           (format)
        .subresource_range(subresource_range)
    ;

    Ok(device.create_image_view(&info, ALLOCATOR)?)
}


unsafe fn create_uniform_buffers(
    device:    &device::Device,
    instance:  &Instance,
    images:    &[vk::Image],
)
    -> Result<Vec<Buffer>>
{

    refresh_uniform_buffers(device, instance, images, Vec::new())
}

unsafe fn refresh_uniform_buffers(
        device:    &device::Device,
        instance:  &Instance,
        images:    &[vk::Image],
    mut buffers:   Vec<Buffer>,
)
    -> Result<Vec<Buffer>>
{

    buffers.clear();

    for _ in 0..images.len() {
        buffers.push(create_buffer(
            instance,
            device,
            size_of::<UniformBufferObject>() as u64,
            vk::BufferUsageFlags   ::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?);

    }

    Ok(buffers)
}

unsafe fn create_descriptor_pool(device: &Device, images: &[vk::Image]) -> Result<vk::DescriptorPool> {

    let ubo_size = vk::DescriptorPoolSize::builder()
        .type_           (vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(images.len() as u32)
    ;

    let sampler_size =  vk::DescriptorPoolSize::builder()
        .type_           (vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(images.len() as u32)
    ;

    let pool_sizes = &[ubo_size, sampler_size];
    let info       = vk::DescriptorPoolCreateInfo::builder()
        .pool_sizes(pool_sizes)
        .max_sets  (images.len() as u32)
    ;

    Ok(device.create_descriptor_pool(&info, ALLOCATOR)?)
}

unsafe fn create_descriptor_sets(
    device:                &Device,
    images:                &[vk::Image],
    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool:       vk::DescriptorPool,
    uniform_buffers:       &[Buffer],
    texture_image:         &TextureImage,
)
    -> Result<Vec<vk::DescriptorSet>>
{

    let layouts = vec![descriptor_set_layout; images.len()];
    let info    = vk::DescriptorSetAllocateInfo::builder()
        .descriptor_pool(descriptor_pool)
        .set_layouts    (&layouts)
    ;

    let descriptor_sets = device.allocate_descriptor_sets(&info)?;

    for i in 0..images.len() {
        let info = vk::DescriptorBufferInfo::builder()
            .buffer(uniform_buffers[i].buffer)
            .offset(0)
            .range(size_of::<UniformBufferObject>() as u64)
        ;

        let buffer_info = &[info];
        let ubo_write   = vk::WriteDescriptorSet::builder()
            .dst_set          (descriptor_sets[i])
            .dst_binding      (0)
            .dst_array_element(0)
            .descriptor_type  (vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info      (buffer_info)
        ;

        let info = vk::DescriptorImageInfo::builder()
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image_view  (texture_image.view)
            .sampler     (texture_image.sampler)
        ;

        let image_info    = &[info];
        let sampler_write = vk::WriteDescriptorSet::builder()
            .dst_set          (descriptor_sets[i])
            .dst_binding      (1)
            .dst_array_element(0)
            .descriptor_type  (vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info       (image_info)
        ;

        device.update_descriptor_sets(
            &[ubo_write, sampler_write],
            &[] as &[vk::CopyDescriptorSet]
        );
    }

    Ok(descriptor_sets)
}


unsafe fn create_command_buffers(
    device:          &Device,
    // swapchain:       &Swapchain,
    render_pass:     &RenderPass,
    pipeline:        &Pipeline,
    descriptor_sets: &[vk::DescriptorSet],
    extent:          vk::Extent2D,
    command_pool:    vk::CommandPool,
    vertex_buffer:   &Buffer,
    index_buffer:    &Buffer,
    indices:         &[Index],
    frame_buffers:   &[vk::Framebuffer]
)
    -> Result<Vec<vk::CommandBuffer>>
{

    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool        (command_pool)
        .level               (vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(frame_buffers.len() as u32)
    ;

    let command_buffers = device.allocate_command_buffers(&allocate_info)?;

    for (i, command_buffer) in command_buffers.iter().enumerate() {
        let inheritance = vk::CommandBufferInheritanceInfo::builder();

        let info = vk::CommandBufferBeginInfo::builder()
            .flags           (vk::CommandBufferUsageFlags::empty())
            .inheritance_info(&inheritance)
        ;

        device.begin_command_buffer(*command_buffer, &info)?;

        let render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(extent)
        ;

        let color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            }
        };

        let depth_clear_value = vk::ClearValue {
            depth_stencil: vk::ClearDepthStencilValue {
                depth:   1.0,
                stencil: 0,
            }
        };

        let clear_values = &[color_clear_value, depth_clear_value];
        let info = vk::RenderPassBeginInfo::builder()
            .render_pass (render_pass.render_pass)
            .framebuffer (frame_buffers[i])
            .render_area (render_area)
            .clear_values(clear_values)
        ;

        device.cmd_begin_render_pass   (*command_buffer, &info, vk::SubpassContents::INLINE);
        device.cmd_bind_pipeline       (*command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline.pipeline);


        device.cmd_bind_vertex_buffers (*command_buffer, 0, &[vertex_buffer.buffer], &[0]);
        device.cmd_bind_index_buffer   (*command_buffer,      index_buffer .buffer,    0, vk::IndexType::UINT32);
        device.cmd_bind_descriptor_sets(
            *command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            pipeline.layout,
            0,
            &[descriptor_sets[i]],
            &[],
        );
        device.cmd_draw_indexed        (*command_buffer, indices.len() as u32, 1, 0, 0, 0);

        device.cmd_end_render_pass     (*command_buffer);

        device.end_command_buffer      (*command_buffer)?;
    }

    Ok(command_buffers)
}


unsafe fn create_framebuffers(
    device:      &Device,
    depth_image: &DepthImage,
    image_views: &[vk::ImageView],
    render_pass: &RenderPass,
    extent:      vk::Extent2D,

)
    -> Result<Vec<vk::Framebuffer>>
{

    Ok(image_views
        .iter()
        .map(|i| {
            // The color attachment differs for every swapchain image, but the same depth image can be
            // used by all of them because only a single subpass is running at the same time due to
            // our semaphores.
            let attachments = &[*i, depth_image.view];

            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass.render_pass)
                .attachments(attachments)
                .width      (extent.width)
                .height     (extent.height)
                .layers     (1)
            ;

            device.create_framebuffer(&create_info, ALLOCATOR)
        })
        .collect::<Result<Vec<_>, _>>()?
    )
}
