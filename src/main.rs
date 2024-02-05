mod shapes;

use std::any::Any;
use std::ops::Deref;
use miniquad::*;
use miniquad::date::now;
use crate::shapes::background::Background;
use crate::shapes::shape::Shape;
use crate::shapes::square::Square;
use crate::shapes::triangle::{Triangle};

struct Stage<> {
    context: Box<dyn RenderingBackend>,
    shapes: Vec<Box<dyn Shape>>,
    time_since_last_frame: f64,
    fps: u8
}

impl<'b> Stage<> {

    pub fn new<>() -> Stage<> {
        let mut context: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let background = Box::new(Background::new(&mut context, 1.000, 0.937, 0.835));
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
            time_since_last_frame: 0f64,
            fps: 60u8
        }
    }

    fn get_frame_time(&self) -> f64 {
        now() - self.time_since_last_frame
    }

    fn get_minimum_frame_time(&self) -> f64 {
        1. / self.fps as f64
    }

    fn is_frame_time_right(&self) -> bool {
        let frame_time = self.get_frame_time();
        frame_time > self.get_minimum_frame_time()
    }

    fn sleep_until_next_frame(&self) {
        let time_to_sleep = (self.get_minimum_frame_time() - self.get_frame_time()) * 1000.;
        // Reportedly problems on web assembly.
        // Need another impl for handling webpage frame limiting.
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
    }
}

impl EventHandler for Stage<> {
    fn update(&mut self) {
        if self.is_frame_time_right() {

        } else {
            self.sleep_until_next_frame();
        }
    }

    fn draw(&mut self) {
        if self.is_frame_time_right() {
            // Begin opengl pass.
            self.context.begin_default_pass(Default::default());

            // Draw each opengl object.
            // Each has its own impl.
            &self.shapes.iter_mut().for_each(|mut a| a.draw(&mut self.context, true));

            // End opengl pass.
            self.context.end_render_pass();
            self.context.commit_frame();
            self.time_since_last_frame = now();
            // println!("Frame drew!!");
        } else {
            self.sleep_until_next_frame();
        }
    }

    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        println!("Key down {:?}", _keycode);


        self.shapes.iter_mut().for_each(|a| {
            a.input_down(_keycode);
        })
    }

    fn key_up_event(&mut self, _keycode: KeyCode, _keymods: KeyMods) {
        println!("Key up {:?}", _keycode);

        self.shapes.iter_mut().for_each(|a| {
            a.input_up(_keycode);
        })
    }
}

fn main() {
    let mut conf = conf::Conf::default();
    conf.platform.apple_gfx_api = conf::AppleGfxApi::OpenGl;
    start(conf, move || Box::new(Stage::new()));
}

