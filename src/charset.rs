use ::std::ops::Index;
use ::std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub enum Case {
    Sensitive,
    Insensitive,
}

#[derive(Debug)]
pub struct Charset {
    values: Vec<char>,
    case: Case,
}

impl Charset {
    pub fn case_sensitive<'a>(data: impl Into<&'a str>) -> Self {
        Charset::new(data, Case::Sensitive)
    }

    pub fn case_insensitive<'a>(data: impl Into<&'a str>) -> Self {
        Charset::new(data, Case::Insensitive)
    }

    /// Panics if the input contains duplicates.
    pub fn new<'a>(data: impl Into<&'a str>, case: Case) -> Self {
        match Charset::try_new(data, case) {
            Some(charset) => charset,
            None => panic!("failed to initialize charset due to duplicate data"),
        }
    }

    /// Empty if the input contains duplicates.
    pub fn try_new<'a>(data: impl Into<&'a str>, case: Case) -> Option<Self> {
        let data = data.into();
        assert!(data.len() > 0);
        let mut seen = HashSet::new();
        let mut values = Vec::with_capacity(data.len());
        for character in data.chars() {
            let unique_repr = match case {
                Case::Sensitive => character,
                Case::Insensitive => character.to_lowercase(),
            };
            if seen.contains(&unique_repr) {
                return None
            }
            seen.insert(unique_repr);
            values.push(character)
        }
        Some(Charset {
            values, case
        })
    }

    /// Number of characters.
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl Index<usize> for Charset {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_charset() {
        let charset = Charset::try_new("Abc", Case::Insensitive);
        assert!(charset.is_some());
        let charset = charset.unwrap();
        assert_eq!(charset.len(), 3);
        assert_eq!(charset[0], 'A');
        assert_eq!(charset[1], 'b');
        assert_eq!(charset[2], 'c');
    }

    #[test]
    fn valid_case_duplicate() {
        let charset = Charset::try_new("abBA", Case::Sensitive);
        assert!(charset.is_some());
        let charset = charset.unwrap();
        assert_eq!(charset.len(), 4);
        assert_eq!(charset[0], 'a');
        assert_eq!(charset[1], 'b');
        assert_eq!(charset[2], 'B');
        assert_eq!(charset[3], 'A');
    }

    #[test]
    fn invalid_duplicate() {
        let charset = Charset::try_new("aba", Case::Insensitive);
        assert!(charset.is_none());
    }

    #[test]
    fn invalid_case_insensitive_duplicate() {
        let charset = Charset::try_new("abBA", Case::Insensitive);
        assert!(charset.is_none());
    }
}
