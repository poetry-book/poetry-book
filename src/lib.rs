//! # Poetry book
//!
//! `poetry_book` allows you to create a poetry book starting from plain text.

mod core;
mod lang;

pub use crate::core::book::Book;
pub use crate::core::book_attributes::BookAttributes;
pub use crate::core::preface::Preface;
pub use crate::core::{poem::Poem, poem_formatting::CenteredVerse, poem_formatting::PoemFormatting};

pub use crate::lang::latex::latex_output::Latex;
