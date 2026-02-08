/// Asserts tokens
#[macro_export]
macro_rules! assert_tokens {
    ($text:expr, $tokens:expr) => {
        let src = std::sync::Arc::new(miette::NamedSource::new("test.gk", $text.to_string()));
        let lexer = geko_lex::lexer::Lexer::new(src, $text);
        assert_eq!(lexer.map(|tk| tk.kind).collect::<Vec<_>>(), $tokens);
    };
}
