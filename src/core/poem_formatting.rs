#[derive(PartialEq, Eq, Debug)]
pub struct PoemFormatting {
    pub centered_verse: CenteredVerse,
}

/// Determine which verse will be centered
#[derive(PartialEq, Eq, Debug)]
pub enum CenteredVerse {
    /// A verse with a number of 'x' equal to the average length among all verses
    Average,
    /// The verse with the longest length
    Longest,
}
