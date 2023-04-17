use tokenizers::Tokenizer;

use crate::TextSplitter;

impl TextSplitter {
    /// Specify a Huggingface Tokenizer to use for calculating length of chunks.
    ///
    /// `max_chunk_size` will then be calculated in tokens instead of characters.
    ///
    /// ```
    /// use text_splitter::TextSplitter;
    /// use tokenizers::Tokenizer;
    ///
    /// let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None).unwrap();
    ///
    /// let splitter = TextSplitter::new(100).with_huggingface_tokenizer(tokenizer);
    /// ```
    ///
    /// # Panics
    ///
    /// Will panic if you don't have a byte-level tokenizer and the splitter
    /// encounters text it can't tokenize.
    #[must_use]
    pub fn with_huggingface_tokenizer(self, tokenizer: Tokenizer) -> Self {
        self.with_length_fn(move |str| {
            tokenizer
                .encode(str, false)
                .map(|enc| enc.len())
                .expect("Unable to tokenize the following string {str}")
        })
    }
}
