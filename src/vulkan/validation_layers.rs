
use std::{collections::HashSet, ffi::{CStr, c_void}};

use anyhow::{Result, Ok, anyhow};
use log::*;
use vulkanalia::{prelude::v1_4::*};

use crate::vulkan::Bytes;

const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

const VALIDATION_ENABLED: bool = super::VALIDATION_ENABLED;


pub unsafe fn get_layers(entry: &Entry) -> Result<Bytes> {

    let available_layers: HashSet<_> = unsafe { entry
        .enumerate_instance_layer_properties()?
    }
        .iter   ()
        .map    (|l| l.layer_name)
        .collect()
    ;

    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
        return Err(anyhow!("Validation layer requested but not supported."));
    }

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    };

    Ok(layers)
}

pub fn push_validation_ext(extensions: &mut Vec<*const i8>) {
    if !VALIDATION_ENABLED {
        return;
    }

    extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
}

pub fn get_debug_info<'a>() -> vk::DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(
              vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .user_callback(Some(debug_callback))
}

extern "system" fn debug_callback(
    severity:        vk::DebugUtilsMessageSeverityFlagsEXT,
    type_:           vk::DebugUtilsMessageTypeFlagsEXT,
    data:     *const vk::DebugUtilsMessengerCallbackDataEXT,
    _:        *mut   c_void,
) 
    -> vk::Bool32
{
    let data    = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    match severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR   => error!("({:?}) {}", type_, message),
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => warn !("({:?}) {}", type_, message),
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO    => debug!("({:?}) {}", type_, message),
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => trace!("({:?}) {}", type_, message),
        _                                              => panic!("({:?}) {}", type_, message),
    };

    vk::FALSE
}
