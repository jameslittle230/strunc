use unicode_segmentation::UnicodeSegmentation;

pub struct Struncable {
    input: String,
    length: usize,
    suffix: String,
}

impl Default for Struncable {
    fn default() -> Self {
        Self {
            input: Default::default(),
            length: 25,
            suffix: "...".to_string(),
        }
    }
}

pub trait Strunc {
    fn strunc(self) -> Struncable;
    fn strunc_len(self, length: usize) -> Struncable;
    fn strunc_len_suf(self, length: usize, suffix: &str) -> Struncable;
}

impl<'a> Strunc for &'a str {
    fn strunc(self) -> Struncable {
        Struncable {
            input: String::from(self),
            ..Default::default()
        }
    }

    fn strunc_len(self, length: usize) -> Struncable {
        Struncable {
            input: String::from(self),
            length,
            ..Default::default()
        }
    }

    fn strunc_len_suf(self, length: usize, suffix: &str) -> Struncable {
        Struncable {
            input: String::from(self),
            length,
            suffix: String::from(suffix),
        }
    }
}

impl ToString for Struncable {
    fn to_string(&self) -> String {
        if self.length < self.suffix.graphemes(true).count() {
            panic!("Suffix is longer than the truncated length! That's no fun.")
        }

        if self.input.graphemes(true).count() <= self.length {
            self.input.clone()
        } else {
            let mut out = self
                .input
                .graphemes(true)
                .take(
                    self.length
                        .saturating_sub(dbg!(self.suffix.graphemes(true).count())),
                )
                .into_iter()
                .collect::<Vec<&str>>()
                .join("")
                .trim()
                .to_string();
            out.push_str(&self.suffix);
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Strunc;

    #[test]
    fn happy_path() {
        let output = "verylong string".strunc_len(8);
        assert_eq!("veryl...", output.to_string());
    }

    #[test]
    fn trims_spaces() {
        let output = "very long string".strunc_len(8);
        assert_eq!("very...", output.to_string());
    }

    #[test]
    fn second_happy_path() {
        let output = "very long string".strunc_len(10);
        assert_eq!("very lo...", output.to_string());
    }

    #[test]
    fn much_smaller_than_input_length() {
        let output = "very long string".strunc_len(25);
        assert_eq!("very long string", output.to_string());
    }

    #[test]
    fn input_length_equals_space_available() {
        let output = "very long string".strunc_len(16);
        assert_eq!("very long string", output.to_string());
    }

    #[test]
    fn input_length_equals_space_available_minus_one() {
        let output = "very long string".strunc_len(15);
        assert_eq!("very long st...", output.to_string());
    }

    #[test]
    fn different_suffix() {
        let output = "very long string".strunc_len_suf(15, "....");
        assert_eq!("very long s....", output.to_string());
    }

    #[test]
    fn suffix_of_length_zero() {
        let output = "very long string".strunc_len_suf(16, "");
        assert_eq!("very long string", output.to_string());
    }

    #[test]
    fn suffix_of_length_zero_minus_one() {
        let output = "very long string".strunc_len_suf(15, "");
        assert_eq!("very long strin", output.to_string());
    }
}
