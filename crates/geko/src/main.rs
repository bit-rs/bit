use std::sync::Arc;

use camino::Utf8Path;
use geko_lex::lexer::Lexer;
use miette::NamedSource;

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
    let text = "12312.1214124 for i { } $";
    let src = Arc::new(NamedSource::new("test.gk", text.to_string()));
    let lexer = Lexer::new(src, &text);
    let mut stream = lexer.into_iter();
    println!(
        "{:?}{:?}{:?}{:?}{:?}{:?}",
        stream.next(),
        stream.next(),
        stream.next(),
        stream.next(),
        stream.next(),
        stream.next()
    )
}
