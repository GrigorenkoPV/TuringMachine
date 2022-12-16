use chumsky::Parser;

use crate::parsing;

#[test]
fn file() {
    dbg!(parsing::parser()
        .parse(include_str!("snippets/zero.out"))
        .unwrap());
}
