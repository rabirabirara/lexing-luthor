// thompson's construction

use crate::fa::{State, StateSet, FA};
use crate::symbol::{Symbol, ASCII};
use crate::transition::Transition;
use std::sync::atomic::{AtomicUsize, Ordering};
// use std::fmt;

use crate::regex_parser::OPERATORS;

// * A special FA with only one start and end state pair.
// TODO: ! Change Vec<state> to StateSet<State>.
#[derive(Debug, Clone)]
struct FAPiece {
    states: StateSet<State>,
    start: State,
    end: State,
    delta: Vec<Transition>,
}

// !! Or, have FAPiece be an ENUM of the various thompson's pieces!!  have each individual thompson's piece be a struct on its own right!  Have each part point to inner parts.

impl FAPiece {
    pub fn new() -> Self {
        Self {
            states: StateSet::new(),
            start: 0,
            end: 0,
            delta: Vec::new(),
        }
    }
    pub fn produce_id() -> State {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
    pub fn num_states(&self) -> usize {
        self.states.len()
    }
    pub fn set_start(&mut self, s: State) {
        self.start = s;
    }
    pub fn set_end(&mut self, s: State) {
        self.end = s;
    }
    pub fn add_state(&mut self, s: State) {
        self.states.insert(s);
    }
    pub fn add_transition(&mut self, t: Transition) {
        self.delta.push(t);
    }
    pub fn remove_state(&mut self, s: State) {
        self.states.remove(&s);
    }
    pub fn states(&self) -> StateSet<State> {
        self.states.clone()
    }
    pub fn start(&self) -> State {
        self.start
    }
    pub fn end(&self) -> State {
        self.end
    }
    pub fn delta(&self) -> &Vec<Transition> {
        &self.delta
    }
    pub fn delta_mut(&mut self) -> &mut Vec<Transition> {
        &mut self.delta
    }
    pub fn just_sym(sym: Symbol) -> Self {
        let mut just_piece = FAPiece::new();

        let start = Self::produce_id();
        let end = Self::produce_id();

        just_piece.add_state(start);
        just_piece.add_state(end);
        just_piece.set_start(start);
        just_piece.set_end(end);

        just_piece.add_transition(Transition::from(sym, start, end));

        return just_piece;
    }
    pub fn new_with_start_end(start: State, end: State) -> Self {
        let mut states = StateSet::new();
        states.insert(start);
        states.insert(end);
        FAPiece {
            states,
            start,
            end,
            delta: Vec::new(),
        }
    }
}

// Define expression.
// Btw, this definition seems robust enough.  See https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html; they do pretty much the same thing.
#[derive(Debug, Clone)]
enum Expr {
    Empty,
    Just(Symbol),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Star(Box<Expr>),
    Plus(Box<Expr>),
    QMark(Box<Expr>),
}

fn parse_string_to_expr(s: &String) -> Option<Expr> {
    let mut expstack: Vec<Expr> = Vec::new();
    for c in s.chars() {
        match c {
            '.' => {
                // ! The expression popped second comes first in the chronological order.
                let sym2 = expstack.pop()?;
                let sym1 = expstack.pop()?;
                expstack.push(Expr::And(Box::new(sym1), Box::new(sym2)));
            }
            '|' => {
                let sym1 = expstack.pop()?;
                let sym2 = expstack.pop()?;
                expstack.push(Expr::Or(Box::new(sym1), Box::new(sym2)));
            }
            '*' => {
                if let Some(sym) = expstack.pop() {
                    expstack.push(Expr::Star(Box::new(sym)));
                } else {
                    eprintln!("Missing an expr from the stack.  Again, this is the Star branch of the match.");
                }
            }
            '+' => {
                if let Some(sym) = expstack.pop() {
                    expstack.push(Expr::Plus(Box::new(sym)));
                } else {
                    eprintln!("Missing an expr from the stack.  Again, this is the Plus branch of the match.");
                }
            }
            '?' => {
                if let Some(sym) = expstack.pop() {
                    expstack.push(Expr::QMark(Box::new(sym)));
                } else {
                    eprintln!("Missing an expr from the stack.  Again, this is the QMark branch of the match.");
                }
            }
            c if ASCII.contains(&c) => {
                expstack.push(Expr::Just(Symbol::Char(c)));
            }
            _ => unreachable!("Illegal character"),
        }
    }
    // The last expr on expstack is the expression we want.
    // If not, then expr parse error.
    if expstack.len() != 1 {
        None
    } else {
        expstack.pop()
    }
}

// Parse an Expression into a recursive set of FAPieces.
// ! Super inefficient... it just reads redundant transitions over and over again.  STOP CREATING NEW FAPIECES!  JUST USE THE OLD ONES!
// DETERMINE A WAY TO CONSTANT TIME APPEND STATES AND TRANSITIONS, INSTEAD OF ITERATING
fn parse(expr: Expr) -> FAPiece {
    // Match on an expression, turning it into a single finite automata.
    // This is done by recursing through the expression and building the piece bit by bit.
    match expr {
        Expr::Empty => FAPiece::just_sym(Symbol::Empty),
        Expr::Just(sym) => FAPiece::just_sym(sym),
        Expr::Or(e1, e2) => {
            let fa_piece1 = parse(*e1);
            let fa_piece2 = parse(*e2);

            let start = FAPiece::produce_id();
            let end = FAPiece::produce_id();

            let mut or_piece = FAPiece::new_with_start_end(start, end);

            fa_piece1
                .states()
                .iter()
                .for_each(|&state| or_piece.add_state(state));
            fa_piece2
                .states()
                .iter()
                .for_each(|&state| or_piece.add_state(state));
            fa_piece1
                .delta()
                .iter()
                .for_each(|&trans| or_piece.add_transition(trans));
            fa_piece2
                .delta()
                .iter()
                .for_each(|&trans| or_piece.add_transition(trans));

            or_piece.add_transition(Transition::from(Symbol::Empty, start, fa_piece1.start()));
            or_piece.add_transition(Transition::from(Symbol::Empty, start, fa_piece2.start()));
            or_piece.add_transition(Transition::from(Symbol::Empty, fa_piece1.end(), end));
            or_piece.add_transition(Transition::from(Symbol::Empty, fa_piece2.end(), end));

            or_piece
        }
        Expr::And(e1, e2) => {
            let fa_piece1 = parse(*e1);
            let mut fa_piece2 = parse(*e2);

            let mut and_piece = FAPiece::new_with_start_end(fa_piece1.start(), fa_piece2.end());

            // Connect the two pieces, removing old states and adding new states and adjusting transiitons.
            let oldstart = fa_piece2.start();
            let newstart = fa_piece1.end();

            fa_piece2.remove_state(fa_piece2.start());
            fa_piece2.add_state(fa_piece1.end());
            fa_piece2.set_start(fa_piece1.end());

            // Change all the transitions so that they use the new starting state.
            fa_piece2.delta_mut().iter_mut().for_each(|trans| {
                if trans.start() == oldstart {
                    trans.set_start(newstart);
                } else if trans.end() == oldstart {
                    trans.set_end(newstart)
                }
            });

            fa_piece1
                .states()
                .iter()
                .for_each(|&state| and_piece.add_state(state));
            fa_piece2
                .states()
                .iter()
                .for_each(|&state| and_piece.add_state(state));
            fa_piece1
                .delta()
                .iter()
                .for_each(|&trans| and_piece.add_transition(trans));
            fa_piece2
                .delta()
                .iter()
                .for_each(|&trans| and_piece.add_transition(trans));

            and_piece
        }
        Expr::Star(e) => {
            let fa_piece = parse(*e);

            let start = FAPiece::produce_id();
            let end = FAPiece::produce_id();

            let mut star_piece = FAPiece::new_with_start_end(start, end);

            fa_piece
                .states()
                .iter()
                .for_each(|&state| star_piece.add_state(state));
            fa_piece
                .delta()
                .iter()
                .for_each(|&trans| star_piece.add_transition(trans));

            star_piece.add_transition(Transition::from(Symbol::Empty, start, fa_piece.start()));
            star_piece.add_transition(Transition::from(Symbol::Empty, fa_piece.end(), end));
            star_piece.add_transition(Transition::from(Symbol::Empty, start, end));
            star_piece.add_transition(Transition::from(
                Symbol::Empty,
                fa_piece.end(),
                fa_piece.start(),
            ));

            star_piece
        }
        Expr::Plus(e) => {
            let fa_piece = parse(*e);

            let start = FAPiece::produce_id();
            let end = FAPiece::produce_id();

            let mut plus_piece = FAPiece::new_with_start_end(start, end);

            fa_piece
                .states()
                .iter()
                .for_each(|&state| plus_piece.add_state(state));
            fa_piece
                .delta()
                .iter()
                .for_each(|&trans| plus_piece.add_transition(trans));

            plus_piece.add_transition(Transition::from(Symbol::Empty, start, fa_piece.start()));
            plus_piece.add_transition(Transition::from(Symbol::Empty, fa_piece.end(), end));
            // plus_piece.add_transition(Transition::from(Symbol::Empty, start, end));
            plus_piece.add_transition(Transition::from(
                Symbol::Empty,
                fa_piece.end(),
                fa_piece.start(),
            ));

            plus_piece
        }
        Expr::QMark(e) => {
            let fa_piece = parse(*e);

            let start = FAPiece::produce_id();
            let end = FAPiece::produce_id();

            let mut qmark_piece = FAPiece::new_with_start_end(start, end);

            fa_piece
                .states()
                .iter()
                .for_each(|&state| qmark_piece.add_state(state));
            fa_piece
                .delta()
                .iter()
                .for_each(|&trans| qmark_piece.add_transition(trans));

            qmark_piece.add_transition(Transition::from(Symbol::Empty, start, fa_piece.start()));
            qmark_piece.add_transition(Transition::from(Symbol::Empty, fa_piece.end(), end));
            qmark_piece.add_transition(Transition::from(Symbol::Empty, start, end));
            // qmark_piece.add_transition(Transition::from(
            //     Symbol::Empty,
            //     fa_piece.end(),
            //     fa_piece.start(),
            // ));

            qmark_piece
        }
    }
}

fn fapiece_to_fa(construction: FAPiece) -> FA {
    let mut fa = FA::new();

    for state in construction.states().clone().into_iter() {
        fa.add_state(state);
    }
    fa.set_start(construction.start());
    fa.add_acceptor(construction.end());
    for transition in construction.delta() {
        fa.add_transition(*transition);
    }

    fa
}

pub fn parse_to_finite_automata(input: &String) -> Option<FA> {
    let expr = parse_string_to_expr(input)?;
    let fa_piece = parse(expr);
    let fa = fapiece_to_fa(fa_piece);
    Some(fa)
}
