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

impl fmt::Debug for Formula {
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

fn gen_var(idx: usize) -> String {
    format!("@var{}", idx)
}

impl Formula {
    pub fn get_cnf(&self) -> Formula {
        Formula::Var(gen_var(0))
        // Formula::Var(format!("x:{}", self.count_nontrivial_subformulas()))
    }

    pub fn reduce(&self, idx: usize) -> Vec<(String, Formula)> {
        let mut assignments: Vec<(String, Formula)> = Vec::new();
        match self {
            Formula::Not(ref sub) => {
                let mut reduced = sub.reduce(idx+1);
                let subvariable = Formula::Var(reduced[0].0.clone());
                assignments.push((
                        gen_var(idx),
                        Formula::Not(Box::new(subvariable))));
                assignments.append(&mut reduced);
            },
            Formula::Var(ref name) => {
                assignments.push((
                        name.clone(),
                        Formula::Var(name.clone())));
            },
            Formula::Or(ref left, ref right) => {
                let mut left_reduced = left.reduce(idx+1);
                let left_var = Formula::Var(left_reduced[0].0.clone());

                let mut right_reduced = right.reduce(idx+left_reduced.len()+1);
                let right_var = Formula::Var(right_reduced[0].0.clone());

                assignments.push((
                        gen_var(idx),
                        Formula::Or(Box::new(left_var), Box::new(right_var))));
                assignments.append(&mut left_reduced);
                assignments.append(&mut right_reduced);
            },
            Formula::And(ref left, ref right) => {
                let mut left_reduced = left.reduce(idx+1);
                let left_var = Formula::Var(left_reduced[0].0.clone());

                let mut right_reduced = right.reduce(idx+left_reduced.len()+1);
                let right_var = Formula::Var(right_reduced[0].0.clone());

                assignments.push((
                        gen_var(idx),
                        Formula::And(Box::new(left_var), Box::new(right_var))));
                assignments.append(&mut left_reduced);
                assignments.append(&mut right_reduced);
            },
            Formula::Constant(ref val) => {
                assignments.push((
                        gen_var(idx),
                        Formula::Constant(*val)));
            },
        };
        assignments
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
