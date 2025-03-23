use render::DataBuff;
use script::{runner, scanner::{Scanner, ScannerError}};
use shape_tree::ShapeTree;
use shapes::{BoundingBox, Line, Location, Rect, ShapeType};
use std::{fs::{self, File}, io::BufWriter, path::Path};

mod shapes;
mod render;
mod color;
mod shape_tree;
mod script;
mod graph;
mod macros;


pub fn main() {
    
    test_png();
    // test_shape_tree();
    test_script();

    
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

fn test_png() {
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
}

fn test_shape_tree() {

    let square = Rect::from(
        &BoundingBox {
            top_left:     Location::new(-0.75,  0.75),
            bottom_right: Location::new(-0.25, -0.75),
        }
    );

    let mut shape_tree = ShapeTree::new();

    shape_tree.add_shape(ShapeType::Rect(square));
}

fn test_script() {
    let path = Path::new("./test_scripts/test.lw");

    runner::run_file(path.as_ref());
}
