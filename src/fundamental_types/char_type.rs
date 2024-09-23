pub fn convert_char() {
    // `char` type represents a single Unicode character, as a 32-bit value, e.g. 8 bytes
    // - `String` is a sequence of UTF-8 bytes, not an array of `char`
    println!("{} as i32 is {}", '*', '*' as i32);
    println!("{} as i32 is {}", '好', '好' as i32);

    // Useful methods on characters in `std` library
    println!("{} is alphabetic: {}", '*', '*'.is_alphabetic());
    println!("{} is alphabetic: {}", '好', '好'.is_alphabetic());
    println!("{} is digit: {:?}", '8', '8'.is_digit(10));

    fn demo_from_digit(num: u32, radix: u32) {
        println!(
            "char is {:?} from digit {} in radix {}",
            std::char::from_digit(num, radix),
            num,
            radix
        );
    }
    demo_from_digit(2, 10);
    // if resulting in more than one characters, return `None`
    demo_from_digit(10, 10);
    demo_from_digit(13, 16);
    demo_from_digit(13, 15);
}
