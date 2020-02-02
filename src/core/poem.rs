use crate::core::{average::Average, paragraph::Paragraph};

#[derive(PartialEq, Eq, Debug)]
pub struct Poem {
    title: String,
    paragraphs: Vec<Paragraph>,
}

impl Poem {
    pub fn new(title: &str, text: &str) -> Poem {
        let paragraphs: Vec<Paragraph> = text
            .split("\n\n")
            .map(Paragraph::new)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        Poem {
            title: title.to_string(),
            paragraphs,
        }
    }

    pub fn get_average_verse_size(&self) -> usize {
        let paragraphs_averages: Vec<u32> = self
            .paragraphs
            .iter()
            .map(Paragraph::verse_average_size)
            .collect();
        let average_size = Average::rounded_average(&paragraphs_averages);
        average_size as usize
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn paragraphs(&self) -> &Vec<Paragraph> {
        &self.paragraphs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static POEM_TITLE: &str = "Lorem ipsum";

    static POEM_TEXT: &str =
"Pellentesque dapibus suscipit ligula.
Donec posuere augue in quam.
Etiam vel tortor sodales tellus ultricies commodo.
Suspendisse potenti.

Aenean in sem ac leo mollis blandit.
Donec neque quam, dignissim in, mollis nec, sagittis eu, wisi.
Phasellus lacus.
Etiam laoreet quam sed arcu.

Phasellus at dui in ligula mollis ultricies.
Integer placerat tristique nisl.
Praesent augue.
Fusce commodo.

Vestibulum convallis, lorem a tempus semper, dui dui euismod elit, vitae placerat urna tortor vitae lacus.
Nullam libero mauris, consequat quis, varius et, dictum id, arcu.
Mauris mollis tincidunt felis.

Aliquam feugiat tellus ut neque.
Nulla facilisis, risus a rhoncus fermentum, tellus tellus lacinia purus, et dictum nunc justo sit amet elit.";

    #[test]
    fn create_poem() {
        let actual_poem = Poem::new(POEM_TITLE, POEM_TEXT);

        let paragraphs = POEM_TEXT
            .split("\n\n")
            .map(|p| Paragraph::new(p).unwrap())
            .collect();
        let expected_poem = Poem {
            title: POEM_TITLE.to_string(),
            paragraphs,
        };
        assert_eq!(actual_poem, expected_poem)
    }
}
