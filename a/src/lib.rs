pub fn mk_struct() -> b::SomeStruct {
    b::SomeStruct
}

#[test]
fn wer() {
    // OK
    assert_eq!(self::mk_struct(), b::SomeStruct);
}
