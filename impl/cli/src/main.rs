use std::sync::Arc;

use lexer::Lexer;
use miette::NamedSource;
use parser::Parser;

fn main() {
    if let Err(e) = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .rgb_colors(miette::RgbColors::Preferred)
                .show_related_errors_as_nested()
                .context_lines(3)
                .build(),
        )
    })) {
        eprintln!("Failed to install hook for Miette: {e:?}");
        std::process::exit(1);
    }

    let sources = r#"
fn hello() {
    println("Hi!")
}

struct House {
  street: u16,
  id: u64,
}
"#;
    let file = Arc::new(NamedSource::new("test.b", sources.to_string()));
    let lexer = Lexer::new(file.clone(), sources);
    let mut parser = Parser::new(file, lexer);
    println!("{:#?}", parser.parse());
}
