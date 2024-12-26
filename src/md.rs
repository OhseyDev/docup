pub mod block;
pub mod text;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyDocument,
    EmptyContent,
    UnexpectedChar(char),
    UnexpectedString(String),
    UnexpectedEnd,
    InvalidUrl(url::ParseError),
    IncompleteBuilderData,
}

pub enum BuildError {
    IncompleteData,
}

pub trait Builder: Default {
    type Output;
    fn build(self) -> Result<Self::Output, BuildError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocItem {}

pub struct Document(Box<[DocItem]>);
pub struct DocumentBuilder {
    items: Vec<DocItem>,
}
