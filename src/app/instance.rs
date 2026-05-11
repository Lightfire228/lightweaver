use std::{collections::HashSet, ffi::{CStr, c_void}, rc::Rc};
use anyhow::{anyhow, Result};

use log::*;
use vulkanalia::{Entry, vk::{self, EntryV1_0, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands}, window as vk_window};
use winit::window::Window;
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::app::{DEVICE_EXTENSIONS, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYER, device::SuitabilityError, instance, surface::{self, Surface}};


pub struct Instance {
    pub entry:     Rc<Entry>,
        instance:  VkInstance,
        messenger: vk::DebugUtilsMessengerEXT,
}

#[derive(Clone, Debug, Default)]
pub struct SwapchainSupport {
    pub capabilities:  vk::SurfaceCapabilitiesKHR,
    pub formats:       Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}


#[derive(Clone, Copy, Debug)]
pub struct QueueFamilyIndices {
    pub graphics: u32,
    pub present:  u32,
}


impl Instance {
    pub fn new(window: &Window, entry: Rc<Entry>) -> Result<Rc<Self>> {

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

        Ok(Rc::new(Self {
            entry,
            instance,
            messenger,
        }))
    }

    pub fn instance(&self) -> &VkInstance {
        &self.instance
    }

    pub fn pick_physical_device(&self, surface: &Surface) -> Result<vk::PhysicalDevice> {

        let devices = unsafe {
            self.instance.enumerate_physical_devices()?
        };

        for physical_device in devices {

            let properties = unsafe {
                self.instance.get_physical_device_properties(physical_device)
            };

            if let Err(error) = self.check_physical_device(surface, physical_device) {
                warn!("Skipping physical device (`{}`): {}", properties.device_name, error);
            }
            else {
                info!("Selected physical device (`{}`)", properties.device_name);
                return Ok(physical_device);
            }
        }

        Err(anyhow!("Failed to find suitable physical device"))
    }

    fn check_physical_device(
        &self,
        surface:         &Surface,
        physical_device: vk::PhysicalDevice,
    )
        -> Result<()>
    {
        let support = unsafe {
            self.get_queue_family_indices(surface, physical_device)?;
            self.check_physical_device_extensions(physical_device)?;

            self.get_swapchain_support(surface, physical_device)?
        };

        if support.formats.is_empty() || support.present_modes.is_empty() {
            return Err(anyhow!(SuitabilityError("Insufficient swapchain support")));
        }


        let features = unsafe {
            self.instance.get_physical_device_features(physical_device)
        };


        if features.sampler_anisotropy != vk::TRUE {
            return Err(anyhow!(SuitabilityError("No sampler anisotropy")));
        }


        Ok(())
    }


    unsafe fn check_physical_device_extensions(
        &self,
        physical_device: vk::PhysicalDevice,
    )
        -> Result<()>
    {
        let extensions: HashSet<_> = unsafe {
            self
                .instance
                .enumerate_device_extension_properties(physical_device, None)?
                .iter   ()
                .map    (|e| e.extension_name)
                .collect()
        };

        if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
            Ok(())
        }
        else {
            Err(anyhow!(SuitabilityError("Missing required device extensions")))
        }
    }

    pub fn get_swapchain_support(
        &self,
        surface:         &Surface,
        physical_device: vk::PhysicalDevice,
    )
        -> Result<SwapchainSupport>
    {
        Ok(unsafe {
            let surface = surface.surface();

            SwapchainSupport {
                capabilities:  self.instance.get_physical_device_surface_capabilities_khr (physical_device, surface)?,
                formats:       self.instance.get_physical_device_surface_formats_khr      (physical_device, surface)?,
                present_modes: self.instance.get_physical_device_surface_present_modes_khr(physical_device, surface)?,
            }
        })
    }

    pub fn get_queue_family_indices(
        &self,
        surface:         &Surface,
        physical_device: vk::PhysicalDevice,
    )
        -> Result<QueueFamilyIndices>
    {
        let properties = unsafe {
            self.instance.get_physical_device_queue_family_properties(physical_device)
        };
        let graphics   = properties
            .iter    ()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map     (|i| i as u32)
        ;

        let mut present = None;
        for (index, _properties) in properties.iter().enumerate() {

            let does_support = unsafe {
                self.instance.get_physical_device_surface_support_khr(physical_device, index as u32, surface.surface())?
            };

            if does_support {
                present = Some(index as u32);
                break;
            }
        }

        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(QueueFamilyIndices { graphics, present })
        }
        else {
            Err(anyhow!(SuitabilityError("Missing required queue families")))
        }

    }

}

impl Drop for Instance {

    fn drop(&mut self) {
        debug!("Dropping Instance");

        unsafe {
            if VALIDATION_ENABLED {
                self.instance.destroy_debug_utils_messenger_ext(self.messenger, None);
            }

            self.instance.destroy_instance(None);
        }

        debug!("/Dropping Instance");
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
