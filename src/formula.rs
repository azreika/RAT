pub enum Formula<'a> {
    And(Box<Formula<'a>>, Box<Formula<'a>>),
    Or(Box<Formula<'a>>, Box<Formula<'a>>),
    Not(Box<Formula<'a>>),
    Var(&'a str),
}
