use std::{collections::HashSet, ffi::{CStr, c_void}, rc::Rc};
use anyhow::{anyhow, Result};

use log::*;
use vulkanalia::{Entry, vk::{self, EntryV1_0, HasBuilder, InstanceV1_0}, window as vk_window};
use winit::window::Window;
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::app::instance;

const PORTABILITY_MACOS_VERSION: Version           = Version::new(1, 3, 216);
const VALIDATION_ENABLED:        bool              = cfg!(debug_assertions);
const VALIDATION_LAYER:          vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");



pub struct Instance {
    entry:     Rc<Entry>,
    instance:  VkInstance,
    messenger: vk::DebugUtilsMessengerEXT,
}


impl Instance {
    pub fn new(window: &Window, entry: Rc<Entry>) -> Result<Self> {

        let application_info = vk::ApplicationInfo::builder()
            .application_name   (b"Lightweaver\0")
            .application_version(vk::make_version(1, 0, 0))
            .engine_name        (b"No Engine\0")
            .engine_version     (vk::make_version(1, 0, 0))
            .api_version        (vk::make_version(1, 0, 0))
        ;

        let available_layers: HashSet<_> = unsafe {
            entry
                .enumerate_instance_layer_properties()?

                .iter   ()
                .map    (|l| l.layer_name)
                .collect()
        };


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

        let instance = unsafe { entry.create_instance(&info, None)? };

        let messenger = if VALIDATION_ENABLED { unsafe {
            // Enable debugging for everything else
            instance.create_debug_utils_messenger_ext(&debug_info, None)?
        }}
        else {
            Default::default()
        };

        Ok(Self {
            entry,
            instance,
            messenger,
        })
    }
}

impl Drop for Instance {

    fn drop(&mut self) {

        unsafe {
            if VALIDATION_ENABLED {
                self.instance.destroy_debug_utils_messenger_ext(self.messenger, None);
            }

            self.instance.destroy_instance(None);
        }
    }
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
