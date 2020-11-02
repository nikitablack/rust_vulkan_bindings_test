//#![feature(extern_types)]

mod utils;
mod vk;
mod vulkan;

//#[macro_use]
//extern crate bitflags;

struct WindowData {
    window: winit::window::Window,
}

#[derive(Default)]
struct VulkanData {
    api_version: vulkan::ApiVersion,
    instance_extensions: Vec<String>,
    instance: vulkan::Instance,
    surface: vulkan::Surface,
}

struct AppData {
    window_data: WindowData,
    vulkan_data: VulkanData,
}

fn get_instance_version(mut app_data: AppData) -> Result<AppData, String> {
    let api_version = match vulkan::enumerate_instance_version() {
        Ok(version) => version,
        Err(_) => return Err(String::from("failed to enumerate instance version")),
    };

    if api_version.major < 1 || api_version.minor < 1 {
        return Err(format!(
            "the current instance version is {}.{}.{}, the minimum supported version is 1.1.0",
            api_version.major, api_version.minor, api_version.patch
        ));
    }

    app_data.vulkan_data.api_version = api_version;

    Ok(app_data)
}

fn get_required_instance_extensions(mut app_data: AppData) -> Result<AppData, String> {
    app_data.vulkan_data.instance_extensions =
        match vulkan::get_required_window_extensions(&app_data.window_data.window) {
            Ok(extensions) => extensions,
            Err(_) => return Err(String::from("failed to get required window extension")),
        };

    Ok(app_data)
}

fn check_required_instance_extensions(app_data: AppData) -> Result<AppData, String> {
    let supported_instance_extensions = match vulkan::enumerate_instance_extension_properies() {
        Ok(props) => props,
        Err(_) => {
            return Err(String::from(
                "failed to enumerate instance extension properies",
            ))
        }
    };

    let mut supported_instance_extensions_set = std::collections::HashSet::new();
    for vulkan::ExtensionProperties { extension_name, .. } in &supported_instance_extensions {
        supported_instance_extensions_set.insert(extension_name);
    }

    for extension_name in &app_data.vulkan_data.instance_extensions {
        if !supported_instance_extensions_set.contains(extension_name) {
            return Err(format!(
                "instance extension {} is not supported",
                extension_name
            ));
        }
    }

    Ok(app_data)
}

fn create_instance(mut app_data: AppData) -> Result<AppData, String> {
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
        enabled_layer_names: vec![],
        enabled_extension_names: &app_data.vulkan_data.instance_extensions,
    };

    app_data.vulkan_data.instance = match vulkan::create_instance(&instance_create_info) {
        Ok(instance) => instance,
        Err(_) => return Err(String::from("failed to get create instance")),
    };

    Ok(app_data)
}

fn create_surface(mut app_data: AppData) -> Result<AppData, String> {
    app_data.vulkan_data.surface = match vulkan::create_surface(
        &app_data.window_data.window,
        &app_data.vulkan_data.instance,
    ) {
        Ok(surface) => surface,
        Err(_) => return Err(String::from("failed to get create surface")),
    };

    Ok(app_data)
}

fn check_required_device_extensions(
    physical_device: &vulkan::PhysicalDevice,
    required_extensions: &Vec<String>,
) -> Result<(), String> {
    let supported_device_extensions =
        match vulkan::enumerate_device_extension_properies(physical_device) {
            Ok(props) => props,
            Err(_) => {
                return Err(String::from(
                    "failed to enumerate instance extension properies",
                ))
            }
        };

    let mut supported_device_extensions_set = std::collections::HashSet::new();
    for vulkan::ExtensionProperties { extension_name, .. } in &supported_device_extensions {
        supported_device_extensions_set.insert(extension_name);
    }

    for extension_name in required_extensions {
        if !supported_device_extensions_set.contains(extension_name) {
            return Err(format!(
                "device extension {} is not supported",
                extension_name
            ));
        }
    }

    Ok(())
}

fn check_device_suitability(
    physical_device: &vulkan::PhysicalDevice,
    required_extensions: &Vec<String>,
) -> Result<(), String> {
    let properties = vulkan::get_physical_device_properties(&physical_device);

    if properties.device_type != vk::VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU {
        return Err(format!(
            "the device {} is not a discrete GPU",
            properties.device_name
        ));
    }

    let features = vulkan::get_physical_device_features(physical_device);

    if !features.tessellation_shader {
        return Err(format!(
            "the device {} does not support tesselation shader",
            properties.device_name
        ));
    }

    if !features.fill_mode_non_solid {
        return Err(format!(
            "the device {} does not support fill mode non solid",
            properties.device_name
        ));
    }

    check_required_device_extensions(physical_device, required_extensions)?;

    Ok(())
}

fn get_physical_devie(mut app_data: AppData) -> Result<AppData, String> {
    let devices = match vulkan::enumerate_physical_devices(&app_data.vulkan_data.instance) {
        Ok(devices) => devices,
        Err(_) => return Err(String::from("failed to enumerate physical devices")),
    };

    for device in &devices {
        match check_device_suitability(device, &Vec::<String>::new()) {
            Ok(_) => (),
            Err(_) => continue,
        }
    }

    Ok(app_data)
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Vulkan Teapot")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    let app_data = AppData {
        window_data: WindowData { window },
        vulkan_data: VulkanData::default(),
    };

    let result = get_instance_version(app_data)
        .and_then(get_required_instance_extensions)
        .and_then(check_required_instance_extensions)
        .and_then(create_instance)
        .and_then(create_surface)
        .and_then(get_physical_devie);

    let app_data = match result {
        Ok(app_data) => app_data,
        Err(err) => panic!(err),
    };

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = winit::event_loop::ControlFlow::Exit;

            vulkan::destroy_surface(
                &app_data.vulkan_data.instance,
                &app_data.vulkan_data.surface,
                std::ptr::null(),
            );
        }
        _ => (),
    });
}
