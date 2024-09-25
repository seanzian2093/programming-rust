// We have a `dot` function for `i64`
fn dot(v1: &[i64], v2: &[i64]) -> i64 {
    let mut total = 0;
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

// We need to modify it to generic function that works with `f64`

// - step1 change `i64` to generic type parameter `N`
// - declare`<N>` after function name `dot_g`

// - step2 we see three errors
// - `can not multiply N * N` - we add `Mul` to N
// - `cannot add N to N` - we add `Add` to N
// - 0 is `{integer}`, not type N - we add `Default` trait to N

// - step3 we see error that
// - `v1[i] * v2[i]` does not produce a N type as expected since we could overload the `*` operator to return any type
// - we limite `Add` and `Mul` to a subset by <Output=N>  - `Output` is the associated type defined in `Add` and `Mul`

// - step4 we see error that
// - cannot move out indexed content from slice, triggered by `v1[i]`, `v2[i]`
// - we add `Copy` to N
use std::ops::{Add, Mul};
fn dot_g<N: Add<Output = N> + Mul<Output = N> + Default + Copy>(v1: &[N], v2: &[N]) -> N {
    let mut total: N = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

pub fn use_dot_g() {
    assert_eq!(dot_g(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot_g(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}
