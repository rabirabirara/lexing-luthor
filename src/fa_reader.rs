// You can specify a finite automata using this syntax.
// statenumber
// symbol -> 

use crate::fa::{FA, Transition};
use crate::symbol::Symbol;

use std::fs::File;
use std::io::{BufReader, BufRead};

pub const STATE_SYMBOL: &'static str = "::";
pub const ACCEPT_SYMBOL: &'static str = "=>";

fn from_iterator(mut lines: impl Iterator<Item = String>) -> Result<FA, Box<dyn std::error::Error>> {
    let mut line_count = 0usize;
    let mut fa = FA::new();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.starts_with("//") || line.is_empty() {
            continue;
        }
        line_count += 1;
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 3 && (parts[1] == STATE_SYMBOL || parts[1] == ACCEPT_SYMBOL) {
            let state = parts[0].parse::<usize>()?;
            let transitions = parts[2].parse::<usize>()?;

            fa.add_state(state);
            if parts[1] == ACCEPT_SYMBOL {
                fa.add_acceptor(state);
            }

            for _ in 0..transitions {
                line_count += 1;
                match lines.next() {
                    Some(transition) => {
                        let parts = transition.split_whitespace().collect::<Vec<&str>>();
                        match parts.len() {
                            2 => {
                                if parts[0] != "->" {
                                    eprintln!("Transition line {} is improperly formed.  Use an arrow.", line_count);
                                }
                                let sym = Symbol::Empty;
                                let begin = state;
                                let end = parts[1].parse::<usize>()?;
                                fa.add_transition(Transition::from(sym, begin, end));
                            },
                            3 => {
                                if parts[1] != "->" {
                                    eprintln!("Transition line {} is improperly formed.  Use an arrow.", line_count);
                                }
                                let sym = match parts[0].chars().next() {
                                    Some(c) => Symbol::Char(c),
                                    None => {
                                        eprintln!("The first portion of the transition at line {} needs to be a single char.", line_count);
                                        panic!()
                                    },
                                };
                                let begin = state;
                                let end = parts[2].parse::<usize>()?;
                                fa.add_transition(Transition::from(sym, begin, end));
                            }
                            _ => eprintln!("Transition line has extraneous parts; correct the formatting at line {} of the file.", line_count),
                        }
                    },
                    None => {
                        eprintln!("The input file has an incorrect transition count at line {} of the file.", line_count);
                    }
                }
            }
        }
    }
    Ok(fa)
}

pub fn from_stdin() -> Result<FA, Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().filter_map(|x| x.ok());
    
    from_iterator(lines)
}

// pub fn from_string(input: &String) -> Result<FA, Box<dyn std::error::Error>> {
//     let stdin = std::io::stdin();
//     let lines = stdin.lock().lines().filter_map(|x| x.ok());
    
//     from_iterator(lines)
// }


pub fn from_file(file_path: &std::path::Path) -> Result<FA, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let file = BufReader::new(file);
    let lines = file.lines().filter_map(|x| x.ok());
    
    from_iterator(lines)
}

// * The impl Display for FA is in fa.rs.