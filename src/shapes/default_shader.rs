pub mod default_shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
        attribute vec2 in_pos;
        attribute vec2 in_uv;

        uniform vec2 offset;

        varying lowp vec2 texcoord;

        void main() {
            gl_Position = vec4(in_pos + offset, 0, 1);
            texcoord = in_uv;
        }
    "#;

    pub const FRAGMENT: &str = r#"#version 100
        varying lowp vec2 texcoord;

        uniform sampler2D tex;

        void main() {
            gl_FragColor = texture2D(tex, texcoord);
        }
    "#;

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
        }
    "#;

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