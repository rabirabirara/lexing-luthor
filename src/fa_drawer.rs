// Converts finite automata into dot language files.

use crate::symbol::Symbol;
use std::fmt::Write;

pub fn draw_fa(fa: crate::fa::FA) -> Result<String, Box<dyn std::error::Error>> {
    let mut output = String::new();

    writeln!(output, "digraph {{")?;
    writeln!(output, "rankdir=LR")?;

    let mut accepting = fa.accepting().clone();
    accepting.sort();
    let mut acceptors_string = String::new();
    for a in accepting {
        acceptors_string.push(' ');
        acceptors_string.push(char::from_digit(a as u32, 10).unwrap());
    }

    // The string `acceptors` has a space before it; it's just easy.  Remember that.
    writeln!(output, "node [shape = doublecircle];{};", acceptors_string)?;
    writeln!(output, "node [shape = circle];")?;

    // let mut states = fa.states();
    // states.sort();
    // for state in states {

    // }

    for t in fa.delta() {
        writeln!(output, "{} -> {} [label=\"'{}'\"]", t.start(), t.end(), {
            match t.sym() {
                Symbol::Char(c) => c,
                Symbol::Empty => '\u{03B5}',
            }
        })?;
    }

    writeln!(output, "}}")?;

    Ok(output)
}
