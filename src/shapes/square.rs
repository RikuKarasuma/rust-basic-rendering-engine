use miniquad::{
    Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, KeyCode,
    Pipeline, RenderingBackend, ShaderMeta, ShaderSource, UniformBlockLayout,
    UniformDesc, UniformsSource, UniformType, VertexAttribute, VertexFormat
};
use crate::shapes::color::Color;
use crate::shapes::shared_c_resources::{Vec2, Vertex};
use crate::shapes::shape::{BaseShape, Shape};
use crate::shapes::default_shader::default_shader;


pub struct Square {
    base_details: BaseShape,
    uniforms: Color
}

pub fn shader_meta() -> ShaderMeta {
    ShaderMeta {
        images: vec![],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("red", UniformType::Float1),
                UniformDesc::new("green", UniformType::Float1),
                UniformDesc::new("blue", UniformType::Float1),
            ],
        },
    }
}

impl Shape for Square {

    fn get_bindings(&self) -> &Bindings {
        &self.base_details.get_bindings()
    }

    fn get_pipeline(&self) -> &Pipeline {
        &self.base_details.get_pipeline()
    }

    fn get_segments(&self) -> i32 {
        self.base_details.get_segments()
    }

    fn set_binding(&mut self, bindings: Bindings) {
        self.base_details.set_binding(bindings);
    }

    fn set_pipeline(&mut self, pipeline: Pipeline) {
        self.base_details.set_pipeline(pipeline);
    }

    fn set_segments(&mut self, segments: i32) {
        self.base_details.set_segments(segments);
    }

    fn draw(&mut self, drawing_context: &mut Box<dyn RenderingBackend>, draw: bool) {
        self.base_details.draw(drawing_context, false);

        drawing_context.apply_uniforms(UniformsSource::table(&self.uniforms));

        if draw {
            drawing_context.draw(0, self.base_details.get_segments(), 1);
        }
    }

    fn input_down(&mut self, key_code: KeyCode) {}
    fn input_up(&mut self, key_code: KeyCode) {}
}
impl Square {

    pub fn new(context: &mut Box<dyn RenderingBackend>,
               x: f32,
               y: f32,
               width: f32,
               height: f32,
               red: f32,
               green: f32,
               blue: f32) -> Square {

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
                ShaderSource::Glsl {
                    vertex: default_shader::VERTEX,
                    fragment: default_shader::FRAGMENT,
                },
                shader_meta(),
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


        Square {
            base_details:
            BaseShape::new (
                bindings,
                pipeline,
                6
            ),
            uniforms:
                Color::new (
                    red,
                    green,
                    blue,
                )
        }
    }
}