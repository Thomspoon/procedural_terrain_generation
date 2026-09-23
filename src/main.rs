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
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, KeyEvent, WindowEvent};
use winit::{
    dpi::LogicalSize,
    event::ElementState,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};
use std::num::NonZeroU32;
use std::time::Instant;
use vek::mat::*;
use vek::vec::*;

struct App {
    window: Window,
    renderer: Renderer,
    terrain_shader: Shader,
    point_grid: Object,
    camera: Camera,
    grass_id: u32,
    last_frame: Instant,
}

impl App {
    fn delta(&mut self) -> f32 {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        dt
    }

    fn render(&mut self) {
        let Self { window, renderer, terrain_shader, point_grid, camera, .. } = self;

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
        terrain_shader.set_sampler2D("texture", self.grass_id);
        terrain_shader.set_vec3f("light_color", &Vec3::new(1.0, 1.0, 1.0));
        terrain_shader.set_vec3f("light_pos", &Vec3::new(250.0, 100.0, 250.0));

        let model = point_grid.get_transform();
        terrain_shader.set_mat4fv("model", &model);
        point_grid.draw();

        renderer.swap_buffers();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let dt = self.delta();

        match event {
            WindowEvent::Resized(size) => {
                let width = NonZeroU32::new(size.width).unwrap();
                let height = NonZeroU32::new(size.height).unwrap();
                self.renderer.resize(width, height);
            }
            WindowEvent::CloseRequested => {
                println!("closing!!");
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key: key, state: ElementState::Pressed, .. },
                ..
            } => match key.as_ref() {
                Key::Named(NamedKey::Escape) => event_loop.exit(),
                Key::Character("w") => {
                    self.camera.process_keyboard_inputs(CameraMovement::FORWARD, dt);
                }
                Key::Character("a") => {
                    self.camera.process_keyboard_inputs(CameraMovement::LEFT, dt);
                }
                Key::Character("s") => {
                    self.camera.process_keyboard_inputs(CameraMovement::BACKWARD, dt);
                }
                Key::Character("d") => {
                    self.camera.process_keyboard_inputs(CameraMovement::RIGHT, dt);
                }
                Key::Character("q") => {
                    self.renderer.polygon_mode(PolygonMode::FILL);
                }
                Key::Character("e") => {
                    self.renderer.polygon_mode(PolygonMode::LINE);
                }
                _ => {}
            },
            _ => {}
        }

        self.render();
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.camera.process_mouse_inputs(delta.0 as _, delta.1 as _);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window_attributes = WindowAttributes::default()
        .with_title("Procedural Generation")
        .with_inner_size(LogicalSize::new(1024.0, 768.0));

    let template = ConfigTemplateBuilder::new();
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

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
    let raw_handle = window.window_handle().unwrap().as_raw();

    let context_attributes = ContextAttributesBuilder::new()
        .with_profile(GlProfile::Core)
        .with_context_api(ContextApi::OpenGl(Some(Version::new(4, 5))))
        .build(Some(raw_handle));

    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(raw_handle));

    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(Some(raw_handle));

    let not_current_gl_context = unsafe {
        display.create_context(&config, &context_attributes)
            .or_else(|_| display.create_context(&config, &fallback_context_attributes))
            .or_else(|_| display.create_context(&config, &legacy_context_attributes))
            .unwrap()
    };

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_handle,
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

    let camera = Camera::new(
        Vec3::new(2.5, 8.0, 2.5),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        -89.0,
        true,
    );

    let mut app = App {
        window,
        renderer,
        terrain_shader,
        point_grid,
        grass_id,
        camera,
        last_frame: Instant::now(),
    };

    event_loop.run_app(&mut app).unwrap();
}
