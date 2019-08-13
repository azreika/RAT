use std::fmt;

pub enum Formula {
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Not(Box<Formula>),
    Var(String),
    Constant(bool),
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formula::And(ref left, ref right) => write!(f, "({} ∧ {})", left, right),
            Formula::Not(ref sub) => write!(f, "¬{}", sub),
            Formula::Or(ref left, ref right) => write!(f, "({} ∨ {})", left, right),
            Formula::Var(ref name) => write!(f, "{}", name),
            Formula::Constant(ref val) => match val {
                true => write!(f, "⊤"),
                false => write!(f, "⊥"),
            },
        }
    }
}

impl Formula {
    pub fn get_cnf(&self) -> Formula {
        Formula::Var(format!("x:{}", self.count_nontrivial_subformulas()))
    }

    pub fn count_nontrivial_subformulas(&self) -> i32 {
        match self {
            Formula::And(ref left, ref right) =>
                left.count_nontrivial_subformulas() +
                right.count_nontrivial_subformulas() +
                1,
            Formula::Or(ref left, ref right) =>
                left.count_nontrivial_subformulas() +
                right.count_nontrivial_subformulas() +
                1,
            Formula::Not(ref sub) =>
                sub.count_nontrivial_subformulas() + 1,
            Formula::Var(_) => 0,
            Formula::Constant(_) => 0,
        }
    }
}
