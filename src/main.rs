mod backend;
mod drawables;

use backend::camera::{Camera, CameraMovement};
use backend::object::Object;
use backend::renderer::{Capabilities, ClearFlags, PolygonMode, Renderer};
use backend::shader::Shader;

use drawables::Terrain;

use glutin::{
    dpi::LogicalSize, ContextBuilder, DeviceEvent, ElementState, Event, EventsLoop, VirtualKeyCode,
    WindowBuilder, WindowEvent,
};

use vek::mat::*;
use vek::vec::*;

fn main() {
    let logical_size = LogicalSize::new(1024.0, 768.0);

    let mut event_loop = EventsLoop::new();

    let window_builder = WindowBuilder::new()
        .with_title("Procedural Generation")
        .with_dimensions(logical_size);

    let windowed_context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let renderer = Renderer::new(windowed_context);
    renderer.enable(Capabilities::DEPTH_TEST);

    let light_shader = Shader::from_file("shaders/terrain.vert", "shaders/terrain.frag");

    // Create object
    let point_grid = Object::new(Terrain, Vec3::new(0.0, 0.0, 0.0), None);

    let mut camera = Camera::new(
        Vec3::new(2.5, 8.0, 2.5),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        -89.0, // look down, careful not to get into gimbal lock and flip unexpectedly
        true,
    );

    let mut last_frame = std::time::Instant::now();
    let mut delta_frame;

    let mut running = true;
    while running {
        let now = std::time::Instant::now();
        delta_frame = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;

        std::thread::sleep(std::time::Duration::from_millis(
            (1000.0 / 60.0 - delta_frame / 1000.0) as u64,
        ));

        event_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    renderer.resize(logical_size);
                }
                WindowEvent::CloseRequested => {
                    running = false;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed {
                        let keycode = input.virtual_keycode.unwrap();
                        if keycode == VirtualKeyCode::Escape {
                            running = false;
                        } else if keycode == VirtualKeyCode::W {
                            camera.process_keyboard_inputs(CameraMovement::FORWARD, delta_frame);
                        } else if keycode == VirtualKeyCode::A {
                            camera.process_keyboard_inputs(CameraMovement::LEFT, delta_frame);
                        } else if keycode == VirtualKeyCode::S {
                            camera.process_keyboard_inputs(CameraMovement::BACKWARD, delta_frame);
                        } else if keycode == VirtualKeyCode::D {
                            camera.process_keyboard_inputs(CameraMovement::RIGHT, delta_frame);
                        } else if keycode == VirtualKeyCode::Q {
                            renderer.polygon_mode(PolygonMode::FILL);
                        } else if keycode == VirtualKeyCode::E {
                            renderer.polygon_mode(PolygonMode::LINE);
                        } else {
                            println!("Unknown scancode: {}", input.scancode);
                        }
                    };
                }
                _ => {}
            },
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    camera.process_mouse_inputs(delta.0 as _, delta.1 as _);
                }
            }
            _ => {}
        });

        // Clear screen for drawing
        renderer.clear(Vec4::new(0.2, 0.3, 0.6, 0.5), ClearFlags::COLOR_DEPTH);

        let projection = Mat4::perspective_rh_zo(
            f32::to_radians(camera.get_zoom()),
            (logical_size.width / logical_size.height) as _,
            0.1,
            1000.0,
        );

        light_shader.use_program();
        light_shader.set_mat4fv("view", &camera.get_view_matrix());
        light_shader.set_mat4fv("projection", &projection);
        light_shader.set_vec3f("light_color", &Vec3::new(1.0, 1.0, 1.0));
        light_shader.set_vec3f("light_pos", &Vec3::new(2.5, 100.0, 2.5));
        light_shader.set_vec3f("view_pos", camera.get_position());

        // Models
        let model = point_grid.get_transform();
        light_shader.set_mat4fv("model", &model);
        point_grid.draw();

        // Swap windows
        renderer.swap_buffers();
    }
}
