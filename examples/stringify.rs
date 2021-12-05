use std::collections::HashMap;

use svg_simple_parser::{stringify_pretty, Element};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let svg = Element::new_width_children((
        "svg",
        HashMap::from([
            ("xmlns".to_owned(), "http://www.w3.org/2000/svg"),
            ("version".to_owned(), "1.1"),
        ]),
        vec![Element::new((
            "circle",
            HashMap::from([
                ("cx".to_owned(), "100"),
                ("cy".to_owned(), "50"),
                ("r".to_owned(), "40"),
                ("stroke".to_owned(), "black"),
                ("stroke-width".to_owned(), "2"),
                ("fill".to_owned(), "red"),
            ]),
        ))],
    ));
    let child = Element::new((
        "circle",
        HashMap::from([
            ("cx".to_owned(), "100"),
            ("cy".to_owned(), "50"),
            ("r".to_owned(), "40"),
            ("stroke".to_owned(), "black"),
            ("stroke-width".to_owned(), "2"),
            ("fill".to_owned(), "red"),
        ]),
    ));
    svg.add_children(vec![child.clone()]);

    println!("{:#?}", stringify_pretty(svg));
    Ok(())
}
