fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

pub fn ref_to_expr() {
    // a ref to expr - compiler creates a temp variable to hold the result of the expr
    // - this temp var lives as long as r
    let r = &factorial(6);

    // - compilr creates a temp var for `&1009` and it lives as long as the enclosing block
    // - arithmetic operator see through one level of reference
    assert_eq!(r + &1009, 1729);

    let x = 1000;
    let rx = &x;
    let rrx = &rx;
    // - see through one level of ref
    let y = rx + 1;
    // - can not see through two levels so we must manually deref
    // let z = rrx + 1;
    let z = *rrx + 1;
}
