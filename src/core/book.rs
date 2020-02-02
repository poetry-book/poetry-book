use crate::core::{
    book_attributes::BookAttributes, poem::Poem, poem_formatting::PoemFormatting, preface::Preface,
};

pub struct Book {
    pub attributes: BookAttributes,
    pub preface: Option<Preface>,
    pub poem_formatting: PoemFormatting,
    pub poems: Vec<Poem>,
}
