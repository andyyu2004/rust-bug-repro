#[derive(PartialEq, Eq, Debug)]
pub struct SomeStruct;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // not ok?
        // expected struct `b::NonGeneric`, found struct `NonGeneric`
        a::mk_struct() == super::SomeStruct;
    }
}
