use crate::parse;

#[test]
fn file() {
    dbg!(parse::file(include_str!("snippets/zero.out")).unwrap());
}
