#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Paragraph(pub(crate) Box<[Item]>);

impl ToString for Paragraph {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for item in self.0.into_iter() {
            s += &item.to_string();
        }
        s
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Heading {
    pub(crate) level: HeadingLvl,
    pub(crate) content: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeadingLvl {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
}

impl HeadingLvl {
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

macro_rules! into_headinglvl {
    ($num:ident) => {
        impl Into<$num> for HeadingLvl {
            fn into(self) -> $num {
                match self {
                    Self::Level1 => 1,
                    Self::Level2 => 2,
                    Self::Level3 => 3,
                    Self::Level4 => 4,
                    Self::Level5 => 5,
                    Self::Level6 => 6,
                }
            }
        }
    };
}

macro_rules! into_headinglvlf {
    ($num:ident) => {
        impl Into<$num> for HeadingLvl {
            fn into(self) -> $num {
                match self {
                    Self::Level1 => 1.0,
                    Self::Level2 => 2.0,
                    Self::Level3 => 3.0,
                    Self::Level4 => 4.0,
                    Self::Level5 => 5.0,
                    Self::Level6 => 6.0,
                }
            }
        }
    };
}

into_headinglvl!(u8);
into_headinglvl!(u16);
into_headinglvl!(u32);
into_headinglvl!(u64);
into_headinglvl!(u128);
into_headinglvl!(usize);
into_headinglvl!(i8);
into_headinglvl!(i16);
into_headinglvl!(i32);
into_headinglvl!(i64);
into_headinglvl!(i128);
into_headinglvl!(isize);
into_headinglvlf!(f32);
into_headinglvlf!(f64);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quote<'a> {
    Nested(&'a Quote<'a>),
    Items(Box<[Item]>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkSource {
    Url(url::Url),
    Ref(Box<str>),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub(crate) name: Box<str>,
    pub(crate) src: LinkSource,
    pub(crate) img: bool,
}

impl Into<Item> for Link {
    fn into(self) -> Item {
        Item::Link(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    Bold(Box<str>),
    BoldItalic(Box<str>),
    Def(Box<str>),
    Italic(Box<str>),
    Link(Link),
    NewLine,
}

impl Item {
    pub fn asterick(&mut self) {
        *self = match self {
            Self::Italic(c) => Self::Bold(c.clone()),
            Self::Bold(c) => Self::BoldItalic(c.clone()),
            Self::BoldItalic(c) => Self::Bold(c.clone()),
            _ => self.clone(),
        }
    }
    pub fn asterick_cons(self) -> Self {
        match self {
            Self::Italic(c) => Self::Bold(c),
            Self::Bold(c) => Self::BoldItalic(c),
            Self::BoldItalic(c) => Self::Bold(c),
            _ => self,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Italic(c) => c.is_empty(),
            Self::Bold(c) => c.is_empty(),
            Self::BoldItalic(c) => c.is_empty(),
            _ => false,
        }
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        return match self {
            Self::Def(s) => s.to_string(),
            Self::Italic(s) => format!("*{}*", s),
            Self::Bold(s) => format!("**{}**", s),
            Self::BoldItalic(s) => format!("***{}***", s),
            Self::Link(l) => format!(
                "{}[{}]({})",
                if l.img { "!" } else { "" },
                l.name,
                l.src.to_string()
            ),
            Self::NewLine => "\n".to_string(),
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reference {
    pub(crate) name: Box<str>,
    pub(crate) title: Box<str>,
    pub(crate) href: url::Url,
}

impl ToString for LinkSource {
    fn to_string(&self) -> String {
        match self {
            LinkSource::None => String::new(),
            LinkSource::Ref(r) => r.to_string(),
            LinkSource::Url(u) => u.to_string(),
        }
    }
}

impl ToString for Reference {
    fn to_string(&self) -> String {
        let mut s = format!("[{}]: <{}>", self.name, self.href.to_string());
        if !self.title.is_empty() {
            s.push_str(&format!("({})", self.title))
        }
        return s;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferenceBuilder {
    name: String,
    href: Option<url::Url>,
    title: String,
}

impl ReferenceBuilder {
    pub fn name(mut self, s: &str) -> Self {
        self.name = s.to_string();
        self
    }
    pub fn title(mut self, s: &str) -> Self {
        self.title = s.to_string();
        self
    }
    pub fn href(mut self, u: url::Url) -> Self {
        self.href = Some(u);
        self
    }
}

impl super::Builder for ReferenceBuilder {
    type Output = Reference;
    fn build(self) -> Result<Self::Output, super::BuildError> {
        let url = if let Some(u) = self.href {
            u
        } else {
            return Err(super::BuildError::IncompleteData);
        };
        if self.name.is_empty() {
            return Err(super::BuildError::IncompleteData);
        }
        Ok(Self::Output {
            name: self.name.into_boxed_str(),
            title: self.title.into_boxed_str(),
            href: url,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemBuilder {
    Bold(String),
    BoldItalic(String),
    Def(String),
    Italic(String),
    Link(Link),
    Undefined,
}

impl ItemBuilder {
    pub fn bold(self) -> Self {
        match self {
            Self::Bold(s) => Self::Def(s),
            Self::BoldItalic(s) => Self::Italic(s),
            Self::Italic(s) => Self::BoldItalic(s),
            Self::Def(s) => Self::Bold(s),
            _ => Self::Undefined,
        }
    }
    pub fn italic(self) -> Self {
        match self {
            Self::Bold(s) => Self::BoldItalic(s),
            Self::BoldItalic(s) => Self::Bold(s),
            Self::Italic(s) => Self::Def(s),
            Self::Def(s) => Self::Italic(s),
            _ => Self::Undefined,
        }
    }
    pub fn link(self, l: Link) -> Self {
        Self::Link(l)
    }
    pub fn content(self, s: String) -> Self {
        match self {
            Self::Bold(_) => Self::Bold(s),
            Self::BoldItalic(_) => Self::BoldItalic(s),
            Self::Italic(_) => Self::Italic(s),
            _ => Self::Def(s),
        }
    }
}

impl super::Builder for ItemBuilder {
    type Output = Item;
    fn build(self) -> Result<Self::Output, super::BuildError> {
        match self {
            Self::Bold(s) => Ok(Self::Output::Bold(s.into_boxed_str())),
            Self::BoldItalic(s) => Ok(Self::Output::BoldItalic(s.into_boxed_str())),
            Self::Def(s) => Ok(Self::Output::Def(s.into_boxed_str())),
            Self::Italic(s) => Ok(Self::Output::Italic(s.into_boxed_str())),
            Self::Link(l) => Ok(Self::Output::Link(l)),
            Self::Undefined => Err(super::BuildError::IncompleteData),
        }
    }
}

impl Default for ItemBuilder {
    fn default() -> Self {
        ItemBuilder::Undefined
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinkBuilder {
    name: String,
    src: LinkSource,
    img: bool,
}

impl LinkBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn name_push(&mut self, c: char) {
        self.name.push(c);
    }
    pub fn name_push_str(&mut self, s: &str) {
        self.name.push_str(s);
    }
    pub fn href(mut self, href: url::Url) -> Self {
        self.src = LinkSource::Url(href);
        self
    }
    pub fn reference(mut self, r: Reference) -> Self {
        self.src = LinkSource::Ref(r.name.clone());
        self
    }
    pub fn make_img(mut self) -> Self {
        self.img = true;
        self
    }
}

impl super::Builder for LinkBuilder {
    type Output = Link;
    fn build(self) -> Result<Self::Output, super::BuildError> {
        if self.name.is_empty() {
            return Err(super::BuildError::IncompleteData);
        }
        if let LinkSource::Ref(s) = &self.src {
            if s.is_empty() {
                return Err(super::BuildError::IncompleteData);
            }
        };
        Ok(Self::Output {
            name: self.name.into_boxed_str(),
            src: self.src,
            img: self.img,
        })
    }
}

impl Default for LinkSource {
    fn default() -> Self {
        Self::None
    }
}

impl Default for HeadingLvl {
    fn default() -> Self {
        Self::Level1
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeadingBuilder {
    content: String,
    level: HeadingLvl,
}

impl HeadingBuilder {
    pub fn content(mut self, s: String) -> Self {
        self.content = s;
        self
    }
    pub fn level(mut self, l: HeadingLvl) -> Self {
        self.level = l;
        self
    }
}

impl super::Builder for HeadingBuilder {
    type Output = Heading;
    fn build(self) -> Result<Self::Output, super::BuildError> {
        if self.content.is_empty() {
            return Err(super::BuildError::IncompleteData);
        }
        Ok(Self::Output {
            level: self.level,
            content: self.content,
        })
    }
}
