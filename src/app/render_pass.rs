use std::{collections::HashSet, ffi::{CStr, c_void}, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{device::{self, Device}, image_view::{self, ImageView}, instance::{self, Instance}, surface::{self, Surface}, swapchain}, rendering::{DEVICE_EXTENSIONS, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYER}};

// TODO: dynamic rendering
// https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/02_Graphics_pipeline_basics/03_Render_passes.html
pub struct RenderPass {
    device:      Rc<Device>,
    render_pass: vk::RenderPass
}


impl RenderPass {

    pub fn new(
        device:    Rc<device::Device>,
        instance:  &Instance,
        format:    vk::Format,
    )
        -> Result<Self>
    {
        let depth_format = unsafe {
            get_depth_format(instance, device.physical_device)?
        };

        let depth_stencil_attachment = vk::AttachmentDescription::builder()
            .format          (depth_format)
            .samples         (vk::SampleCountFlags ::_1)
            .load_op         (vk::AttachmentLoadOp ::CLEAR)
            .store_op        (vk::AttachmentStoreOp::DONT_CARE)
            .stencil_load_op (vk::AttachmentLoadOp ::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout  (vk::ImageLayout      ::UNDEFINED)
            .final_layout    (vk::ImageLayout      ::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
        ;

        let depth_stencil_attachment_ref = vk::AttachmentReference::builder()
            .attachment(1)
            .layout    (vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
        ;

        let color_attachment = vk::AttachmentDescription::builder()
            .format          (format)
            .samples         (vk::SampleCountFlags ::_1)
            .load_op         (vk::AttachmentLoadOp ::CLEAR)
            .store_op        (vk::AttachmentStoreOp::STORE)
            .stencil_load_op (vk::AttachmentLoadOp ::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout  (vk::ImageLayout      ::UNDEFINED)
            .final_layout    (vk::ImageLayout      ::PRESENT_SRC_KHR)
        ;

        let color_attachment_ref = vk::AttachmentReference::builder()
            .attachment(0)
            .layout    (vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        ;

        let color_attachments = &[color_attachment_ref];
        let subpass = vk::SubpassDescription::builder()
            .pipeline_bind_point     (vk::PipelineBindPoint::GRAPHICS)
            .color_attachments       (color_attachments)
            .depth_stencil_attachment(&depth_stencil_attachment_ref)
        ;

        let dependency = vk::SubpassDependency::builder()
            .src_subpass    (vk::SUBPASS_EXTERNAL)
            .dst_subpass    (0)
            .src_stage_mask (
                    vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
            )
            .src_access_mask(
                vk::AccessFlags::empty()
            )
            .dst_stage_mask (
                    vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
            )
            .dst_access_mask(
                    vk::AccessFlags::COLOR_ATTACHMENT_WRITE
                | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
            )
        ;


        let attachments  = &[color_attachment, depth_stencil_attachment];
        let subpasses    = &[subpass];
        let dependencies = &[dependency];

        let info = vk::RenderPassCreateInfo::builder()
            .attachments (attachments)
            .subpasses   (subpasses)
            .dependencies(dependencies)
        ;

        let render_pass = unsafe {
            device.device().create_render_pass(&info, None)?
        };

        Ok(Self {
            device,

            render_pass,
        })
    }


    pub unsafe fn render_pass(&self) -> vk::RenderPass {
        self.render_pass
    }

}

impl Drop for RenderPass {

    fn drop(&mut self) {
        debug!("Dropping Render pass");

        unsafe {
            self.device.device().destroy_render_pass(self.render_pass, None);
        }

        debug!("/Dropping Render pass");
    }
}



pub unsafe fn get_depth_format(instance: &Instance, physical_device: vk::PhysicalDevice) -> Result<vk::Format> {
    let candidates = &[
        vk::Format::D32_SFLOAT,
        vk::Format::D32_SFLOAT_S8_UINT,
        vk::Format::D24_UNORM_S8_UINT,
    ];

    get_supported_format(
        instance,
        physical_device,
        candidates,
        vk::ImageTiling       ::OPTIMAL,
        vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
    )
}


unsafe fn get_supported_format(
    instance:        &Instance,
    physical_device: vk::PhysicalDevice,
    candidates:      &[vk::Format],
    tiling:          vk::ImageTiling,
    features:        vk::FormatFeatureFlags,
)
    -> Result<vk::Format>
{
    candidates
        .iter  ()
        .cloned()
        .find  (|f| {
            let properties = instance.instance().get_physical_device_format_properties(
                physical_device,
                *f
            );

            match tiling {
                vk::ImageTiling::LINEAR  => properties.linear_tiling_features .contains(features),
                vk::ImageTiling::OPTIMAL => properties.optimal_tiling_features.contains(features),

                _ => false
            }
        })
        .ok_or_else(|| anyhow!("Failed to find supported format"))
}
