
#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    path: Box<std::path::Path>,
    content: Box<[DocItem]>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DocItem {
    Heading(Heading),
    Paragraph(Paragraph),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuoteItem {
    Heading(Heading),
    Paragraph(Paragraph),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeadingLevel { Level1,Level2,Level3,Level4,Level5,Level6 }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextToken {
    Def(String),
    Bold(String),
    Italic(String),
    BoldItalic(String),
    NewLine(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Heading {
    level: HeadingLevel,
    content: Box<[TextToken]>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Paragraph(Box<[TextToken]>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quote(Box<[QuoteItem]>);

// impl Paragraph

