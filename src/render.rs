use svg::{
    node::element::Path,
    node::element::path::Data,
    Document,
};

use crate::shapes::{Location, Rect, Size};

pub fn test() {

    let rect = Rect::new(
        Location {
            x: 10,
            y: 10,
            z: 1,
        },
        Size {
            width:  50,
            height: 50,
        }
    );

    let rect2 = Rect::new(
        Location {
            x: 70,
            y: 10,
            z: 1,
        },
        Size {
            width:  10,
            height: 50,
        }
    );

    let mut data = Data::new()
        .move_to(rect.location.to_parameters())
    ;

    for tuple in rect.to_path() {
        data = data.line_by(tuple);
    }
    data = data.close();


    let path = Path::new()
        .set("fill",         "none")
        .set("stroke",       "white")
        .set("stroke-width", 0.1)
        .set("d",            data)
    ;
    
    let rect_data = rect2.to_svg_rect()
        .set("fill",         "none")
        .set("stroke",       "white")
        .set("stroke-width", 1)
    ;

    let document = Document::new()
        .set("viewbox", (0, 0, 40, 40))
        .add(path)
        .add(rect_data)
    ;

    // TODO: not hardcode out path
    svg::save("out/test.svg", &document).unwrap();


}