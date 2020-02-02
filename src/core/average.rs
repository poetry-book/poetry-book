pub struct Average;

impl Average {
    pub fn rounded_average_length(strings: &[&str]) -> u32 {
        let strings_lengths: Vec<u32> = strings.iter().map(|s| s.len() as u32).collect();
        Self::rounded_average(&strings_lengths)
    }

    pub fn rounded_average(numbers: &[u32]) -> u32 {
        let sum: u32 = Iterator::sum(numbers.iter());
        let mean: f64 = f64::from(sum) / (numbers.len() as f64);
        mean.round() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_average() {
        let strings = vec!["aaa", "bb", "dddd"];
        let average = Average::rounded_average_length(&strings);
        assert_eq!(3, average);
    }
}
