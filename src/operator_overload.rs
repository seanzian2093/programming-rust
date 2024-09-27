fn equivalence_compare() {
    // expression with no appropriate value like `0.0/0.0` must return NaN values
    // - NaN isunequal to every other, including NaN iteself
    // - any ordered comparison with NaN returns false
    // - `std::cmp:PartialEq` implement this
    // - std library types that implement `PartialEq` also implement `Eq` except f32 and f64
    assert!(f64::is_nan(0.0 / 0.0));
    assert_eq!(0.0 / 0.0 == 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 != 0.0 / 0.0, true);
    assert_eq!(0.0 / 0.0 <= 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 < 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 >= 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 > 0.0 / 0.0, false);
}

// ordered comparison `>=`, `<=`, `<`, `>` are implement in `std::cmp::PartialOrd` trait
// - `PartialOrd` extends `PartialEq`, which means that any type implements `PartialOrd` must also implement `PartialEq`
// - the only method we must implement of `PartialOrd` is `partial_cmp` which returns an Option
// - almost all types that implement `PartialOrd` implment `Ord` except f32 f64
pub fn operator_overload() {
    equivalence_compare();
}
