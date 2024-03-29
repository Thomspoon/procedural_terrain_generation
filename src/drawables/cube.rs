use crate::backend::drawable::*;

use std::mem;

pub struct Cube;

impl Drawable for Cube {
    fn vertex_attributes(&self) -> DrawableAttributes {
        #[rustfmt::skip]
        let vertex_attributes = [
            -0.5, -0.5, -0.5,
             0.0,  0.0, -1.0,
             0.5, -0.5, -0.5,
             0.0,  0.0, -1.0,
             0.5,  0.5, -0.5,
             0.0,  0.0, -1.0,
             0.5,  0.5, -0.5,
             0.0,  0.0, -1.0,
            -0.5,  0.5, -0.5,
             0.0,  0.0, -1.0,
            -0.5, -0.5, -0.5,
             0.0,  0.0, -1.0,
            -0.5, -0.5,  0.5,
             0.0,  0.0,  1.0,
             0.5, -0.5,  0.5,
             0.0,  0.0,  1.0,
             0.5,  0.5,  0.5,
             0.0,  0.0,  1.0,
             0.5,  0.5,  0.5,
             0.0,  0.0,  1.0,
            -0.5,  0.5,  0.5,
             0.0,  0.0,  1.0,
            -0.5, -0.5,  0.5,
             0.0,  0.0,  1.0,
            -0.5,  0.5,  0.5,
            -1.0,  0.0,  0.0,
            -0.5,  0.5, -0.5,
            -1.0,  0.0,  0.0,
            -0.5, -0.5, -0.5,
            -1.0,  0.0,  0.0,
            -0.5, -0.5, -0.5,
            -1.0,  0.0,  0.0,
            -0.5, -0.5,  0.5,
            -1.0,  0.0,  0.0,
            -0.5,  0.5,  0.5,
            -1.0,  0.0,  0.0,
             0.5,  0.5,  0.5,
             1.0,  0.0,  0.0,
             0.5,  0.5, -0.5,
             1.0,  0.0,  0.0,
             0.5, -0.5, -0.5,
             1.0,  0.0,  0.0,
             0.5, -0.5, -0.5,
             1.0,  0.0,  0.0,
             0.5, -0.5,  0.5,
             1.0,  0.0,  0.0,
             0.5,  0.5,  0.5,
             1.0,  0.0,  0.0,
            -0.5, -0.5, -0.5,
             0.0, -1.0,  0.0,
             0.5, -0.5, -0.5,
             0.0, -1.0,  0.0,
             0.5, -0.5,  0.5,
             0.0, -1.0,  0.0,
             0.5, -0.5,  0.5,
             0.0, -1.0,  0.0,
            -0.5, -0.5,  0.5,
             0.0, -1.0,  0.0,
            -0.5, -0.5, -0.5,
             0.0, -1.0,  0.0,
            -0.5,  0.5, -0.5,
             0.0,  1.0,  0.0,
             0.5,  0.5, -0.5,
             0.0,  1.0,  0.0,
             0.5,  0.5,  0.5,
             0.0,  1.0,  0.0,
             0.5,  0.5,  0.5,
             0.0,  1.0,  0.0,
            -0.5,  0.5,  0.5,
             0.0,  1.0,  0.0,
            -0.5,  0.5, -0.5,
             0.0,  1.0,  0.0,
        ]
        .to_vec();

        let vertex_attribute_pointers = vec![
            VertexAttribPointer {
                index: 0,
                size: 3,
                stride: 6 * mem::size_of::<f32>(),
                offset: 0,
            },
            VertexAttribPointer {
                index: 1,
                size: 3,
                stride: 6 * mem::size_of::<f32>(),
                offset: 3 * mem::size_of::<f32>() as usize,
            },
        ];

        DrawableAttributes {
            buffer: Buffer::ArrayBuffer {
                vertex_attributes,
                vertex_attribute_pointers,
            },
            draw_count: 132,
            draw_primitive: DrawPrimitive::TRIANGLES,
        }
    }
}
