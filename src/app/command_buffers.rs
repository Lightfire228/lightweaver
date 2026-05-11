
use std::{collections::HashSet, ffi::{CStr, c_void}, mem, rc::Rc, ops::Index as IndexOp};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, bytecode::Bytecode, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{Index, Vertex, buffer::Buffer, command_pool::CommandPool, depth_image::DepthImage, descriptor_sets::DescriptorSets, device::{self, Device}, framebuffers::Framebuffers, image_view::{self, ImageView}, instance::{self, Instance}, pipeline::Pipeline, render_pass::RenderPass, surface::{self, Surface}, swapchain}};


pub struct CommandBuffers {
    device:          Rc<Device>,
    command_pool:    Rc<CommandPool>,

    command_buffers: Vec<vk::CommandBuffer>,

}


impl CommandBuffers {

    pub fn new(
        device:          Rc<Device>,
        command_pool:    Rc<CommandPool>,
        render_pass:     &RenderPass,
        pipeline:        &Pipeline,
        descriptor_sets: &DescriptorSets,
        extent:          vk::Extent2D,
        vertex_buffer:   &Buffer,
        index_buffer:    &Buffer,
        indices:         &[Index],
        frame_buffers:   &Framebuffers,
    )
        -> Result<Self>
    {

        let d = device.device();

        let allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool        (command_pool.command_pool())
            .level               (vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(frame_buffers.frame_buffers().len() as u32)
        ;

        let command_buffers = unsafe { d.allocate_command_buffers(&allocate_info)? };

        for (i, command_buffer) in command_buffers.iter().enumerate() {
            let inheritance = vk::CommandBufferInheritanceInfo::builder();

            let info = vk::CommandBufferBeginInfo::builder()
                .flags           (vk::CommandBufferUsageFlags::empty())
                .inheritance_info(&inheritance)
            ;

            unsafe {
                d.begin_command_buffer(*command_buffer, &info)?
            };

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
                .render_pass (render_pass.render_pass())
                .framebuffer (frame_buffers.frame_buffers()[i])
                .render_area (render_area)
                .clear_values(clear_values)
            ;

            unsafe {
                d.cmd_begin_render_pass   (*command_buffer, &info, vk::SubpassContents::INLINE);
                d.cmd_bind_pipeline       (*command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline.pipeline());


                d.cmd_bind_vertex_buffers (*command_buffer, 0, &[vertex_buffer.buffer()], &[0]);
                d.cmd_bind_index_buffer   (*command_buffer,      index_buffer .buffer(),    0, vk::IndexType::UINT32);
                d.cmd_bind_descriptor_sets(
                    *command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    pipeline.layout(),
                    0,
                    &[descriptor_sets[i]],
                    &[],
                );
                d.cmd_draw_indexed        (*command_buffer, indices.len() as u32, 1, 0, 0, 0);

                d.cmd_end_render_pass     (*command_buffer);

                d.end_command_buffer      (*command_buffer)?;
            };
        }

        Ok(Self {
            device,
            command_buffers,
            command_pool,
        })
    }

}

impl Drop for CommandBuffers {

    fn drop(&mut self) {
        debug!("Dropping Command Buffers");

        unsafe {
            self.device.device().free_command_buffers(self.command_pool.command_pool(), &self.command_buffers);
        }

        debug!("/Dropping Command Buffers");
    }
}

impl IndexOp<usize> for CommandBuffers {
    type Output = vk::CommandBuffer;

    fn index(&self, index: usize) -> &Self::Output {
        &self.command_buffers[index]
    }
}
