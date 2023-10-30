mod gate;
use gate::*;

fn main() {
    let a = Gate::ID("A".to_string());
    let b = Gate::ID("B".to_string());
    let c = Gate::ID("C".to_string());

    let expr = Gate::OR( vec![ a.clone(), Gate::NOT(Box::new(c)), Gate::AND( vec![ a.clone(), b]) ]);

    println!("{}", expr);
}

