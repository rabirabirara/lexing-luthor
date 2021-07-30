#[allow(dead_code)]

// mod state_set;
mod fa;
mod symbol;
mod parse_fa;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fa = parse_fa::build_fa("D:\\opus\\rupo\\finite-automata\\test.fa".to_string())?;

    println!("{:?}", fa);
    // println!("{}", fa.dfa_accepts("ab".to_string()));

    for id in 0..fa.num_states() {
        println!("{:?} -- {:?}", id, fa.epsilon_closure(id));
    }

    fa.test();

    Ok(())

}
