use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Left,
    InPlace,
    Right,
}

pub type Symbol = Rc<str>;

#[derive(Clone, Debug)]
pub struct Tape {
    tape: VecDeque<Symbol>,
    blank: Symbol,
    pos: usize,
}

impl Tape {
    pub fn new(mut tape: VecDeque<Symbol>, blank: Symbol) -> Self {
        if tape.is_empty() {
            tape.push_back(blank.clone());
        }
        Self {
            tape,
            blank,
            pos: 0,
        }
    }

    pub fn get_blank(&self) -> Symbol {
        self.blank.clone()
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
                    self.tape.push_front(self.get_blank())
                } else {
                    self.pos -= 1
                }
            }
            Right => {
                self.pos += 1;
                if self.pos == self.tape.len() {
                    self.tape.push_back(self.get_blank())
                }
            }
        }
    }
    pub fn put(&mut self, symbol: Symbol) {
        self.tape[self.pos] = symbol;
    }
}

pub type State = Rc<str>;
pub type RuleLHS = (State, Symbol);
pub type RuleRHS = (State, Symbol, Direction);
pub type Rules = HashMap<RuleLHS, RuleRHS>;

#[derive(Clone, Debug)]
pub struct TuringMachine {
    pub tape: Tape,
    pub state: State,
    pub accept: State,
    pub reject: State,
    pub rules: Rules,
}

pub enum StepResult {
    InProgress,
    Accept,
    RejectByRule,
    RejectByNoRule,
}

impl TuringMachine {
    pub fn step(&mut self) -> StepResult {
        use StepResult::*;
        if let Some((state, symbol, step)) = self
            .rules
            .get(&(self.state.to_owned(), self.tape.get_current()))
            .map(|x| x.to_owned())
        {
            self.state = state;
            self.tape.put(symbol);
            self.tape.step(step);
            if self.state == self.accept {
                Accept
            } else if self.state == self.reject {
                RejectByRule
            } else {
                InProgress
            }
        } else {
            self.state = self.reject.to_owned();
            RejectByNoRule
        }
    }
}
