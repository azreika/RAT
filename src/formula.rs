use std::fmt;

pub enum Formula<'a> {
    And(Box<Formula<'a>>, Box<Formula<'a>>),
    Or(Box<Formula<'a>>, Box<Formula<'a>>),
    Not(Box<Formula<'a>>),
    Var(&'a str),
}

impl<'a> fmt::Display for Formula<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formula::And(ref left, ref right) => write!(f, "({} ∧ {})", left, right),
            Formula::Not(ref sub) => write!(f, "¬{}", sub),
            Formula::Or(ref left, ref right) => write!(f, "({} ∨ {})", left, right),
            Formula::Var(ref name) => write!(f, "{}", name),
        }
    }
}
