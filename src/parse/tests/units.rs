use crate::parse;
use turing_machine::*;

#[test]
fn symbol() {
    assert_eq!(parse::symbol("symbol"), Ok(("", Symbol::from("symbol"))));
    assert_eq!(
        parse::symbol("symbol not-a-symbol"),
        Ok((" not-a-symbol", Symbol::from("symbol")))
    );
    assert!(parse::symbol(" symbol").is_err());
}

#[test]
fn state() {
    assert_eq!(parse::state("state"), Ok(("", State::from("state"))));
    assert_eq!(
        parse::state("state not-a-state"),
        Ok((" not-a-state", State::from("state")))
    );
    assert!(parse::state(" state").is_err());
}

#[test]
fn direction() {
    assert_eq!(parse::direction("<"), Ok(("", Direction::Left)));
    assert_eq!(parse::direction("^"), Ok(("", Direction::InPlace)));
    assert_eq!(parse::direction(">"), Ok(("", Direction::Right)));
    assert_eq!(parse::direction("<>"), Ok((">", Direction::Left)))
}

#[test]
fn header_entry_with_name() {
    let mut parser = parse::header_entry_with_name("header", parse::symbol);
    assert_eq!(parser("header: symbol"), Ok(("", Symbol::from("symbol"))));
    assert_eq!(parser("header: wow "), Ok((" ", Symbol::from("wow"))));
    assert!(parser("headers: symbol").is_err());
    assert!(parser("header : symbol").is_err());
    assert!(parser("header :symbol").is_err());
}

#[test]
fn rule() {
    assert_eq!(
        parse::rule("state1 symbol1 -> state2 symbol2 ^"),
        Ok((
            "",
            (
                (State::from("state1"), Symbol::from("symbol1"),),
                (
                    State::from("state2"),
                    Symbol::from("symbol2"),
                    Direction::InPlace
                )
            )
        ))
    );
    assert_eq!(
        parse::rule("state1   \t  symbol1   \t  ->  \t  state2 \t   symbol2    ^ \t\r\n"),
        Ok((
            " \t\r\n",
            (
                (State::from("state1"), Symbol::from("symbol1"),),
                (
                    State::from("state2"),
                    Symbol::from("symbol2"),
                    Direction::InPlace
                )
            )
        ))
    );
    assert!(parse::rule("state1 symbol1 - > state2 symbol2 ^").is_err());
    assert!(parse::rule(" state1 symbol1 -> state2 symbol2 ^").is_err());
    assert!(parse::rule("state1 symbol 1 -> state2 symbol2 ^").is_err());
    assert!(parse::rule("state1 symbol1 -> st ate2 symbol2 ^").is_err());
    assert!(parse::rule("state1 symbol1 -> state2 symbol2 .").is_err());
    assert!(parse::rule("state1 symbol1 -> state2 symbol2").is_err());
}

mod drain0 {
    use super::*;

    #[test]
    fn complete() {
        use nom::bytes::complete::take;
        assert_eq!(parse::drain0(take(0usize))("abc"), Ok(("abc", ())));
        assert_eq!(parse::drain0(take(1usize))("abc"), Ok(("", ())));
        assert_eq!(parse::drain0(take(2usize))("abc"), Ok(("c", ())));
        assert_eq!(parse::drain0(take(3usize))("abc"), Ok(("", ())));
        assert_eq!(parse::drain0(take(4usize))("abc"), Ok(("abc", ())));
    }

    #[test]
    fn streaming() {
        use nom::bytes::streaming::take;
        assert_eq!(parse::drain0(take(0usize))("abc"), Ok(("abc", ())));
        for i in 1usize..=4 {
            assert!(matches!(
                parse::drain0(take(i))("abc"),
                Err(nom::Err::Incomplete(_))
            ));
        }
    }

    #[test]
    fn combined() {
        use nom::{character::complete::space1, sequence::terminated};
        assert_eq!(
            parse::drain0(terminated(parse::symbol, space1))("1 1 1 2"),
            Ok(("2", ()))
        );
        assert_eq!(
            parse::drain0(terminated(parse::symbol, space1))("1"),
            Ok(("1", ()))
        );
    }
}

mod drain1 {
    use super::*;

    #[test]
    fn complete() {
        use nom::bytes::complete::take;
        assert_eq!(parse::drain1(take(0usize))("abc"), Ok(("abc", ())));
        assert_eq!(parse::drain1(take(1usize))("abc"), Ok(("", ())));
        assert_eq!(parse::drain1(take(2usize))("abc"), Ok(("c", ())));
        assert_eq!(parse::drain1(take(3usize))("abc"), Ok(("", ())));
        assert_eq!(parse::drain1(take(4usize))("abc"), Err(nom::Err::Error(())));
    }

    #[test]
    fn streaming() {
        use nom::bytes::streaming::take;
        assert_eq!(parse::drain1(take(0usize))("abc"), Ok(("abc", ())));
        for i in 1usize..=4 {
            assert!(matches!(
                parse::drain1(take(i))("abc"),
                Err(nom::Err::Incomplete(_))
            ));
        }
    }

    #[test]
    fn combined() {
        use nom::{character::complete::space1, sequence::terminated};
        assert_eq!(
            parse::drain1(terminated(parse::symbol, space1))("1 1 1 2"),
            Ok(("2", ()))
        );
        assert_eq!(
            parse::drain1(terminated(parse::symbol, space1))("1"),
            Err(nom::Err::Error(()))
        );
    }
}
