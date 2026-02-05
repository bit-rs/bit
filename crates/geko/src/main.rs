use geko_lex::lexer::Lexer;
use geko_parse::parser::Parser;
use miette::NamedSource;
use std::sync::Arc;

fn main() {
    let _ = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(false)
                .rgb_colors(miette::RgbColors::Preferred)
                .show_related_errors_as_nested()
                .context_lines(3)
                .build(),
        )
    }));
    let text = r#"
fn hello() {
    for i in 0..100 {
        if i > 10 {
            io.println("Hello, world!") = 3;
        }
    }
}
"#;
    let src = Arc::new(NamedSource::new("test.gk", text.to_string()));
    let lexer = Lexer::new(src.clone(), &text);
    let mut parser = Parser::new(src, lexer);
    println!("{:#?}", parser.parse())
}
