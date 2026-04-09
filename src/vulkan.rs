#![deny(unsafe_op_in_unsafe_fn)]

mod validation_layers;

use anyhow::{Result, Ok, anyhow};
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;
use winit::window::Window;
use log::*;
use vulkanalia::Version;
use vulkanalia::loader::{LIBRARY, LibloadingLoader};
use vulkanalia::prelude::v1_4::*;
use vulkanalia::window as vk_window;

type VkAllocator<'a> = Option<&'a vk::AllocationCallbacks>;
const ALLOCATOR: VkAllocator = None;

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

pub const VALIDATION_ENABLED: bool = cfg!(debug_assertions);



pub type Bytes = Vec<*const i8>;


#[derive(Debug)]
pub struct VulkanApp {
    entry:    Entry,
    instance: Instance,
    data:     AppData,
}

#[derive(Debug, Default)]
struct AppData {
    messenger: vk::DebugUtilsMessengerEXT,
}

impl VulkanApp {
    pub unsafe fn new(window: &Window) -> Result<Self> { unsafe {

        let mut data = AppData::default();

        let loader   = LibloadingLoader::new(LIBRARY)?;
        let entry    = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let instance = data.create_instance(window, &entry)?;

        Ok(Self {
            entry,
            instance,
            data,
        })
    }}
}


impl AppData {

    unsafe fn create_instance(&mut self, window: &Window, entry: &Entry) -> Result<Instance> {
        let application_info = vk::ApplicationInfo::builder()
            .application_name   (b"Lightweaver\0")
            .application_version(vk::make_version(1, 0, 0))
            .engine_name        (b"No Engine\0")
            .engine_version     (vk::make_version(1, 0, 0))
            .api_version        (vk::make_version(1, 4, 0))
        ;

        let mut extensions: Vec<_> = vk_window::get_required_instance_extensions(window)
            .iter   ()
            .map    (|e| e.as_ptr())
            .collect()
        ;

        validation_layers::push_validation_ext(&mut extensions);

        let flags  = get_mac_compat_flags(&mut extensions, entry)?;
        let layers = unsafe {
            validation_layers::get_layers(&entry)?
        };

        let mut info = vk::InstanceCreateInfo::builder()
            .application_info       (&application_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names    (&layers)
            .flags                  (flags)
        ;

        let mut debug_info = validation_layers::get_debug_info();

        let instance = 
            if VALIDATION_ENABLED {

                unsafe {
                    info = info.push_next(&mut debug_info);

                    let instance   = entry.create_instance(&info, ALLOCATOR)?;
                    self.messenger = instance.create_debug_utils_messenger_ext(&debug_info, ALLOCATOR)?;

                    instance
                }
            }
            else {
                unsafe { entry.create_instance(&info, ALLOCATOR)? }
            }
        ;

        Ok(instance)

    }

}

impl Drop for VulkanApp {
    fn drop(&mut self) { unsafe {

        if VALIDATION_ENABLED {
            self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, ALLOCATOR);
        }

        self.instance.destroy_instance(ALLOCATOR);
    }}
}

fn get_mac_compat_flags(extensions: &mut Bytes, entry: &Entry)
    -> Result<vk::InstanceCreateFlags>
{

    // Required by Vulkan SDK on macOS since 1.3.216.
    if !(
        cfg!(target_os = "macos") 
        && entry.version()? >= PORTABILITY_MACOS_VERSION
    ) {
        return Ok(vk::InstanceCreateFlags::empty());
    }

    info!("Enabling extensions for macOS portability.");
    extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
    extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION        .name.as_ptr());

    Ok(vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR)

}