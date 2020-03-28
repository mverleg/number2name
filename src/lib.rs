
mod typ;
mod charset;
mod builtin;
mod encode;
mod decode;

pub use crate::charset::Charset;
pub use crate::typ::N2NErr;
pub use crate::encode::number2name;
pub use crate::decode::name2number;

#[cfg(test)]
mod tests {

    #[test]
    fn demo() {
        unimplemented!()
    }
}

