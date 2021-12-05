use std::collections::HashMap;
use std::rc::Rc;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till, take_until, take_while},
    character::complete::{alphanumeric1, one_of, space1},
    combinator::{cut, map, opt},
    error::{context, ContextError, ParseError},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::ast::Element;

/// remove whitespace ` \t\r\n`
#[inline(always)]
fn sp<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

/// parse a text wrapped in `"` or `'`
/// ## Example
/// ``` ignore
///  "100" -> "100"
///  '100' -> "100"
/// ```
#[inline(always)]
fn attribute_value<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    let mark = "\"\'";
    delimited(one_of(mark), take_till(|c| mark.contains(c)), one_of(mark))(input)
}

/// parse a text with key-value format`
/// ## Example
/// ``` ignore
/// width = "100" -> ("width","100")
/// width = '100' -> ("width","100")
/// ```
#[inline(always)]
fn attribute<'a, E>(input: &'a str) -> IResult<&'a str, (&'a str, &'a str), E>
where
    E: ParseError<&'a str>,
{
    separated_pair(is_not(" ="), tag("="), attribute_value)(input)
}

/// parse a text what is base on a lot oof key-value's format text`
/// ## Example
/// ``` ignore
/// width = "100" height = "200"
///
/// // ↓↓↓↓↓↓↓↓ transform ↓↓↓↓↓↓↓↓
///
/// use std::collections::HashMap;
///
/// HashMap::from([
///     ("width".to_owned(),"100"),
///     ("height".to_owned(),"200"),
/// ]);
/// ```
#[inline(always)]
pub fn attribute_hash<'a, E>(input: &'a str) -> IResult<&'a str, HashMap<String, &'a str>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    context(
        "attribute_hash",
        preceded(
            sp,
            cut(terminated(
                map(separated_list0(space1, attribute), |tuple_vec| {
                    tuple_vec
                        .into_iter()
                        .map(|(k, v)| (String::from(k), v))
                        .collect()
                }),
                sp,
            )),
        ),
    )(input)
}
/// parse the preix of the element
///
/// ## Example
/// ``` ignore
/// <svg  -> "svg"
/// ```
#[inline(always)]
fn element_start<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    context(
        "element_start",
        preceded(tag("<"), preceded(sp, alphanumeric1)),
    )(input)
}
/// parse a single element
///
/// ## Example
/// ``` ignore
/// <rect width="100"/>
///
/// // ↓↓↓↓↓↓↓↓ transform ↓↓↓↓↓↓↓↓
///
/// use std::collections::HashMap;
/// use std::cell::RefCell;
///
/// Element{
///   ele_type:"rect",
///   attributes:RefCell::new(HashMap::from([
///     ("width","100"),
///   ])),
///   children:vec![],
/// }
/// ```
#[inline(always)]
pub fn single_element<'a, E>(input: &'a str) -> IResult<&'a str, Rc<Element>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    context(
        "single_element",
        map(
            pair(element_start, terminated(attribute_hash, tag("/>"))),
            Element::new,
        ),
    )(input)
}

/// parse a double element
///
/// ## Example
/// ``` ignore
/// <rect width="100">
///     <rect width="100"/>
/// </rect>
///
/// // ↓↓↓↓↓↓↓↓ transform ↓↓↓↓↓↓↓↓
///
/// use std::collections::HashMap;
/// use std::cell::RefCell;
///
/// Element{
///   ele_type:"rect",
///   attributes:RefCell::new(HashMap::from([
///     ("width".to_owned(),"100"),
///   ])),
///   children:vec![
///     Element{
///       ele_type:"rect",
///       attributes:RefCell::new(HashMap::from([
///         ("width".to_owned(),"100"),
///       ])),
///       children:vec![],
///     },
///   ],
/// }
/// ```
#[inline(always)]
pub fn double_element<'a, E>(input: &'a str) -> IResult<&'a str, Rc<Element>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let attributes_pattern = terminated(attribute_hash, tag(">"));
    let children_pattern = terminated(element_list, terminated(take_until(">"), tag(">")));
    context(
        "double_element",
        map(
            tuple((element_start, attributes_pattern, children_pattern)),
            Element::new_width_children,
        ),
    )(input)
}

/// parse a double element or a single element
fn element<'a, E>(input: &'a str) -> IResult<&'a str, Rc<Element>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    context(
        "element",
        delimited(sp, alt((double_element, single_element)), opt(sp)),
    )(input)
}

