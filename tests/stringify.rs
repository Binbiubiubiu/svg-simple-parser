use std::collections::HashMap;

use svg_simple_parser::{stringify, Element};

#[test]
fn test_parse() {
    let root = Element::new_width_children((
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
    assert_eq!(
        stringify(&root),
        r#"<svg version="1.1" xmlns="http://www.w3.org/2000/svg"><circle cx="100" cy="50" fill="red" r="40" stroke="black" stroke-width="2"/></svg>"#
    );
}
