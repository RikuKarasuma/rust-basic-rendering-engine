use miniquad::{ Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, KeyCode, Pipeline, RenderingBackend, ShaderMeta, ShaderSource, UniformBlockLayout, UniformDesc, UniformsSource, UniformType, VertexAttribute, VertexFormat};
use crate::shapes::shared_c_resources::{Vec2, Vertex};
use crate::shapes::shape::{BaseShape, Shape};
use crate::shapes::default_shader::default_shader;

pub struct Triangle {
    base_details: BaseShape,
    uniforms: TriangleModel
}

#[repr(C)]
pub struct TriangleModel {
    red: f32,
    green: f32,
    blue: f32,
    offset: Vec2,
    input: InputModel
}

#[repr(C)]
pub struct InputModel {
    keys_down: Vec<KeyCode>
}

impl TriangleModel {
    pub fn get_red(&self) -> f32 {
        self.red
    }

    pub fn get_green(&self) -> f32 {
        self.green
    }

    pub fn get_blue(&self) -> f32 {
        self.blue
    }
}

fn shader_meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("red", UniformType::Float1),
                UniformDesc::new("green", UniformType::Float1),
                UniformDesc::new("blue", UniformType::Float1),
                UniformDesc::new("offset", UniformType::Float2),
            ],
        },
    }
}

impl Shape for Triangle {

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

        const ACCEL: f32 = 0.009;

        if let Some(key_code) = self.uniforms.input.keys_down.iter().position(|key_down | key_down == &KeyCode::Down) {
            self.uniforms.offset.y -= ACCEL;
        }
        if let Some(key_code) = self.uniforms.input.keys_down.iter().position(|key_down | key_down == &KeyCode::Up) {
            self.uniforms.offset.y += ACCEL;
        }
        if let Some(key_code) = self.uniforms.input.keys_down.iter().position(|key_down | key_down == &KeyCode::Left) {
            self.uniforms.offset.x -= ACCEL;
        }
        if let Some(key_code) = self.uniforms.input.keys_down.iter().position(|key_down | key_down == &KeyCode::Right) {
            self.uniforms.offset.x += ACCEL;
        }

        self.uniforms.red -= 0.1;

        drawing_context.apply_uniforms(UniformsSource::table(&self.uniforms));

        if draw {
            drawing_context.draw(0, self.base_details.get_segments(), 1);
        }
    }

    fn input_down(&mut self, key_code: KeyCode) {
        if let None = self.uniforms.input.keys_down.iter().position(|key_down | key_down == &key_code) {
            self.get_uniform().input.keys_down.push(key_code);
        }
    }

    fn input_up(&mut self, key_code: KeyCode) {
        if let Some(found_index) = self.get_uniform().input.keys_down.iter().position(|key| key == &key_code) {
            self.get_uniform().input.keys_down.remove(found_index);
        }
    }
}

impl Triangle {

    pub fn get_uniform(&mut self) -> &mut TriangleModel {
        &mut self.uniforms
    }

    pub fn new(context: &mut Box<dyn RenderingBackend>,
               x: f32,
               y: f32,
               width: f32,
               height: f32,
               red: f32,
               green: f32,
               blue: f32) -> Triangle {

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


        Triangle {
            base_details:
                BaseShape::new (
                    bindings,
                    pipeline,
                    3
                ),
            uniforms:
                TriangleModel {
                    red,
                    green,
                    blue,
                    offset: Vec2 {
                        x: 0f32,
                        y: 0f32
                    },
                    input: InputModel {
                        keys_down: vec![]
                    }
                }
        }
    }
}