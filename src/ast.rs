use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type NewWithChildren<'a> = (&'a str, HashMap<String, &'a str>, Vec<Rc<Element<'a>>>);

/// AST struct
///
/// `ele_type` the type of the element
///
/// `attributes` the attributes in the element
///
/// `children` the children in the element
#[derive(Debug, Clone)]
pub struct Element<'a> {
    pub ele_type: &'a str,
    pub attributes: RefCell<HashMap<String, &'a str>>,
    pub parent: RefCell<Weak<Element<'a>>>,
    pub children: RefCell<Vec<Rc<Element<'a>>>>,
}

impl<'a> Element<'a> {
    /// new a element without children
    /// ## Example
    ///
    /// ``` rust
    /// use std::collections::HashMap;
    /// use svg_simple_parser::Element;
    ///
    /// Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// ```
    ///
    pub fn new((ele_type, attributes): (&'a str, HashMap<String, &'a str>)) -> Rc<Self> {
        Rc::new(Element {
            ele_type,
            parent: RefCell::new(Weak::new()),
            attributes: RefCell::new(attributes),
            children: RefCell::new(vec![]),
        })
    }

    /// new a element with children
    /// ## Example
    ///
    /// ``` rust
    /// use std::collections::HashMap;
    /// use svg_simple_parser::Element;
    ///
    /// let child = Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// Element::new_width_children(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")]),vec![child]));
    /// ```
    ///
    pub fn new_width_children((ele_type, attributes, children): NewWithChildren<'a>) -> Rc<Self> {
        let parent = Rc::new(Element {
            ele_type,
            attributes: RefCell::new(attributes),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        *parent.children.borrow_mut() = children
            .iter()
            .map(|node| {
                let node = node.clone();
                *node.parent.borrow_mut() = Rc::downgrade(&parent);
                node
            })
            .collect();
        parent
    }

    /// add a element to the children of the element.
    ///
    /// ## Example
    ///
    /// ``` rust
    /// use std::collections::HashMap;
    /// use svg_simple_parser::Element;
    ///
    /// let parent = Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// let child = Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// Element::add_child(parent.clone(),child.clone());
    /// assert_eq!(parent.borrow().children.borrow().get(0),Some(&child));
    /// ```
    ///
    pub fn add_child(self: &Rc<Element<'a>>, new_item: Rc<Element<'a>>) {
        let node = new_item.clone();
        *node.parent.borrow_mut() = Rc::downgrade(self);
        self.children.borrow_mut().push(new_item);
    }

    /// add a list of element to the children of the element.
    ///
    /// ## Example
    ///
    /// ``` rust
    /// use std::collections::HashMap;
    /// use svg_simple_parser::Element;
    ///
    /// let parent = Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// let child = Element::new(("rect",HashMap::from([("width".to_owned(), "100"),("height".to_owned(), "100")])));
    /// Element::add_children(parent.clone(),vec![child.clone()].as_mut());
    /// assert_eq!(parent.borrow().children.borrow().get(0),Some(&child));
    /// ```
    ///
    pub fn add_children(self: &Rc<Element<'a>>, new_items: Vec<Rc<Element<'a>>>) {
        let mut new_items = new_items
            .iter()
            .map(|node| {
                let node = node.clone();
                *node.parent.borrow_mut() = Rc::downgrade(self);
                node
            })
            .collect();
        (self.children.borrow_mut()).append(&mut new_items);
    }
}
