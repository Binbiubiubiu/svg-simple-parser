use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use svg_simple_parser::{parse, stringify, Element};

fn parse_benchmark(c: &mut Criterion) {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1">
    <circle cx="100" cy="50" r="40" stroke="black" stroke-width="2" fill="red"/>
</svg>"#;
    c.bench_function("parse_benchmark", |b| b.iter(|| parse(black_box(svg))));
}

fn strigify_benchmark(c: &mut Criterion) {
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
    c.bench_function("strigify_benchmark", |b| {
        b.iter(|| stringify(black_box(&root)))
    });
}

criterion_group!(benches, parse_benchmark, strigify_benchmark);
criterion_main!(benches);
