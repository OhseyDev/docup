use std::string::ToString;
use std::str::{Chars, FromStr};
use url::{Url, ParseError as ParseErrorUrl};

pub trait ParseStr: FromStr {
    fn parse_str(s: &str, ctx: &Context) -> Result<Self, Self::Err>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Context {
}

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    path: Box<std::path::Path>,
    content: Box<[DocItem]>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseError {
    EmptyDocument(),
    EmptyContent(),
    UnexpectedChar(char),
    UnexpectedEnd(),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Code {
    content: String,
    block: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DocItem {
    Heading(Heading),
    Paragraph(Paragraph),
    Code(Code),
    Blockquote(String),
    HorizontalRule(),
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
        let mut tokens = Vec::<DocItem>::new();
        let mut buffer = String::new();
        for item in s.split("\n") {
            let c = item.to_string().pop();
            if c == Some('\\') {
                buffer.push_str(item);
                continue;
            }
        }
        todo!()
    }
}

impl DocItem {
    fn _parse_code(chars: &mut Chars) -> Result<Self, ParseError> {
        let mut r: u8 = 0;
        let mut block = None;
        let mut code: Option<String> = None;
        while let Some(c) = chars.next() {
            match c {
                '`' => {
                    r += 1;
                    if let Some(b) = block {
                        if r != b {
                            continue;
                        }
                        return if let Some(c) = code { Ok(DocItem::Code(Code { content: c, block: b>1 })) } else { Err(ParseError::EmptyContent()) };
                    } else if r == 3 {
                        block = Some(r);
                        r = 0;
                    }
                },
                _ => {
                    if r != 0 {
                        block = Some(r);
                        r = 0;
                    }
                }
            }
        }
        return Err(ParseError::UnexpectedEnd());
    }
}

impl FromStr for DocItem {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            return match c {
                '#' => {
                    let h = Heading::from_str(&format!("{}{}",c,chars.as_str()))?;
                    Ok(DocItem::Heading(h))
                }
                ' ' => continue,
                '`' => Self::_parse_code(&mut chars),
                _ => {
                    let p = Paragraph::from_str(&format!("{}{}",c,chars.as_str()))?;
                    Ok(DocItem::Paragraph(p))
                },
            };
        }
        return Err(ParseError::UnexpectedEnd());
    }
}

impl ToString for DocItem {
    fn to_string(&self) -> String {
        return match self {
            Self::Heading(h) => h.to_string(),
            Self::Paragraph(p) => p.to_string(),
            Self::Code(c) => c.to_string(),
            Self::List(l) => todo!(),
            Self::HorizontalRule() => todo!(),
            Self::Blockquote(s) => todo!(),
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
        let mut mode: u8 = 0;
        while let Some(c) = chars.next() {
            match c {
                '*' => { todo!() },
                '!' => if i == 0 { mode = 0x11; } else {
                    
                },
                '[' => if mode == 0x11 {
                    todo!()
                } else {
                    mode = 0x10;
                    todo!()
                }
                _ => content.push(c),
            }
            i += 1;
        }
        Err(ParseError::UnexpectedEnd())
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

