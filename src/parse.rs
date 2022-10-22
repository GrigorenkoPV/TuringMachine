use nom::{
    bytes::complete::{tag, take_till1},
    character::complete::{char, line_ending, one_of, space1},
    combinator::{eof, map},
    multi::many0,
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};
use turing_machine::*;

#[cfg(test)]
mod tests;

pub type Input<'a> = &'a str;
pub type ParseError = ();
// FIXME: make error something more useful than a ()
pub type ParseResult<'a, T> = IResult<Input<'a>, T, ParseError>;

// TODO: caching
pub fn symbol(input: Input) -> ParseResult<Symbol> {
    map(take_till1(char::is_whitespace), Symbol::from)(input)
}

// TODO: caching
pub fn state(input: Input) -> ParseResult<State> {
    map(take_till1(char::is_whitespace), State::from)(input)
}

pub fn direction(input: Input) -> ParseResult<Direction> {
    const LEFT: char = '<';
    const IN_PLACE: char = '^';
    const RIGHT: char = '>';
    use Direction::*;
    map(one_of([LEFT, IN_PLACE, RIGHT].as_slice()), |c| match c {
        LEFT => Left,
        IN_PLACE => InPlace,
        RIGHT => Right,
        _ => unreachable!(),
    })(input)
}

#[derive(Debug, Clone)]
pub struct Header {
    pub start: State,
    pub accept: State,
    pub reject: State,
    pub blank: Symbol,
}

fn header_entry_with_name<'a, P, T>(
    name: &'static str,
    parser: P,
) -> impl FnMut(Input<'a>) -> ParseResult<'a, T>
where
    P: FnMut(Input) -> ParseResult<T>,
{
    preceded(tuple((tag(name), char(':'), space1)), parser)
}

pub fn header(input: Input) -> ParseResult<Header> {
    map(
        tuple((
            header_entry_with_name("start", state),
            line_ending,
            header_entry_with_name("accept", state),
            line_ending,
            header_entry_with_name("reject", state),
            line_ending,
            header_entry_with_name("blank", symbol),
        )),
        |(start, _, accept, _, reject, _, blank)| Header {
            start,
            accept,
            reject,
            blank,
        },
    )(input)
}

pub fn rule(input: Input) -> ParseResult<(RuleLHS, RuleRHS)> {
    separated_pair(
        separated_pair(state, space1, symbol),
        separated_pair(space1, tag("->"), space1),
        tuple((state, preceded(space1, symbol), preceded(space1, direction))),
    )(input)
}

fn drain0<'a, P, T>(mut parser: P) -> impl FnMut(Input<'a>) -> ParseResult<'a, ()>
where
    P: FnMut(Input<'a>) -> ParseResult<T>,
{
    move |mut i| loop {
        match parser(i) {
            Ok((rest, _)) => {
                if rest == i {
                    break Ok((rest, ()));
                } else {
                    i = rest
                }
            }
            Err(nom::Err::Error(_)) => break Ok((i, ())),
            Err(e) => break Err(e),
        }
    }
}

fn drain1<'a, P, T>(mut parser: P) -> impl FnMut(Input<'a>) -> ParseResult<'a, ()>
where
    P: FnMut(Input<'a>) -> ParseResult<T>,
{
    move |i| {
        let (i, _) = parser(i)?;
        drain0(&mut parser)(i)
    }
}

pub fn file(input: Input) -> Result<(Header, Vec<(RuleLHS, RuleRHS)>), ParseError> {
    let rules = many0(preceded(drain1(line_ending), rule));
    let body = pair(header, rules);
    let tail = preceded(drain0(line_ending), eof);

    let mut file_parser = terminated(body, tail);

    file_parser(input).finish().map(|(rest, parsed)| {
        debug_assert_eq!(rest, "");
        parsed
    })
}
