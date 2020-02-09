use std::ops::Deref;

/// Use `BookAttributesBuilder` to build this.
pub struct BookAttributes {
    author: String,
    title: String,
    language: Option<String>,
    toc_title: Option<String>,
}

impl BookAttributes {
    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    /// e.g. English, Italian..
    pub fn language(&self) -> Option<&str> {
        self.language.as_ref().map(Deref::deref)
    }

    /// table of contents title
    pub fn toc_title(&self) -> Option<&str> {
        self.toc_title.as_ref().map(Deref::deref)
    }
}

pub mod builder {
    use super::BookAttributes;

    /// Use this to construct `BookAttributes`.
    pub struct BookAttributesBuilder {
        author: String,
        title: String,
        language: Option<String>,
        toc_title: Option<String>,
    }

    impl BookAttributesBuilder {
        pub fn new(author: &str, title: &str) -> BookAttributesBuilder {
            BookAttributesBuilder {
                author: author.to_string(),
                title: title.to_string(),
                language: None,
                toc_title: None,
            }
        }

        /// e.g. English, Italian..
        pub fn language(mut self, language: &str) -> BookAttributesBuilder {
            self.language = Some(language.to_string());
            self
        }

        /// table of contents title
        pub fn toc_title(mut self, toc_title: &str) -> BookAttributesBuilder {
            self.toc_title = Some(toc_title.to_string());
            self
        }

        pub fn finish(self) -> BookAttributes {
            BookAttributes {
                author: self.author,
                title: self.title,
                language: self.language,
                toc_title: self.toc_title,
            }
        }
    }
}
