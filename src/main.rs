mod gate;
use gate::*;

fn main() {
    let inner = gate!(". !A B");
    println!("{}", inner);

    let gates = gate!("+ A !C {}", inner);

    println!("Circuit: {}", gates);
    println!("Debug:   {:?}", gates);
    gates.truth_table();

}

