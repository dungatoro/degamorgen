#[derive(Clone)]
pub enum Gate {
    ID(String),
    NOT(Box<Gate>),
    AND(Vec<Gate>),
    OR(Vec<Gate>),
}

impl Gate {
    fn to_string(&self) -> String {
        match self {
            Self::ID(ident) => ident.clone(),
            Self::NOT(input) => format!("!{}", input.to_string()),
            Self::AND(inputs) => {
                let args: String = inputs
                    .iter()
                    .map(|ip| format!(" {}", ip.to_string()))
                    .collect();
                format!("(.{args})") 
            }
            Self::OR(inputs) => {
                let args: String = inputs
                    .iter()
                    .map(|ip| format!(" {}", ip.to_string()))
                    .collect();
                format!("(+{args})") 
            }
        }

    }
}

impl std::fmt::Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
