mod shapes;

use miniquad::*;
use crate::shapes::background::Background;
use crate::shapes::shape::Shape;
use crate::shapes::square::Square;
use crate::shapes::triangle::Triangle;

struct Stage<> {
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
                Square::new(&mut context, 0.45f32, 0.0f32, 0.05, 0.05),
                Square::new(&mut context, -0.75f32, 0.0f32, 0.05, 0.05),
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

        // println!("Number of shapes: {}", self.shapes.len());
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

