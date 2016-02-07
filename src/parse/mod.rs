pub use self::parsetree::ParseTree;
pub use self::parsetree::Data;
pub use self::parsetree::Statement;
pub use self::parsetree::ConditionalStatement;
pub use self::parsetree::Position;


mod parsetree;

#[derive(Copy,Clone)]
pub enum SingleOperators
{
    Add
}
#[derive(Copy,Clone)]
pub enum ConditionalOperators
{
    Equal
}
#[derive(Copy,Clone)]
pub enum VecOperators
{
    Add
}
#[derive(Copy,Clone)]
pub enum MapOperators
{
    Add
}
#[derive(Copy,Clone)]
pub enum ReduceOperators
{
    Add
}

pub fn generate_new_parsetree()
{

}
