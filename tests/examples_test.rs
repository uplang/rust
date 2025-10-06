use std::fs;
use uplang::parse;

#[test]
fn test_example_01_basic_scalars() {
    let content = fs::read_to_string("../spec/examples/01-basic-scalars.up")
        .expect("Failed to read example file");
    let result = parse(&content);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let doc = result.unwrap();
    assert_eq!(doc.nodes.len(), 19);
}

#[test]
fn test_example_02_blocks() {
    let content = fs::read_to_string("../spec/examples/02-blocks.up")
        .expect("Failed to read example file");
    let result = parse(&content);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let doc = result.unwrap();
    assert_eq!(doc.nodes.len(), 4);
}

#[test]
fn test_example_03_lists() {
    let content = fs::read_to_string("../spec/examples/03-lists.up")
        .expect("Failed to read example file");
    let result = parse(&content);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let doc = result.unwrap();
    assert!(doc.nodes.len() >= 5);
}

#[test]
fn test_example_04_multiline() {
    let content = fs::read_to_string("../spec/examples/04-multiline.up")
        .expect("Failed to read example file");
    let result = parse(&content);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let doc = result.unwrap();
    assert!(doc.nodes.len() >= 5);
}

#[test]
fn test_example_06_comments() {
    let content = fs::read_to_string("../spec/examples/06-comments.up")
        .expect("Failed to read example file");
    let result = parse(&content);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let doc = result.unwrap();
    assert_eq!(doc.nodes.len(), 6);
}

