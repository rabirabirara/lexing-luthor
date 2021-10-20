use crate::fa::{State, StateSet};
use crate::symbol::Symbol;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Transition {
    sym: Symbol,
    start: State,
    end: State,
}

impl Transition {
    pub fn from(sym: Symbol, start: State, end: State) -> Self {
        Self { sym, start, end }
    }
    pub fn sym(&self) -> Symbol {
        self.sym
    }
    pub fn start(&self) -> State {
        self.start
    }
    pub fn set_start(&mut self, s: State) {
        self.start = s;
    }
    pub fn end(&self) -> State {
        self.end
    }
    pub fn set_end(&mut self, s: State) {
        self.end = s;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SetTransition<State> {
    sym: Symbol,
    begin: StateSet<State>,
    end: StateSet<State>,
}

impl SetTransition<State> {
    pub fn from(sym: Symbol, begin: StateSet<State>, end: StateSet<State>) -> Self {
        Self { sym, begin, end }
    }
    pub fn sym(&self) -> Symbol {
        self.sym
    }
    pub fn begin(&self) -> &StateSet<State> {
        &self.begin
    }
    pub fn end(&self) -> &StateSet<State> {
        &self.end
    }
}
