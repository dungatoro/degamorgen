use std::collections::HashMap;
use std::iter::zip;
use itertools::Itertools;

mod helper;
use helper::*;

#[derive(Clone, Debug)]
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
        // string with lispy notation
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

    pub fn eval(&self, inputs: &HashMap<&str, bool>) -> bool {
        match self {
            Gate::ID(id) => inputs[id.as_str()],
            Gate::NOT(gate) => !gate.eval(inputs),
            Gate::AND(gates) => gates
                .iter()
                .fold(true, |acc, gate| acc & gate.eval(inputs)),
            Gate::OR(gates) => gates
                .iter()
                .fold(false, |acc, gate| acc | gate.eval(inputs))
        }
    }

    fn find_ids(&self) -> Vec<&str> {
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
        ids = ids.into_iter().unique().collect();
        ids.sort();
        ids
    }

    pub fn truth_table(&self) {
        let ids = self.find_ids();
        // header collected into string so its len can be used to underline
        let mut top = String::new();
        for id in ids.iter() { 
            top += format!(" {id} |").as_str(); 
        }
        top += " @\n";
        
        for _ in 0..top.len() { 
            top += "-"
        }
        println!("{top}");

        let mut values = vec![false;ids.len()];
        let mut inputs = HashMap::new();
        
        while !values.is_empty() {
            for (id, val) in zip(&ids, &values) {
                 inputs.insert( *id, *val);

                 for _ in 0..id.len() -1 { print!(" ") }
                 print!(" {} |", *val as u8);
            }

            let out = self.eval(&inputs);
            print!(" {}\n", out as u8); 

            values.next_input();
        }
    }

    fn push(&mut self, gate: Gate) {
        match self {
            Gate::OR(gates) =>  { gates.push(gate) },
            Gate::AND(gates) => { gates.push(gate) },
            _ => {}
        }
    }

    fn try_not(self, not: &mut bool) -> Gate {
        if *not {
            *not = false;
            Gate::NOT( Box::new(self))
        } else {
            self
        }
    }
        

    pub fn from(expr: String) -> Gate {
        let mut gate = Gate::OR( Vec::new() );
        let mut id = String::new();
        let mut not = false;

        let chars: Vec<char> = expr.chars().collect();
        let mut idx = 0;

        while idx < expr.len() {
            match chars[idx] {
                '(' => {
                    let start = idx;
                    to_block_end(&chars, &mut idx);

                    let block = Gate::from(expr[start+1..=idx].to_string());
                    gate.push( block.try_not(&mut not));
                }
                ')' | ' ' => 
                    if !id.is_empty() {
                        let id_gate = Gate::ID(id);
                        gate.push(id_gate.try_not(&mut not));

                        id = String::new();
                    }
                '+' => gate = Gate::OR( Vec::new() ),
                '.' => gate = Gate::AND( Vec::new() ),
                '!' => not = true,
                _ => id.push( chars[idx]),
            }
            idx += 1;
        }
        if !id.is_empty() {
            let id_gate = Gate::ID(id);
            gate.push(id_gate.try_not(&mut not));
        }
        gate
    }
}

macro_rules! gate {
    ($($arg:tt)*) => {{
        let gates = std::fmt::format(std::format_args!($($arg)*));
        Gate::from(gates)
    }}
}
pub(crate) use gate;
