use std::fmt;

pub enum Formula<'a> {
    And(Box<Formula<'a>>, Box<Formula<'a>>),
    Or(Box<Formula<'a>>, Box<Formula<'a>>),
    Not(Box<Formula<'a>>),
    Var(&'a str),
    Constant(bool),
}

impl<'a> fmt::Display for Formula<'a> {
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

impl<'a> Formula<'a> {
    pub fn get_cnf(&self) -> Formula {
        // TODO
        Formula::Constant(false)
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
