//! UP (Unified Properties) parser for Rust
//!
//! A modern, human-friendly data serialization format parser.
//!
//! # Example
//! ```
//! use uplang::{Parser, parse};
//!
//! let input = r#"
//! name John Doe
//! age!int 30
//! "#;
//!
//! let doc = parse(input).unwrap();
//! ```

use std::collections::HashMap;

/// Parse UP document from a string (convenience function)
pub fn parse(input: &str) -> Result<Document, ParseError> {
    Parser::new().parse_document(input)
}

/// Represents a parsed UP document
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Document {
    /// Top-level nodes in the document
    pub nodes: Vec<Node>,
}

impl Document {
    /// Create a new empty document
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if document is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

/// A key-value node with optional type annotation
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// The key name
    pub key: String,
    /// Optional type annotation (e.g., "int", "bool", "list")
    pub type_annotation: Option<String>,
    /// The value
    pub value: Value,
}

/// Represents any UP value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// String scalar
    String(String),
    /// Block (nested key-value pairs)
    Block(HashMap<String, Value>),
    /// List of values
    List(Vec<Value>),
    /// Table with columns and rows
    Table { columns: Vec<Value>, rows: Vec<Vec<Value>> },
}

/// Parse errors
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Invalid syntax
    InvalidSyntax(String),
    /// Unexpected end of input
    UnexpectedEof,
    /// Invalid list format
    InvalidList(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
            ParseError::UnexpectedEof => write!(f, "Unexpected end of input"),
            ParseError::InvalidList(msg) => write!(f, "Invalid list: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

/// UP document parser with configurable behavior
pub struct Parser;

impl Parser {
    /// Create a new parser with default configuration
    pub fn new() -> Self {
        Self
    }

    /// Parse a UP document from a string
    pub fn parse_document(&self, input: &str) -> Result<Document, ParseError> {
        let lines: Vec<&str> = input.lines().collect();
        let mut line_iter = lines.iter().enumerate().peekable();
        let mut nodes = Vec::new();

        while let Some((line_num, line)) = line_iter.next() {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            match self.parse_line(&mut line_iter, line, line_num) {
                Ok(node) => nodes.push(node),
                Err(e) => return Err(ParseError::InvalidSyntax(format!("line {}: {}", line_num + 1, e))),
            }
        }

        Ok(Document { nodes })
    }

    fn parse_line<'a, I>(&self, lines: &mut std::iter::Peekable<I>, line: &str, _line_num: usize) -> Result<Node, ParseError>
    where
        I: Iterator<Item = (usize, &'a &'a str)>,
    {
        let (key_part, val_part) = self.split_key_value(line);
        let (key, type_annotation) = self.parse_key_and_type(key_part);

        let value = self.parse_value(lines, val_part, type_annotation.as_deref())?;

        Ok(Node {
            key: key.to_string(),
            type_annotation,
            value,
        })
    }

    fn split_key_value<'a>(&self, line: &'a str) -> (&'a str, &'a str) {
        if let Some(idx) = line.find(|c: char| c.is_whitespace()) {
            (line[..idx].trim(), line[idx..].trim())
        } else {
            (line, "")
        }
    }

    fn parse_key_and_type<'a>(&self, key_part: &'a str) -> (&'a str, Option<String>) {
        if let Some(idx) = key_part.find('!') {
            (
                &key_part[..idx],
                Some(key_part[idx + 1..].to_string()),
            )
        } else {
            (key_part, None)
        }
    }

    fn parse_value<'a, I>(&self, lines: &mut std::iter::Peekable<I>, val_part: &str, type_annotation: Option<&str>) -> Result<Value, ParseError>
    where
        I: Iterator<Item = (usize, &'a &'a str)>,
    {
        match val_part {
            "{" => self.parse_block(lines),
            "[" => self.parse_list(lines),
            s if s.starts_with("```") => self.parse_multiline(lines, type_annotation),
            s if s.starts_with('[') && s.ends_with(']') => {
                // Inline list
                Ok(Value::List(self.parse_inline_list(s)?))
            }
            _ => Ok(Value::String(val_part.to_string())),
        }
    }

    fn parse_multiline<'a, I>(&self, lines: &mut std::iter::Peekable<I>, type_annotation: Option<&str>) -> Result<Value, ParseError>
    where
        I: Iterator<Item = (usize, &'a &'a str)>,
    {
        let mut content = Vec::new();

        for (_, line) in lines.by_ref() {
            let trimmed = line.trim();
            if trimmed == "```" {
                break;
            }
            content.push(line.to_string());
        }

        let mut text = content.join("\n");

        // Apply dedenting if type annotation is a number
        if let Some(type_str) = type_annotation {
            if let Ok(dedent_amount) = type_str.parse::<usize>() {
                text = self.dedent(&text, dedent_amount);
            }
        }

        Ok(Value::String(text))
    }

    fn parse_block<'a, I>(&self, lines: &mut std::iter::Peekable<I>) -> Result<Value, ParseError>
    where
        I: Iterator<Item = (usize, &'a &'a str)>,
    {
        let mut block = HashMap::new();

        while let Some((line_num, line)) = lines.next() {
            let trimmed = line.trim();

            if trimmed == "}" {
                break;
            }

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let node = self.parse_line(lines, trimmed, line_num)?;
            block.insert(node.key, node.value);
        }

        Ok(Value::Block(block))
    }

    fn parse_list<'a, I>(&self, lines: &mut std::iter::Peekable<I>) -> Result<Value, ParseError>
    where
        I: Iterator<Item = (usize, &'a &'a str)>,
    {
        let mut list = Vec::new();

        while let Some((_, line)) = lines.next() {
            let trimmed = line.trim();

            if trimmed == "]" {
                break;
            }

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Handle inline list within a multiline list
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                let inner_list = self.parse_inline_list(trimmed)?;
                list.push(Value::List(inner_list));
            } else if trimmed.starts_with('{') {
                let block = self.parse_block(lines)?;
                list.push(block);
            } else {
                list.push(Value::String(trimmed.to_string()));
            }
        }

        Ok(Value::List(list))
    }

    fn parse_inline_list(&self, s: &str) -> Result<Vec<Value>, ParseError> {
        let s = s.trim();
        let s = s.strip_prefix('[').unwrap_or(s);
        let s = s.strip_suffix(']').unwrap_or(s);

        if s.trim().is_empty() {
            return Ok(Vec::new());
        }

        let items: Vec<Value> = s
            .split(',')
            .map(|item| Value::String(item.trim().to_string()))
            .collect();

        Ok(items)
    }

    fn dedent(&self, text: &str, amount: usize) -> String {
        text.lines()
            .map(|line| {
                if line.len() >= amount {
                    &line[amount..]
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let result = parse("");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_parse_simple_scalar() {
        let result = parse("name John Doe");
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.nodes.len(), 1);
        assert_eq!(doc.nodes[0].key, "name");
        assert_eq!(doc.nodes[0].value, Value::String("John Doe".to_string()));
    }

    #[test]
    fn test_parse_type_annotation() {
        let result = parse("age!int 30");
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.nodes.len(), 1);
        assert_eq!(doc.nodes[0].key, "age");
        assert_eq!(doc.nodes[0].type_annotation, Some("int".to_string()));
        assert_eq!(doc.nodes[0].value, Value::String("30".to_string()));
    }

    #[test]
    fn test_parse_block() {
        let input = r#"
server {
host localhost
port!int 8080
}
"#;
        let result = parse(input);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.nodes.len(), 1);
        assert_eq!(doc.nodes[0].key, "server");
        match &doc.nodes[0].value {
            Value::Block(block) => {
                assert_eq!(block.len(), 2);
                assert_eq!(block.get("host"), Some(&Value::String("localhost".to_string())));
            }
            _ => panic!("Expected block"),
        }
    }

    #[test]
    fn test_parse_list() {
        let input = r#"
fruits [
apple
banana
cherry
]
"#;
        let result = parse(input);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.nodes.len(), 1);
        match &doc.nodes[0].value {
            Value::List(list) => {
                assert_eq!(list.len(), 3);
                assert_eq!(list[0], Value::String("apple".to_string()));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_parse_inline_list() {
        let result = parse("colors [red, green, blue]");
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.nodes.len(), 1);
        match &doc.nodes[0].value {
            Value::List(list) => {
                assert_eq!(list.len(), 3);
                assert_eq!(list[0], Value::String("red".to_string()));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_parse_multiline() {
        let input = r#"
description ```
Line 1
Line 2
```
"#;
        let result = parse(input);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.nodes.len(), 1);
        match &doc.nodes[0].value {
            Value::String(s) => {
                assert!(s.contains("Line 1"));
                assert!(s.contains("Line 2"));
            }
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_skip_comments() {
        let input = r#"
# This is a comment
name John
# Another comment
age!int 30
"#;
        let result = parse(input);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.nodes.len(), 2);
    }

    #[test]
    fn test_document_default() {
        let doc = Document::default();
        assert_eq!(doc, Document { nodes: vec![] });
    }
}
