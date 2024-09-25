use core::num;

fn match_number(n: i32) {
    match n {
        // `match` arms move or copy the var that is matched, depending on if it is a `Copy` type or not
        0 => {
            return ();
        }
        1 => println!("one"),
        n => println!("{}", n),
        _ => panic!("what?"),
    }

    println!("{}", n);
}

fn match_tuple(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;
    let res = match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        _ => "somewhere else",
    };

    println!("{} and {}", x, y);
    res
}

fn match_array(hsl: [u8; 3]) -> [u8; 3] {
    let res = match hsl {
        [_, _, 0] => [0, 0, 0],
        [_, _, 255] => [255, 255, 255],
        _ => [1, 1, 1],
    };

    // array is a `Copy` type
    println!("{:?}", hsl);
    res
}

fn match_slice(names: &[&str]) {
    match names {
        // `&&str` is `Copy` type
        [] => println!("hello nobody"),
        [a] => println!("hello, {}", a),
        [a, b] => println!("hello, {} and {}", a, b),
        [a, .., b] => println!("hello, everyone from {} to {}", a, b),
    }

    println!("{:?}", names);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    u: i32,
    p: i32,
}

fn match_struct(p: Point) {
    match p {
        // `i32` is `Copy` type
        // use `..` to disregard all other fields
        Point {
            x: 0, y: height, ..
        } => println!("straight up {} meters", height),
        // use shorthand if we use same var names as the corresponding fields
        Point { x, y, .. } => println!("at {}m, {}m", x, y),
    }

    // struct by default is not a `Copy` type but its fields are all `Copy` type so not moved?
    println!("{:?}", p);
}

#[derive(Debug)]
struct Account {
    name: String,
    language: String,
}

fn match_ref(a: Account) {
    match a {
        Account {
            // Account's fileds are not a `Copy` type so
            // a would be partially moved in matching pattern `Account` without `ref`, then a will not be usable
            // name,
            // language,

            // use `ref` to borrow in matching pattern `Account`, instead of moving
            // - `ref mut` to borrow mutably
            ref name,
            ref language,
        } => {
            // so a is usable here
            println!("{}: {} in {:?}", name, language, a);
        }
    }
}

fn match_ampesand(p: &Point) {
    match p {
        // p is `&Point`, when matching to pattern `&Point`, x and y are `i32`
        // - `i32` is `Copy` type so they are copied
        &Point {
            x: 0, y: height, ..
        } => println!("straight up {} meters", height),
        &Point { x, y, .. } => println!("at {}m, {}m", x, y),
    }

    println!("{:?}", p);
}

fn match_ampesand2(p: &Point) {
    match p {
        // - p is `&Point`, so when matching to pattern `Point`,  x and y are `&i32`
        Point {
            x: 0, y: height, ..
        } => println!("straight up {} meters", height),
        Point { x, y, .. } => println!("at {}m, {}m", x, y),
    }

    println!("{:?}", p);
}

fn match_ampesand3(a: &Account) {
    // a is `&Account`, when matching to pattern `Account`, name and language is a ref to its fields, i.e., `&String`
    match a {
        Account { name, language } => {
            // so a is usable here
            println!("{}: {} in {:?}", name, language, a);
        }
    }
}

fn match_ampesand4(a: &Account) {
    match a {
        &Account {
            // a is `&Account`, when matching to pattern `&Account`, name and language would be `String` if without `ref`
            // name,
            // language,

            // use `ref` to borrow in matching patterning, instead of moving
            // - `ref mut` to borrow mutably
            // - name and language is a ref to its fields, i.e., `&String`
            ref name,
            ref language,
        } => {
            // so a is usable here
            println!("{}: {} in {:?}", name, language, a);
        }
    }
}

fn match_guard(num: Option<i32>, special: i32) {
    match num {
        Some(v) if v == special => println!("A special number: {}", v),
        Some(v) => println!("Oh it is just {}", v),
        _ => println!("Nothing to see here"),
    }
}

fn match_multiple_possibilities(c: Option<char>) {
    match c {
        Some('\r') | Some('\n') | None => println!("white space"),
        _ => println!("Non-white space"),
    }
}

fn match_range(c: char) {
    match c {
        // end-inclusive - different from the slice range syntax
        '0'..'9' => println!("Digits"),
        'a'..'z' | 'A'..'Z' => println!("Letters"),
        ' ' | '\n' | '\r' => println!("White spaces"),
        _ => println!("Punctuations?"),
    }
}

fn match_binding_with_at(a: Account) {
    let e = "english".to_string();
    let c = "chinese".to_string();

    match &a {
        // with `var @ pattern` we can only create a single var and move/copy matched value to it
        // method call is not allowed
        acc @ Account {
            name: _,
            language: e,
        } => println!("An account in {}", acc.language),

        acc @ Account {
            name: _,
            language: c,
        } => println!("An account in {}", acc.language),
    }
}

pub fn match_patterns() {
    match_number(0);
    match_number(1);
    match_number(2);

    println!("{}", match_tuple(0, 0));
    println!("{}", match_tuple(1, 0));
    println!("{}", match_tuple(0, 1));
    println!("{}", match_tuple(1, 1));
    println!("{}", match_tuple(-1, 1));

    match_struct(Point {
        x: 1,
        y: 1,
        z: 1,
        u: 1,
        p: 9,
    });
    match_struct(Point {
        x: 0,
        y: 2,
        z: 1,
        u: 1,
        p: 9,
    });

    println!("{:?}", match_array([0, 0, 0]));
    println!("{:?}", match_array([0, 0, 255]));
    println!("{:?}", match_array([1, 2, 5]));

    match_slice(&["sean"]);
    match_slice(&["sean", "lucia"]);
    match_slice(&["sean", "lucia", "emma"]);

    match_ref(Account {
        name: "sean".into(),
        language: "english".into(),
    });

    match_guard(Some(8), 8);
    match_guard(Some(1), 8);
    match_guard(None, 8);

    match_multiple_possibilities(Some('c'));
    match_multiple_possibilities(Some('\n'));
    match_multiple_possibilities(Some('\r'));
    match_multiple_possibilities(None);

    match_range('1');
    match_range('c');
    match_range('C');
    match_range('\r');
    match_range(',');
}
