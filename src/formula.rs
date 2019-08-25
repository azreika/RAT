use std::fmt;

pub enum Formula {
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Not(Box<Formula>),
    Var(String),
    Constant(bool),
}

pub struct Conjunction {
    disjunctions: Vec<Disjunction>,
}

impl Conjunction {
    pub fn new() -> Conjunction {
        Conjunction {
            disjunctions: Vec::new(),
        }
    }

    pub fn add_disjunction(&mut self, disjunction: Disjunction) {
        self.disjunctions.push(disjunction);
    }
}

pub struct Disjunction {
    literals: Vec<Literal>,
}

impl Disjunction {
    pub fn new() -> Disjunction {
        Disjunction {
            literals: Vec::new(),
        }
    }

    pub fn add_literal(&mut self, literal: Literal) {
        self.literals.push(literal);
    }
}

pub struct Literal {
    name: String,
    negated: bool,
}

impl Literal {
    pub fn new(name: String, negated: bool) -> Literal {
        Literal {
            name: name,
            negated: negated,
        }
    }
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

impl fmt::Debug for Conjunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("(")?;
        let mut delim = "";
        for disjunction in self.disjunctions.iter() {
            f.write_str(delim)?;
            write!(f, "{:?}", disjunction)?;
            delim = " ∧ ";
        }
        f.write_str(")")?;
        Ok(())
    }
}

impl fmt::Debug for Disjunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("(")?;
        let mut delim = "";
        for literal in self.literals.iter() {
            f.write_str(delim)?;
            write!(f, "{:?}", literal)?;
            delim = " ∨ ";
        }
        f.write_str(")")?;
        Ok(())
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.negated {
            f.write_str("¬")?;
        }
        write!(f, "{}", self.name)
    }
}

fn gen_var(idx: usize) -> String {
    format!("@var{}", idx)
}

impl Formula {
    pub fn get_cnf(&self) -> Conjunction {
        let mut conj = Conjunction::new();

        let subformulas = self.reduce(0);
        for formula in subformulas.into_iter() {
            match formula {
                (ref var_name, Formula::Not(ref sub)) => {
                    let mut disj = Disjunction::new();
                    disj.add_literal(Literal::new("NOT-.".to_string(), true));
                    conj.add_disjunction(disj);
                },
                (ref var_name, Formula::And(ref left, ref right)) => {
                    let mut disj = Disjunction::new();
                    disj.add_literal(Literal::new("AND-.".to_string(), false));
                    conj.add_disjunction(disj);
                },
                (ref var_name, Formula::Or(ref left, ref right)) => {
                    let mut disj = Disjunction::new();
                    disj.add_literal(Literal::new("OR-.".to_string(), false));
                    conj.add_disjunction(disj);
                },
                (ref var_name, Formula::Constant(ref val)) => {
                    let mut disj = Disjunction::new();
                    disj.add_literal(Literal::new("CONST-.".to_string(), false));
                    conj.add_disjunction(disj);
                },
                (ref var_name, Formula::Var(ref name)) => {
                    let mut disj = Disjunction::new();
                    disj.add_literal(Literal::new("NAME-.".to_string(), false));
                    conj.add_disjunction(disj);
                }
            }
        }
        conj
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
