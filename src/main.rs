//#![feature(extern_types)]

mod vk;
mod vulkan;

//#[macro_use]
//extern crate bitflags;

fn main() {
    match vulkan::enumerate_instance_version() {
        Ok(version) => println!("{:?}", version),
        Err(err) => println!("{:?}", err),
    }

    match vulkan::enumerate_instance_extension_properies() {
        Ok(props) => println!("{:?}", props),
        Err(err) => println!("{:?}", err),
    };

    let application_info = vulkan::ApplicationInfo {
        application_name: String::from("Vulkan Teapot"),
        application_version: 1,
        engine_name: String::from(""),
        engine_version: 0,
        api_version: vulkan::ApiVersion {
            major: 1,
            minor: 1,
            patch: 0,
        },
    };

    let instance_create_info = vulkan::InstanceCreateInfo {
        next: std::ptr::null(),
        flags: 0,
        application_info: Some(&application_info),
        enabled_layer_names: vec![
            String::from("VK_LAYER_KHRONOS_validation"),
            String::from("VK_LAYER_LUNARG_monitor"),
        ],
        enabled_extension_names: vec![],
    };
    match vulkan::create_instance(&instance_create_info) {
        Ok(instance) => println!("{:?}", instance),
        Err(err) => println!("{:?}", err),
    };
}
