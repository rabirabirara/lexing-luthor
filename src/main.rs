#[allow(dead_code)]

// mod state_set;
mod fa;
mod symbol;
mod fa_reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fa = fa_reader::from_file("D:\\opus\\rupo\\finite-automata\\test.fa".to_string())?;

    println!("{}", fa);

    let dfa = fa.dfa_from();

    println!("{}", dfa);

    // println!("{:?}", fa);
    // println!("{}", fa.dfa_accepts("ab".to_string()));

    // for id in 0..fa.num_states() {
    //     println!("{:?} -- {:?}", id, fa.epsilon_closure(id));
    // }

    // fa.test();

    Ok(())

}
