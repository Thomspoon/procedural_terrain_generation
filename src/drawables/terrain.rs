use arrayvec::ArrayVec;

use noise::{NoiseFn, Perlin};

use vek::vec::{Vec3, Vec2};

use std::mem;

use crate::backend::drawable::*;

//
// Grid size used for terrain total size, will make a grid with GRID_SIZE rows and columns
//
const GRID_SIZE: u32 = 500;
const GRID_SIZE_MEM: usize = (GRID_SIZE * GRID_SIZE) as usize;

#[derive(Debug)]
struct Vertex {
    pos: Vec3<f32>,
    normal: Vec3<f32>,
    texture_uv: Vec2<f32>
}

pub struct Terrain;

pub const SEA_LEVEL: f32 = -20.0; 

impl Drawable for Terrain {
    fn vertex_attributes(&self) -> DrawableAttributes {
        let height = RandomHeightGenerator::generate_perlin();

        let mut vertex_attributes: Vec<Vertex> = Vec::with_capacity(GRID_SIZE_MEM);
        for x in 0..GRID_SIZE {
            for z in 0..GRID_SIZE {
                let height = height.inner[x as usize][z as usize];
                //let color = Vec3::new(0.137, 0.47, 0.22);

                // let color = if height > SEA_LEVEL {
                //     Vec3::new(0.137, 0.47, 0.22)
                // } else {
                //     Vec3::new(0.06, 0.28, 0.58)
                // };

                let texture_uv = if x % 2 == 1 && z % 2 == 1 {
                    Vec2::new(0.0, 0.0)
                } else if x % 2 == 1 && z % 2 == 0  {
                    Vec2::new(0.0, 1.0)
                } else if x % 2 == 0 && z % 2 == 1 {
                    Vec2::new(1.0, 0.0)
                } else {
                    Vec2::new(1.0, 1.0)
                };

                vertex_attributes.push(Vertex {
                    pos: Vec3::new(x as _, height as _, z as _),
                    normal: Vec3::new(0.0, -1.0, 0.0),
                    texture_uv
                });
            }
        }

        let mut indices = Vec::with_capacity(GRID_SIZE_MEM * 2 * 3);
        for x in 0..GRID_SIZE - 1 {
            if x % 2 == 0 {
                for z in 0..GRID_SIZE {
                    indices.push((z + x * GRID_SIZE) as u32);
                    indices.push((z + (x + 1) * GRID_SIZE) as u32);
                }
            } else {
                for z in (1..GRID_SIZE).rev() {
                    indices.push((z + (x + 1) * GRID_SIZE) as u32);
                    indices.push((z - 1 + (x * GRID_SIZE)) as u32);
                }
            }
        }

        fn calculate_normal(vertex_attributes: &Vec<Vertex>, indices: ArrayVec<[u32; 18]>, num_triangles: usize) -> Vec3<f32> {
            let mut normal = Vec3::new(0.0, 0.0, 0.0);
            for triangle in 0..num_triangles {
                let vertex_attribute_index = triangle * 3usize;
                let va = vertex_attributes[indices[vertex_attribute_index] as usize].pos;
                let vb = vertex_attributes[(indices[vertex_attribute_index + 1]) as usize].pos;
                let vc = vertex_attributes[(indices[vertex_attribute_index + 2]) as usize].pos;

                normal += Vec3::cross(vb - va, vc - va);
            }

            (normal / num_triangles as f32).normalized()
        };

        // Calculate normals -- refactor into better means
        // NOTE: Normals are calculated by a top-down birds-eye view of the grid
        //       with (0, 0) in the bottom left of the X-Z plane
        let mut va_index = 0;
        for x in 0..GRID_SIZE {
            for z in 0..GRID_SIZE {
                if x == 0 && z == 0 {
                    // Origin
                    let mut triangle_indices = ArrayVec::<[u32; 18]>::new();
                    triangle_indices.push(0);
                    triangle_indices.push(1);
                    triangle_indices.push(GRID_SIZE);
                    vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 1);
                } else if x == GRID_SIZE - 1 && z == 0 {
                    // Top Left
                    let mut triangle_indices = ArrayVec::<[u32; 18]>::new();
                    //      right triangle
                    triangle_indices.push(GRID_SIZE);
                    triangle_indices.push(GRID_SIZE * x + 1);
                    triangle_indices.push(GRID_SIZE * (x - 1) + 1);

                    //      left triangle
                    triangle_indices.push(GRID_SIZE);
                    triangle_indices.push(GRID_SIZE * (x - 1) + 1);
                    triangle_indices.push(GRID_SIZE * (x - 1));

                    vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 2);

                } else if x == GRID_SIZE - 1 && z == GRID_SIZE - 1 {
                    // Top Right
                    let mut triangle_indices = ArrayVec::<[u32; 18]>::new();
                    triangle_indices.push(GRID_SIZE * GRID_SIZE - 1);
                    triangle_indices.push(GRID_SIZE * GRID_SIZE - 2);
                    triangle_indices.push(GRID_SIZE * GRID_SIZE - 1 - GRID_SIZE);

                    vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 1);

                } else if x == 0 && z == GRID_SIZE - 1 {
                    // Bottom Right
                    let mut triangle_indices = ArrayVec::<[u32; 18]>::new();
                    //      right triangle
                    triangle_indices.push(GRID_SIZE - 1);
                    triangle_indices.push(GRID_SIZE - 1 + GRID_SIZE);
                    triangle_indices.push(GRID_SIZE - 1 + GRID_SIZE - 1);

                    //      left triangle
                    triangle_indices.push(GRID_SIZE - 1);
                    triangle_indices.push(GRID_SIZE - 1 + GRID_SIZE);
                    triangle_indices.push(GRID_SIZE - 2);

                    vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 2);

                } else if x == 0 && z > 0 && z < GRID_SIZE - 1 {
                    // Along x == 0 axis when Z is zero and Z is not GRID_SIZE
                    let mut triangle_indices = ArrayVec::<[u32; 18]>::new();
                    //      Left
                    triangle_indices.push(z);
                    triangle_indices.push(z + GRID_SIZE - 1);
                    triangle_indices.push(z - 1);
                    
                    //      Middle
                    triangle_indices.push(z);
                    triangle_indices.push(z + GRID_SIZE);
                    triangle_indices.push(z + GRID_SIZE - 1);
                    //      Right
                    triangle_indices.push(z);
                    triangle_indices.push(z + 1);
                    triangle_indices.push(z + GRID_SIZE);

                    vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 3);
                
                } else if x == GRID_SIZE - 1 && z > 0 && z < GRID_SIZE - 1 {
                    // Along x == GRID_SIZE - 1 axis when Z is GRID_SIZE and Z is not GRID_SIZE
                    let mut triangle_indices = ArrayVec::<[u32; 18]>::new();
                    //      Left
                    triangle_indices.push((x * GRID_SIZE) + z);
                    triangle_indices.push((x * GRID_SIZE) + z - 1);
                    triangle_indices.push((x * GRID_SIZE) + z - GRID_SIZE);
                    
                    //      Middle
                    triangle_indices.push((x * GRID_SIZE) + z);
                    triangle_indices.push((x * GRID_SIZE) + z - GRID_SIZE);
                    triangle_indices.push((x * GRID_SIZE) + z - GRID_SIZE + 1);
                    
                    //      Right
                    triangle_indices.push((x * GRID_SIZE) + z);
                    triangle_indices.push((x * GRID_SIZE) + z - GRID_SIZE + 1);
                    triangle_indices.push((x * GRID_SIZE) + z + 1);
                    vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 3);
                } else if x > 0 && z == 0 && x != GRID_SIZE - 1 {
                    // Along Z axis when X is zero and X is not GRID_SIZE - 1
                    if z % 2 != 0 {
                        // Odd indices have four triangles
                        let mut triangle_indices = ArrayVec::<[u32; 18]>::new();
                        
                        // Triangle 1 (top)
                        triangle_indices.push(x * GRID_SIZE);
                        triangle_indices.push(x * GRID_SIZE + 1);
                        triangle_indices.push(x * GRID_SIZE + GRID_SIZE + 1);
                        
                        // Triangle 2 (Mid upper)
                        triangle_indices.push(x * GRID_SIZE + GRID_SIZE);
                        triangle_indices.push(x * GRID_SIZE + GRID_SIZE + 1);
                        triangle_indices.push(x * GRID_SIZE);
                        
                        // Triangle 3 (Mid lower)
                        triangle_indices.push(x * GRID_SIZE + z + GRID_SIZE - 1);
                        triangle_indices.push(x * GRID_SIZE + z + GRID_SIZE);
                        triangle_indices.push(x * GRID_SIZE + z);
                        
                        // Triangle 4 (bottom)
                        triangle_indices.push(x * GRID_SIZE + z - 1);
                        triangle_indices.push(x * GRID_SIZE + z + GRID_SIZE - 1);
                        triangle_indices.push(x * GRID_SIZE + z);
                        vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 1);

                    } else {
                        // Even indices have two triangles
                        let mut triangle_indices = ArrayVec::<[u32; 18]>::new();
                        
                        // Triangle 1 (top)
                        triangle_indices.push((x * GRID_SIZE) + z);
                        triangle_indices.push((x * GRID_SIZE) + z + 1);
                        triangle_indices.push((x * GRID_SIZE) + z + GRID_SIZE);
                        
                        // Triangle 2 (Mid upper)
                        triangle_indices.push((x * GRID_SIZE) + z);
                        triangle_indices.push((x * GRID_SIZE) + z - 1);
                        triangle_indices.push((x * GRID_SIZE) + z + GRID_SIZE);
                        vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 2);
                    }
                } else if x > 0 && z == GRID_SIZE - 1 && x != GRID_SIZE - 1 {
                    // Along Z == GRID_SIZE - 1 axis when X is zero and X is not GRID_SIZE - 1
                    if x % 2 == 0 {
                        // Odd indices have four triangles
                        let mut triangle_indices = ArrayVec::<[u32; 18]>::new();
                        
                        // Triangle 1 (top)
                        triangle_indices.push(x * (GRID_SIZE - 1));
                        triangle_indices.push(x * (GRID_SIZE - 1) + GRID_SIZE);
                        triangle_indices.push(x * (GRID_SIZE - 1) + GRID_SIZE - 1);
                        
                        // Triangle 2 (Mid upper)
                        triangle_indices.push(x * (GRID_SIZE - 1));
                        triangle_indices.push(x * (GRID_SIZE - 1) + GRID_SIZE - 1);
                        triangle_indices.push(x * (GRID_SIZE - 1) - 1);
                        
                        // Triangle 3 (Mid lower)
                        triangle_indices.push(x * (GRID_SIZE - 1));
                        triangle_indices.push(x * (GRID_SIZE - 1) - 1);
                        triangle_indices.push(x * (GRID_SIZE - 1) - 1 - GRID_SIZE);
                        
                        // Triangle 4 (bottom)
                        triangle_indices.push(x * (GRID_SIZE - 1));
                        triangle_indices.push(x * (GRID_SIZE - 1) - 1 - GRID_SIZE);
                        triangle_indices.push(x * (GRID_SIZE - 1) - GRID_SIZE);
                        vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 1);

                    } else {
                        // Even indices have two triangles
                        let mut triangle_indices = ArrayVec::<[u32; 18]>::new();
                        
                        // Triangle 1 (top)
                        triangle_indices.push(x * (GRID_SIZE - 1));
                        triangle_indices.push(x * (GRID_SIZE - 1) + GRID_SIZE);
                        triangle_indices.push(x * (GRID_SIZE - 1) - 1);
                        
                        // Triangle 2 (Mid upper)
                        triangle_indices.push(x * (GRID_SIZE - 1) + GRID_SIZE);
                        triangle_indices.push(x * (GRID_SIZE - 1) - 1 + GRID_SIZE);
                        triangle_indices.push(x * (GRID_SIZE - 1));
                        vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 2);
                    }
                } else {
                    // Middle of grid
                    if x % 2 != 0 {
                        // Even indices have two triangles
                        let mut triangle_indices = ArrayVec::<[u32; 18]>::new();
                        
                        // Triangle 1 (top left)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z + GRID_SIZE);
                        triangle_indices.push(x * (GRID_SIZE) + z - 1);
                        
                        // Triangle 2 (bottom left)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z - 1);
                        triangle_indices.push(x * (GRID_SIZE) + z - GRID_SIZE);

                        // Triangle 3 (right bottom)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z - GRID_SIZE);
                        triangle_indices.push(x * (GRID_SIZE) + z - GRID_SIZE + 1);

                        // Triangle 4 (right lower middle)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z - GRID_SIZE + 1);
                        triangle_indices.push(x * (GRID_SIZE) + z + 1);

                        // Triangle 5 (right upper middle)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z + 1);
                        triangle_indices.push(x * (GRID_SIZE) + z + GRID_SIZE + 1);

                        // Triangle 6 (right top)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z + GRID_SIZE + 1);
                        triangle_indices.push(x * (GRID_SIZE) + z + GRID_SIZE);

                        vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 6);
                    } else {
                        // Even indices have two triangles
                        let mut triangle_indices = ArrayVec::<[u32; 18]>::new();

                        // Triangle 1 (left top)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z + GRID_SIZE);
                        triangle_indices.push(x * (GRID_SIZE) + z + GRID_SIZE - 1);
                        
                        // Triangle 2 (left upper middle)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z + GRID_SIZE - 1);
                        triangle_indices.push(x * (GRID_SIZE) + z - 1);

                        // Triangle 3 (left lower middle)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z - 1);
                        triangle_indices.push(x * (GRID_SIZE) + z - GRID_SIZE - 1);

                        // Triangle 4 (left bottom)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z - GRID_SIZE - 1);
                        triangle_indices.push(x * (GRID_SIZE) + z - GRID_SIZE);

                        // Triangle 5 (right top)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z - GRID_SIZE);
                        triangle_indices.push(x * (GRID_SIZE) + z + 1);

                        // Triangle 6 (right bottom)
                        triangle_indices.push(x * (GRID_SIZE) + z);
                        triangle_indices.push(x * (GRID_SIZE) + z + 1);
                        triangle_indices.push(x * (GRID_SIZE) + z + GRID_SIZE);

                        vertex_attributes[va_index].normal = calculate_normal(&vertex_attributes, triangle_indices, 6);
                    }
                }

                va_index += 1;
             }
        }

        // Use ArrayVec until [f32; N] gets into_iterator
        let vertex_attributes = vertex_attributes
            .iter()
            .map(|vertex| {
                ArrayVec::from([
                    vertex.pos[0],
                    vertex.pos[1],
                    vertex.pos[2],
                    vertex.normal[0],
                    vertex.normal[1],
                    vertex.normal[2],
                    vertex.texture_uv[0],
                    vertex.texture_uv[1]
                ])
            })
            .flatten()
            .collect::<Vec<_>>();

        let draw_count = indices.len();

        let vertex_attribute_pointers = vec![
            VertexAttribPointer {
                index: 0,
                size: 3,
                stride: 8 * mem::size_of::<f32>(),
                offset: 0,
            },
            VertexAttribPointer {
                index: 1,
                size: 3,
                stride: 8 * mem::size_of::<f32>(),
                offset: 3 * mem::size_of::<f32>() as usize,
            },
            VertexAttribPointer {
                index: 2,
                size: 2,
                stride: 8 * mem::size_of::<f32>(),
                offset: 6 * mem::size_of::<f32>() as usize,
            },
        ];

        DrawableAttributes {
            buffer: Buffer::IndexBuffer {
                vertex_attributes,
                vertex_attribute_pointers,
                indices,
            },
            draw_count,
            draw_primitive: DrawPrimitive::TRIANGLE_STRIP,
        }
    }
}

struct RandomHeightGenerator {
    pub inner: [[f32; GRID_SIZE as _]; GRID_SIZE as _]
}

impl RandomHeightGenerator {
    fn generate_perlin() -> Self {
        let mut s = Self {
            inner: [[0f32; GRID_SIZE as _]; GRID_SIZE as _]
        };

        let perlin = Perlin::new();

        let frequency = 3.0;

        // let mut xoff = 0.0;
        for x in 0..GRID_SIZE {
            // let mut zoff = 0.0;
            for z in 0..GRID_SIZE {
                let nx = (x as f64 / GRID_SIZE as f64) - 0.5;
                let ny = (z as f64 / GRID_SIZE as f64) - 0.5;

                s.inner[x as usize][z as usize] = (perlin.get([frequency * nx, frequency *ny]) 
                                          + 0.5  * perlin.get([frequency * 2.0 * nx, frequency * 2.0 * ny])
                                          + 0.25 * perlin.get([frequency * 4.0 * nx, frequency * 4.0 * ny])) as f32 * 25.0;
            }
        }

        s
    }
}
