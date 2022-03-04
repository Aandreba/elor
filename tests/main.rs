use etot::Either::{self, *};

#[test]
fn test () {
    let alpha : Either<String, Vec<u8>> = Left("hello world".to_string());
    let beta : Either<&str, &[u8]> = alpha.as_deref();
    assert_eq!(beta, Left("hello world"))
}