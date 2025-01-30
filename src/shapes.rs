
pub struct Shape {
    location:     Location,
    bounding_box: Size,

}

pub struct Rect {
    location: Location,

}

pub struct Location {
    x: f64,
    y: f64,
    z: f64,
}

// This is a scale factor, relative to other elements
pub struct Size {
    width:  f64,
    height: f64,
}

pub enum SizeErr {
    WidthNegative,
    HeightNegative,
}

pub type SizeResult = Result<Size, SizeErr>;

impl Size {
    pub fn new(width: f64, height: f64) -> SizeResult {
        
        Size::validate(width, height)?;

        Ok(Size {
            width,
            height,
        })
    }

    fn validate(width: f64, height: f64) -> Result<(), SizeErr> {
        if width < 0.0 {
            Err(SizeErr::WidthNegative)
        }
        else if height < 0.0 {
            Err(SizeErr::HeightNegative)
        }
        else {
            Ok(())
        }
    }
    
}
