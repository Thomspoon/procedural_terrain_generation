mod backend;
mod drawables;

use backend::camera::Camera;
use backend::object::Object;
use backend::renderer::{Renderer, Capabilities, ClearFlags, PolygonMode};
use backend::shader::Shader;
use backend::texture::Texture;

use backend::CameraMovement;
use drawables::Terrain;

use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, GlProfile, Version},
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::event::DeviceEvent;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};
use std::num::NonZeroU32;
use vek::mat::*;
use vek::vec::*;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window_builder = WindowBuilder::new()
        .with_title("Procedural Generation")
        .with_inner_size(LogicalSize::new(1024.0, 768.0));

    let template = ConfigTemplateBuilder::new();
    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, config) = display_builder
        .build(&event_loop, template, |configs| configs.reduce(|accum, config| {
            if config.supports_transparency().unwrap_or(false) {
                config
            } else {
                accum
            }
        }).unwrap())
        .unwrap();

    let window = window.unwrap();

    let display = config.display();

    let context_attributes = ContextAttributesBuilder::new()
        .with_profile(GlProfile::Core)
        .with_context_api(ContextApi::OpenGl(Some(Version::new(4, 5))))
        .build(Some(window.raw_window_handle()));

    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(window.raw_window_handle()));

    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(Some(window.raw_window_handle()));

    let not_current_gl_context = unsafe {
        display.create_context(&config, &context_attributes)
            .or_else(|_| display.create_context(&config, &fallback_context_attributes))
            .or_else(|_| display.create_context(&config, &legacy_context_attributes))
            .unwrap()
    };

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window.raw_window_handle(),
        NonZeroU32::new(window.inner_size().width).unwrap(),
        NonZeroU32::new(window.inner_size().height).unwrap(),
    );

    let surface = unsafe {
        display.create_window_surface(&config, &attrs).unwrap()
    };

    let gl_context = not_current_gl_context.make_current(&surface).unwrap();

    let renderer = Renderer::new(gl_context, surface, &display);
    renderer.enable(Capabilities::DEPTH_TEST);

    let terrain_shader = Shader::from_file("shaders/terrain.vert", "shaders/terrain.frag");

    let (grass, grass_id) = Texture::new("textures/low_def_grass.jpg");

    let point_grid = Object::new(Terrain, Vec3::new(0.0, 0.0, 0.0), Some(grass));

    let mut camera = Camera::new(
        Vec3::new(2.5, 8.0, 2.5),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        -89.0,
        true,
    );

    let mut last_frame = std::time::Instant::now();
    let mut delta_frame = 0f32;

    event_loop.run(move |event, event_loop| {
        let now = std::time::Instant::now();
        delta_frame = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    let width = NonZeroU32::new(size.width).unwrap();
                    let height = NonZeroU32::new(size.height).unwrap();
                    renderer.resize(width, height);
                }
                WindowEvent::CloseRequested => {
                    println!("closing!!");
                    event_loop.exit()
                }
                WindowEvent::KeyboardInput {
                    event: KeyEvent { logical_key: key, state: ElementState::Pressed, .. },
                    ..
                } => match key.as_ref() {
                        Key::Named(NamedKey::Escape) => {
                            event_loop.exit()
                        }
                        Key::Character("w") => {
                            camera.process_keyboard_inputs(CameraMovement::FORWARD, delta_frame);
                        }
                        Key::Character("a") => {
                            camera.process_keyboard_inputs(CameraMovement::LEFT, delta_frame);
                        }
                        Key::Character("s") => {
                            camera.process_keyboard_inputs(CameraMovement::BACKWARD, delta_frame);
                        }
                        Key::Character("d") => {
                            camera.process_keyboard_inputs(CameraMovement::RIGHT, delta_frame);
                        }
                        Key::Character("q") => {
                            renderer.polygon_mode(PolygonMode::FILL);
                        }
                        Key::Character("e") => {
                            renderer.polygon_mode(PolygonMode::LINE);
                        }
                        _ => {}
                }

                _ => {}
            },
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    camera.process_mouse_inputs(delta.0 as _, delta.1 as _);
                }
            }
            _ => {}
        }

        renderer.clear(Vec4::new(0.2, 0.3, 0.6, 0.5), ClearFlags::COLOR_DEPTH);

        let projection = Mat4::perspective_rh_zo(
            f32::to_radians(camera.get_zoom()),
            window.inner_size().width as f32 / window.inner_size().height as f32,
            0.1,
            1000.0,
        );

        terrain_shader.use_program();
        terrain_shader.set_mat4fv("view", &camera.get_view_matrix());
        terrain_shader.set_mat4fv("projection", &projection);
        terrain_shader.set_sampler2D("texture", grass_id);
        terrain_shader.set_vec3f("light_color", &Vec3::new(1.0, 1.0, 1.0));
        terrain_shader.set_vec3f("light_pos", &Vec3::new(250.0, 100.0, 250.0));

        let model = point_grid.get_transform();
        terrain_shader.set_mat4fv("model", &model);
        point_grid.draw();

        renderer.swap_buffers();
    }).unwrap();
}
