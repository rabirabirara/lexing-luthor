#[allow(dead_code)]

// mod state_set;
mod fa;
mod symbol;
mod fa_reader;
mod fa_drawer;
mod regex_parser;
mod thompsons;

use std::io::Write;
use std::ffi::OsString;
use pico_args::Arguments;

const USAGE: &'static str = "
Run and stuff.
";


struct Args {
    help: bool,
    inputs: Vec<OsString>,
}

fn parse_args(mut args: Arguments) -> Result<Args, Box<dyn std::error::Error>> {
    Ok(
        Args {
            help: args.contains(["-h", "--help"]),
            inputs: args.finish(),
        }
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let args = parse_args(Arguments::from_env())?;

    // if args.help {
    //     println!("{}", USAGE);
    //     return Ok(());
    // }

    // if args.inputs.len() != 1 {
    //     eprintln!("Please input exactly one argument for the input file.");
    //     return Ok(());
    // }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    let regex = input;
    let output = regex_parser::add_concatenation(regex);
    println!("{}", output);

    let post = regex_parser::to_postfix(output);
    println!("{}", post);

    let fa = thompsons::parse_to_finite_automata(post).unwrap();
    println!("{}", fa);

    let dfa = fa.dfa_from();
    println!("{}", dfa);

    let dotfile = fa_drawer::draw_fa(dfa)?;

    let mut file = std::fs::File::create("new.gv")?;
    file.write_all(dotfile.as_bytes())?;
    Ok(())
}
