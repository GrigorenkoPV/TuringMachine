use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

// TODO: manual Debug impl's

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Left,
    InPlace,
    Right,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Symbol {
    Blank,
    NonBlank(Rc<str>),
}

#[derive(Clone, Debug)]
pub struct Tape {
    tape: VecDeque<Symbol>,
    pos: usize,
}

impl Tape {
    pub fn new(mut tape: VecDeque<Symbol>) -> Self {
        if tape.is_empty() {
            tape.push_back(Symbol::Blank);
        }
        Self { tape, pos: 0 }
    }

    pub fn get_current(&self) -> Symbol {
        self.tape[self.pos].clone()
    }

    pub fn step(&mut self, direction: Direction) {
        use Direction::*;
        match direction {
            InPlace => {}
            Left => {
                if self.pos == 0 {
                    self.tape.push_front(Symbol::Blank)
                } else {
                    self.pos -= 1
                }
            }
            Right => {
                self.pos += 1;
                if self.pos == self.tape.len() {
                    self.tape.push_back(Symbol::Blank)
                }
            }
        }
    }
    pub fn put(&mut self, symbol: Symbol) {
        self.tape[self.pos] = symbol;
    }
}

pub type IntermediateStateName = Rc<str>;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum State {
    Accept,
    Reject,
    Intermediate(IntermediateStateName),
}

pub type RuleLHS = (IntermediateStateName, Symbol);
pub type RuleRHS = (State, Symbol, Direction);
pub type Rules = HashMap<RuleLHS, RuleRHS>;

#[derive(Clone, Debug)]
pub struct TuringMachine {
    pub tape: Tape,
    pub rules: Rules,
    pub state: State,
}

impl TuringMachine {
    pub fn step(&mut self) {
        let State::Intermediate(current_state_name) = &self.state else{
            todo!("Can't run from non-intermediate state")
        };

        if let Some((state, symbol, step)) = self
            .rules
            .get(&(current_state_name.to_owned(), self.tape.get_current()))
            .map(ToOwned::to_owned)
        {
            self.tape.put(symbol);
            self.tape.step(step);
            self.state = state;
        } else {
            self.state = State::Reject;
        }
    }
}
