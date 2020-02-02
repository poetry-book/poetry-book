pub struct BookAttributes {
    pub author: String,
    pub title: String,
    /// e.g. english, italian..
    pub language: Option<String>,
    /// table of contents title
    pub toc_title: Option<String>,
}
