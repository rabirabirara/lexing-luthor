#[allow(dead_code)]

// mod state_set;
mod fa;
mod symbol;
mod fa_reader;
mod fa_drawer;

use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fa = fa_reader::from_file("D:\\opus\\rupo\\lexing-luthor\\test.fa".to_string())?;

    println!("{}", fa);

    let dfa = fa.dfa_from();

    println!("{}", dfa);

    let dotfile = fa_drawer::draw_fa(dfa)?;

    let mut file = std::fs::File::create("new.gv")?;
    file.write_all(dotfile.as_bytes())?;
    Ok(())
}
