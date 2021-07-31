# lexing-luthor
Rust implementations of lexical analysis operations (through finite automata) as outlined in Aho et al. (Dragon Book) and Cooper/Torczon. (Engineering a Compiler)

Currently, capabilities for specifying finite automata simply (through plaintext files of certain syntax) and converting nondeterministic finite automata to deterministic finite 
automata are implemented.  Upcoming is a simple display for finite automata that adheres to the simple syntax in `parse_fa.rs`, and in the future, a visualization of the automata through
GraphViz.

Eventually, a true regex-to-NFA module will be added, completing the project as a pedagogic tool to accompany my readings of these two compiler books.

## Theory

Finite-automata are similar to state machines.  They are represented like directed graphs, except edges have a special "weight" - a symbol is attached to each edge.

The vertices of this graph are called "states", and the edges are called "transitions". Each automaton has a starting state and at least one accepting state.  

Finite automata are built in accordance to a specification of a regular language.  Regular expressions are the specification - the grammar rules, so to speak -
while the language itself consists of all strings that fit those rules.  
The purpose of the automaton, of course, is to "accept" strings that are correct (part of the language), and to reject strings that are incorrect (not part of the language).

You can work the automaton by starting from the start and reading the input string one character at a time; if you can traverse down a transition with the given character you are at,
then move on to that transition's end state; if there are no transitions and still more input, reject the string.  If you have exhausted all the input characters, and are at an
accepting state, then accept the string.

Nondeterministic finite automata are the default kind; they include "empty" transitions, or transitions with no symbol.  
You can travel down an empty transition with or without a character.
They also can have duplicate transitions beginning at the same state, labelled with the same symbol.  Therein lies the nondeterminism - if you have two transitions with
the same letter 'a', and input gives you the letter 'a', which transition must you travel down?

Deterministic finite automata eliminate these two elements.  As such, they are easier to simulate with programs - 
which is why we have so many tools for creating DFAs out of other equivalent structures.

## Notes on writing

BTreeSet is used because it implements Hash.  This makes it ideal as a representation of a set of states, which is an intermediate form that DFA states must take while being created
from NFA states.
