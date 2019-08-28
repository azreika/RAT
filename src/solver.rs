use crate::cnf;
use crate::formula;
use std::collections::HashMap;

pub struct Solver {
    formula: cnf::Conjunction,
    assignments: HashMap<String, bool>,
}

impl Solver {
    pub fn new(formula: formula::Formula) -> Solver {
        Solver {
            formula: formula.get_cnf(),
            assignments: HashMap::new(),
        }
    }

    pub fn is_satisfiable(&mut self) -> bool {
        let simpl = self.formula.simplify(&self.assignments);
        self.run_dpll(&simpl)
    }

    pub fn run_dpll(&mut self, formula: &cnf::Conjunction) -> bool {
        let simpl = formula.simplify(&self.assignments);
        if simpl.is_trivially_true() {
            true
        } else if simpl.is_trivially_false() {
            false
        } else {
            let (variable, value) = self.choose_variable(&simpl);
            self.set_variable(variable.clone(), value);
            if self.run_dpll(&simpl) {
                true
            } else {
                self.set_variable(variable.clone(), !value);
                self.run_dpll(&simpl)
            }
        }
    }

    pub fn set_variable(&mut self, name: String, value: bool) {
        self.assignments.insert(name, value);
    }

    pub fn get_assignments(&self) -> &HashMap<String, bool> {
        return &self.assignments;
    }

    pub fn choose_variable(&self, formula: &cnf::Conjunction) -> (String, bool) {
        for disj in formula.get_disjunctions() {
            for literal in disj.get_literals() {
                return (literal.get_name().clone(), !literal.is_negated());
            }
        }
        panic!("impossible!");
    }
}
