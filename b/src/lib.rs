#[derive(PartialEq, Eq, Debug)]
pub struct SomeStruct;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // not ok?
        // expected struct `b::SomeStruct`, found struct `SomeStruct`
        a::mk_struct() == super::SomeStruct;
    }
}
