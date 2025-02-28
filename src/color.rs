

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}


impl Color {
    pub fn new() -> Color {
        Color {r: 0, g: 0, b: 0, a: 0}
    }

    pub fn set_black(&mut self) {
        self.r = 0;
        self.g = 0;
        self.b = 0;
        self.a = 255;
    }

    pub fn set_white(&mut self) {
        self.r = 255;
        self.g = 255;
        self.b = 255;
        self.a = 255;
    }

    pub fn set_white_alpha(&mut self, alpha: f64) {


        let alpha = alpha.clamp(0.0, 1.0);
        let alpha = 1.0 - alpha;

        let val = (alpha * 255.0) as u8;

        self.r = 255;
        self.g = 255;
        self.b = 255;
        self.a = val;
    }

    pub fn set_gray(&mut self, brightness: f64) {

        let brightness = brightness.clamp(0.0, 1.0);

        let val = (brightness * 255.0) as u8;
        
        self.r = val;
        self.g = val;
        self.b = val;
        self.a = 255;
    }

    pub fn into_vec(&self) -> Vec<u8> {
        vec![
            self.r,
            self.g,
            self.b,
            self.a,
        ]
    }
}


impl Into<Vec<u8>> for Color {
    fn into(self) -> Vec<u8> {
        self.into_vec()
    }
}