use bitflags::bitflags;

use vek::vec::Vec4;

use glutin::{NotCurrent, PossiblyCurrent, WindowedContext};

use crate::backend::gl_gen::gl;

bitflags! {
    pub struct ClearFlags: u32 {
        const COLOR = gl::COLOR_BUFFER_BIT;
        const DEPTH = gl::DEPTH_BUFFER_BIT;
        const COLOR_DEPTH = Self::COLOR.bits | Self::DEPTH.bits;
    }
}

bitflags! {
    pub struct PolygonMode: u32 {
        const LINE = gl::LINE;
        const FILL = gl::FILL;
    }
}

bitflags! {
    pub struct Capabilities: u32 {
        const BLEND = gl::BLEND;
        const CULL_FACE = gl::CULL_FACE;
        const DEPTH_TEST = gl::DEPTH_TEST;
        const SCISSOR_TEST = gl::SCISSOR_TEST;
        const STENCIL_TEST = gl::STENCIL_TEST;
    }
}

pub struct Renderer(WindowedContext<PossiblyCurrent>);

#[allow(dead_code)]
impl Renderer {
    pub fn new(windowed_context: WindowedContext<NotCurrent>) -> Self {
        let window = unsafe { windowed_context.make_current().unwrap() };

        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

        Self(window)
    }

    pub fn enable(&self, capabilities: Capabilities) {
        unsafe {
            gl::Enable(capabilities.bits());
        }
    }

    pub fn disable(&self, capabilities: Capabilities) {
        unsafe {
            gl::Disable(capabilities.bits());
        }
    }

    pub fn polygon_mode(&self, mode: PolygonMode) {
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, mode.bits());
        }
    }

    pub fn resize(&self, size: glutin::dpi::LogicalSize) {
        self.0.window().set_inner_size(size);
        unsafe {
            gl::Viewport(0, 0, size.width as _, size.height as _);
        }
    }

    pub fn clear(&self, color: Vec4<f32>, flags: ClearFlags) {
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
            gl::Clear(flags.bits as _);
        }
    }

    pub fn check_errors(&self) -> gl::types::GLenum {
        unsafe { gl::GetError() }
    }

    pub fn swap_buffers(&self) {
        self.0.swap_buffers().expect("Unable to swap buffers");
    }

    pub fn window(&self) -> &WindowedContext<PossiblyCurrent> {
        &self.0
    }
}
