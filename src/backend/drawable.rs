use std::mem;

use bitflags::bitflags;

use crate::backend::gl_gen::gl;
use crate::backend::gl_gen::gl::types::*;

bitflags! {
    pub struct DrawType: u32 {
        const BUFFERED = gl::ARRAY_BUFFER;
        const INDEXED = gl::ELEMENT_ARRAY_BUFFER;
    }
}

bitflags! {
    pub struct DrawPrimitive: u32 {
        const POINTS = gl::POINTS;
        const LINES = gl::LINES;
        const TRIANGLES = gl::TRIANGLES;
        const TRIANGLE_STRIP = gl::TRIANGLE_STRIP;
    }
}

#[allow(dead_code)]
pub enum Buffer {
    ArrayBuffer {
        vertex_attributes: Vec<f32>,
        vertex_attribute_pointers: Vec<VertexAttribPointer>,
    },
    IndexBuffer {
        vertex_attributes: Vec<f32>,
        vertex_attribute_pointers: Vec<VertexAttribPointer>,
        indices: Vec<u32>,
    },
}

pub struct VertexArrayObject(pub GLuint, Vec<Box<dyn GlBuffer>>);

impl VertexArrayObject {
    pub fn new() -> Self {
        let mut vao = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArrayObject(vao, vec![])
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.0);
        }
    }

    pub fn attach_buffer<B: GlBuffer + 'static>(&mut self, buffer: B) {
        self.1.push(Box::new(buffer));
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.0);
        }
    }
}

pub trait GlBuffer {}

pub enum BufferHint {
    STATIC,
    _DYNAMIC,
    _STREAM,
}

pub struct ArrayBuffer(GLuint);

impl GlBuffer for ArrayBuffer {}

impl ArrayBuffer {
    pub fn new() -> Self {
        let mut vbo = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        ArrayBuffer(vbo)
    }

    pub fn bind_data(
        &mut self,
        vertex_attributes: Vec<f32>,
        vertex_attribute_pointers: Vec<VertexAttribPointer>,
        hint: BufferHint,
    ) {
        let data = &vertex_attributes;

        let hint = match hint {
            BufferHint::_DYNAMIC => gl::DYNAMIC_DRAW,
            BufferHint::STATIC => gl::STATIC_DRAW,
            BufferHint::_STREAM => gl::STREAM_DRAW,
        };

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.0);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of_val(&data[0])) as _,
                data.as_ptr() as _,
                hint,
            );

            for ptr in &vertex_attribute_pointers {
                gl::VertexAttribPointer(
                    ptr.index,
                    ptr.size as _,
                    gl::FLOAT,
                    0,
                    ptr.stride as _,
                    ptr.offset as _,
                );
                gl::EnableVertexAttribArray(ptr.index);
            }

            // Unbind vertex array
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.0);
        }
    }
}

pub struct ElementBuffer(GLuint);

impl GlBuffer for ElementBuffer {}

impl ElementBuffer {
    pub fn new() -> Self {
        let mut vbo = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        ElementBuffer(vbo)
    }

    pub fn bind_data(&mut self, indices: Vec<u32>, hint: BufferHint) {
        let data = &indices;

        let hint = match hint {
            BufferHint::_DYNAMIC => gl::DYNAMIC_DRAW,
            BufferHint::STATIC => gl::STATIC_DRAW,
            BufferHint::_STREAM => gl::STREAM_DRAW,
        };

        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.0);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * mem::size_of_val(&data[0])) as _,
                data.as_ptr() as _,
                hint,
            );
        }
    }
}

impl Drop for ElementBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.0);
        }
    }
}

pub struct DrawableAttributes {
    pub buffer: Buffer,
    pub draw_count: usize,
    pub draw_primitive: DrawPrimitive,
}

pub trait Drawable {
    fn vertex_attributes(&self) -> DrawableAttributes;
}

pub struct VertexAttribPointer {
    pub index: u32,
    pub size: usize,
    pub stride: usize,
    pub offset: usize,
}
