use std::{collections::HashSet, ffi::{CStr, c_void}, marker::PhantomData, ops::{Deref, DerefMut}};
use log::*;

use vulkanalia::vk::{DebugUtilsMessengerEXT, EntryV1_0, HasBuilder, InstanceV1_0};
use vulkanalia::{Entry, vk::{self}, window as vk_window};
use crate::rendering::{ALLOCATOR, AppData, PORTABILITY_MACOS_VERSION};

use super::{VALIDATION_ENABLED, VALIDATION_LAYER};

use std::result::Result::Ok;
use anyhow::{Result, anyhow};
use winit::window::Window;

// Note: This trait was called `ExtDebugUtilsExtension` in versions of `vulkanalia` prior to `v0.31.0`.
use vulkanalia::vk::{ExtDebugUtilsExtensionInstanceCommands};

#[derive(Debug)]
pub struct Instance<'a> {
    instance: vulkanalia::Instance,
    p:        PhantomData<&'a Entry>,
}

impl<'a> Instance<'a> {
    pub unsafe fn new<'b: 'a>(
        window: &Window,
        entry:  &'b Entry,
    )
        -> Result<(Self, DebugUtilsMessengerEXT)>
    {

        let application_info = vk::ApplicationInfo::builder()
            .application_name   (b"Lightweaver\0")
            .application_version(vk::make_version(1, 0, 0))
            .engine_name        (b"No Engine\0")
            .engine_version     (vk::make_version(1, 0, 0))
            .api_version        (vk::make_version(1, 0, 0))
        ;

        let available_layers: HashSet<_> = entry
            .enumerate_instance_layer_properties()?

            .iter   ()
            .map    (|l| l.layer_name)
            .collect()
        ;


        if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
            return Err(anyhow!("Validation layer requested but not supported"));
        }

        let layers = if VALIDATION_ENABLED {
            vec![VALIDATION_LAYER.as_ptr()]
        }
        else {
            vec![]
        };

        let mut extensions: Vec<_> = vk_window::get_required_instance_extensions(window)
            .iter   ()
            .map    (|e| e.as_ptr())
            .collect()
        ;

        if VALIDATION_ENABLED {
            extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
        }

        // Required by Vulkan SDK on macOS since 1.3.216.
        let flags = if
               cfg!(target_os = "macos")
            && entry.version()? >= PORTABILITY_MACOS_VERSION
        {
            info!("Enabling extensions for macOS portability");
            extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
            extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION        .name.as_ptr());

            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        }
        else {
            vk::InstanceCreateFlags::empty()
        };

        let mut info = vk::InstanceCreateInfo::builder()
            .application_info       (&application_info)
            .enabled_layer_names    (&layers)
            .enabled_extension_names(&extensions)
            .flags                  (flags)
        ;

        type Severity  = vk::DebugUtilsMessageSeverityFlagsEXT;
        type Type      = vk::DebugUtilsMessageTypeFlagsEXT;
        type Messenger = vk::DebugUtilsMessengerCreateInfoEXT;

        let mut debug_info = Messenger::builder()
            .message_severity(Severity::all())
            .message_type    (
                  Type::GENERAL
                | Type::VALIDATION
                | Type::PERFORMANCE
            )
            .user_callback(Some(debug_callback))
            // .user_data    (data)
        ;


        if VALIDATION_ENABLED {
            // Enable debugging during the creation and destruction of an instance
            info = info.push_next(&mut debug_info);
        }

        let instance = entry.create_instance(&info, ALLOCATOR)?;

        let messenger = if VALIDATION_ENABLED {
            // Enable debugging for everything else
            instance.create_debug_utils_messenger_ext(&debug_info, ALLOCATOR)?
        }
        else {
            Default::default()
        };

        let instance = Instance {
            instance,
            p: PhantomData::<&'b Entry>,
        };

        Ok((instance, messenger))
    }
}

impl<'a> Drop for Instance<'a> {
    fn drop(&mut self) {
        trace!("dropping instance");
        unsafe {
            self.instance.destroy_instance(ALLOCATOR);
        }
        trace!("dropped instance");
    }
}

impl<'a> DerefMut for Instance<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instance
    }
}

impl<'a> Deref for Instance<'a> {
    type Target = vulkanalia::Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

extern "system" fn debug_callback(
    severity:  vk::DebugUtilsMessageSeverityFlagsEXT,
    type_:     vk::DebugUtilsMessageTypeFlagsEXT,
    data:      *const vk::DebugUtilsMessengerCallbackDataEXT,
    _app_data: *mut   c_void,
)
    -> vk::Bool32
{
    let data     = unsafe { *data };
    let message  = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    type F = vk::DebugUtilsMessageSeverityFlagsEXT;

    if      severity >= F::ERROR {
        error!("({:?}) {}", type_, message);
    }
    else if severity >= F::WARNING {
        warn! ("({:?}) {}", type_, message);
    }
    else if severity >= F::INFO {
        debug!("({:?}) {}", type_, message);
    }
    else {
        trace!("({:?}) {}", type_, message);
    }

    vk::FALSE
}
