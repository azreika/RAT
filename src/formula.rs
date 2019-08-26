use std::fmt;
use crate::cnf;

/// Represents a general propositional formula.
/// Used to model any well-formed input formula.
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
        write!(f, "{}", self)
    }
}

/// Generate an internal variable name with the given label.
fn gen_var(idx: usize) -> String {
    format!("x[{}]", idx)
}

impl Formula {
    /// Convert the formula into CNF.
    /// Uses the Tseytin transformation to achieve this.
    pub fn get_cnf(&self) -> cnf::Conjunction {
        let mut conj = cnf::Conjunction::new();

        // Extract the subformulas making up the statement
        let subformulas = self.reduce(0);
        for formula in subformulas.into_iter() {
            match formula {
                // -- Var --
                (var_name, Formula::Var(other_name)) => {
                    // inp: x <-> y
                    // out: (!x ; y) , (x ; !y)
                    if other_name == var_name {
                        // same name, don't add anything
                    } else {
                        let mut disj1 = cnf::Disjunction::new();
                        disj1.add_literal(cnf::Literal::new(var_name.clone(), true));
                        disj1.add_literal(cnf::Literal::new(other_name.clone(), false));
                        conj.add_disjunction(disj1);

                        let mut disj2 = cnf::Disjunction::new();
                        disj2.add_literal(cnf::Literal::new(var_name.clone(), false));
                        disj2.add_literal(cnf::Literal::new(other_name.clone(), true));
                        conj.add_disjunction(disj2);
                    }
                },

                // -- Not --
                (var_name, Formula::Not(sub)) => {
                    // inp: x <-> !y
                    // out: (!x ; !y) , (x ; y)
                    if let Formula::Var(sub_name) = *sub {
                        let mut disj1 = cnf::Disjunction::new();
                        disj1.add_literal(cnf::Literal::new(var_name.clone(), false));
                        disj1.add_literal(cnf::Literal::new(sub_name.clone(), false));
                        conj.add_disjunction(disj1);

                        let mut disj2 = cnf::Disjunction::new();
                        disj2.add_literal(cnf::Literal::new(var_name.clone(), true));
                        disj2.add_literal(cnf::Literal::new(sub_name.clone(), true));
                        conj.add_disjunction(disj2);
                    } else {
                        panic!("oh no!");
                    }
                },

                // -- And --
                (var_name, Formula::And(left, right)) => {
                    // inp: x <-> l ^ r
                    // out: (!x ; l) , (!x ; r) , (x ; !l ; !r)
                    if let Formula::Var(left_name) = *left {
                        if let Formula::Var(right_name) = *right {
                            // (!x ; l)
                            let mut disj1 = cnf::Disjunction::new();
                            disj1.add_literal(cnf::Literal::new(var_name.clone(), true));
                            disj1.add_literal(cnf::Literal::new(left_name.clone(), false));
                            conj.add_disjunction(disj1);

                            // (!x ; r)
                            let mut disj2 = cnf::Disjunction::new();
                            disj2.add_literal(cnf::Literal::new(var_name.clone(), true));
                            disj2.add_literal(cnf::Literal::new(right_name.clone(), false));
                            conj.add_disjunction(disj2);

                            // (x ; !l ; !r)
                            let mut disj3 = cnf::Disjunction::new();
                            disj3.add_literal(cnf::Literal::new(var_name.clone(), false));
                            disj3.add_literal(cnf::Literal::new(left_name.clone(), true));
                            disj3.add_literal(cnf::Literal::new(right_name.clone(), true));
                            conj.add_disjunction(disj3);
                        } else {
                            panic!("oh no!");
                        }
                    } else{
                        panic!("oh no!");
                    }
                },

                // -- Or --
                (var_name, Formula::Or(left, right)) => {
                    // inp: x <-> l v r
                    // out: (x ; !l) , (x ; !r) , (!x ; l ; r)
                    if let Formula::Var(left_name) = *left {
                        if let Formula::Var(right_name) = *right {
                            let mut disj1 = cnf::Disjunction::new();
                            disj1.add_literal(cnf::Literal::new(var_name.clone(), false));
                            disj1.add_literal(cnf::Literal::new(left_name.clone(), true));
                            conj.add_disjunction(disj1);

                            let mut disj2 = cnf::Disjunction::new();
                            disj2.add_literal(cnf::Literal::new(var_name.clone(), false));
                            disj2.add_literal(cnf::Literal::new(right_name.clone(), true));
                            conj.add_disjunction(disj2);

                            let mut disj3 = cnf::Disjunction::new();
                            disj3.add_literal(cnf::Literal::new(var_name.clone(), true));
                            disj3.add_literal(cnf::Literal::new(left_name.clone(), false));
                            disj3.add_literal(cnf::Literal::new(right_name.clone(), false));
                            conj.add_disjunction(disj3);
                        } else {
                            panic!("oh no!");
                        }
                    } else{
                        panic!("oh no!");
                    }
                },

                // -- Constant --
                (var_name, Formula::Constant(val)) => {
                    // inp: x <-> T (or x <-> F)
                    // out: x (or !x)
                    let mut disj = cnf::Disjunction::new();
                    disj.add_literal(cnf::Literal::new(var_name, val));
                    conj.add_disjunction(disj);
                },
            }
        }

        // Final state of conjunction is the original formula in CNF
        conj
    }

