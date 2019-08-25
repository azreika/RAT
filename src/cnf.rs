use std::fmt;

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
