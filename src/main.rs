mod shapes;

use std::any::Any;
use std::ops::Deref;
use miniquad::*;
use crate::shapes::shape::Shape;
use crate::shapes::square::Square;
use crate::shapes::triangle::{Triangle};

struct Stage<> {
    context: Box<dyn RenderingBackend>,
    shapes: Vec<Box<dyn Shape>>
}

impl<'b> Stage<> {

    pub fn new<>() -> Stage<> {
        let mut context: Box<dyn RenderingBackend> = window::new_rendering_backend();


        let background = Box::new(Square::new(&mut context, -0.95f32, 0.95f32, 2., 2., 1.000, 0.937, 0.835));
        let green_triangle = Box::new(Triangle::new(&mut context, -0.25f32, -0.2f32, 0.05, 0.05, 0.0, 1f32, 0.0));
        let green_square = Box::new(Square::new(&mut context, -0.4f32, -0.2f32, 0.05, 0.05, 0.0, 1f32, 0.0));


        Stage {
            // Order is important for Z drawing.
            shapes: vec![
                background,
                green_triangle,
                green_square,
            ],
            context,
        }
    }
}

impl EventHandler for Stage<> {
    fn update(&mut self) {}

    fn draw(&mut self) {
        // Begin opengl pass.
        self.context.begin_default_pass(Default::default());

        // Draw each opengl object.
        // Each has its own impl.
        &self.shapes.iter_mut().for_each(|mut a| a.draw(&mut self.context, true));

        // End opengl pass.
        self.context.end_render_pass();
        self.context.commit_frame();
    }

}

fn main() {
    let mut conf = conf::Conf::default();
    conf.platform.apple_gfx_api = conf::AppleGfxApi::OpenGl;
    start(conf, move || Box::new(Stage::new()));
}

