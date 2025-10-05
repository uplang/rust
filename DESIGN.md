# Design Documentation - Rust Implementation

This document describes the architecture and design decisions of the Rust UP parser implementation.

## Overview

The Rust implementation prioritizes:

- **Memory Safety** - Rust's ownership system
- **Zero-Cost** - No runtime overhead
- **Performance** - Blazingly fast parsing
- **Type Safety** - Strong type system
- **Zero Dependencies** - Pure Rust

## Architecture

### Data Structures

```rust
pub struct Node {
    pub key: String,
    pub type_annotation: Option<String>,
    pub value: Value,
}

pub struct Document {
    pub nodes: Vec<Node>,
}

pub enum Value {
    Scalar(String),
    Block(HashMap<String, Value>),
    List(Vec<Value>),
    Table(Table),
    Multiline(String),
}
```

### Ownership Model

```rust
// Document owns nodes
pub struct Document {
    nodes: Vec<Node>,  // Owned
}

// Node owns its value
pub struct Node {
    key: String,       // Owned
    value: Value,      // Owned
}
```

## Parser Implementation

### Zero-Copy Where Possible

```rust
// Borrows input, returns owned Document
pub fn parse(&self, input: &str) -> Result<Document, ParseError>
```

### Error Handling

```rust
#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl std::error::Error for ParseError {}
```

## Performance

- **Single-pass** parsing
- **Minimal allocations**
- **Stack-based** where possible
- **No regex** in hot path

## Design Decisions

### Why Enum for Value?

**Pros:**
- Pattern matching
- Compile-time exhaustiveness
- Zero-cost abstraction

**Decision:** Rust idiom, perfect fit

### Why HashMap for Blocks?

**Pros:**
- O(1) lookup
- Standard library
- Well-optimized

**Decision:** Performance and simplicity

## Contributing

When contributing:

1. **cargo fmt** - Format code
2. **cargo clippy** - Lint code
3. **Tests** - Maintain coverage
4. **Documentation** - Doc comments

## References

- [UP Specification](https://github.com/uplang/spec)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
