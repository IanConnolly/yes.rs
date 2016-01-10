use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    match args.len() {
        1 => loop { println!("y") },
        _ => loop { println!("{}", args[1]) },
    };
}
