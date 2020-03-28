use crate::Charset;

pub fn number2name(number: impl Into<u64>, charset: &Charset) -> String {
    let size = charset.len() as u64;
    dbg!(size);  //TODO @mverleg: remove
    let mut remainder = number.into();
    dbg!(remainder);  //TODO @mverleg: remove
    let mut name = Vec::new();
    loop {
        let index = remainder % size;
        // dbg!(index);  //TODO @mverleg: remove
        name.push(index as usize);
        remainder /= size;
        dbg!(remainder);  //TODO @mverleg: remove
        if remainder == 0 {
            break;
        }
        remainder -= 1;
    }
    name.into_iter()
        .map(|index| charset[index])
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Charset;

    //TODO @mark: test close to overflow limit

    #[test]
    fn demo() {
        let charset = Charset::case_sensitive("aBcD");
        for i in 0..70u64 {
            let text = number2name(i, &charset);
            println!(">> {} {}", i, text);  //TODO @mark: TEMPORARY! REMOVE THIS!
        }
        unimplemented!()
        //assert_eq!(text, "aaaaa");
    }
}