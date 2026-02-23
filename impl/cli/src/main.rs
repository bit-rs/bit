mod args_parser;
mod tools;

use core::error::Error;
use std::{
    path::PathBuf,
    sync::Arc,
};

use lexer::Lexer;
use miette::NamedSource;
use parser::Parser;

use crate::{args_parser::SubCommand, tools::Platform};

struct CompilerSession {
    filepath: PathBuf,
    output_name: String,
}

fn compile(sess: CompilerSession) -> Result<(), Box<dyn Error>> {
    // TODO: Used this to silence warning. When we get to codegen, remove that statement.
    let _ = sess.output_name;
    
    let sources = std::fs::read_to_string(&sess.filepath)?;

    let file = Arc::new(NamedSource::new(
        sess.filepath.as_os_str().to_str().unwrap(),
        sources.clone(),
    ));

    let lexer = Lexer::new(file.clone(), &sources);
    let mut parser = Parser::new(file, lexer);

    println!("{:#?}", parser.parse());

    Ok(())
}

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

    let args = args_parser::parse_args();

    match args.command {
        SubCommand::Compile { filepath, platform } => {
            // TODO: Cross-compilation platform detection (maybe a platform detection from a `target triple`)
            let output_filepath = tools::source_file_path_to_binary_path(
                &filepath,
                platform.unwrap_or(Platform::native()),
            );

            let compilation_result = compile(CompilerSession {
                filepath: filepath.into(),
                output_name: output_filepath,
            });

            if let Err(err) = compilation_result {
                eprintln!("bit compiler exited with error: {err:?}");
            }
        }
    }
}
