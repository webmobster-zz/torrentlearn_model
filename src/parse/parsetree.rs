use super::SingleOperators;
use super::ConditionalOperators;
use super::AllOperators;
use std::mem;
use rand::Rng;

// As each compiled operator can have 2 exit values that can only be rpresented by one ConditionalStatement
// split so therefore for simplicites sake I have defined a compiled operator as a list of SingleOperators
// followed by one conditonal ones
#[derive(Clone)]
pub enum ParseTree {
    Continuation(Box<ParseTree>, Statement),
    EndSingle(Statement),
    EndConditional(ConditionalStatement),
}

impl ParseTree {
    pub fn get_sucessors(&self) -> u8 {
        match self {
            &ParseTree::EndSingle(_) => 1,
            &ParseTree::EndConditional(_) => 2,
            &ParseTree::Continuation(ref parsetree, _) => parsetree.get_sucessors(),
        }
    }
    // TODO: Check logic/unit test
    pub fn size(&self) -> u64 {
        match self {
            &ParseTree::EndSingle(_) => 1,
            &ParseTree::EndConditional(_) => 1,
            &ParseTree::Continuation(ref parsetree, _) => 1 + parsetree.size(),
        }
    }
    pub fn generate_parse_tree<T: Rng>(generated_operator: AllOperators, mut rng: &mut T) -> ParseTree {
        match generated_operator {
            AllOperators::Single(op) => ParseTree::EndSingle(Statement::SingleStatement(op,Position::random(&mut rng),Data::random(&mut rng))),
            AllOperators::Conditional(op) => ParseTree::EndConditional(ConditionalStatement(op,Position::random(&mut rng),Data::random(&mut rng))),
            AllOperators::Special(_) => panic!("This shouldn't be here")
        }
    }
    // TODO: Check logic/unit test and rework as its kinda ugly
    pub fn split_off(&mut self, position: usize) -> Option<ParseTree> {

        if position > 0 {
            match *self {
                ParseTree::Continuation(ref mut parsetree, _) => parsetree.split_off(position - 1),
                ParseTree::EndConditional(_) |  ParseTree::EndSingle(_) => None,
            }
        } else {
            match self.clone() {
                ParseTree::Continuation(_, statement) => {
                    let mut new = ParseTree::EndSingle(statement);
                    {
                        let new_mut = &mut new;
                        mem::swap(self, new_mut);
                    }
                    match new {
                        ParseTree::Continuation(parsetree, _) => Some(*parsetree),
                        ParseTree::EndConditional(_) |  ParseTree::EndSingle(_) => panic!("Invalid logic in method"),
                    }
                }
                ParseTree::EndConditional(_) |  ParseTree::EndSingle(_) => None,
            }
        }
    }
    pub fn append(&mut self, tree: ParseTree) {
        match self.clone() {
            ParseTree::EndSingle(end_statement) => {
                *self = ParseTree::Continuation(Box::new(tree), end_statement);
            }
            ParseTree::EndConditional(_) => panic!("Can't attach tree to conditional"),
            ParseTree::Continuation(ref mut parsetree, _) => parsetree.append(tree),
        }
    }
    pub fn calculate_cost(&self,
                          base: fn(&AllOperators) -> u64,
                          combination: fn(u64, u64) -> u64)
                          -> u64 {
        combination(self.recurse(base), self.size())
    }
    fn recurse(&self, base: fn(&AllOperators) -> u64) -> u64 {
        match self {
            &ParseTree::EndSingle(ref statement) => base(&statement.operator()),
            &ParseTree::EndConditional(ref statement) => base(&statement.operator()),
            &ParseTree::Continuation(ref parsetree, ref statement) => {
                base(&statement.operator()) + parsetree.recurse(base)
            }
        }
    }
}

// dest, source
#[derive(Clone)]
pub struct ConditionalStatement(pub ConditionalOperators, pub Position, pub Data);
impl ConditionalStatement {
    pub fn operator(&self) -> AllOperators {
        let ConditionalStatement(operator, _, _) = *self;
        AllOperators::Conditional(operator)
    }
}

#[derive(Clone)]
pub enum Statement {
    // dest, source
    SingleStatement(SingleOperators, Position, Data),
    // dest low, source low, length both
    //VecStatement(VecOperators, Position, Position, Position)
    /*
    // dest low, length dest, source
    MapStatement(MapOperators, Position, Position, Data),
    // dest, source low, source length
    ReduceStatement(ReduceOperators, Position, Position, Position),*/
}
impl Statement {
    pub fn operator(&self) -> AllOperators {
        match self {
            &Statement::SingleStatement(operator, _, _) => AllOperators::Single(operator),
        }
    }
}
#[derive(Clone)]
pub enum Data {
    Val(u64),
    Pos(Position),
}
impl Data {
    pub fn random<T: Rng>(mut rng: T) -> Data {
        if rng.gen() {
            Data::Val(rng.gen())
        } else {
            Data::Pos(Position::random(rng))
        }
    }
}
#[derive(Clone)]
pub enum Position {
    EndPos(u64),
    // Is this the first, second, etc argument
    ContPos(Box<Position>),
}
impl Position {
    pub fn random<T: Rng>(mut rng: T) -> Position {
        if rng.gen() {
            Position::EndPos(rng.gen())
        } else {
            Position::ContPos(Box::new(Position::random(rng)))
        }
    }
}
