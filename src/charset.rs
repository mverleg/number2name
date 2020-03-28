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
