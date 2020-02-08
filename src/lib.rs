//! # Poetry book
//!
//! `poetry-book` allows you to create a poetry book in latex, starting from plain text.

#![forbid(unsafe_code)]

mod core;
mod lang;

pub use crate::core::{
    book::{builder::BookBuilder, Book},
    book_attributes::{builder::BookAttributesBuilder, BookAttributes},
    poem::Poem,
    poem_formatting::CenteredVerse,
    poem_formatting::PoemFormatting,
    preface::Preface,
};

pub use crate::lang::latex::latex_output::Latex;
