use std::collections::HashMap;

pub type State = String;
pub type Symbol = String;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadMove {
    Left,
    Stay,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tape {
    pub blank: Symbol,
    left_half: Vec<Symbol>,
    right_half: Vec<Symbol>,
}

impl Tape {
    pub fn new<T: Into<Vec<Symbol>>>(blank: Symbol, tape: T) -> Self {
        let mut result = Self {
            blank,
            left_half: vec![],
            right_half: tape.into(),
        };
        if result.right_half.is_empty() {
            result.right_half.push(result.blank.clone())
        }
        result
    }

    pub fn get_current_index(&self) -> usize {
        self.left_half.len()
    }

    pub fn move_head(&mut self, direction: HeadMove) {
        match direction {
            HeadMove::Left => {
                let symbol = self.left_half.pop().unwrap_or(self.blank.clone());
                self.right_half.push(symbol)
            }
            HeadMove::Stay => {}
            HeadMove::Right => {
                let symbol = self.right_half.pop().unwrap_or(self.blank.clone());
                self.left_half.push(symbol);
                if self.right_half.is_empty() {
                    self.right_half.push(self.blank.clone())
                }
            }
        }
    }

    pub fn get_current_symbol(&self) -> &Symbol {
        self.right_half.last().unwrap()
    }

    pub fn set_current_symbol(&mut self, symbol: Symbol) {
        *self.right_half.last_mut().unwrap() = symbol
    }

    pub fn iter(&self) -> impl Iterator<Item = &Symbol> {
        self.left_half.iter().rev().chain(self.right_half.iter())
    }
}

pub type Rules = HashMap<State, HashMap<Symbol, (State, Symbol, HeadMove)>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TuringMachine {
    pub rules: Rules,
    pub tape: Tape,
    pub state: State,
}

impl TuringMachine {
    pub fn make_step(&mut self) -> Result<(), ()> {
        let (new_state, new_symbol, head_move) = self
            .rules
            .get(&self.state)
            .ok_or(())?
            .get(self.tape.get_current_symbol())
            .ok_or(())?
            .to_owned();
        self.tape.set_current_symbol(new_symbol);
        self.tape.move_head(head_move);
        self.state = new_state;
        Ok(())
    }

    pub fn next(mut self) -> Result<Self, Tape> {
        match self.make_step() {
            Ok(()) => Ok(self),
            Err(()) => Err(self.tape),
        }
    }
}
