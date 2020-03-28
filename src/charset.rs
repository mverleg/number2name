use ::std::ops::Index;
use ::std::collections::HashSet;

pub struct Charset {
    values: Vec<char>
}

impl Charset {
    /// Panics if the input contains duplicates.
    pub fn new<'a>(data: impl Into<&'a str>) -> Self {
        match Charset::try_new(data) {
            Some(charset) => charset,
            None => panic!("failed to initialize charset due to duplicate data"),
        }
    }

    /// Empty if the input contains duplicates.
    pub fn try_new<'a>(data: impl Into<&'a str>) -> Option<Self> {
        let data = data.into();
        assert!(data.len() > 0);
        let mut seen = HashSet::new();
        let mut values = Vec::with_capacity(data.len());
        for character in data.chars() {
            if seen.contains(&character) {
                return None
            }
            seen.insert(character);
            values.push(character)
        }
        Some(Charset {
            values
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

impl Index<u32> for Charset {
    type Output = char;

    fn index(&self, index: u32) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<u64> for Charset {
    type Output = char;

    fn index(&self, index: u64) -> &Self::Output {
        &self[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_charset() {
        let charset = Charset::try_new("abc");
        assert!(charset.is_some());
        let charset = charset.unwrap();
        assert_eq!(charset.len(), 3);
        assert_eq!(charset[0u32], 'a');
        assert_eq!(charset[1u32], 'b');
        assert_eq!(charset[2u32], 'c');
    }

    #[test]
    fn invalid_charset() {
        let charset = Charset::try_new("aba");
        assert!(charset.is_none());
    }
}
