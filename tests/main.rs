use etot::Either::{self, *};

#[test]
fn test () {
    let alpha : Either<String, Vec<u8>> = Right(b"hello world".to_vec());
    let beta : Either<&str, &[u8]> = alpha.as_deref();
    assert_eq!(beta, Right(b"hello world"))
}