    /// Produce a unique label for each subformula in the formula.
    /// The subformula tied to each label is given in its reduced form.
    ///
    /// The first label of the returned vector will always be equivalent
    /// to the original formula.
    ///
    /// E.g.: "!((p ^ q) v r)" returns a vector equivalent to
    ///       [(x0,!x1),(x1,x2 v x3), (x2,x4 ^ x5), (x3,r), (x4,p), (x5,q)]
    fn reduce(&self, idx: usize) -> Vec<(String, Formula)> {
        let mut assignments: Vec<(String, Formula)> = Vec::new();
        match self {
            // -- Var --
            Formula::Var(ref name) => {
                assignments.push((
                        name.clone(),
                        Formula::Var(name.clone())));
            },

            // -- Not --
            Formula::Not(ref sub) => {
                let mut reduced = sub.reduce(idx+1);
                let subvariable = Formula::Var(reduced[0].0.clone());
                assignments.push((
                        gen_var(idx),
                        Formula::Not(Box::new(subvariable))));
                assignments.append(&mut reduced);
            },

            // -- And --
            Formula::And(ref left, ref right) => {
                // Reduce the left side
                let mut left_reduced = left.reduce(idx+1);
                let left_var = Formula::Var(left_reduced[0].0.clone());

                // Reduce the right side
                let mut right_reduced = right.reduce(idx+left_reduced.len()+1);
                let right_var = Formula::Var(right_reduced[0].0.clone());

                // First label is for the full formula
                assignments.push((
                        gen_var(idx),
                        Formula::And(Box::new(left_var), Box::new(right_var))));

                // Add in the left and right half labels
                assignments.append(&mut left_reduced);
                assignments.append(&mut right_reduced);
            },

            // -- Or --
            Formula::Or(ref left, ref right) => {
                // Reduce the left side
                let mut left_reduced = left.reduce(idx+1);
                let left_var = Formula::Var(left_reduced[0].0.clone());

                // Reduce the right side
                let mut right_reduced = right.reduce(idx+left_reduced.len()+1);
                let right_var = Formula::Var(right_reduced[0].0.clone());

                // First label is for the full formula
                assignments.push((
                        gen_var(idx),
                        Formula::Or(Box::new(left_var), Box::new(right_var))));

                // Add in the left and right half labels
                assignments.append(&mut left_reduced);
                assignments.append(&mut right_reduced);
            },

            // -- Constant --
            Formula::Constant(ref val) => {
                assignments.push((
                        gen_var(idx),
                        Formula::Constant(*val)));
            },
        };
        assignments
    }
}
