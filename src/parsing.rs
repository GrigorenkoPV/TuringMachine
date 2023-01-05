use chumsky::prelude::*;
use turing_machine::*;

#[cfg(test)]
mod tests;

pub type Input = char;
pub type ParsingError = Simple<Input>;

// TODO: better checks for reject/accept
pub fn parser() -> impl Parser<Input, (Rules, State), Error = ParsingError> {
    let ident = filter(|c: &Input| !c.is_whitespace())
        .repeated()
        .at_least(1)
        .collect::<String>();

    let header = |name| just(name).ignore_then(just(": ")).ignore_then(ident);

    header("start")
        .then_ignore(text::newline())
        .then(header("accept"))
        .then_ignore(text::newline())
        .then(header("reject"))
        .then_ignore(text::newline())
        .then(header("blank"))
        .then_with(move |(((start, accept), reject), blank)| {
            let start = State::Intermediate(start.into());
            let symbol = ident.map(move |s| {
                if s == blank {
                    Symbol::Blank
                } else {
                    Symbol::NonBlank(s.into())
                }
            });
            let state = ident.map(move |s| {
                if s == accept {
                    State::Accept
                } else if s == reject {
                    State::Reject
                } else {
                    State::Intermediate(s.into())
                }
            });

            let whitespace =
                filter(|&c: &Input| c.is_whitespace() && !"\r\n".contains(c)).ignored();
            let some_whitespace = whitespace.repeated().at_least(1);

            let rule_lhs = ident
                .map(IntermediateStateName::from)
                .then_ignore(some_whitespace)
                .then(symbol.clone())
                .labelled("Rule LHS");

            let direction = choice((
                just('<').to(Direction::Left),
                just('^').to(Direction::InPlace),
                just('>').to(Direction::Right),
            ));

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

            text::newline()
                .repeated()
                .at_least(1)
                .ignore_then(rule)
                .repeated()
                .collect::<Rules>()
                .labelled("Rules")
                .map(move |rules| (rules, start.to_owned()))
        })
        .then_ignore(text::newline().repeated())
        .then_ignore(end())
        .labelled("Turing machine")
}
