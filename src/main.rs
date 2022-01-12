#![allow(dead_code)]
// mod state_set;
mod fa;
mod fa_drawer;
mod fa_reader;
mod regex_parser;
mod symbol;
mod thompsons;
mod transition;

use clap::{App, Arg};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

// TODO: Allow drawing option, implement DFA search functionality, make it an option, output file for FA spec, input an FA spec to receive a matcher and let stdin input be matched.
// TODO: Allow DFA simplification in DFA minimization, where multiple transitions with same begin and end states (but different symbols) are combined into one transition with a list of states, made for easy searching.  Vecs are hash after all and can be put in BTreeSets no problem.  Or maybe use an enum, which is either single symbol or Vec<Symbol>?
//  * Possibly, rewrite Transition to use Vec?
// TODO: Implement support of Kleene Plus and question mark operators.  X+ = X X* ; X? = X | \varepsilon 
//  * Though granted, the Kleene plus is just Kleene star (in Thompson's) without the epsilon transition between start and end.  I'm sure the DFA factors all that out anyway.
//  * And the question mark is just Kleene star without the looping backwards epsilon transition from the end of the inner piece to its start.  This is of course simpler than converting X? into (X|eps).

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("regex-visualizer")
        .author("Spencer G. <swyverng55@g.ucla.edu>")
        .about("Parses and displays regex or an NFA specification into a DFA.")
        .arg(
            Arg::with_name("input-file")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Specify a regex/NFA by file.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("specify")
                .short("s")
                .long("specify")
                .help("Set this to read an NFA specification instead of a regex."),
        )
        .arg(
            Arg::with_name("output-graphviz")
                .short("g")
                .long("graphviz")
                .value_name("OUTPUT")
                .help("Output a .gv file which displays your finite automaton.")
                .takes_value(true),
        )
        .get_matches();

    // * right now, specifying an FA doesn't do anything useful.
    if matches.is_present("specify") {
        if let Some(file) = matches.value_of("input-file") {
            let file_path = Path::new(&file);
            let fa = fa_reader::from_file(file_path)?;

            println!("{}", fa);
        } else {
            // let mut input = String::new();
            // std::io::stdin().read_line(&mut input)?;
            // let input = input.trim().to_string();

            let fa = fa_reader::from_stdin()?;
            println!("{}", fa);
        }
    } else {
        // regex
        if let Some(file) = matches.value_of("input-file") {
            // user enters file with regex
            let file_path = Path::new(&file);
            let input = std::fs::read_to_string(file_path)?;
            if let Some(fa) = regex_parser::parse_to_dfa(&input) {
                println!("{}", fa);

                if let Some(filepath) = matches.value_of("output-graphviz") {
                    let dotfile = fa_drawer::draw_fa(fa)?;
                    let mut file = File::create(filepath)?;
                    file.write_all(dotfile.as_bytes())?;
                }
            } else {
                println!("failed to parse:\n{}", input);
            }
        } else {
            // user enters regex manually
            println!("Enter the regex you want to visualize.");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim().to_string();

            if let Some(fa) = regex_parser::parse_to_dfa(&input) {
                println!("{}", fa);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                let input = input.trim().to_string();
                println!("{}", fa.dfa_accepts(input));

                if let Some(filepath) = matches.value_of("output-graphviz") {
                    let dotfile = fa_drawer::draw_fa(fa)?;
                    let mut file = File::create(filepath)?;
                    file.write_all(dotfile.as_bytes())?;
                }
            } else {
                println!("failed to parse:\n{}", input);
            }
        }
    }

    Ok(())
}
