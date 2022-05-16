use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
    process::exit,
};

use anyhow::{Context, Error, Result};
use clap::Parser;

mod cli;

use cli::Cli;
use turing_machine::*;

struct Parsed {
    start_state: State,
    accpet_state: State,
    reject_state: State,
    blank_symbol: Symbol,
    rules: Rules,
}

fn parse_machine_file(input: impl BufRead) -> Result<Parsed> {
    let mut start: Option<State> = None;
    let mut accept: Option<State> = None;
    let mut reject: Option<State> = None;
    let mut blank: Option<Symbol> = None;
    let mut rules = Rules::new();

    for (line_no, line) in input.lines().enumerate() {
        let line = line?;
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }

        let line_info = || Error::msg(format!("Line #{}:\n>>> {:?}", line_no + 1, line));

        macro_rules! try_read_header_entry {
            ($header:ident) => {
                if let Some(name) = line.strip_prefix(concat!(stringify!($header), ":")) {
                    if $header.is_some() {
                        return Err(line_info()
                            .context(concat!("Double definition of ", stringify!($header))));
                    } else {
                        let s = name.trim();

                        $header = Some(
                            if s.is_empty() {
                                Err(line_info().context("Identifier is empty"))
                            } else if s.chars().any(|c| c.is_whitespace()) {
                                Err(line_info()
                                    .context(format!("Indentifier {:?} contains whitespaces", s)))
                            } else {
                                Ok(s)
                            }
                            .context(concat!("Invalid name for ", stringify!($header)))?
                            .into(),
                        )
                    }
                    continue;
                }
            };
        }
        try_read_header_entry!(start);
        try_read_header_entry!(accept);
        try_read_header_entry!(reject);
        try_read_header_entry!(blank);

        let [mut lhs, mut rhs] = {
            let mut parts = line.split("->").map(&str::trim);
            let lhs = parts.next().unwrap();
            let get_arrow_error_message = || line_info().context("Expected exatly one ->");
            let rhs = parts.next().ok_or_else(get_arrow_error_message)?;
            if !parts.next().is_none() {
                return Err(get_arrow_error_message());
            }
            [lhs.split_whitespace(), rhs.split_whitespace()]
        };

        let initial_state = State::from(lhs.next().ok_or_else(|| {
            line_info()
                .context("Expected idnetifier for the initial state, found nothing")
                .context("Left-hand side of the rule is too short")
        })?);
        let initial_symbol = Symbol::from(lhs.next().ok_or_else(|| {
            line_info()
                .context("Expected idnetifier for the initial symbol, found nothing")
                .context("Left-hand side of the rule is too short")
        })?);
        if !lhs.next().is_none() {
            return Err(line_info().context("Left-hand side of the rule is too long"));
        }

        let new_state = State::from(rhs.next().ok_or_else(|| {
            line_info()
                .context("Expected idnetifier for the new state, found nothing")
                .context("Right-hand side of the rule is too short")
        })?);
        let new_symbol = Symbol::from(rhs.next().ok_or_else(|| {
            line_info()
                .context("Expected idnetifier for the new symbol, found nothing")
                .context("Right-hand side of the rule is too short")
        })?);
        let head_move = match rhs.next().ok_or_else(|| {
            line_info()
                .context("Expected idnetifier for the new symbol, found nothing")
                .context("Right-hand side of the rule is too short")
        })? {
            "<" => HeadMove::Left,
            "^" => HeadMove::Stay,
            ">" => HeadMove::Right,
            unkown => {
                return Err(
                    line_info().context(format!("Invalid symbol for the head move: {:?}", unkown))
                )
            }
        };
        if !rhs.next().is_none() {
            return Err(line_info().context("Right-hand side of the rule is too long"));
        }
        use std::collections::hash_map::Entry::*;
        let transitions = match rules.entry(initial_state.clone()) {
            Occupied(e) => e.into_mut(),
            Vacant(e) => e.insert(HashMap::new()),
        };
        match transitions.entry(initial_symbol) {
            Occupied(e) => {
                return Err(match e.get() {
                    (new_state, new_symbol, head_move) => line_info().context(format!(
                        concat!(
                            "Duplicate for rule from state={:?} and symbol={:?}. ",
                            "Already exists with new_state={:?}, new_symbol={:?}, head_move={}."
                        ),
                        initial_state,
                        e.key(),
                        new_state,
                        new_symbol,
                        match &head_move {
                            HeadMove::Left => '<',
                            HeadMove::Stay => '^',
                            HeadMove::Right => '>',
                        }
                    )),
                })
            }
            Vacant(e) => {
                e.insert((new_state, new_symbol, head_move));
            }
        }
    }

    macro_rules! unwrap {
        ($header: ident) => {
            $header.ok_or(Error::msg(concat!(
                "No definition found for ",
                stringify!($header)
            )))?
        };
    }
    let accpet_state = unwrap!(accept);
    let reject_state = unwrap!(reject);
    if accpet_state == reject_state {
        Err(Error::msg(format!(
            "Accept state and reject state have the same value: {:?}",
            accpet_state
        )))
    } else {
        Ok(Parsed {
            start_state: unwrap!(start),
            accpet_state,
            reject_state,
            blank_symbol: unwrap!(blank),
            rules,
        })
    }
}

