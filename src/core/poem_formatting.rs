#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct PoemFormatting {
    centered_verse: CenteredVerse,
}

impl PoemFormatting {
    pub fn new(centered_verse: CenteredVerse) -> PoemFormatting {
        PoemFormatting { centered_verse }
    }

    pub fn centered_verse(self) -> CenteredVerse {
        self.centered_verse
    }
}

/// Determine which verse will be centered
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum CenteredVerse {
    /// A verse with a number of 'x' equal to the average length among all verses
    Average,
    /// The verse with the longest length
    Longest,
}
