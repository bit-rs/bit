use camino::Utf8PathBuf;
use geko_rt::interpreter::Interpreter;

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
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret_module(Utf8PathBuf::from("/home/vyacheslav/geko/test.gk"));
}