fn read_tape(input: impl BufRead) -> Result<Vec<Symbol>> {
    Ok(input
        .lines()
        .next()
        .transpose()?
        .unwrap_or(String::new())
        .split_whitespace()
        .map(Symbol::from)
        .collect())
}

fn print_centered(s: &str, width: usize) {
    let n = s.len();
    let missing_spaces = width.saturating_sub(n);
    let spaces_on_the_right = missing_spaces / 2;
    let spaces_on_the_left = missing_spaces - spaces_on_the_right;
    print!(
        "{}{}{}",
        " ".repeat(spaces_on_the_left),
        s,
        " ".repeat(spaces_on_the_right)
    );
}

fn print(machine: &TuringMachine) {
    let width = machine.tape.iter().map(|s| s.len()).max().unwrap();

    let mut iter = machine.tape.iter();
    print_centered(&iter.next().unwrap(), width);

    let mut n = 0;
    for symbol in iter {
        n += 1;
        print!(" ");
        print_centered(&symbol, width);
    }
    let n = n;
    println!();

    let current = machine.tape.get_current_index();
    for i in 0..=n {
        let empty = " ".repeat(width);
        if i == current {
            print_centered("^", width);
        } else {
            print!("{}", empty)
        }
        if i != n {
            print!(" ");
        }
    }
    println!();
    println!("State: {}", machine.state);
    println!();
}

enum Verdict {
    Accepted,
    Rejected,
    NoEdge,
    TL,
}
use Verdict::*;

fn run(
    mut machine: TuringMachine,
    accpet_state: State,
    reject_state: State,
    tl: Option<NonZeroUsize>,
) -> (Verdict, TuringMachine) {
    let mut step = 0;
    let verdict = loop {
        println!("Step: {}", step);
        print(&machine);
        println!();
        step += 1;
        if machine.make_step().is_err() {
            break NoEdge;
        } else if machine.state == accpet_state {
            break Accepted;
        } else if machine.state == reject_state {
            break Rejected;
        } else if tl.map(|tl| tl.get() == step).unwrap_or(false) {
            break TL;
        }
    };
    (verdict, machine)
}

fn main() -> Result<()> {
    let Cli {
        machine_file,
        input_file,
        time_limit,
    } = Cli::parse();

    let Parsed {
        start_state,
        accpet_state,
        reject_state,
        blank_symbol,
        rules,
    } = parse_machine_file(BufReader::new(
        File::open(machine_file).context("Error opening the turing machine file")?,
    ))
    .context("Error reading the turing machine file")?;

    let tape_input: Box<dyn BufRead> = match input_file {
        Some(path) => Box::new(BufReader::new(
            File::open(path).context("Error opening the input file")?,
        )),
        None => Box::new(BufReader::new(io::stdin())),
    };
    let tape = read_tape(tape_input).context("Error reading the input")?;

    let machine = TuringMachine {
        rules,
        tape: Tape::new(blank_symbol, tape),
        state: start_state,
    };

    let (verdict, machine) = run(
        machine,
        accpet_state,
        reject_state,
        NonZeroUsize::new(time_limit),
    );
    println!(
        "Result: {}",
        match verdict {
            Accepted => "Accepted",
            Rejected => "Rejected",
            NoEdge => "Rejected: no suitable rule found",
            TL => "Time limit exceeded",
        }
    );
    print(&machine);
    exit(match verdict {
        Accepted => 0,
        Rejected => -1,
        NoEdge => -2,
        TL => -3,
    })
}
