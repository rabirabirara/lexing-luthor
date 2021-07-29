use crate::symbol::Symbol;

// * Look into using GraphViz to visualize the finite automata, with the 'dot' crate.

use std::collections::HashMap;
use std::collections::HashSet;


type State = usize;

// The finite automata holds both the mathematical tuple representation and the graph representation, which is really just a table of beginnings of transitions to full transitions.
#[derive(Debug, Clone)]
pub struct FA {
    states: Vec<State>,
    starting: State,
    accepting: Vec<State>,
    delta: Vec<Transition>,
    graph: HashMap<State, Vec<Transition>>
}

impl FA {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            starting: 0,
            accepting: Vec::new(),
            delta: Vec::new(),
            graph: HashMap::new(),
        }
    }
    pub fn num_states(&self) -> usize {
        self.states.len()
    }
    pub fn add_state(&mut self, s: State) {
        self.states.push(s);
        self.graph.insert(s, Vec::new());
    }
    pub fn add_acceptor(&mut self, accept: State) {
        self.accepting.push(accept);
    }
    pub fn add_transition(&mut self, t: Transition) {
        self.delta.push(t);
        if let Some(v) = self.graph.get_mut(&t.begin()) {
            v.push(t);
        } else {
            self.graph.insert(t.begin(), vec![t]);
        }
    }
    // Does not work on NFAs.  DFAs only.
    pub fn dfa_accepts(&self, string: String) -> bool {
        let mut cur = self.starting;

        for c in string.chars() {
            // locate a transition with begin==cur and sym==c
            if let Some(tr) = self.delta.iter().find(|&x| x.begin == cur && x.sym == Symbol::Char(c)) {
                // if found, continue to new state and next char
                cur = tr.end;
            } else {
                // if no such transition, return false (don't follow epsilon closure)
                return false;
            }
        }
        
        return self.accepting.contains(&cur)
    }

    //pub fn subset_construction(&mut self) {
    //}
    // Given a state in the finite automaton, return its epsilon closure.
    // pub fn epsilon_closure(s: State) -> Vec<State> {
    //     // Perform BFS under epsilon-edges.
    // }
    pub fn states(&self) -> Vec<State> {
        self.states.clone()
    }
    pub fn starting(&self) -> State {
        self.starting
    }
    pub fn accepting(&self) -> Vec<State> {
        self.accepting.clone()
    }
    pub fn delta(&self) -> Vec<Transition> {
        self.delta.clone()
    }
    pub fn transitions_of(&self, id: State) -> Option<&Vec<Transition>> {
        // self.delta.clone().into_iter().filter(|trans| trans.begin == id).collect::<Vec<Transition>>()
        self.graph.get(&id)
    }
    pub fn epsilon_closure(&self, st: State) -> Vec<State> {
        // Start with the current node.  A state is always in its own epsilon closure.
        let mut closure = HashSet::new();
        closure.insert(st);
        let mut todo = vec![st];

        while let Some(top) = todo.pop() {
            if let Some(transitions) = self.transitions_of(top) {
                for t in transitions {
                    if t.sym() == Symbol::Empty && t.begin() == top {
                        closure.insert(t.end());
                        todo.push(t.end());
                    }
                }
            }
        }

        use std::iter::FromIterator;
        Vec::from_iter(closure.into_iter())
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Transition {
    sym: Symbol,
    begin: State,
    end: State,
}

impl Transition {
    pub fn from(sym: Symbol, begin: State, end: State) -> Self {
        Self {
            sym,
            begin,
            end
        }
    }
    pub fn sym(&self) -> Symbol {
        self.sym
    }
    pub fn begin(&self) -> State {
        self.begin
    }
    pub fn end(&self) -> State {
        self.end
    }
}
