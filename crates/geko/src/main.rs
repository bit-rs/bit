use camino::Utf8Path;
use geko_lex::lexer::Lexer;

fn main() {
    let text = "12312.1214124 for i { }";
    let path = Utf8Path::new("");
    let lexer = Lexer::new(&path, &text);
    let mut stream = lexer.into_iter();
    println!(
        "{:?}{:?}{:?}{:?}{:?}",
        stream.next(),
        stream.next(),
        stream.next(),
        stream.next(),
        stream.next()
    )
}
