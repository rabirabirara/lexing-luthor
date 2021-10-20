#[allow(dead_code)]
// mod state_set;
mod fa;
mod fa_drawer;
mod fa_reader;
mod regex_parser;
mod symbol;
mod thompsons;

use clap::{App, Arg};
use std::path::Path;

// TODO: Allow drawing option, implement DFA search functionality, make it an option, output file for FA spec, input an FA spec to receive a matcher and let stdin input be matched.
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
        .get_matches();

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
            let file_path = Path::new(&file);
            let input = std::fs::read_to_string(file_path)?;
            if let Some(fa) = regex_parser::parse_to_dfa(&input) {
                println!("{}", fa);
            } else {
                println!("failed to parse:\n{}", input);
            }
        } else {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim().to_string();

            if let Some(fa) = regex_parser::parse_to_dfa(&input) {
                println!("{}", fa);
            } else {
                println!("failed to parse:\n{}", input);
            }
        }
    }

    // let dotfile = fa_drawer::draw_fa(dfa)?;

    // let mut file = std::fs::File::create("new.gv")?;
    // file.write_all(dotfile.as_bytes())?;
    Ok(())
}
