use std::collections::HashMap;
use itertools::Itertools;

mod helper;
use helper::*;


#[derive(Clone)]
pub enum Gate {
    ID(String),
    NOT(Box<Gate>),
    AND(Vec<Gate>),
    OR(Vec<Gate>),
}

impl std::fmt::Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Gate {
    fn to_string(&self) -> String {
        match self {
            Self::ID(id) => id.clone(),
            Self::NOT(gate) => format!("!{}", gate.to_string()),
            Self::AND(gates) => {
                let args: String = gates
                    .iter()
                    .map(|gate| format!(" {}", gate.to_string()))
                    .collect();
                format!("(.{args})") 
            }
            Self::OR(gates) => {
                let args: String = gates
                    .iter()
                    .map(|gate| format!(" {}", gate.to_string()))
                    .collect();
                format!("(+{args})") 
            }
        }
    }

    pub fn eval(&self, ips: &HashMap<&str, bool>) -> bool {
        match self {
            Gate::ID(id) => ips[id.as_str()],
            Gate::NOT(gate) => !gate.eval(ips),
            Gate::AND(gates) => 
                gates.iter().fold(true, |acc, gate| acc & gate.eval(ips)),
            Gate::OR(gates) => 
                gates.iter().fold(false, |acc, gate| acc | gate.eval(ips))
        }
    }

    pub fn find_ids(&self) -> Vec<&str> {
        let mut ids: Vec<&str> = Vec::new();
        match self {
            Gate::ID(id) => ids.push(id.as_str()),
            Gate::NOT(gate) => ids.append(&mut gate.find_ids()),
            Gate::AND(gates) => 
                for gate in gates {
                    ids.append(&mut gate.find_ids());
                }
            Gate::OR(gates) => 
                for gate in gates {
                    ids.append(&mut gate.find_ids());
                }
        }
        ids.sort();
        ids.into_iter().unique().collect()
    }

    pub fn truth_table(&self) {
        let ids = self.find_ids();

        let mut top = String::new();
        for id in ids.iter() { 
            top += format!(" {id} |").as_str(); 
        }
        top += " @\n";
        
        for _ in 0..top.len() { 
            top += "-"
        }
        println!("{top}");

        let num_ips = ids.len();
        let mut vals = vec![false;num_ips];
        let mut ips = HashMap::new();
        
        loop {
            for idx in 0..num_ips { // update hashmap
                ips.insert( ids[idx], vals[idx]);
            }

            for val in vals.iter() { // fill columns
                print!(" {} |", val.as_bit());
            }

            let out = self.eval(&ips);
            print!(" {}\n", out.as_bit()); // add answer

            vals.next_input();
            if vals == vec![false;num_ips] { break; }
        }
    }
}

