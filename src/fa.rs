use crate::symbol::{Symbol, ASCII};
use crate::fa_reader;

// * Look into using GraphViz to visualize the finite automata, with the 'dot' crate.

use std::collections::HashMap;
use std::iter::FromIterator;


type State = usize;
// A state set should be hashable, so that it can be used as the key to a HashSet or HashMap in subset construction.  Basically, DFA states must be temporarily represented sets with multiple elements; we want to be able to hash them as easily as with NFA states.
type StateSet<T> = std::collections::BTreeSet<T>;

// The finite automata holds both the mathematical tuple representation and the graph representation, which is really just a table of beginnings of transitions to full transitions.
#[derive(Debug, Clone)]
pub struct FA {
    states: Vec<State>,
    starting: State,
    accepting: Vec<State>,
    delta: Vec<Transition>,
    // * Graph invariant: the state used as key value is the beginning state of its transitions.
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
    pub fn set_start(&mut self, s: State) {
        self.starting = s;
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
    pub fn is_accepting(&self, id: State) -> bool {
        self.accepting.contains(&id)
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
    pub fn epsilon_closure(&self, st: State) -> StateSet<State> {
        // Start with the current node.  A state is always in its own epsilon closure.
        let mut closure = StateSet::new();
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
        closure
        // Vec::from_iter(closure.into_iter())
    }
    // Compute the epsilon-closure of each state in T; return the union of these closures.
    pub fn epsilon_closure_set(&self, t: StateSet<State>) -> StateSet<State> {
        let mut res = StateSet::new();
        
        for states in t.iter().map(|&st| self.epsilon_closure(st)) {
            for state in states {
                res.insert(state);
            }
        }
        
        res
        // Vec::from_iter(res.into_iter())
    }
    // Compute the union of states that can be moved to through the symbol c, from all the states in T.
    pub fn delta_move(&self, t: &StateSet<State>, c: Symbol) -> Option<StateSet<State>> {
        let mut res = StateSet::new();
        
        for state in t {
            if let Some(states) = self.graph.get(&state) {
                states.iter().filter(|&t| t.sym == c).map(|t| t.end).for_each(|s| {
                    res.insert(s);
                });
            }
        }
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }
    // Rather than alter in place, this generates a new finite automata altogether.
    fn subset_construction(&self) -> (HashMap<StateSet<State>, Vec<SetTransition<State>>>, StateSet<State>) {
        let mut dfa: HashMap<StateSet<State>, Vec<SetTransition<State>>> = HashMap::new();
        let mut todo: Vec<StateSet<State>> = Vec::new();

        // First DFA state: ε-closure of first NFA state.
        let q0 = self.epsilon_closure(self.starting);
        dfa.insert(q0.clone(), Vec::new());
        todo.push(q0.clone());

        while let Some(state_set) = todo.pop() {
            for c in ASCII {
                let sym = Symbol::Char(c);
                if let Some(m) = self.delta_move(&state_set, sym) {
                    let u = self.epsilon_closure_set(m);
                    if let Some(v) = dfa.get_mut(&u) {
                        v.push(SetTransition::from(sym, state_set.clone(), u.clone()));
                    } else {
                        // This is the missing link; check if something is in the map and then add to its vec.  Remember, you always want to update the transition table, whether or not the new states were already found.
                        if let Some(v) = dfa.get_mut(&state_set) {
                            v.push(SetTransition::from(sym, state_set.clone(), u.clone()));
                        }
                        dfa.insert(u.clone(), Vec::new());
                        todo.push(u);
                    }

                }
                // else, just continue to the next symbol
            }
        }
        (dfa, q0)
    }
    pub fn dfa_from(&self) -> Self {
        let (dfa, start) = self.subset_construction();

        let mut fa = FA::new();

        let mut map = HashMap::new();
        let mut acceptors = Vec::new();
        let mut i = 1usize;
        
        for state in dfa.keys() {
            // If any of the NFA states in this DFA state are accepting, the resulting DFA state is accepting.
            if self.accepting.iter().any(|st| state.contains(st)) {
                acceptors.push(i);
                fa.add_acceptor(i);
            }
            // If this is the epsilon-closure of the starting NFA state, then this DFA state is accepting.
            if *state == start {
                map.insert(state, 0);
                fa.add_state(0);
            } else {
                map.insert(state, i);
                fa.add_state(i);
                i += 1;    
            }
        }

        for ts in dfa.values() {
            for t in ts {
                let begin = map.get(&t.begin).unwrap();
                let end = map.get(&t.end).unwrap();
                fa.add_transition(
                    Transition::from(t.sym, *begin, *end)
                );
            }
        }

        fa.set_start(0);

        fa
    }
    // pub fn test(&self) {
    //     let dfa = self.subset_construction();
    //     for (stateset, transits) in dfa {
    //         for t in transits {
    //             println!("{:?}: {:?} -- {:?} -> {:?}", stateset, t.begin, t.sym, t.end);
    //         }
    //     }
    // }
}

impl std::fmt::Display for FA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Write;
        let mut v = Vec::from_iter(self.graph.iter());
        v.sort_by(|a, b| (a.0).partial_cmp(b.0).unwrap());

        let mut output = String::new();
        let mut state_count = 0;
        let mut transition_count = 0;

        for (state, transitions) in v {
            state_count += 1;
            writeln!(output, "{} {} {}", state, {
                if self.accepting.contains(state) { fa_reader::ACCEPT_SYMBOL } else { fa_reader::STATE_SYMBOL }
            }, transitions.len())?;

            for t in transitions {
                transition_count += 1;
                writeln!(output, "{} -> {}", {
                    match t.sym {
                        Symbol::Char(c) => c,
                        Symbol::Empty => ' ',
                    }
                }, t.end)?;
            }

            writeln!(output, "")?;
        }

        writeln!(output, "// {} states, {} transitions", state_count, transition_count)?;
        
        write!(f, "{}", output)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SetTransition<State> {
    sym: Symbol,
    begin: StateSet<State>,
    end: StateSet<State>,
}

impl SetTransition<State> {
    pub fn from(sym: Symbol, begin: StateSet<State>, end: StateSet<State>) -> Self {
        Self {
            sym,
            begin,
            end
        }
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