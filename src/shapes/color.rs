pub struct Color {
    red: f32,
    green: f32,
    blue: f32
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color {
            red,
            green,
            blue
        }
    }

    pub fn get_red(&self) -> f32 {
        self.red
    }

    pub fn get_green(&self) -> f32 {
        self.green
    }

    pub fn get_blue(&self) -> f32 {
        self.blue
    }

    pub fn set_red(&mut self, red: f32) {
        self.red = red
    }

    pub fn set_green(&mut self, green: f32) {
        self.green = green;
    }

    pub fn set_blue(&mut self, blue: f32) {
        self.blue = blue;
    }
}