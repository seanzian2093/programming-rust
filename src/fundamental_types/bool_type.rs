pub fn bool_to_integer() {
    // `as` operator can convert `bool` type to `integers` but not verse versa
    println!("\n{} as i32 is {}", false, false as i32);
    assert_eq!(false as i32, 0);
    println!("{} as i32 is {}", true, true as i32);
    assert_eq!(true as i32, 1);
}
