use std::sync::Arc;

use lexer::Lexer;
use miette::NamedSource;

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
    let sources = "1.3.2";
    let file = Arc::new(NamedSource::new("test.b", sources.to_string()));
    let mut lexer = Lexer::new(file, sources);
    println!("{:?}", lexer.next());
}
