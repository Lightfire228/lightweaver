

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
        self.a = u8::MAX;
    }

    pub fn set_white(&mut self) {
        self.r = u8::MAX;
        self.g = u8::MAX;
        self.b = u8::MAX;
        self.a = u8::MAX;
    }

    pub fn set_white_alpha(&mut self, alpha: f64) {
        self.set_white();

        let alpha = alpha.clamp(0.0, 1.0);
        let alpha = 1.0 - alpha;

        let val = (alpha * u8::MAX as f64) as u8;

        self.a = val;
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