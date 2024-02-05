use miniquad::{Bindings, KeyCode, Pipeline, RenderingBackend};

pub trait Shape {

    fn get_bindings(&self) -> &Bindings;
    fn get_pipeline(&self) -> &Pipeline;
    fn get_segments(&self) -> i32;
    fn set_binding(&mut self, bindings: Bindings);
    fn set_pipeline(&mut self, pipeline: Pipeline);
    fn set_segments(&mut self, segments: i32);
    fn draw(&mut self, drawing_context: &mut Box<dyn RenderingBackend>, draw: bool);
    fn input_down(&mut self, key_code: KeyCode);
    fn input_up(&mut self, key_code: KeyCode);
}

pub struct BaseShape {
    // Vertex/Index buffers & Texture
    bindings: Bindings,
    pipeline: Pipeline,
    segments: i32,
}

impl Shape for BaseShape {

    fn get_bindings(&self) -> &Bindings {
        &self.bindings
    }

    fn get_pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    fn get_segments(&self) -> i32 {
        self.segments
    }

    fn set_binding(&mut self, bindings: Bindings) {
        self.bindings = bindings;
    }

    fn set_pipeline(&mut self, pipeline: Pipeline) {
        self.pipeline = pipeline;
    }

    fn set_segments(&mut self, segments: i32) {
        self.segments = segments;
    }

    fn draw(&mut self, drawing_context: &mut Box<dyn RenderingBackend>, draw: bool) {
        drawing_context.apply_pipeline(&self.pipeline);
        drawing_context.apply_bindings(&self.bindings);

        if draw {
            drawing_context.draw(0, self.segments, 1);
        }
    }

    fn input_down(&mut self, key_code: KeyCode) {} // NOT IMPLEMENTED
    fn input_up(&mut self, key_code: KeyCode) {} // NOT IMPLEMENTED
}

impl BaseShape {

    pub fn new<'a>(bindings: Bindings, pipeline: Pipeline, segments: i32) -> BaseShape {
        BaseShape {
            bindings,
            pipeline,
            segments
        }
    }
}