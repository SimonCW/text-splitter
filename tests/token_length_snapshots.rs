use std::fs;

use once_cell::sync::Lazy;
use text_splitter::TextSplitter;
use tokenizers::Tokenizer;

static TOKENIZER: Lazy<Tokenizer> =
    Lazy::new(|| Tokenizer::from_pretrained("bert-base-cased", None).unwrap());

#[test]
fn huggingface_paragraph_long_chunk() {
    let text = fs::read_to_string("tests/room_with_a_view.txt").unwrap();
    let splitter = TextSplitter::new(1000).with_huggingface_tokenizer(TOKENIZER.clone());
    let chunks = splitter
        .chunk_by_paragraphs(&text)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    insta::assert_yaml_snapshot!(chunks);
}

#[test]
fn huggingface_paragraph_short_chunk() {
    let text = fs::read_to_string("tests/room_with_a_view.txt").unwrap();
    let splitter = TextSplitter::new(100).with_huggingface_tokenizer(TOKENIZER.clone());
    let chunks = splitter
        .chunk_by_paragraphs(&text)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    insta::assert_yaml_snapshot!(chunks);
}

#[test]
fn huggingface_paragraph_tiny_chunk() {
    let text = fs::read_to_string("tests/room_with_a_view.txt").unwrap();
    let splitter = TextSplitter::new(10).with_huggingface_tokenizer(TOKENIZER.clone());
    let chunks = splitter
        .chunk_by_paragraphs(&text)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    insta::assert_yaml_snapshot!(chunks);
}

#[test]
fn huggingface_paragraph_long_chunk_trim() {
    let text = fs::read_to_string("tests/room_with_a_view.txt").unwrap();
    let splitter = TextSplitter::new(1000)
        .with_huggingface_tokenizer(TOKENIZER.clone())
        .with_trim_chunks(true);
    let chunks = splitter
        .chunk_by_paragraphs(&text)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    insta::assert_yaml_snapshot!(chunks);
}

#[test]
fn huggingface_paragraph_short_chunk_trim() {
    let text = fs::read_to_string("tests/room_with_a_view.txt").unwrap();
    let splitter = TextSplitter::new(100)
        .with_huggingface_tokenizer(TOKENIZER.clone())
        .with_trim_chunks(true);
    let chunks = splitter
        .chunk_by_paragraphs(&text)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    insta::assert_yaml_snapshot!(chunks);
}

#[test]
fn huggingface_paragraph_tiny_chunk_trim() {
    let text = fs::read_to_string("tests/room_with_a_view.txt").unwrap();
    let splitter = TextSplitter::new(10)
        .with_huggingface_tokenizer(TOKENIZER.clone())
        .with_trim_chunks(true);
    let chunks = splitter
        .chunk_by_paragraphs(&text)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    insta::assert_yaml_snapshot!(chunks);
}