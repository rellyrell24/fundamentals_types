fn main() {
    floating_point_types();
    bool_types();
}

fn floating_point_types() {
    // 31415.926e-4f64
    // integer + fractional + exponent + type suffix
    // integer (required) and need one of the other parts

    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN , f32::MAX);

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE
    assert_eq!((-1.01f64).floor(), -2.0);
}

fn bool_types() {
    // if x != 0 { ... } correct
    // if x { ... } incorrect
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    // uses a byte for bool value in memory,
    // so you can create a pointer to it
}
