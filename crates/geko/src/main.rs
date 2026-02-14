/// Modules
mod io;

/// Imports
use crate::io::CliIO;
use camino::Utf8PathBuf;
use geko_rt::{interpreter::Interpreter, io::IO};

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
    let io = CliIO;
    let path = Utf8PathBuf::from("/home/vyacheslav/geko/examples/math/a.gk");
    let code = io.read(&path);
    let mut interpreter = Interpreter::new(io);
    let _ = interpreter.interpret_module("a", &code);
}
