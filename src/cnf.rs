use std::fmt;

/// Represents the overall conjunction in a CNF.
/// Composed of a set of disjunctions.
#[derive(Clone)]
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

    pub fn get_disjunctions(&self) -> &Vec<Disjunction> {
        &self.disjunctions
    }
}

impl fmt::Display for Conjunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("(")?;
        let mut delim = "";
        for disjunction in self.disjunctions.iter() {
            f.write_str(delim)?;
            write!(f, "{}", disjunction)?;
            delim = " ∧ ";
        }
        f.write_str(")")?;
        Ok(())
    }
}

/// Represents the inner disjunction-sets in a CNF.
/// Composed of a set of literals.
#[derive(Clone)]
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

    pub fn get_literals(&self) -> &Vec<Literal> {
        &self.literals
    }
}

impl fmt::Display for Disjunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("(")?;
        let mut delim = "";
        for literal in self.literals.iter() {
            f.write_str(delim)?;
            write!(f, "{}", literal)?;
            delim = " ∨ ";
        }
        f.write_str(")")?;
        Ok(())
    }
}

/// Represents the literals in a disjunction within a CNF.
/// Can be either a variable or its negation.
#[derive(Clone)]
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

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_negated(&self) -> bool {
        self.negated
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.negated {
            f.write_str("¬")?;
        }
        write!(f, "{}", self.name)
    }
}
