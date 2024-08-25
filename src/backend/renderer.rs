use bitflags::bitflags;
use glutin::prelude::*;
use glutin::context::PossiblyCurrentContext;
use glutin::surface::Surface;

use std::ffi::CString;
use std::num::NonZeroU32;
use vek::vec::Vec4;

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

pub struct Renderer {
    context: PossiblyCurrentContext,
    surface: Surface<glutin::surface::WindowSurface>,
}

#[allow(dead_code)]
impl Renderer {
    pub fn new<D: GlDisplay>(
        context: PossiblyCurrentContext,
        surface: Surface<glutin::surface::WindowSurface>,
        gl_display: &D,
    ) -> Self {
        let _ = gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });
        Self { context, surface }
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

    pub fn resize(&self, width: NonZeroU32, height: NonZeroU32) {
        self.surface
            .resize(&self.context, width, height);
        unsafe {
            gl::Viewport(0, 0, width.get() as _, height.get() as _);
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
        self.surface
            .swap_buffers(&self.context)
            .expect("Unable to swap buffers");
    }
}
