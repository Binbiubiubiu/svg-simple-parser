use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast::Element;

const TAB: &str = "  ";
const LINE: &str = "\r\n";

/// hashmap to string
#[inline(always)]
fn stringify_attrubutes_hash(attribute_hash: &RefCell<HashMap<String, &str>>) -> String {
    let mut arr: Vec<(String, &str)> = attribute_hash
        .borrow_mut()
        .iter()
        .map(|(k, v)| ((*k).clone(), *v))
        .collect();
    arr.sort_by(|(ak, _), (bk, _)| ak.cmp(bk));
    arr.iter()
        .fold("".to_string(), |c, (k, v)| format!("{} {}=\"{}\"", c, k, v))
}

/// element to string
fn trasverse(
    ele: Rc<Element>,
    z: usize,
    (tab_mark, line_mark): (&'static str, &'static str),
) -> String {
    let Element {
        ele_type,
        attributes,
        parent: _,
        children,
    } = &*ele;
    let attrs_str = stringify_attrubutes_hash(attributes);
    let content = if children.borrow().is_empty() {
        format!(
            "{}<{}{}/>{}",
            tab_mark.repeat(z),
            ele_type,
            attrs_str,
            line_mark
        )
    } else {
        let children_str = children
            .borrow()
            .iter()
            .map(|t| trasverse(t.clone(), z + 1, (tab_mark, line_mark)))
            .collect::<Vec<String>>()
            .join("");
        format!(
            "{}<{}{}>{}{}{}</{}>{}",
            tab_mark.repeat(z),
            ele_type,
            attrs_str,
            line_mark,
            children_str,
            tab_mark.repeat(z),
            ele_type,
            line_mark
        )
    };
    content
}

/// transform a Element(AST struct) to svg
///
/// return string.
///
/// ## Example
/// ```rust
/// use svg_simple_parser::{Element,stringify};
/// use std::collections::HashMap;
///
/// let root = Element::new_width_children((
///   "rect",
///   HashMap::from([
///     ("width".to_owned(),"100"),
///   ]),
///   vec![
///     Element::new_width_children((
///       "rect",
///       HashMap::from([
///         ("width".to_owned(),"100"),
///       ]),
///       vec![],
///     )),
///   ],
/// ));
/// let svg = stringify(root);
/// assert_eq!(svg,"<rect width=\"100\"><rect width=\"100\"/></rect>");
/// ```
///
pub fn stringify(ele: Rc<Element>) -> String {
    trasverse(ele, 0, ("", ""))
}

/// transform a Element(AST struct) to svg with pretty format
///
/// return string.
///
/// ## Example
/// ```rust
/// use svg_simple_parser::{Element,stringify_pretty};
/// use std::collections::HashMap;
///
/// let root = Element::new_width_children((
///   "rect",
///   HashMap::from([
///     ("width".to_owned(),"100"),
///   ]),
///   vec![
///     Element::new_width_children((
///       "rect",
///       HashMap::from([
///         ("width".to_owned(),"100"),
///       ]),
///       vec![],
///     )),
///   ],
/// ));
/// let svg = stringify_pretty(root);
/// assert_eq!(svg,"<rect width=\"100\">\r\n  <rect width=\"100\"/>\r\n</rect>\r\n");
/// ```
///
pub fn stringify_pretty(ele: Rc<Element>) -> String {
    trasverse(ele, 0, (TAB, LINE))
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::HashMap};

    use crate::{
        stringify::{stringify_attrubutes_hash, trasverse},
        Element,
    };

    #[test]
    fn test_stringify_attrubutes_hash() {
        let attrs = RefCell::new(HashMap::from([
            ("cx".to_owned(), "100"),
            ("cy".to_owned(), "50"),
            ("r".to_owned(), "40"),
            ("stroke".to_owned(), "black"),
            ("stroke-width".to_owned(), "2"),
            ("fill".to_owned(), "red"),
        ]));
        assert_eq!(
            stringify_attrubutes_hash(&attrs),
            r#" cx="100" cy="50" fill="red" r="40" stroke="black" stroke-width="2""#
        );
    }

    #[test]
    fn test_trasverse() {
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
            trasverse(root.clone(), 0, ("", "")),
            r#"<svg version="1.1" xmlns="http://www.w3.org/2000/svg"><circle cx="100" cy="50" fill="red" r="40" stroke="black" stroke-width="2"/></svg>"#
        );

        assert_eq!(
            trasverse(root.clone(), 0, ("  ", "\r\n")),
            "<svg version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">\r\n  <circle cx=\"100\" cy=\"50\" fill=\"red\" r=\"40\" stroke=\"black\" stroke-width=\"2\"/>\r\n</svg>\r\n"
        );
    }
}
