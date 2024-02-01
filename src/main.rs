mod shapes;

use miniquad::*;
use crate::shapes::background::Background;
use crate::shapes::shape::Shape;
use crate::shapes::square::Square;
use crate::shapes::triangle::Triangle;

struct Stage<> {
    // ctx: Box<dyn RenderingBackend>,
    //
    // pipeline: Pipeline,
    // bindings: Bindings,
    context: Box<dyn RenderingBackend>,
    shapes: Vec<Shape>
}

impl<'b> Stage<> {

    pub fn new<>() -> Stage<> {
        let mut context: Box<dyn RenderingBackend> = window::new_rendering_backend();

        Stage {
            // Order is important for Z drawing.
            shapes: vec![
                Background::new(&mut context),
                Square::new(&mut context, 0.25f32, 0.0f32, 0.05, 0.05),
                Triangle::new(&mut context, -0.25f32, -0.2f32, 0.05, 0.05),
            ],
            context,
        }
    }
}

impl EventHandler for Stage<> {
    fn update(&mut self) {}

    fn draw(&mut self) {
        let t = date::now();

        self.context.begin_default_pass(Default::default());

        // self.context.apply_pipeline(&self.pipeline);
        // self.context.apply_bindings(&self.bindings);
        // let mut radius = 0.3f64;
        // for i in 0..1 {
        //     let t = t + i as f64 * 0.3;
        //
        //     let theta = 2.0 * PI as f64  * radius;
        //
        //     self.context
        //         .apply_uniforms(UniformsSource::table(&shader::Uniforms {
        //             offset: ( 0., 0.)//( theta.cos() as f32 , theta.sin() as f32  ),
        //         }));
        //     self.context.draw(0, 6, 1);
        //
        //     radius += radius ;
        // }
        println!("Number of shapes: {}", self.shapes.len());
        for shape in &self.shapes {
            shape.draw(&mut self.context);
        }
        self.context.end_render_pass();

        self.context.commit_frame();
    }
}

fn main() {
    let mut conf = conf::Conf::default();
    let metal = std::env::args().nth(1).as_deref() == Some("metal");
    conf.platform.apple_gfx_api = if metal {
        conf::AppleGfxApi::Metal
    } else {
        conf::AppleGfxApi::OpenGl
    };

    miniquad::start(conf, move || Box::new(Stage::new()));
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;

    uniform vec2 offset;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(in_pos + offset, 0, 1);
        texcoord = in_uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub const METAL: &str = r#"
    #include <metal_stdlib>

    using namespace metal;

    struct Uniforms
    {
        float2 offset;
    };

    struct Vertex
    {
        float2 in_pos   [[attribute(0)]];
        float2 in_uv    [[attribute(1)]];
    };

    struct RasterizerData
    {
        float4 position [[position]];
        float2 uv       [[user(locn0)]];
    };

    vertex RasterizerData vertexShader(
      Vertex v [[stage_in]],
      constant Uniforms& uniforms [[buffer(0)]])
    {
        RasterizerData out;

        out.position = float4(v.in_pos.xy + uniforms.offset, 0.0, 1.0);
        out.uv = v.in_uv;

        return out;
    }

    fragment float4 fragmentShader(RasterizerData in [[stage_in]], texture2d<float> tex [[texture(0)]], sampler texSmplr [[sampler(0)]])
    {
        return tex.sample(texSmplr, in.uv);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
    }
}