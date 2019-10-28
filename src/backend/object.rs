use vek::mat::Mat4;
use vek::vec::Vec3;

use crate::backend::drawable::{
    ArrayBuffer, Buffer, BufferHint, DrawPrimitive, DrawType, Drawable, DrawableAttributes,
    ElementBuffer, VertexArrayObject,
};
use crate::backend::gl_gen::gl;

use crate::backend::texture::Texture;

#[allow(dead_code)]
pub enum TransformType {
    Scale(Vec3<f32>),
    Rotate(f32, Vec3<f32>),
    Translate(Vec3<f32>),
}

pub struct Transform {
    pub translation: Vec3<f32>,
    pub rotation: (f32, Vec3<f32>),
    pub scale: Vec3<f32>,
}

impl Transform {
    fn new(origin: Vec3<f32>) -> Self {
        Self {
            translation: origin,
            rotation: (0.0, Vec3::new(1.0, 1.0, 1.0)),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

pub struct Object {
    vao: VertexArrayObject,
    draw_count: usize,
    transform: Transform,
    draw_type: DrawType,
    draw_primitive: DrawPrimitive,
    texture: Option<Texture>,
}

#[allow(dead_code)]
impl Object {
    pub fn new<D: Drawable>(drawable: D, origin: Vec3<f32>, texture: Option<Texture>) -> Self {
        let mut vao = VertexArrayObject::new();

        vao.bind();

        let DrawableAttributes {
            buffer,
            draw_count,
            draw_primitive,
        } = Drawable::vertex_attributes(&drawable);

        let mut vbo = ArrayBuffer::new();

        let transform = Transform::new(origin);

        let draw_type = match buffer {
            Buffer::ArrayBuffer {
                vertex_attributes,
                vertex_attribute_pointers,
            } => {
                vbo.bind_data(
                    vertex_attributes,
                    vertex_attribute_pointers,
                    BufferHint::STATIC,
                );
                vao.attach_buffer(vbo);

                DrawType::BUFFERED
            }
            Buffer::IndexBuffer {
                vertex_attributes,
                vertex_attribute_pointers,
                indices,
            } => {
                vbo.bind_data(
                    vertex_attributes,
                    vertex_attribute_pointers,
                    BufferHint::STATIC,
                );
                vao.attach_buffer(vbo);

                let mut ebo = ElementBuffer::new();

                ebo.bind_data(indices, BufferHint::STATIC);

                vao.attach_buffer(ebo);

                DrawType::INDEXED
            }
        };

        Object {
            vao,
            draw_count,
            transform,
            draw_type,
            draw_primitive,
            texture,
        }
    }

    pub fn transform(&mut self, transform_type: TransformType) {
        let mut transform = &mut self.transform;

        match transform_type {
            TransformType::Translate(t) => {
                transform.translation = t;
            }
            TransformType::Rotate(r, v) => {
                transform.rotation = (r, v);
            }
            TransformType::Scale(s) => {
                transform.scale = s;
            }
        }
    }

    pub fn get_translation(&self) -> Vec3<f32> {
        self.transform.translation
    }

    pub fn get_transform(&self) -> Mat4<f32> {
        let mut model = Mat4::identity();
        model.scale_3d(self.transform.scale);
        model.rotate_3d(
            f32::to_radians(self.transform.rotation.0),
            self.transform.rotation.1,
        );
        model.translate_3d(self.transform.translation);
        model
    }

    pub fn draw(&self) {
        unsafe {
            if self.texture.is_some() {
                self.texture.as_ref().unwrap().bind();
            }

            self.vao.bind();

            match self.draw_type {
                DrawType::BUFFERED => {
                    gl::DrawArrays(self.draw_primitive.bits(), 0, self.draw_count as _);
                }
                DrawType::INDEXED => {
                    gl::DrawElements(
                        self.draw_primitive.bits(),
                        self.draw_count as _,
                        gl::UNSIGNED_INT,
                        std::ptr::null(),
                    );
                }
                _ => unreachable!(),
            }
        }
    }
}
