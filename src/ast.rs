use std::collections::HashMap;

/// AST struct
///
/// `ele_type` the type of the element
///
/// `attributes` the attributes in the element
///
/// `children` the children in the element
#[derive(Debug, PartialEq, Eq)]
pub struct Element<'a> {
    pub ele_type: &'a str,
    pub attributes: HashMap<String, &'a str>,
    pub children: Vec<Element<'a>>,
}

impl<'a> Element<'a> {
    /// new a element without children
    /// ## Example
    ///
    /// ``` rust
    /// use std::collections::HashMap;
    ///
    /// Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// ```
    ///
    pub fn new((ele_type, attributes): (&'a str, HashMap<String, &'a str>)) -> Self {
        Element {
            ele_type,
            attributes,
            children: vec![],
        }
    }

    /// new a element with children
    /// ## Example
    ///
    /// ``` rust
    /// use std::collections::HashMap;
    /// let child = Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// Element::new_width_children(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")]),vec![child]));
    /// ```
    ///
    pub fn new_width_children(
        (ele_type, attributes, children): (&'a str, HashMap<String, &'a str>, Vec<Element<'a>>),
    ) -> Self {
        Element {
            ele_type,
            attributes,
            children,
        }
    }

    /// add a element to the children of the element.
    ///
    /// ## Example
    ///
    /// ``` rust
    /// use std::collections::HashMap;
    /// let parent = Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// let child = Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// parent.add(child)
    /// ```
    ///
    pub fn add_child(&mut self, new_item: Element<'a>) {
        self.children.push(new_item);
    }

    /// add a list of element to the children of the element.
    ///
    /// ## Example
    ///
    /// ``` rust
    /// use std::collections::HashMap;
    /// let parent = Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// let child = Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// parent.add_children(vec![child])
    /// ```
    ///
    pub fn add_children(&mut self, new_items: &mut Vec<Element<'a>>) {
        self.children.append(new_items);
    }
}
