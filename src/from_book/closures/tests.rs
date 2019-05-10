use super::*;

#[test]
fn call_with_different_values() {
    let mut c = Cache::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn can_use_different_types() {
    let mut c = Cache::new(|a| a);

    let v1 = c.value("String 1");
    let v2 = c.value("String 2");

    assert_eq!("String 2", v2);
}
