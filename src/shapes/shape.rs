use std::f32::consts::PI;
use miniquad::{Bindings, Pipeline, RenderingBackend, UniformsSource};
use crate::shader;

pub struct Shape{
    // Vertex/Index buffers & Texture
    bindings: Bindings,
    pipeline: Pipeline,
    segments: i32
}

impl Shape {

    pub fn new<'a>(bindings: Bindings, pipeline: Pipeline, segments: i32) -> Shape<> {
        Shape {
            bindings,
            pipeline,
            segments
        }
    }

    pub fn draw(&self, drawing_context: &mut Box<dyn RenderingBackend>) {
        drawing_context.apply_pipeline(&self.pipeline);
        drawing_context.apply_bindings(&self.bindings);
        let mut radius = 0.3f64;

        let theta = 2.0 * PI as f64  * radius;

        // drawing_context
        //     .apply_uniforms(UniformsSource::table(&shader::Uniforms {
        //         offset: ( 0., 0.)//( theta.cos() as f32 , theta.sin() as f32  ),
        //     }));
        // println!("Segs: {}", self.segments);
        drawing_context.draw(0, self.segments, 1);

        radius += radius ;
    }
}