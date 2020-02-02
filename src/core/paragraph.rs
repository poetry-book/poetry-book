use crate::core::average::Average;

#[derive(PartialEq, Eq, Debug)]
pub struct Paragraph {
    lines: Vec<String>,
}

impl Paragraph {
    pub fn new(paragraph: &str) -> Option<Paragraph> {
        let lines: Vec<String> = paragraph
            .split('\n')
            .filter(|s| Paragraph::is_line_empty(s))
            .map(|s| s.to_string())
            .collect();
        if lines.is_empty() {
            None
        } else {
            Some(Paragraph { lines })
        }
    }

    pub fn verse_average_size(&self) -> u32 {
        let lines: Vec<&str> = self.lines.iter().map(AsRef::as_ref).collect();
        Average::rounded_average_length(&lines)
    }

    fn is_line_empty(line: &str) -> bool {
        line.chars().any(|c| c != ' ')
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }
}
