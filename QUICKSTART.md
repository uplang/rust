# Quick Start Guide - Rust

Get started with the UP Parser for Rust in 5 minutes!

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
up-lang = "1.0"
```

## Your First Program

Create `main.rs`:

```rust
use up_parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    
    let doc = parser.parse(r#"
name Alice
age!int 30
active!bool true
    "#)?;
    
    println!("Name: {}", doc.get_scalar("name").unwrap());
    
    for node in &doc.nodes {
        println!("{} = {:?}", node.key, node.value);
    }
    
    Ok(())
}
```

Run it:

```bash
cargo run
```

## Common Use Cases

### 1. Configuration Files

```rust
use std::fs;
use up_parser::Parser;

let content = fs::read_to_string("config.up")?;
let doc = Parser::new().parse(&content)?;

if let Some(server) = doc.get_block("server") {
    println!("Host: {:?}", server.get("host"));
}
```

### 2. Pattern Matching

```rust
use up_parser::Value;

match &node.value {
    Value::Scalar(s) => println!("Scalar: {}", s),
    Value::Block(b) => println!("Block: {} entries", b.len()),
    Value::List(l) => println!("List: {} items", l.len()),
    Value::Table(t) => println!("Table: {} rows", t.rows.len()),
    Value::Multiline(m) => println!("Multiline: {} chars", m.len()),
}
```

## Next Steps

- Read the [DESIGN.md](DESIGN.md) for implementation details
- Explore the [UP Specification](https://github.com/uplang/spec)
- Check out [docs.rs](https://docs.rs/up-lang) for API docs

## Need Help?

- 📚 [Full Documentation](README.md)
- 💬 [Discussions](https://github.com/uplang/spec/discussions)
- 🐛 [Report Issues](https://github.com/uplang/rust/issues)
