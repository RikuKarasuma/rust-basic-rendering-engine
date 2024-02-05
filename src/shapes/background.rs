use miniquad::{Bindings, KeyCode, Pipeline, RenderingBackend};
use crate::shapes::color::Color;
use crate::shapes::shape::{ Shape };
use crate::shapes::square::{ Square};

pub struct Background {
    base_details: Square,
    uniforms: Color
}

impl Shape for Background {

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
        self.base_details.draw(drawing_context, true);
    }

    fn input_down(&mut self, key_code: KeyCode) {}

    fn input_up(&mut self, key_code: KeyCode) {}
}
impl Background {

    pub fn new(context: &mut Box<dyn RenderingBackend>,
               red: f32,
               green: f32,
               blue: f32) -> Background {

        Background {
            base_details:
                // Static for now(800x600), needs to be based on the window
                // configuration i.e window width + height.
                // Realistically need a camera object for advanced viewing, which
                // this background would stick to when active.
                Square::new(
                    context,
                    -0.95f32,
                    0.95f32,
                    2.,
                    2.,
                    red,
                    green,
                    blue
                ),
            uniforms:
                Color::new(
                    red,
                    green,
                    blue,
                )
        }
    }
}