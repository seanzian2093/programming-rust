// Make it pub so we can use it in other modules
pub fn convert_integer_in_range() {
    println!("\nConvert from one integer to another integer type using `as` operator");

    // Conversion in range
    println!("Conversion in range");
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
    println!("  , so 65535_u16 as i32 is: 65535_i32");
}

pub fn convert_integer_out_of_range() {
    println!("\nConvert from one integer to another integer type using `as` operator");
    // Conversion out of range
    println!("\nConversion out of range");
    print!("u8 is between {} and {}", u8::min_value(), u8::max_value());
    assert_eq!(1000_i16 as u8, 232_u8);
    println!(
        " so 1000_i16 as u8 is 232_u8, because 1000 - 256*3 = {}",
        1000 - 256 * 3
    );

    print!(
        "i16 is between {} and {}",
        i16::min_value(),
        i16::max_value()
    );
    assert_eq!(65535_u32 as i16, -1_i16);
    println!("so 65535_u32 as i16 is -1_i16");

    print!("i8 is between {} and {}", i8::min_value(), i8::max_value());
    assert_eq!(-1_i8 as u8, 255_u8);
    println!("so -1_i8 as u8 is 255_u8");

    // Operations
    // - use method
    assert_eq!(2_u16.pow(4), 16);
    // - method's precedence is higher that arithmetic operation so use parenthesis
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);
    // - or use associated function
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));
}

pub fn check_arithmetic_methods() {
    let mut i: i32 = 1;
    loop {
        print!("{} -> ", i);
        // checked arithmetic method return an `Option`
        if let Some(res) = i.checked_mul(10) {
            i = res
        } else {
            println!(
                "### mul operation result overflowed - max of i32 is: {} ###",
                i32::MAX
            );
            break;
        };
    }
}

pub fn wrapping_arithmetic_methods() {
    // Wrapping operations return the value equivalent to the mathematically correct result modulo the range of the type
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    let wrapped_res = (500_u32 * 500_u32 % 2_u32.pow(16)) as u16;
    println!(
        "\nu16 max is {}, 500*500=250000, so 250000 % {} is {}",
        u16::MAX,
        u16::MAX,
        wrapped_res
    );
    assert_eq!(500_u16.wrapping_mul(500), wrapped_res);

    // Operations on signed types may wrap to negative
    assert_eq!(500_i16.wrapping_mul(500), -12144);
}

pub fn saturating_arithmetic_methods() {
    // Saturating operations return the representable value that is closest to the mathematically correct result

    println!(
        "\ni16 max is {}, 32760_i16 + 10=32770, but is clamped to {}",
        i16::MAX,
        i16::MAX,
    );
    assert_eq!(32760_i16.saturating_add(10), 32767);

    println!(
        "i16 min is {}, -32760_i16 - 10 = -32770, but is clamped to {}",
        i16::MIN,
        i16::MIN,
    );
    assert_eq!((-32760_i16).saturating_sub(10), -32768);
}

pub fn overflowed_arithmetic_methods() {
    // Overflowing operations return a tuple (result, overflowed) where
    // - result is the result of wrapping version
    // - overflowed is a bool indicating whether an overflow occurred
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
}