/// parse a list of the element
fn element_list<'a, E>(input: &'a str) -> IResult<&'a str, Vec<Rc<Element>>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    context("element_list", many0(element))(input)
}
/// transform svg to a Element(AST struct)
///
/// return a result.
/// if transformed successfullly,return a tulp which includes the rest input text and a element;
/// if transformed Error.return `ParseError`
///
///
/// ## Example
/// ```rust
/// use svg_simple_parser::parse;
/// use std::collections::HashMap;
///
/// let svg = r#"
///     <svg xmlns="http://www.w3.org/2000/svg" version="1.1">
///         <circle cx="100" cy="50" r="40" />
///     </svg>
/// "#;
/// let (_, root) = parse(svg).unwrap();
/// assert_eq!(root.ele_type, "svg");
/// assert_eq!(*root.attributes.borrow(), HashMap::from([
///     ("xmlns".to_owned(), "http://www.w3.org/2000/svg"),
///     ("version".to_owned(), "1.1"),
/// ]));
/// let child = &*root.children.borrow()[0];
/// assert_eq!(child.ele_type, "circle");
/// assert_eq!(*child.attributes.borrow(), HashMap::from([
///     ("cx".to_owned(), "100"),
///     ("cy".to_owned(), "50"),
///     ("r".to_owned(), "40"),
/// ]));
/// ```
///
pub fn parse<'a>(input: &'a str) -> IResult<&'a str, Rc<Element>> {
    element(input)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;
    use std::collections::HashMap;

    use crate::parse::{
        attribute, attribute_hash, attribute_value, double_element, element_list, single_element,
    };

    #[test]
    fn test_elements() {
        let (_, v) = element_list::<(&str, ErrorKind)>(
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1"/>"#,
        )
        .unwrap();
        let one = &v[0];
        assert_eq!(one.ele_type, "svg");
        assert_eq!(
            *one.attributes.borrow(),
            HashMap::from([
                ("xmlns".to_owned(), "http://www.w3.org/2000/svg"),
                ("version".to_owned(), "1.1"),
            ])
        );
    }

    #[test]
    fn test_double_element() {
        let (_, root) = double_element::<(&str, ErrorKind)>(
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1"></svg>"#,
        )
        .unwrap();
        assert_eq!(root.ele_type, "svg");
        assert_eq!(
            *root.attributes.borrow(),
            HashMap::from([
                ("xmlns".to_owned(), "http://www.w3.org/2000/svg"),
                ("version".to_owned(), "1.1"),
            ])
        );
    }

    #[test]
    fn test_single_element() {
        let (_, root) = single_element::<(&str, ErrorKind)>(
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" />"#,
        )
        .unwrap();
        assert_eq!(root.ele_type, "svg");
        assert_eq!(
            *root.attributes.borrow(),
            HashMap::from([
                ("xmlns".to_owned(), "http://www.w3.org/2000/svg"),
                ("version".to_owned(), "1.1"),
            ])
        );
    }

    #[test]
    fn test_attribute_hash() {
        assert_eq!(
            attribute_hash::<(&str, ErrorKind)>("a=\"123\" b=\"456\" "),
            Ok((
                "",
                HashMap::from([("a".to_owned(), "123"), ("b".to_owned(), "456")])
            ))
        );
        assert_eq!(
            attribute_hash::<(&str, ErrorKind)>("b=\'123\' c=\'456\' "),
            Ok((
                "",
                HashMap::from([("b".to_owned(), "123"), ("c".to_owned(), "456")])
            ))
        );
    }

    #[test]
    fn test_attribute() {
        assert_eq!(
            attribute::<(&str, ErrorKind)>("a=\"123\""),
            Ok(("", ("a", "123")))
        );
        assert_eq!(
            attribute::<(&str, ErrorKind)>("b=\'123\'"),
            Ok(("", ("b", "123")))
        );
    }

    #[test]
    fn test_attribute_value() {
        assert_eq!(
            attribute_value::<(&str, ErrorKind)>("\'123\'"),
            Ok(("", "123"))
        );
        assert_eq!(attribute_value::<(&str, ErrorKind)>("\'\'"), Ok(("", "")));
        assert_eq!(
            attribute_value::<(&str, ErrorKind)>("\"123\""),
            Ok(("", "123"))
        );
        assert_eq!(attribute_value::<(&str, ErrorKind)>("\"\""), Ok(("", "")));
    }
}
