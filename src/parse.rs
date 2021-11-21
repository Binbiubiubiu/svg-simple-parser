use std::collections::HashMap;

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

/// remove whitespace
/// ## Example
/// ` \t\r\n`
fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

/// parse a text wrapped in `"` or `'`
/// ## Example
/// ```
///  "100" -> "100"
///  '100' -> "100"
/// ```
fn attribute_value<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let mark = "\"\'";
    delimited(one_of(mark), take_till(|c| mark.contains(c)), one_of(mark))(input)
}

/// parse a text with key-value format`
/// ## Example
/// ```
/// width = "100" -> ("width","100")
/// width = '100' -> ("width","100")
/// ```
fn attribute<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (&'a str, &'a str), E> {
    separated_pair(is_not(" ="), tag("="), attribute_value)(input)
}

/// parse a text what is base on a lot oof key-value's format text`
/// ## Example
/// ```
/// width = "100" height = "200"
///
/// // ↓↓↓↓↓↓↓↓ transform ↓↓↓↓↓↓↓↓
///
/// std::collections::HashMap::from([
///     ("width".to_owned(),"100"),
///     ("height".to_owned(),"200"),
/// ]);
/// ```
pub fn attribute_hash<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, HashMap<String, &'a str>, E> {
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
/// ```
/// <svg  -> "svg"
/// ```
fn element_start<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "element_start",
        preceded(tag("<"), preceded(sp, alphanumeric1)),
    )(input)
}
/// parse a single element
///
/// ## Example
/// ```
/// <rect width="100"/>
///
/// // ↓↓↓↓↓↓↓↓ transform ↓↓↓↓↓↓↓↓
///
/// Element{
///   ele_type:"rect",
///   attributes:std::collections::HashMap::from([
///     ("width","100"),
///   ]),
///   children:vec![],
/// }
/// ```
pub fn single_element<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Element, E> {
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
/// ```
/// <rect width="100">
///     <rect width="100"/>
/// </rect>
///
/// // ↓↓↓↓↓↓↓↓ transform ↓↓↓↓↓↓↓↓
///
/// Element{
///   ele_type:"rect",
///   attributes:std::collections::HashMap::from([
///     ("width".to_owned(),"100"),
///   ]),
///   children:vec![
///     Element{
///       ele_type:"rect",
///       attributes:std::collections::HashMap::from([
///         ("width".to_owned(),"100"),
///       ]),
///       children:vec![],
///     },
///   ],
/// }
/// ```
pub fn double_element<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Element, E> {
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
fn element<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Element, E> {
    context(
        "element",
        delimited(sp, alt((double_element, single_element)), opt(sp)),
    )(input)
}

/// parse a list of the element
fn element_list<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<Element>, E> {
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
/// let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" version="1.1"><script xmlns=""/>
///
/// <path d="M153 334 C153 334 151 334 151 334 C151 339 153 344 156 344 C164 344 171 339 171 334 C171 322 164 314 156 314 C142 314 131 322 131 334 C131 350 142 364 156 364 C175 364 191 350 191 334 C191 311 175 294 156 294 C131 294 111 311 111 334 C111 361 131 384 156 384 C186 384 211 361 211 334 C211 300 186 274 156 274" style="fill:white;stroke:red;stroke-width:2"/>
///     
/// </svg>"#;
/// let (_, root) = parse(svg).unwrap();
/// println!("{:#?}", root);
/// ```
///
pub fn parse<'a>(input: &'a str) -> IResult<&'a str, Element> {
    element(input)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;
    use std::collections::HashMap;

    use crate::ast::Element;

    use crate::parse::{
        attribute, attribute_hash, attribute_value, double_element, element_list, single_element,
    };

    #[test]
    fn test_elements() {
        let root = vec![
            Element::new((
                "svg",
                HashMap::from([
                    ("xmlns".to_owned(), "http://www.w3.org/2000/svg"),
                    ("version".to_owned(), "1.1"),
                ]),
            )),
            Element::new((
                "svg",
                HashMap::from([
                    ("xmlns".to_owned(), "http://www.w3.org/2000/svg"),
                    ("version".to_owned(), "1.1"),
                ]),
            )),
        ];
        assert_eq!(
            element_list::<(&str, ErrorKind)>(
                r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1"/>
                <svg xmlns="http://www.w3.org/2000/svg" version="1.1"/>"#
            ),
            Ok(("", root))
        );
    }

    #[test]
    fn test_double_element() {
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
            double_element::<(&str, ErrorKind)>(
                r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1">
            <circle cx="100" cy="50" r="40" stroke="black" stroke-width="2" fill="red"/>
        </svg>"#
            ),
            Ok(("", root))
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(
            single_element::<(&str, ErrorKind)>(r#"<rect width="123" height="456" />"#),
            Ok((
                "",
                Element::new((
                    "rect",
                    HashMap::from([("width".to_owned(), "123"), ("height".to_owned(), "456")])
                ))
            ))
        );
        assert_eq!(
            single_element::<(&str, ErrorKind)>(r#"<rect width=" 1 2 3 " height=" 4 5 6 " />"#),
            Ok((
                "",
                Element::new((
                    "rect",
                    HashMap::from([
                        ("width".to_owned(), " 1 2 3 "),
                        ("height".to_owned(), " 4 5 6 ")
                    ])
                ))
            ))
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
