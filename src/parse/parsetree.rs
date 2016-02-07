use super::SingleOperators;
use super::VecOperators;
use super::MapOperators;
use super::ReduceOperators;
use super::ConditionalOperators;


//As each compiled operator can have 2 exit values that can only be rpresented by one ConditionalStatement
//split so therefore for simplicites sake I have defined a compiled operator as a list of SingleOperators
// followed by one conditonal ones
pub enum ParseTree
{
    Continuation(Box<ParseTree>,Statement),
    EndSingle(Statement),
    EndConditional(ConditionalStatement)
}

//dest, source
pub struct ConditionalStatement(ConditionalOperators,Position,Data);

pub enum Statement
{
    //dest, source
    SingleStatement(SingleOperators,Position,Data),
    //dest low, source low, length both
    VecStatement(VecOperators,Position,Position,Position),
    //dest low, length dest, source
    MapStatement(MapOperators,Position,Position,Data),
    //dest, source low, source length
    ReduceStatement(ReduceOperators,Position,Data,Data)
}


pub enum Data
{
    Val(u64),
    Pos(Position)
}
pub enum Position
{
    ConstPos(u64),
    //Is this the first, second, etc argument
    VarPos(u8),
}
