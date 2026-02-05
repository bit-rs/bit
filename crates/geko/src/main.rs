use geko_lex::lexer::Lexer;
use geko_parse::parser::Parser;
use geko_rt::interpreter::Interpreter;
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
let const = 5;

type Dog {
    fn init(a) {
        self.a = a;
        self.b = const;
    }
}

let dog = Dog(3);
let dog2 = dog;
println(dog.a);
println(dog2.a);
println(dog.b);
println(dog2.b);
"#;
    let src = Arc::new(NamedSource::new("test.gk", text.to_string()));
    let lexer = Lexer::new(src.clone(), &text);
    let mut parser = Parser::new(src, lexer);
    let ast = parser.parse();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.exec_block(&ast, false);
}
