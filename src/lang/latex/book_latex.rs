use crate::core::{book::Book, poem_formatting::CenteredVerse};
use crate::lang::latex::{book_body_latex::BookBodyLatex, latex_output::Latex};

static COPYRIGHT: &str = r#"%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Poem
% LaTeX Template
% Version 1.0 (2/11/2015)
%
% This template has been downloaded from:
% http://www.LaTeXTemplates.com
%
% Original author:
% Vel (vel@latextemplates.com)
%
% License:
% CC BY-NC-SA 3.0 (http://creativecommons.org/licenses/by-nc-sa/3.0/)
%
% General notes:
% 1) All lines in a verse environment must end with \\, the last verse in a stanza
% must end in \\!
% 2) This template is based on the verse package, see the package documentation
% included with the template for further customisation options
%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\documentclass[11pt, a4paper]{article} % Document font size and paper size

% allow different paper format, like a5paper
\usepackage[pass]{geometry}

\usepackage{verse} % Required for typesetting poems - this package drives this template

\usepackage[T1]{fontenc} % International character encodings
\usepackage{palatino} % Use the Palatino font by default"#;

static LONGEST_VERSE_MACRO: &str = r"
\usepackage{varwidth,environ}

\newsavebox{\versebox}
\NewEnviron{cverse}{%
\setlength{\leftmargini}{0pt}%
\begin{lrbox}{\versebox}
    \begin{varwidth}{\textwidth}
    \begin{verse}
        \BODY
    \end{verse}
    \end{varwidth}
\end{lrbox}%
\begin{verse}[\wd\versebox]
    \BODY
\end{verse}
}";

impl Latex for Book {
    fn latex(&self) -> String {
        let mut header = COPYRIGHT.to_string();

        if let Some(lang) = &self.attributes.language() {
            header.push_str(r"\usepackage[");
            header.push_str(&lang);
            header.push_str(r"]{babel}");
        }

        if let CenteredVerse::Longest = self.poem_formatting.centered_verse {
            header.push_str(&LONGEST_VERSE_MACRO.to_string());
        }

        header.push_str(
            r"

\setlength{\parindent}{0pt} % Disable paragraph indentation

\renewcommand{\poemtoc}{subsection}",
        );

        if let Some(toc_title) = &self.attributes.toc_title() {
            header.push_str("\n\n");
            header.push_str(r"\renewcommand*\contentsname{");
            header.push_str(&toc_title);
            header.push_str(r"}");
        }

        header.push_str(
            r#"

\setlength{\stanzaskip}{0.75\baselineskip} % The distance between stanzas

\title{"#,
        );
        header.push_str(&self.attributes.title());
        header.push_str("}\n");
        header.push_str(r#"\author{"#);
        header.push_str(&self.attributes.author());
        header.push_str(
            r#"}

\begin{document}

\maketitle
\thispagestyle{empty}
\newpage

\pagenumbering{roman}
\tableofcontents
\newpage
"#,
        );

        if let Some(preface) = &self.preface {
            header.push_str(&preface.latex());
        }

        header.push_str(
            r#"
\pagestyle{plain}
\pagenumbering{arabic}
"#,
        );

        let footer = r#"\end{document}"#;

        let book_body = BookBodyLatex {
            poem_formatting: &self.poem_formatting,
            poems: &self.poems,
        };

        let poems = book_body.latex();

        let book = format!("{}\n\n{}\n\n{}", header, poems, footer);
        book
    }
}
