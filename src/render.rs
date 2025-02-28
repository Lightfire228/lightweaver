use crate::shapes::{Line, Location, Rect};


pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct DataBuff {
    pub width:  usize,
    pub height: usize,

    pub data: Vec<Pixel>,
}

struct PixelLoc {
    pub x: f64,
    pub y: f64,
}

impl PixelLoc {

    /**
       Converts UDC coordinates into pixel space coordinates
     */
    pub fn new(x: f64, y: f64, width: usize, height: usize) -> PixelLoc {

        let half_width  = width  as f64 / 2.0;
        let half_height = height as f64 / 2.0;

        let y = -y;

        PixelLoc {
            x: (x * half_width ) + half_width,
            y: (y * half_height) + half_height,
        }
    }

    /**
       Converts UDC coordinates into pixel space coordinates
     */
    pub fn from_udc(loc: &Location, width: usize, height: usize) -> PixelLoc {
        Self::new(loc.x, loc.y, width, height)
    }
}

impl DataBuff {
    pub fn new(width: usize, height: usize) -> DataBuff {
        let     size = width * height;
        let mut data = Vec::with_capacity(size);

        for _ in 0..size {
            data.push(Pixel::new());
        }

        DataBuff {
            width,
            height,

            data,
        }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn render_rect(&mut self, rect: &Rect) {
        let bounding = rect.bounding_box();

        let top    = Line::new(bounding.top_left.clone(), bounding.top_right());
        let bottom = Line::new(bounding.bottom_left(),    bounding.bottom_right.clone());
        let left   = Line::new(bounding.top_left.clone(), bounding.bottom_left());
        let right  = Line::new(bounding.top_right(),      bounding.bottom_right.clone());

        self.render_line(&top);
        self.render_line(&bottom);
        self.render_line(&left);
        self.render_line(&right);
    }


    pub fn render_line(&mut self, line: &Line) {
        
        let thickness = 2.0;
        let bounding  = line.bounding_box();

        let top_left     = PixelLoc::from_udc(&bounding.top_left,     self.width, self.height);
        let bottom_right = PixelLoc::from_udc(&bounding.bottom_right, self.width, self.height);

        let line_start   = PixelLoc::from_udc(&line.start,            self.width, self.height);
        let line_end     = PixelLoc::from_udc(&line.end,              self.width, self.height);

        let start_y = ((top_left.y     + 0.0      ) as usize).clamp(0, self.height);
        let end_y   = ((bottom_right.y + thickness) as usize).clamp(0, self.height);
        
        let start_x = ((top_left.x     + 0.0      ) as usize).clamp(0, self.width);
        let end_x   = ((bottom_right.x + thickness) as usize).clamp(0, self.width);

        for y in start_y..end_y {

            for x in start_x..end_x {

                let pixel = PixelLoc { x: x as f64, y: y as f64, };
                let dist  = distance(&pixel, &line_start, &line_end);

                if dist > thickness {
                    continue;
                }

                let val = dist / thickness;
                let ind = self.index(x, y);

                self.data[ind].set_white_alpha(val);

            }
        }
    }
}

// https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Line_defined_by_two_points
fn distance(pixel: &PixelLoc, line_start: &PixelLoc, line_end: &PixelLoc) -> f64 {

    let y_diff = line_end.y - line_start.y;
    let x_diff = line_end.x - line_start.x;

    let y_diff_sq = y_diff * y_diff;
    let x_diff_sq = x_diff * x_diff;

    let numerator   = ( y_diff*pixel.x - x_diff*pixel.y + line_end.x*line_start.y - line_end.y*line_start.x).abs();
    let denominator = ( y_diff_sq + x_diff_sq ).powf(0.5);

    numerator / denominator
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel {r: 0, g: 0, b: 0, a: 0}
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
}