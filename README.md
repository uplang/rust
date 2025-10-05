# UP Parser for Rust

[![Crates.io](https://img.shields.io/crates/v/up-lang.svg)](https://crates.io/crates/up-lang)
[![Documentation](https://docs.rs/up-lang/badge.svg)](https://docs.rs/up-lang)
[![CI](https://github.com/uplang/rust/workflows/CI/badge.svg)](https://github.com/uplang/rust/actions)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

Official Rust implementation of the UP (Unified Properties) language parser.

📚 **[API Documentation](https://docs.rs/up-lang)** | 🧪 **[Test Status](https://github.com/uplang/rust/actions)** | 📖 **[Specification](https://github.com/uplang/spec)**

> **Zero-Cost Abstractions** - Memory safe, blazingly fast

## Features

- ✅ **Full UP Syntax Support** - Scalars, blocks, lists, tables, multiline strings
- ✅ **Type Annotations** - Parse and preserve type hints (`!int`, `!bool`, etc.)
- ✅ **Memory Safe** - Rust's ownership system prevents bugs
- ✅ **Zero-Cost** - No runtime overhead
- ✅ **Well-Tested** - Comprehensive test suite
- ✅ **Zero Dependencies** - Pure Rust implementation
- ✅ **CLI Tool** - Command-line utility included

## Requirements

- Rust 1.70 or later

## Installation

```toml
# Cargo.toml
[dependencies]
up-lang = "1.0"
```

Or via cargo:

```bash
cargo add up-lang
```

## Quick Start

```rust
use up_parser::Parser;

fn main() {
    let parser = Parser::new();

    let doc = parser.parse(r#"
name Alice
age!int 30
config {
  debug!bool true
}
    "#).unwrap();

    // Access values
    if let Some(name) = doc.get_scalar("name") {
        println!("Name: {}", name);
    }

    // Iterate nodes
    for node in &doc.nodes {
        println!("{} = {:?}", node.key, node.value);
    }
}
```

**📖 For detailed examples and tutorials, see [QUICKSTART.md](QUICKSTART.md)**

## Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - Getting started guide with examples
- **[DESIGN.md](DESIGN.md)** - Architecture and design decisions
- **[UP Specification](https://github.com/uplang/spec)** - Complete language specification

## API Overview

### Core Types

- **`Parser`** - Main parser for converting UP text into documents
- **`Document`** - Parsed document with convenient access methods
- **`Node`** - Key-value pair with optional type annotation
- **`Value`** - Enum for all value types (scalar, block, list, table)

### Basic Usage

```rust
use up_parser::{Parser, Value};

let parser = Parser::new();
let doc = parser.parse(content)?;

// Access values
let name = doc.get_scalar("name");
let server = doc.get_block("server");
let tags = doc.get_list("tags");

// Pattern matching on Value
match &node.value {
    Value::Scalar(s) => println!("Scalar: {}", s),
    Value::Block(b) => println!("Block with {} entries", b.len()),
    Value::List(l) => println!("List with {} items", l.len()),
    _ => {}
}
```

**See [DESIGN.md](DESIGN.md) for complete API documentation and implementation details.**

## CLI Tool

```bash
# Parse and display
up-parse config.up

# Validate syntax
up-validate config.up

# Convert to JSON
up-convert config.up --format json
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## Project Structure

```
rust/
├── src/
│   ├── lib.rs           # Library root
│   ├── parser.rs        # Main parser implementation
│   ├── types.rs         # Data structures
│   └── cli.rs           # CLI tool
├── tests/
│   └── integration.rs   # Integration tests
├── Cargo.toml
├── README.md            # This file
├── QUICKSTART.md        # Getting started guide
├── DESIGN.md            # Architecture documentation
└── LICENSE              # GNU GPLv3
```

## Contributing

Contributions are welcome! Please see the main [CONTRIBUTING.md](https://github.com/uplang/spec/blob/main/CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Links

- **[UP Language Specification](https://github.com/uplang/spec)** - Official language spec
- **[Syntax Reference](https://github.com/uplang/spec/blob/main/SYNTAX-REFERENCE.md)** - Quick syntax guide
- **[UP Namespaces](https://github.com/uplang/ns)** - Official namespace plugins

### Other Implementations

- **[Go](https://github.com/uplang/go)** - Reference implementation
- **[Java](https://github.com/uplang/java)** - Modern Java 21+ with records and sealed types
- **[JavaScript/TypeScript](https://github.com/uplang/js)** - Browser and Node.js support
- **[Python](https://github.com/uplang/py)** - Pythonic implementation with dataclasses
- **[C](https://github.com/uplang/c)** - Portable C implementation

## Support

- **Issues**: [github.com/uplang/rust/issues](https://github.com/uplang/rust/issues)
- **Discussions**: [github.com/uplang/spec/discussions](https://github.com/uplang/spec/discussions)
