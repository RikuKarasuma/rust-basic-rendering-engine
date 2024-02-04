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
        uniform highp float red;
        uniform highp float green;
        uniform highp float blue;

        void main() {
            // gl_FragColor = texture2D(tex, texcoord);
            gl_FragColor = vec4( red, green, blue, 0.0);
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