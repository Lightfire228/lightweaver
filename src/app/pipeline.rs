use std::{collections::HashSet, ffi::{CStr, c_void}, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, bytecode::Bytecode, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{Vertex, descriptor_set_layout::DescriptorSetLayout, device::{self, Device}, image_view::{self, ImageView}, instance::{self, Instance}, render_pass::RenderPass, surface::{self, Surface}, swapchain}, rendering::{DEVICE_EXTENSIONS, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYER}};


pub struct Pipeline {
    device: Rc<Device>,

    pipeline: vk::Pipeline,
    layout:   vk::PipelineLayout,
}


impl Pipeline {

    pub fn new(
        device:                Rc<Device>,
        extent:                vk::Extent2D,
        descriptor_set_layout: &DescriptorSetLayout,
        render_pass:           &RenderPass,
    )
        -> Result<Self>
    {
        // TODO: compile shaders at runtime
        // https://stackoverflow.com/a/73591683
        let vert = include_bytes!("../../shaders/vert.spv");
        let frag = include_bytes!("../../shaders/frag.spv");

        let vert_shader_module = create_shader_module(&device, &vert[..])?;
        let frag_shader_module = create_shader_module(&device, &frag[..])?;

        let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage (vk::ShaderStageFlags::VERTEX)
            .module(vert_shader_module)
            .name  (b"main\0")
        ;
        let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage (vk::ShaderStageFlags::FRAGMENT)
            .module(frag_shader_module)
            .name  (b"main\0")
        ;

        let binding_descriptions   = [Vertex::binding_description()];
        let attribute_descriptions = Vertex::attribute_descriptions();
        let vertex_input_state     = vk::PipelineVertexInputStateCreateInfo  ::builder()
            .vertex_binding_descriptions  (&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions)
        ;

        let input_assembly_state   = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology                (vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
        ;

        let viewport = vk::Viewport::builder()
            .x        (0.0)
            .y        (0.0)
            .width    (extent.width  as f32)
            .height   (extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
        ;
        let scissor = vk::Rect2D::builder()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(extent)
        ;

        let viewports      = &[viewport];
        let scissors       = &[scissor];
        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(viewports)
            .scissors (scissors)
        ;

        let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable       (false)
            .rasterizer_discard_enable(false)
            .polygon_mode             (vk::PolygonMode::FILL)
            .line_width               (1.0)
            .cull_mode                (vk::CullModeFlags::BACK)
            .front_face               (vk::FrontFace    ::COUNTER_CLOCKWISE)
            .depth_bias_enable        (false)
        ;

        let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::_1)
        ;

        let attachment = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask      (vk::ColorComponentFlags::all())
            .blend_enable          (false)

            // Optional
            .src_color_blend_factor(vk::BlendFactor::ONE)
            .dst_color_blend_factor(vk::BlendFactor::ZERO)
            .color_blend_op        (vk::BlendOp    ::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            .alpha_blend_op        (vk::BlendOp    ::ADD)
        ;

        let attachments = &[attachment];
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op       (vk::LogicOp::COPY)
            .attachments    (attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0])
        ;


        let set_layouts = unsafe { &[descriptor_set_layout.layout()] };
        let layout_info = vk::PipelineLayoutCreateInfo::builder()
            .set_layouts(set_layouts)
        ;

        let pipeline_layout = unsafe {
            device.device().create_pipeline_layout(&layout_info, None)?
        };

        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable       (true)
            .depth_write_enable      (true)
            .depth_compare_op        (vk::CompareOp::LESS)
            .depth_bounds_test_enable(false)
            .min_depth_bounds        (0.0) // Optional.
            .max_depth_bounds        (1.0) // Optional.
        ;

        let stages = &[vert_stage, frag_stage];
        let info   = vk::GraphicsPipelineCreateInfo::builder()
            .stages              (stages)
            .vertex_input_state  (&vertex_input_state)
            .input_assembly_state(&input_assembly_state)
            .viewport_state      (&viewport_state)
            .rasterization_state (&rasterization_state)
            .multisample_state   (&multisample_state)
            .depth_stencil_state (&depth_stencil_state)
            .color_blend_state   (&color_blend_state)
            .layout              (pipeline_layout)
            .render_pass         (unsafe { render_pass.render_pass() })
            .subpass             (0)
            .base_pipeline_handle(vk::Pipeline::null())
            .base_pipeline_index (-1)
        ;

        let pipeline = unsafe {
            device.device().create_graphics_pipelines(vk::PipelineCache::null(), &[info], None)?.0[0]
        };

        // shader bytecode is compiled to machine code when the pipeline is created,
        // so the module (which is just a thin wrapper) is no longer needed
        unsafe {
            device.device().destroy_shader_module(vert_shader_module, None);
            device.device().destroy_shader_module(frag_shader_module, None);
        };

        Ok(Self {
            device,
            pipeline,
            layout: pipeline_layout,
        })
    }

}

impl Drop for Pipeline {

    fn drop(&mut self) {
        debug!("Dropping Pipeline");

        unsafe {
            self.device.device().destroy_pipeline_layout(self.layout,   None);
            self.device.device().destroy_pipeline       (self.pipeline, None);
        }

        debug!("/Dropping Pipeline");
    }
}


fn create_shader_module(
    device:   &Device,
    bytecode: &[u8],
)
    -> Result<vk::ShaderModule>
{
    let bytecode = Bytecode::new(bytecode).unwrap();

    let info = vk::ShaderModuleCreateInfo::builder()
        .code     (bytecode.code     ())
        .code_size(bytecode.code_size())
    ;

    unsafe {
        Ok(device.device().create_shader_module(&info, None)?)
    }
}
