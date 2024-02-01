use miniquad::{Backend, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, Pipeline, RenderingBackend, ShaderSource, UniformsSource, VertexAttribute, VertexFormat};
use crate::{shader };
use crate::shapes::shared_c_resources::{Vec2, Vertex};
use crate::shapes::shape::Shape;

pub struct Square;
impl Square {

    pub fn new(context: &mut Box<dyn RenderingBackend>,
               x: f32,
               y: f32,
               width: f32,
               height: f32) -> Shape {

        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -width + x, y: -height + y }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x: width + x, y: -height + y }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x: width + x, y: height + y }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -width + x, y: height + y }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = context.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        // Used to specify the order of the vertex indices
        // Is used to form a square using two triangles.
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
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
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    },
                    Backend::Metal => ShaderSource::Msl {
                        program: shader::METAL,
                    },
                },
                shader::meta(),
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
            6
        )
    }
}