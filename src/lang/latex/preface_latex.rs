use crate::core::preface::Preface;
use crate::lang::latex::latex_output::Latex;

impl Latex for Preface {
    fn latex(&self) -> String {
        let mut preface = r"\section*{".to_string();
        preface.push_str(&self.title);
        preface.push_str("}\n\n");

        preface.push_str(&self.body);

        preface.push_str("\n\n");

        preface.push_str(r"\newpage");
        preface
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_latex() {
        let expected_latex = r"\section*{my preface}

Mauris ac felis vel velit tristique imperdiet.

\newpage";
        let preface = Preface {
            title: "my preface".to_string(),
            body: "Mauris ac felis vel velit tristique imperdiet.".to_string(),
        };

        let actual_latex = preface.latex();

        assert_eq!(expected_latex, actual_latex);
    }
}
