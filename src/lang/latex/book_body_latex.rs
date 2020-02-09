use crate::core::{poem::Poem, poem_formatting::CenteredVerse, poem_formatting::PoemFormatting};
use crate::lang::latex::latex_output::Latex;

pub struct BookBodyLatex<'a> {
    pub poem_formatting: &'a PoemFormatting,
    pub poems: &'a Vec<Poem>,
}

impl<'a> Latex for BookBodyLatex<'a> {
    fn latex(&self) -> String {
        let poems: Vec<String> = self.poems.iter().map(|p| self.poem_latex(p)).collect();
        poems.join("\n\n\n")
    }
}

impl<'a> BookBodyLatex<'a> {
    fn poem_latex(&self, poem: &Poem) -> String {
        let mut output = format!("\\poemtitle{{{}}}\n", poem.title());
        output.push_str(&self.get_poem_begin(poem));
        output.push_str("\n\n");
        output.push_str(&poem.latex());
        output.push_str("\n\n");
        output.push_str(self.get_poem_end());
        output.push_str("\n\\newpage");
        output
    }

    fn get_poem_begin(&self, poem: &Poem) -> String {
        match self.poem_formatting.centered_verse() {
            CenteredVerse::Average => {
                let mut poem_begin = r"\settowidth{\versewidth}".to_string();
                let average_verse_size = poem.get_average_verse_size();
                let average_sized_verse = "x".repeat(average_verse_size);
                let average_sized_verse = format!("{{{}}}\n", average_sized_verse);
                poem_begin.push_str(&average_sized_verse);
                poem_begin.push_str("\\begin{verse}[\\versewidth]");
                poem_begin
            }
            CenteredVerse::Longest => r"\begin{cverse}".to_string(),
        }
    }

    fn get_poem_end(&self) -> &str {
        match self.poem_formatting.centered_verse() {
            CenteredVerse::Average => r"\end{verse}",
            CenteredVerse::Longest => r"\end{cverse}",
        }
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
    fn create_latex() {
        let expected_latex = r"\poemtitle{Lorem ipsum}
\settowidth{\versewidth}{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx}
\begin{verse}[\versewidth]

Pellentesque dapibus suscipit ligula. \\
Donec posuere augue in quam. \\
Etiam vel tortor sodales tellus ultricies commodo. \\
Suspendisse potenti. \\!

Aenean in sem ac leo mollis blandit. \\
Donec neque quam, dignissim in, mollis nec, sagittis eu, wisi. \\
Phasellus lacus. \\
Etiam laoreet quam sed arcu. \\!

Phasellus at dui in ligula mollis ultricies. \\
Integer placerat tristique nisl. \\
Praesent augue. \\
Fusce commodo. \\!

Vestibulum convallis, lorem a tempus semper, dui dui euismod elit, vitae placerat urna tortor vitae lacus. \\
Nullam libero mauris, consequat quis, varius et, dictum id, arcu. \\
Mauris mollis tincidunt felis. \\!

Aliquam feugiat tellus ut neque. \\
Nulla facilisis, risus a rhoncus fermentum, tellus tellus lacinia purus, et dictum nunc justo sit amet elit. \\!

\end{verse}
\newpage";

        let poem = Poem::new(POEM_TITLE, POEM_TEXT);

        let book_body = BookBodyLatex {
            poem_formatting: &PoemFormatting::new(CenteredVerse::Average),
            poems: &vec![poem],
        };

        let actual_latex = book_body.latex();
        assert_eq!(actual_latex, expected_latex);
    }
}
