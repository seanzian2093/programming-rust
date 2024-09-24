use std::path::Components;

// Move leaves the source uninitialized, not deleted or dropped
// so using uninitialized is not allowed but we can reassign to it
pub fn move_operations() {
    let mut s = "Govinda".to_string();
    let t = s;
    // s is moved by assigning to t so we cannot use it
    // println!("{}", s);
    // but we can reassign to s
    s = "Siddhartha".to_string();
    println!("{}", s);
}

pub fn move_control_flow() {
    let mut x = vec![1, 2, 3];
    fn f(x: Vec<i32>) -> Vec<i32> {
        x
    };
    fn g(x: Vec<i32>) -> Vec<i32> {
        x
    };
    let c: bool = false;
    if c {
        // x would be moved here
        f(x)
    } else {
        // x would be moved here too - but it is ok since only one branch will be executed
        g(x)
    };

    x = vec![1, 2, 3];
    while c {
        // not working if just `f(x)` alone - x would be moved in first iteration
        f(x);
        // but working if we give x new value after move
        x = vec![1, 1, 1];
    }
}

pub fn move_indexed_content() {
    // Not every kind of owner is prepared to become uninitialized
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // move an element of vec is not allowed - because it is indexed content
    // - compiler does now allow one element of a vector become uninitialized
    // let third = v[2];

    // Ways to move an element out of a vector

    // - pop the last element
    let fifth = v.pop().expect("empty vector");
    println!("Fifth element of v is moved: {}", fifth);

    // - swap_move, i.e., move out an element of a given index and move last element into its spot
    let second = v.swap_remove(1);
    println!(
        "Second element of v is moved and swapped: {}, now second is {}, last is {}",
        second, &v[1], &v[2]
    );

    // - use std::mem::replace
    let third = std::mem::replace(&mut v[2], "src".to_string());
    println!("Third element of v is swapped: {}, now v is {:?}", third, v);

    // Collection types like Vec also offer methods to consume their elements in a loop
    let v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    // - `for loop` moves all element from v so v is uninitialized
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
    // - we cannot use v after the for loop
    // println!("now v is {:?}", v);

    let mut v2 = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    // - use `&v` to make `for loop` borrow all element from v so v will not become uninitialized
    // - more use `&v mut` since we intend to mutate the elements
    for mut s in &mut v2 {
        s.push('!');
        println!("{}", s);
    }
    // - we cannot use v after the for loop
    println!("now v2 is {:?}", v2);

    // Use `Option` to dynamically track movedness
    struct Person {
        name: Option<String>,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("sean".to_string()),
        birth: 1983,
    });

    composers.push(Person {
        name: Some("lucia".to_string()),
        birth: 1988,
    });

    // we cannot move out element of a vec, even just for its field
    // let first_name = composers[0].name;
    // - we can replace it with None since it is a Option
    let first_name = std::mem::replace(&mut composers[0].name, None);
    assert_eq!(first_name, Some("sean".to_string()));
    assert_eq!(composers[0].name, None);

    // `Option` provides a method `take` to this same purpose
    let second_name = composers[1].name.take();
    assert_eq!(second_name, Some("lucia".to_string()));
    assert_eq!(composers[1].name, None);
}
