use render::DataBuff;
use shape_tree::{Direction, ShapeTreeNode};
use shapes::{BoundingBox, Line, Location, Rect, ShapeType};
use std::{fs::File, path::Path, io::BufWriter};

mod shapes;
mod render;
mod color;
mod shape_tree;

// TODO: make two rects, and join them with a connector
// then render that

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

    let mut buff = DataBuff::new(width as usize, height as usize);

    buff.render_rect(&square);
    buff.render_line(&line);

    write_png(&buff);

    let mut root = ShapeTreeNode::new(Direction::HORT);

    root.add(ShapeType::Rect(square))

}


fn write_png(buff: &DataBuff) {

    let path = Path::new("./out/test.png");
    let file = File::create(path).unwrap();

    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, buff.width as u32, buff.height as u32);

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

    let data: Vec<u8> = buff.data.iter().flat_map(|x| x.into_vec()).collect();

    writer.write_image_data(&data).unwrap();


}