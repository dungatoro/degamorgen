#[derive(Clone)]
enum BoolOp {
    NOT,
    AND,
    OR,
    NONE,
}

impl BoolOp {
    fn as_str(&self) -> &str {
        match self {
            BoolOp::NOT  => "!",
            BoolOp::AND  => ".",
            BoolOp::OR   => "+",
            BoolOp::NONE => "",
        }
    }
}

#[derive(Clone)]
struct BoolExpr {
    operator: BoolOp,
    operands: Vec<BoolExpr>,
    id: Option<String>
}

impl BoolExpr {
    fn expand_to_string(&self) -> String {

        if let Some(id) = &self.id {
            format!("{}{}", self.operator.as_str(), id)
        } else {
            let mut expanded = self.operator.as_str().to_string();
            for input in &self.operands {
                expanded = format!("{} {}", expanded, input.expand_to_string());
            }
            format!("({})", expanded)
        }

    }
}

impl std::fmt::Display for BoolExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        write!(f, "{}", self.expand_to_string())

    }
}

fn main() {
    
    let a = BoolExpr {
        operator: BoolOp::NONE,
        operands: vec![],
        id: Some("a".to_string())
    };

    let b = BoolExpr {
        operator: BoolOp::NONE,
        operands: vec![],
        id: Some("b".to_string())
    };

    let c = BoolExpr {
        operator: BoolOp::NOT,
        operands: vec![],
        id: Some("c".to_string())
    };

    let a_and_b = BoolExpr {
        operator: BoolOp::AND,
        operands: vec![a.clone(), b],
        id: None
    };

    let a_or_not_c_or_a_and_b = BoolExpr {
        operator: BoolOp::OR,
        operands: vec![a.clone(), c, a_and_b.clone()],
        id: None
    };

    println!("{}", a_or_not_c_or_a_and_b);

}
