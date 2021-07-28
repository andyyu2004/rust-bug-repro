pub fn mk_non_generic() -> b::NonGeneric {
    b::NonGeneric
}

#[test]
fn wer() {
    // OK
    assert_eq!(self::mk_non_generic(), b::NonGeneric);
}
