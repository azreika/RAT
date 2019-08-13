mod solver;
mod formula;

fn main() {
    println!("frontend not implemented");
}

#[cfg(test)]
mod tests {
    use super::formula::Formula::*;

    #[test]
    fn simple_construct() {
        let p = Box::new(Var("p"));
        let q = Box::new(Var("q"));
        let r = Box::new(Var("r"));

        let conj = Box::new(And(p, q));
        let disj = Box::new(Or(conj, r));

        let not = Box::new(Not(disj));
        let false_val = Box::new(Constant(false));
        let res = Box::new(Or(not, false_val));
    }
}
