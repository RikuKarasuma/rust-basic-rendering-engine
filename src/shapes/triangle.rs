use miniquad::{Backend, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, Pipeline, RenderingBackend, ShaderSource, UniformsSource, VertexAttribute, VertexFormat};
use crate::shapes::shared_c_resources::{Vec2, Vertex};
use crate::shapes::shape::Shape;
use crate::shapes::default_shader::default_shader;

pub struct Triangle;
impl Triangle {

    pub fn new(context: &mut Box<dyn RenderingBackend>,
               x: f32,
               y: f32,
               width: f32,
               height: f32) -> Shape {

        #[rustfmt::skip]
        let vertices: [Vertex; 3] = [
            Vertex { pos : Vec2 { x: -width + x, y: -height + y }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x: width + x, y: -height + y }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x, y: height + y }, uv: Vec2 { x: 1., y: 1. } },
        ];
        let vertex_buffer = context.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        // Used to specify the order of the vertex indices
        // Is used to form a square using two triangles.
        let indices: [u16; 3] = [0, 1, 2];
        let index_buffer = context.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let pixels: [u8; 4 * 4 * 4] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];
        let texture = context.new_texture_from_rgba8(4, 4, &pixels);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = context
            .new_shader(
                match context.info().backend {
                    Backend::OpenGl => ShaderSource::Glsl {
                        vertex: default_shader::VERTEX,
                        fragment: default_shader::FRAGMENT,
                    },
                    Backend::Metal => ShaderSource::Msl {
                        program: default_shader::METAL,
                    },
                },
                default_shader::meta(),
            )
            .unwrap();

        let pipeline = context.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
            ],
            shader,
        );


        Shape::new (
            bindings,
            pipeline,
            3
        )
    }
}

// fn generate_circle_vertices_and_indices(center_x: f32, center_y: f32, radius: f32, segments: u32) -> (Vec<Vertex>, Vec<u16>) {
//     let mut vertices = Vec::with_capacity(segments as usize + 1);
//     let mut indices = Vec::with_capacity(segments as usize * 3);
//
//     // Center vertex
//     vertices.push(Vertex { position: [center_x, center_y] });
//
//     // Circle vertices
//     for i in 0..segments {
//         let theta = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
//         let x = center_x + radius * theta.cos();
//         let y = center_y + radius * theta.sin();
//
//         vertices.push(Vertex { position: [x, y] });
//         indices.push(0);
//         indices.push(i + 1);
//         indices.push((i + 1) % segments);
//     }
//
//     (vertices, indices)
// }