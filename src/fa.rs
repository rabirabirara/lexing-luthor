use crate::symbol::{Symbol, ASCII};

// * Look into using GraphViz to visualize the finite automata, with the 'dot' crate.

use std::collections::HashMap;
use std::collections::HashSet;


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
    fn subset_construction(&self) -> (HashMap<StateSet<State>, State>, HashMap<State, Vec<SetTransition<State>>>) {
        let mut dfa_states: HashMap<StateSet<State>, State> = HashMap::new();
        let mut dfa_table: HashMap<State, Vec<SetTransition<State>>> = HashMap::new();

        let mut todo: Vec<StateSet<State>> = Vec::new();
        let mut id = 0usize;

        // First DFA state: ε-closure of first NFA state.
        let q0 = self.epsilon_closure(self.starting);
        dfa_states.insert(q0.clone(), id);
        todo.push(q0);

        // TODO: So much cloning... can I just use RC at this point?  I should be able to put RC in at least todo and maybe dfa_states if I rely on dfa_table instead, and vice versa.

        while let Some(state_set) = todo.pop() {
            for c in ASCII {
                let sym = Symbol::Char(c);
                if let Some(m) = self.delta_move(&state_set, sym) {
                    let u = self.epsilon_closure_set(m);

                    // either state has been visited or not.
                    // if visited, then no new entry to states or todo.  no increment of id. instead, update dfa_table with transition under appropriate index.
                    if let Some(st) = dfa_states.get(&u) {
                        if let Some(v) = dfa_table.get_mut(&st) {
                            v.push(SetTransition::from(sym, state_set.clone(), u));
                        } else {    // if, for some reason, the id isn't already in the hashmap even though it's in the state, something is wrong; crash.
                            // dfa_table.insert(id, vec![SetTransition::from(sym, state_set.clone(), u)]);
                            unreachable!()
                        }
                    } else {    // if not visited.  then must add to states and todo, and dfa_table must gain appropriate entry.  increment id, because a new state just used an id..
                        dfa_states.insert(u.clone(), id);
                        todo.push(u.clone());
                        dfa_table.insert(id, vec![SetTransition::from(sym, state_set.clone(), u)]);
                        id += 1;
                    }
                }
                // else, just continue to the next symbol
            }
        }

        (dfa_states, dfa_table)
    }
    pub fn test(&self) {
        let (states, table) = self.subset_construction();
        println!("{:?}", states);
        for (id, tab) in table {
            for t in tab {
                println!("{}: {:?} -> {:?}", id, t.sym, states.get(&t.end).unwrap());
            }
        }
        // println!("{:?}", table);
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


// TODO: The `begin` field is not needed.  Look into whether removing it is possible, i.e. if anything uses the begin field.
#[derive(Debug, Clone)]
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