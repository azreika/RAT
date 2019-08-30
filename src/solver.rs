use crate::cnf;
use crate::formula;
use std::collections::HashMap;

/// A CNF formula solver.
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

    pub fn get_assignments(&self) -> &HashMap<String, bool> {
        return &self.assignments;
    }

    /// Checks if the formula is satisfiable.
    /// If it is, a sufficient set of assignments will be determined.
    pub fn is_satisfiable(&mut self) -> bool {
        let simpl = self.formula.simplify(&self.assignments);
        self.run_dpll(&simpl)
    }

    /// Runs the actual sat-check algorithm.
    fn run_dpll(&mut self, formula: &cnf::Conjunction) -> bool {
        // Simplify the formula based on the current set of assignments
        let simpl = formula.simplify(&self.assignments);

        if simpl.is_trivially_true() {
            true
        } else if simpl.is_trivially_false() {
            false
        } else {
            // Not trivially determined yet, so try out a new variable assignment
            let (variable, value) = Solver::choose_variable(&simpl);
            self.set_variable(variable.clone(), value);

            if self.run_dpll(&simpl) {
                // Works out! Done.
                true
            } else {
                // Did not work out, so try the opposite assignment
                self.set_variable(variable.clone(), !value);
                self.run_dpll(&simpl)
            }
        }
    }

    /// Set the value of a variable in the solver.
    fn set_variable(&mut self, name: String, value: bool) {
        self.assignments.insert(name, value);
    }

    /// Choose an unassigned variable to assign next.
    fn choose_variable(formula: &cnf::Conjunction) -> (String, bool) {
        // Just choose the first unassigned literal
        for disj in formula.get_disjunctions() {
            for literal in disj.get_literals() {
                return (literal.get_name().clone(), !literal.is_negated());
            }
        }
        panic!("Impossible case!");
    }
}
