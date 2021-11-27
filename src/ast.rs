use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type NewWithChildren<'a> = (
    &'a str,
    HashMap<String, &'a str>,
    Vec<Rc<RefCell<Element<'a>>>>,
);

/// AST struct
///
/// `ele_type` the type of the element
///
/// `attributes` the attributes in the element
///
/// `children` the children in the element
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Element<'a> {
    pub ele_type: &'a str,
    pub attributes: Rc<RefCell<HashMap<String, &'a str>>>,
    pub children: RefCell<Vec<Rc<RefCell<Element<'a>>>>>,
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
    pub fn new((ele_type, attributes): (&'a str, HashMap<String, &'a str>)) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Element {
            ele_type,
            attributes: Rc::new(RefCell::new(attributes)),
            children: RefCell::new(vec![]),
        }))
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
    pub fn new_width_children(
        (ele_type, attributes, children): NewWithChildren<'a>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Element {
            ele_type,
            attributes: Rc::new(RefCell::new(attributes)),
            children: RefCell::new(children),
        }))
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
    pub fn add_child(ele: Rc<RefCell<Element<'a>>>, new_item: Rc<RefCell<Element<'a>>>) {
        ele.borrow().children.borrow_mut().push(new_item);
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
    pub fn add_children(
        ele: Rc<RefCell<Element<'a>>>,
        new_items: &mut Vec<Rc<RefCell<Element<'a>>>>,
    ) {
        ele.borrow().children.borrow_mut().append(new_items);
    }
}
