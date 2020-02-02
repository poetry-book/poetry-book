use poetry_book::Book;
use poetry_book::BookAttributes;
use poetry_book::Latex;
use poetry_book::PoemFormatting;
use poetry_book::CenteredVerse;
use poetry_book::Poem;
use poetry_book::Preface;

static EXPECTED_LATEX: &str = r#"%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
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
\usepackage{palatino} % Use the Palatino font by default

\setlength{\parindent}{0pt} % Disable paragraph indentation

\renewcommand{\poemtoc}{subsection}

\renewcommand*\contentsname{toc title}

\setlength{\stanzaskip}{0.75\baselineskip} % The distance between stanzas

\title{book title}
\author{book author}

\begin{document}

\maketitle
\thispagestyle{empty}
\newpage

\pagenumbering{roman}
\tableofcontents
\newpage
\section*{preface title}

preface body, preface body, preface body.

\newpage
\pagestyle{plain}
\pagenumbering{arabic}


\poemtitle{poem one}
\settowidth{\versewidth}{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx}
\begin{verse}[\versewidth]

Pellentesque dapibus suscipit ligula. \\
Donec posuere augue in quam. \\!

Etiam vel tortor sodales tellus ultricies commodo. \\
Suspendisse potenti. \\!

\end{verse}
\newpage


\poemtitle{poem two}
\settowidth{\versewidth}{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx}
\begin{verse}[\versewidth]

Aenean in sem ac leo mollis blandit. \\
Donec neque quam, dignissim in, mollis nec. \\!

Phasellus lacus. \\
Etiam laoreet quam sed arcu. \\!

\end{verse}
\newpage

\end{document}"#;

    static POEM_ONE_TEXT: &str =
"Pellentesque dapibus suscipit ligula.
Donec posuere augue in quam.

Etiam vel tortor sodales tellus ultricies commodo.
Suspendisse potenti.";

    static POEM_TWO_TEXT: &str =
"Aenean in sem ac leo mollis blandit.
Donec neque quam, dignissim in, mollis nec.

Phasellus lacus.
Etiam laoreet quam sed arcu.";

#[test]
fn create_latex_book() {
    let book_attributes = BookAttributes {
        author: "book author".to_string(),
        title: "book title".to_string(),
        language: None,
        toc_title: Some("toc title".to_string())
    };

    let preface = Preface {
        title: "preface title".to_string(),
        body: "preface body, preface body, preface body.".to_string(),

    };

    let poems = vec![
        Poem::new("poem one", POEM_ONE_TEXT),
        Poem::new("poem two", POEM_TWO_TEXT),
    ];

    let book = Book {
        attributes: book_attributes,
        preface: Some(preface),
        poems,
        poem_formatting: PoemFormatting {
            centered_verse: CenteredVerse::Average
        }
    };

    let actual_latex = book.latex();
    assert_eq!(EXPECTED_LATEX, actual_latex);
}
