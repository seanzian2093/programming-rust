pub fn use_string_literals() {
    // a string literal can span multiple lines
    println!(
        "a
    b
    c"
    );

    // if one line of string ends with a backslash, then newline and leading whitespace on next line are dropped

    println!(
        "a \
        b \
          c",
    );

    println!("a b c");

    // raw string
    println!(r"\\\\\\n\t\b are verbatim, i.e., not escaped");
    // use `###` to contro start and end of raw string
    println!(
        r###"
        This is raw string started with 'r###'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three pound signs ('###'):
    "###
    );
}

pub fn byte_strings() {
    // a string literal with `b` prefix is byte string, a slice of `u8` values, bytes, rather than Unicode text
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    // byte string can use all other string syntax - multiple lines, escape sequences, backslash to join lines
    // - raw byte string start with `br`
}

pub fn strings_in_memory() {
    // Rust strings are sequences of Unicode characters but are not stored in memory as arrays of `char`s
    // they are stored in memory using UTF-8, a variable-width encoding - `Vec<u8>` each element holds well-formed UTF-8
    // - ASCII character are stored in one byte, other maybe in multiple bytes
    // `&str` can be thought of `&[u8]` each holds well-formed UTF-8

    // so `String` or `&str` `len()` methods return length measured in bytes, not character
    let str = "こんにちは";
    let string = str.to_string();

    println!("{} has {} bytes", string, string.len());
    println!(
        "{} has {} bytes, but {} characters",
        string,
        string.len(),
        string.chars().count(),
    );

    println!("{} has {} bytes", str, str.len());
    println!(
        "{} has {} bytes, but {} characters",
        str,
        str.len(),
        str.chars().count(),
    );
}

pub fn strings() {
    // `.to_owned()` and `.to_string()` return a new `String` by copying
    let str = "こんにちは";
    let string = str.to_string();

    // `format!()` macro works like `println!()` except it returns a new `String`, does not take ownership
    let trans = format!("{} means good morning", str);
    println!("{}", trans);

    // array, slices and vectors of string have `concat()` and `join()` that form a new `String` from many
    let bits = vec!["vendi", "vidi", "vici"];
    println!("concat produces: {}", bits.concat());
    println!("join produces: {}", bits.join("\\"));

    // equality - same order and same character
    println!(
        "{} == {} is {}",
        "One".to_lowercase(),
        "one",
        "One".to_lowercase() == "one"
    );

    assert!("peanut".contains("nut"));
    println!("{} contains {}", "peanut", "nut");

    assert_eq!("peanut".replace("nut", "nutbutter"), "peanutbutter");
    assert_eq!("  clean\n\t\r".trim(), "clean");
    assert!("vidi".starts_with("v"));
}
