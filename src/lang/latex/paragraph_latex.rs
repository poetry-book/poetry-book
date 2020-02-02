use crate::core::paragraph::Paragraph;
use crate::lang::latex::latex_output::Latex;

impl Latex for Paragraph {
    fn latex(&self) -> String {
        let lines: Vec<String> = self.lines().iter().map(|s| format!(r"{} \\", s)).collect();
        let latex: String = lines.join("\n");
        latex + "!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static PARAGRAPH_TEXT: &str = "Pellentesque dapibus suscipit ligula.
Donec posuere augue in quam.
Etiam vel tortor sodales tellus ultricies commodo.

";

    #[test]
    fn create_latex() {
        let expected_latex = r"Pellentesque dapibus suscipit ligula. \\
Donec posuere augue in quam. \\
Etiam vel tortor sodales tellus ultricies commodo. \\!";
        let paragraph = Paragraph::new(PARAGRAPH_TEXT).unwrap();
        let actual_latex = paragraph.latex();
        assert_eq!(actual_latex, expected_latex);
    }
}
