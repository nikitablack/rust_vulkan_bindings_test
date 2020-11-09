mod app;
pub mod utils;
pub mod vk;
pub mod vulkan;

#[macro_use]
extern crate bitflags;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Vulkan Teapot")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    let mut app_data = app::AppData {
        window_data: app::WindowData { window },
        vulkan_data: app::VulkanData::default(),
    };

    app_data
        .vulkan_data
        .device_extensions
        .push(String::from("VK_KHR_swapchain"));

    let result = app::get_instance_version(app_data)
        .and_then(app::get_required_instance_extensions)
        .and_then(app::check_required_instance_extensions)
        .and_then(app::create_instance)
        .and_then(app::create_surface)
        .and_then(app::get_physical_device)
        .and_then(app::create_logical_device)
        .map(app::get_device_queue);

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
