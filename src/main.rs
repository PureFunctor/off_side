use lexer::lex;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod lexer;

pub const SOURCE: &str = r#"
module Test where

test = case a, b of
  c, d
   | e ->
     case e of
       f | true -> bar
         | false -> baz
   | f -> g
"#;

fn main() {
    let source = SOURCE.trim();
    let tokens = lex(source).expect("a valid source file");
    let mut start = 0;
    let mut end = 0;
    for token in tokens {
        match token.kind {
            lexer::TokenKind::LayoutStart => start += 1,
            lexer::TokenKind::LayoutEnd => end += 1,
            _ => (),
        }
        println!("{:?}", token.kind);
    }
    assert_eq!(start, end);
}
