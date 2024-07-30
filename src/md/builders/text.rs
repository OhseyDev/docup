use crate::md::elements::text::*;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinkBuilder {
    name: String,
    href: Option<Url>,
    img: bool,
}

impl LinkBuilder {
    pub fn name(mut self, name: String) -> LinkBuilder {
        self.name = name;
        self
    }
    pub fn name_push(&mut self, c: char) {
        self.name.push(c);
    }
    pub fn name_push_str(&mut self, s: &str) {
        self.name.push_str(s);
    }
    pub fn href(mut self, href: url::Url) -> LinkBuilder {
        self.href = Some(href);
        self
    }
    pub fn make_img(mut self) -> LinkBuilder {
        self.img = true;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeadingBuilder {
    content: String,
    level: HeadingLvl,
    href: Option<Url>,
    img: bool,
}

impl super::Builder for LinkBuilder {
    type Output = Link;
    fn build(self) -> Result<Self::Output, super::Error> {
        if self.name.is_empty() {
            return Err(super::Error::IncompleteStructure);
        }
        let href = if let Some(url) = self.href {
            url
        } else {
            return Err(super::Error::IncompleteStructure);
        };
        return Ok(Link {
            name: self.name.into_boxed_str(),
            href,
            img: self.img,
        });
    }
    fn new() -> Self {
        LinkBuilder {
            name: String::new(),
            href: None,
            img: false,
        }
    }
}
