use std::string::ToString;
use std::str::{Chars, FromStr};
use url::{Url, ParseError as ParseErrorUrl};

pub trait Builder {
    type Output;
    fn new() -> impl Builder + Sized;
    fn build(&self) -> Self::Output;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseError {
    EmptyDocument,
    EmptyContent,
    UnexpectedChar(char),
    UnexpectedEnd,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Code {
    content: String,
    block: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DocumentItem {
    Heading(Heading),
    Paragraph(Paragraph),
    Code(Code),
    Blockquote(String),
    HorizontalRule,
    List(List),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuoteItem {
    Heading(Heading),
    Paragraph(Paragraph),
    List(List),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeadingLevel { Level1,Level2,Level3,Level4,Level5,Level6 }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextToken {
    Bold(Box<str>),
    BoldItalic(Box<str>),
    Def(Box<str>),
    Italic(Box<str>),
    Link(Link),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Heading {
    level: HeadingLevel,
    content: Box<[TextToken]>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    name: Box<str>,
    href: Url,
    img: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct List {
    items: Box<[Box<str>]>,
    len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Paragraph(Box<[TextToken]>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quote(Box<[QuoteItem]>);

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    name: DocumentName,
    content: Box<[DocumentItem]>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DocumentName {
    String(Box<str>),
    Path(Box<std::path::Path>),
}

impl ToString for Document {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for item in self.content.iter() {
            string += &item.to_string();
        }
        return string
    }
}

impl FromStr for Document {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = Vec::<DocumentItem>::new();
        let mut buffer = String::new();
        for item in s.split("\n") {
            let c = item.to_string().pop();
            if c == Some('\\') {
                buffer.push_str(item);
                continue;
            }
        }
        let name = DocumentName::String(s.into());
        let content = Box::<[DocumentItem]>::from(tokens);
        Ok(Document { name, content })
    }
}

impl DocumentItem {
    fn _parse_code(chars: &mut Chars, mut r: u8) -> Result<Self, ParseError> {
        let c = loop {
            let c = if let Some(c) = chars.next() { c } else { break '\0' };
            match c {
                '`' => {
                    r += 1;
                    if r == 3 { break c }
                },
                _ => break c,
            }
        };
        let mut b: u8 = 0;
        let mut content = String::new();
        if c == '\0' { return Err(ParseError::UnexpectedEnd) }
        else if c != '`' { content.push(c) }
        while let Some(c) = chars.next() {
            match c {
                '`' => {
                    b += 1;
                    if b == r { break }
                },
                _ => content.push(c),
            }
        }
        if content.is_empty() { return Err(ParseError::EmptyContent) }
        return Ok(DocumentItem::Code(Code { content, block: b == 3}));
    }
}

impl FromStr for DocumentItem {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            let content = format!("{}{}",c,chars.as_str());
            return match c {
                '#' => {
                    let h = Heading::from_str(&content)?;
                    Ok(DocumentItem::Heading(h))
                }
                ' ' => continue,
                '`' => Self::_parse_code(&mut chars, 1),
                _ => {
                    let p = Paragraph::from_str(&content)?;
                    Ok(DocumentItem::Paragraph(p))
                },
            };
        }
        return Err(ParseError::UnexpectedEnd);
    }
}

impl ToString for DocumentItem {
    fn to_string(&self) -> String {
        return match self {
            Self::Heading(h) => h.to_string(),
            Self::Paragraph(p) => p.to_string(),
            Self::Code(c) => c.to_string(),
            Self::List(l) => l.to_string(),
            Self::HorizontalRule => String::from("___"),
            Self::Blockquote(s) => s.to_string(),
        }
    }
}

impl FromStr for Code {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl ToString for Code {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl HeadingLevel {
    pub fn increment(self) -> Self {
        match self {
            Self::Level1 => Self::Level2,
            Self::Level2 => Self::Level3,
            Self::Level3 => Self::Level4,
            Self::Level4 => Self::Level5,
            Self::Level5 => Self::Level6,
            Self::Level6 => Self::Level6,
        }
    }
    pub fn decrement(self) -> Self {
        match self {
            Self::Level1 => Self::Level1,
            Self::Level2 => Self::Level1,
            Self::Level3 => Self::Level2,
            Self::Level4 => Self::Level3,
            Self::Level5 => Self::Level4,
            Self::Level6 => Self::Level5,
        }
    }
}

impl FromStr for Heading {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl ToString for Heading {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl FromStr for Paragraph {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl ToString for Paragraph {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl FromStr for TextToken {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i: usize = 0;
        let mut chars = s.chars();
        let mut content = String::new();
        let mut last_char: Option<char> = None;
        while let Some(c) = chars.next() {
            match c {
                '*' => { todo!() },
                '[' => if let Some('!') = last_char {
                    todo!()
                } else {
                    todo!()
                },
                ']' => todo!(),
                _ => {
                    if let Some(c) = last_char {
                        content.push(c);
                    }
                    last_char = Some(c);
                }
            }
            i += 1;
        }
        Err(ParseError::UnexpectedEnd)
    }
}

impl ToString for TextToken {
    fn to_string(&self) -> String {
        return match self {
            Self::Def(s) => s.to_string(),
            Self::Italic(s) => format!("*{}*", s),
            Self::Bold(s) => format!("**{}**", s),
            Self::BoldItalic(s) => format!("***{}***", s),
            Self::Link(l) => format!("{}[{}]({})", if l.img { "!" } else { "" }, l.name, l.href),
        }
    }
}

impl ToString for List {
    fn to_string(&self) -> String {
        todo!()
    }
}

