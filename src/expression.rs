pub fn control_flow_in_loop() {
    // a `break` can be followed by an expression whose result will be the value of the loop
    let mut a = 0;
    let s = loop {
        if a == 9 {
            break a;
        } else {
            a += 1;
        }
    };
    println!("{}", s);
}
