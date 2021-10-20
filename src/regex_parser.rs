use crate::symbol::ASCII;

use phf::phf_map;

static OPERATORS: phf::Map<char, usize> = phf_map! {
    '*' => 10,
    '.' => 5,
    '|' => 0,
};

// More efficient implementations might use PHF - perfect hash function libraries.

// Shunting-yard, and concatenation in regex.

// We want to export this as an object.  Or maybe not.  Yeah, no need.

// Also need to write a regex verifier.
// Verify that the string is all ASCII, then verify that the string is a valid regex (might be done in the parsing stage).

// First, a function that converts a simple regex to a simple regex with concatenation operator '.'
pub fn add_concatenation(regex: &String) -> String {
    let mut output = String::new();
    let len = regex.len();
    let regex = regex
        .as_bytes()
        .iter()
        .map(|&c| c as char)
        .collect::<Vec<char>>();

    for i in 0..len {
        let c = regex[i];
        output.push(c);

        // * Later, add a `if c == '\' to factor in escaped characters.
        if c == '(' || c == '|' {
            continue;
        } else if c == ')' || ASCII.contains(&c) {
            if i + 1 < len {
                // Look ahead a char; if it is a left parentheses or another character add a concatenation.
                match regex[i+1] {
                    ')' | '|' | '*' | '+' | '?' => continue,
                    '(' => output.push('.'),
                    c if ASCII.contains(&c) => output.push('.'),
                    // ! ASCII also contains ')', '|', and such... make note of this in case errors arise.
                    _ => continue,
                }
            }
        }
    }
    output
}

pub fn to_postfix(regex: &String) -> String {
    let mut output = String::new();
    let mut opstack: Vec<char> = Vec::new();

    for c in regex.chars() {
        // println!("{:?}, CH - '{}'", opstack, c);
        match c {
            '(' => opstack.push(c),
            ')' => {
                while let Some(op) = opstack.pop() {
                    // println!("paren: {:?}", opstack);
                    if op == '(' {
                        break;
                    }
                    output.push(op);
                }
            },
            op if OPERATORS.contains_key(&c) => {
                while let Some(&top) = opstack.last() {
                    if top == '(' {
                        opstack.push(op);
                        break;
                    } else {
                        match (OPERATORS.get(&top), OPERATORS.get(&op)) {
                            (Some(&topprec), Some(&opprec)) => {
                                if topprec >= opprec {
                                    opstack.pop();
                                    output.push(top);
                                } else {
                                    opstack.push(op);
                                    break;
                                }
                            },
                            (_, _) => panic!(),
                        }
                    }
                }
                if opstack.is_empty() {
                    opstack.push(op);
                }
            },
            c if ASCII.contains(&c) => {
                output.push(c);
            },
            c => {
                // output.push(c);
                // println!("char: {} :", c);
                println!("invalid: {}", c);
            }
        }
    }
    // Push remaining operations to output, starting from end.
    for &op in opstack.iter().rev() {
        output.push(op);
    }
    output
}

use crate::fa::FA;
use crate::thompsons;

// Now, turn the postfix notation into something wrapped with functions OR, AND, and STAR (repeat).
pub fn parse_to_nfa(input: &String) -> Option<FA> {
    let with_concat = add_concatenation(input);
    let postfix = to_postfix(&with_concat);
    thompsons::parse_to_finite_automata(&postfix)
}

pub fn parse_to_dfa(input: &String) -> Option<FA> {
    if let Some(nfa) = parse_to_nfa(input) {
        Some(nfa.dfa_from())
    } else {
        None
    }
}


// fn produce_

// Given two things, produce thompson's...