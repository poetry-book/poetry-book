use crate::core::{paragraph::Paragraph, poem::Poem};
use crate::lang::latex::latex_output::Latex;

impl Latex for Poem {
    fn latex(&self) -> String {
        let paragraphs: Vec<String> = self.paragraphs().iter().map(Paragraph::latex).collect();
        paragraphs.join("\n\n")
    }
}
