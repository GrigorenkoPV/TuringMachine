use chumsky::prelude::*;
use turing_machine::*;

#[cfg(test)]
mod tests;

pub type Input = char;
pub type ParsingError = Simple<Input>;

fn header<P, T>(name: &'static str, p: P) -> impl Parser<Input, T, Error = ParsingError>
where
    P: Parser<Input, T, Error = ParsingError>,
    T: Clone,
{
    just(name).ignore_then(just(": ")).ignore_then(p)
}

pub fn parser() -> impl Parser<Input, TuringMachine, Error = ParsingError> {
    let ident = filter(|c: &Input| !c.is_whitespace())
        .repeated()
        .at_least(1)
        .collect::<String>();

    let symbol = ident.map(Symbol::from);
    let state = ident.map(State::from);

    let direction = choice((
        just('<').to(Direction::Left),
        just('^').to(Direction::InPlace),
        just('>').to(Direction::Right),
    ));

    let whitespace = filter(|&c: &Input| c.is_whitespace() && !"\r\n".contains(c)).ignored();
    let some_whitespace = whitespace.repeated().at_least(1);

    let rule_lhs = state
        .then_ignore(some_whitespace)
        .then(symbol)
        .labelled("Rule LHS");
    let rule_rhs = state
        .then_ignore(some_whitespace)
        .then(symbol)
        .then_ignore(some_whitespace)
        .then(direction)
        .map(|((state, symbol), direction)| (state, symbol, direction))
        .labelled("Rule RHS");
    let rule = rule_lhs
        .then_ignore(some_whitespace)
        .then_ignore(just("->"))
        .then_ignore(some_whitespace)
        .then(rule_rhs)
        .labelled("Rule");

    header("start", state)
        .then_ignore(text::newline())
        .then(header("accept", state))
        .then_ignore(text::newline())
        .then(header("reject", state))
        .then_ignore(text::newline())
        .then(header("blank", symbol))
        .then(
            text::newline()
                .repeated()
                .at_least(1)
                .ignore_then(rule)
                .repeated()
                .collect::<Rules>()
                .labelled("Rules"),
        )
        .map(
            |((((start, accept), reject), blank), rules)| TuringMachine {
                tape: Tape::empty(blank),
                state: start,
                accept,
                reject,
                rules,
            },
        )
        .then_ignore(text::newline().repeated())
        .then_ignore(end())
        .labelled("Turing machine")
}
