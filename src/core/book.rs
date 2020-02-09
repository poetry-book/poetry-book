use crate::core::{
    book_attributes::BookAttributes, poem::Poem, poem_formatting::PoemFormatting, preface::Preface,
};

/// Use `BookBuilder` to build this.
pub struct Book {
    attributes: BookAttributes,
    poems: Vec<Poem>,
    preface: Option<Preface>,
    poem_formatting: PoemFormatting,
}

impl Book {
    pub fn attributes(&self) -> &BookAttributes {
        &self.attributes
    }

    pub fn poems(&self) -> &Vec<Poem> {
        &self.poems
    }

    pub fn preface(&self) -> Option<&Preface> {
        self.preface.as_ref()
    }

    pub fn poem_formatting(&self) -> PoemFormatting {
        self.poem_formatting
    }
}

pub mod builder {
    use super::Book;
    use crate::core::{
        book_attributes::BookAttributes, poem::Poem, poem_formatting::CenteredVerse,
        poem_formatting::PoemFormatting, preface::Preface,
    };

    /// Use this to construct a `Book`.
    pub struct BookBuilder {
        attributes: BookAttributes,
        poems: Vec<Poem>,
        preface: Option<Preface>,
        poem_formatting: PoemFormatting,
    }

    impl BookBuilder {
        pub fn new(attributes: BookAttributes, poems: Vec<Poem>) -> BookBuilder {
            BookBuilder {
                attributes,
                preface: None,
                poem_formatting: PoemFormatting::new(CenteredVerse::Longest),
                poems,
            }
        }

        /// Set preface.
        pub fn preface(mut self, preface: Preface) -> BookBuilder {
            self.preface = Some(preface);
            self
        }

        /// Set poem formatting.
        pub fn poem_formatting(mut self, poem_formatting: PoemFormatting) -> BookBuilder {
            self.poem_formatting = poem_formatting;
            self
        }

        pub fn finish(self) -> Book {
            Book {
                attributes: self.attributes,
                preface: self.preface,
                poem_formatting: self.poem_formatting,
                poems: self.poems,
            }
        }
    }
}
