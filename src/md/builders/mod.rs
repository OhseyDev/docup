pub mod text;

pub trait Builder {
    type Output;
    fn new() -> impl Builder + Sized;
    fn build(self) -> Result<Self::Output, Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    IncompleteStructure,
}
