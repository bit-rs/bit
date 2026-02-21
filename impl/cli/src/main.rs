use std::sync::Arc;

use lexer::Lexer;
use miette::NamedSource;
use parser::Parser;

fn main() {
    let _ = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .rgb_colors(miette::RgbColors::Preferred)
                .show_related_errors_as_nested()
                .context_lines(3)
                .build(),
        )
    }));
    let sources = r#"
fn hello() {
    println("Hi!")
}
"#;
    let file = Arc::new(NamedSource::new("test.b", sources.to_string()));
    let lexer = Lexer::new(file.clone(), sources);
    let mut parser = Parser::new(file, lexer);
    println!("{:#?}", parser.parse());
}
