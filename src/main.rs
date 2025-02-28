use render::DataBuff;
use shapes::{BoundingBox, Line, Location, Rect};
use std::{fs::File, path::Path, io::BufWriter};

mod shapes;
mod render;


pub fn main() {
    
    let width  = 1000;
    let height = 1000;

    let square = Rect::from(
        &BoundingBox {
            top_left:     Location::new(-0.75,  0.75),
            bottom_right: Location::new(-0.25, -0.75),
        }
    );

    let line = Line::new(
        Location::new(0.25, -0.75),
        Location::new(0.75,  0.75),
    );

    let path = Path::new("./out/test.png");
    let file = File::create(path).unwrap();

    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);

    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth ::Eight);
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    encoder.set_source_chromaticities(png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    ));
    
    let mut writer = encoder.write_header().unwrap();

    let mut buff = DataBuff::new(width as usize, height as usize);

    buff.render_rect(&square);
    buff.render_line(&line);

    // TODO: not copy the thing?
    let data = buff.data.iter().flat_map(|p| vec![p.r, p.g, p.b, p.a]).collect::<Vec<u8>>();

    writer.write_image_data(&data).unwrap();


}
