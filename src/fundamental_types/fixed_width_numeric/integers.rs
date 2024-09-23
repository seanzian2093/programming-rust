// Make it pub so we can use it in other modules
pub fn reprocude_examples() {
    println!("\nConvert from one integer to another integer type using `as` operator");

    print!(
        "u16 is between {} and {}",
        u16::min_value(),
        u16::max_value()
    );
    assert_eq!(10_i8 as u16, 10_u16);
    println!(", so 10_i8 as u6 is: 10_u16");

    print!(
        "i16 is between {} and {}",
        i16::min_value(),
        i16::max_value(),
    );
    assert_eq!(2525_u16 as i16, 2525_i16);
    println!(", so 2525_u16 as i16 is: 2525_i16");

    print!(
        "i32 is between {} and {}",
        i32::min_value(),
        i32::max_value(),
    );
    assert_eq!(-1_i16 as i32, -1_i32);
    println!(", so -1_i16 as i32 is: -1_i32");

    assert_eq!(65535_u16 as i32, 65535_i32);
    println!(", so 65535_u16 as i32 is: 65535_i32");
}
