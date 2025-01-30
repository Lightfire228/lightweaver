
use svg::node::element::Rectangle as svg_Rect;

pub struct Rect {
    pub location: Location,
    pub size:     Size,
}

impl Rect {
    pub fn new(location: Location, size: Size) -> Rect {
        Rect {
            location,
            size,
        }
    }

    pub fn to_path(&self) -> Vec<(i32, i32)> {
        let w = self.size.width  as i32;
        let h = self.size.height as i32;
        vec![
            (0,  h),
            (w,  0),
            (0, -h),
        ]
    }

    pub fn to_svg_rect(&self) -> svg_Rect {
        svg_Rect::new()
            .set("width",  self.size.width)
            .set("height", self.size.height)
            .set("x",      self.location.x)
            .set("y",      self.location.y)
    }
}

pub struct Location {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Location {
    pub fn to_parameters(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

pub struct Size {
    pub width:  u32,
    pub height: u32,
}


impl Size {
    pub fn new(width: u32, height: u32) -> Size {
        
        Size {
            width,
            height,
        }
    }

}
