# svg-simple-parser

a simple parser for svg

## Installation

```
[dependencies]
svg-simple-parser = "0.0.1"
```

## Usage

``` rust
use svg_simple_parser::{parse,stringify};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" version="1.1"><script xmlns=""/>

    <path d="M153 334 C153 334 151 334 151 334 C151 339 153 344 156 344 C164 344 171 339 171 334 C171 322 164 314 156 314 C142 314 131 322 131 334 C131 350 142 364 156 364 C175 364 191 350 191 334 C191 311 175 294 156 294 C131 294 111 311 111 334 C111 361 131 384 156 384 C186 384 211 361 211 334 C211 300 186 274 156 274" style="fill:white;stroke:red;stroke-width:2"/>
    
    </svg>"#;
    let (_, root) = parse(svg).unwrap();
    println!("parse result: {:#?}", root.clone());

    println!("stringify result: {:#?}", stringify(root));
    Ok(())
}
```

## Thanks

[nom](https://crates.io/crates/nom)

