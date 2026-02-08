/// Asserts tokens
#[macro_export]
macro_rules! assert_tokens {
    ($text:expr, $tokens:expr) => {
        let src = std::sync::Arc::new(miette::NamedSource::new("test.gk", $text.to_string()));
        let lexer = geko_lex::lexer::Lexer::new(src, $text);
        assert_eq!(lexer.map(|tk| tk.kind).collect::<Vec<_>>(), $tokens);
    };
}

/// Asserts ast
#[macro_export]
macro_rules! assert_ast {
    ($text:expr) => {{
        let src = std::sync::Arc::new(miette::NamedSource::new("test.gk", $text.to_string()));
        let lexer = geko_lex::lexer::Lexer::new(src.clone(), $text);
        let mut parser = geko_parse::parser::Parser::new(src, lexer);
        insta::assert_snapshot!(format!("{:#?}", parser.parse()));
    }};
}
