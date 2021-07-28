#[derive(PartialEq, Eq, Debug)]
pub struct NonGeneric;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // not ok?
        // expected struct `b::NonGeneric`, found struct `NonGeneric`
        a::mk_non_generic() == super::NonGeneric;
    }
}
