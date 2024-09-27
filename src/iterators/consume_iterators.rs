// simple accumulation: count, sum, product

use std::cmp::Ordering;
use std::collections::HashMap;

fn triangle(n: u64) -> u64 {
    (1..n).sum()
}

fn factorial(n: u64) -> u64 {
    (1..n).product()
}

fn use_simple_accumulation() {
    println!("factorial(20) is {}", factorial(20));
    println!("triangle(20) is {}", triangle(20));
}

// max, min
// - return Option, None if cannot produce max or min
// - max_by, min_by produce by customized comparision function
// - max_by_key, min_by_key produce by  apply a closure to each item

fn cmp(lhs: &i32, rhs: &i32) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

fn use_max_min() {
    // max, min
    let v = vec![-2, -1, 0, 1, 2, 3];
    println!("{:?} max: {:?}", v, v.iter().max());
    println!("{:?} min: {:?}", v, v.iter().min());

    // max_by, min_by
    println!(
        "{:?} max_by: {:?}",
        v,
        v.iter().max_by(|lhs, rhs| cmp(lhs, rhs))
    );
    println!(
        "{:?} min_by: {:?}",
        v,
        v.iter().min_by(|lhs, rhs| cmp(lhs, rhs))
    );

    // max_by_key, min_by_key
    let mut populations = HashMap::new();
    populations.insert("Portland", 583776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorm", 2);
    populations.insert("Boring", 7762);
    populations.insert("The Dalles", 15340);

    println!(
        "populations {:?} max_by_key pop is: {:?}",
        populations,
        populations.iter().max_by_key(|&(name, pop)| pop)
    );

    println!(
        "populations {:?} min_by_key pop is: {:?}",
        populations,
        populations.iter().min_by_key(|&(name, pop)| pop)
    );
}

// Comparing item sequences
// - iterators do not support comparison operators
// - they provide methods like `eq`,`lt`, `gt`, etc which draw pairs of items from iterators and compare untill a decision can be reached

fn use_comparison() {
    let packed = "Helle of Troy";
    let spaced = "Helle    of Troy";
    let obscure = "Helle of Sandusky";

    // this is apparent - spaced has more spaces
    println!("{} == {}: {}", packed, spaced, packed == spaced);
    // `split_whitespace()` produces an iterator of String, so "Helle" from packed == "Helle" from spaced
    println!(
        "{}.split_whitespace() == {}.split_whitespace(): {}",
        packed,
        spaced,
        packed.split_whitespace().eq(spaced.split_whitespace())
    );

    // ' ' > 'o', so
    println!("{} < {}: {}", spaced, obscure, spaced < obscure);
    // "Troy" > "Sandusky" because 'T' > 'S'
    println!(
        "{}.split_whitespace() > {}.split_whitespace(): {}",
        spaced,
        obscure,
        spaced.split_whitespace().gt(obscure.split_whitespace())
    );
}

// `any` and `all` apply a closure to each item the iterator produces and return true if the closure
// - returns true for any item or
// - returns true for all item

// `position`, `rposition`
// - `position` applies a clousre to each item from the iterator and returns
// - the Some(index) of the first item from which the closure returns true
// - None if no items return true

// - `rposition` is similar but start from right, back, with additional requirements on the iterator
// - reversible, i.e., `std::iter::DoubleEndedIterator` so possible to draw item from right end
// - `std::ExactSizeIterator: Iterator` so possible to assign index as the `position` would

fn use_position() {
    let text = "Xerxes";

    println!(
        "The index of first 'e' in {} is: {:?}",
        text,
        text.chars().position(|c| c == 'e')
    );

    println!(
        "The index of first 'z' in {} is: {:?}",
        text,
        //`chars()` return an iterator over `char`, not `&char`
        text.chars().position(|c| c == 'z')
    );

    let text = b"Xerxes";
    println!(
        "The index of first b'e' from right in {:?} is: {:?}",
        text,
        // `iter()` return an iterator over `&u8`, not `u8`
        text.iter().rposition(|&c| c == b'e')
    );

    println!(
        "The index of first b'z' from right in {:?} is: {:?}",
        text,
        text.iter().rposition(|&c| c == b'z')
    );
}

// `fold` and `rfold`, `try_fold` and `try_rfold`
// - takes an initial value, i.e. the accumulator, and a closure,
// - repeatedly applies the closure to current accumulator and next item of the iterator
// - return value of the closure is taken as new/next accumulator

// - if the iterator is empty, returns the initial value
// - `rfold` start from right
// - reversible, i.e., `std::iter::DoubleEndedIterator` so possible to draw item from right end

// - `try_fold` is similar but may the process may exit early with consuming all values of iterator
// - the closure we provide must return `Result` or `Option`
// - if the closure returns `Err(e)`, then `try_fold` return immediately `Err(e)`

fn use_fold() {
    let a = [5, 6, 7, 8, 9, 10];
    println!("Sum of {:?} is: {}", a, a.iter().fold(0, |n, i| n + i));
    println!("Count of {:?} is: {}", a, a.iter().fold(0, |n, _| n + 1));
    println!("Product of {:?} is: {}", a, a.iter().fold(1, |n, i| n * i));

    println!("Sum of {:?} is: {}", a, a.iter().rfold(0, |n, i| n + i));
    println!("Count of {:?} is: {}", a, a.iter().rfold(0, |n, _| n + 1));
    println!("Product of {:?} is: {}", a, a.iter().rfold(1, |n, i| n * i));
}

// `nth` and `nth_back()`, `last`
// - takes an index n, skips and discards n items from the iterator and return next item
// - None if the sequence ends before that point
// - `nth(0)` is equivalent to `next()`
// - `nth(0)` will produce different result just as `next()`

// - `last` consumes the iterator and returns the last item in the iterator
// - even if the iterator is reversible
// - `iter.next_back()` if you need to iterator for later use
fn use_nth() {
    let mut squares = (0..10).map(|n| n * n);
    println!("{:?}", squares);
    println!("squares[4] is: {:?}", squares.nth(4));
    println!("squares[6] is: {:?}", squares.nth(6));

    let mut squares = (0..10).map(|n| n * n);
    println!("squares.last() is: {:?}", squares.last());
}

// `find`, `rfind` and `find_map`
// - `find` draws  items from iterator, returns first item for which the given closure returns true on it
// - `find` returns NOne if sequence ends before a true is found
// - `find_map` takes a closure that returns not a `bool` but a `Option`, `find_map` returns the first `Option` that is a `Some`
fn use_find() {
    let mut populations = HashMap::new();
    populations.insert("Portland", 583776);
    populations.insert("Markham", 553776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorm", 2);
    populations.insert("Boring", 7762);
    populations.insert("The Dalles", 15340);

    // there are two generic type parameters in `hash_map::Iter`, in this case they are `&str`, `i32`?
    // - `iter` borrows, so we have `&&str, &i32`
    // - `find` borrows again and pack all parameter to a tuple as one parameter, so we have `&(&&str, &i32)`
    let _res = populations.iter();
    // - in pattern matching, use one & ( more than one & is supported)
    // - so double deref to get i32
    let res1 = populations.iter().find(|&(&_k, &v)| v > 1_000_000);

    // working
    // let res1 = populations.iter().find(|e| *e.1 > 1_000_000);
    // let res1 = populations.iter().find(|&(city, &pop)| pop > 1_000_000);
    println!(
        "`find()` found a city with population > 1_000_000: {:?}",
        res1
    );

    println!(
        "`find()` found a city with population > 500_000: {:?}",
        populations.iter().find(|&(city, &pop)| pop > 500_000)
    );
}

// `collect`
// - natually `collect` does not know how to construct target type
// - if a type, i.e., Vec or HashMap, knows how to construct itself from a iterator, it implement `std::iter::FromIterator` trait

// `extend`
// - if a type implement `std::iter::Extend` trait, its `extend` method adds an iteratable's items to its collection
// - all standard collections implement `Extend`, including `String`
// - fixed length type collections like `array`, `slice` do not

fn use_rfind() {
    // `HashMap::Iter` does not implement `DoubleEndedIterator`
    // `Vec<>::Iter` does
    let mut populations = Vec::new();
    populations.push(("Portland", 583776));
    populations.push(("Markham", 553776));
    populations.push(("Fossil", 449));
    populations.push(("Greenhorm", 2));
    populations.push(("Boring", 7762));
    populations.push(("The Dalles", 15340));

    // `iter` borrows and `rfind` borrows again, so e is `&&(&str, i32)`
    // let res1 = populations.iter().rfind(|&&e| e.1 > 1_000_000);

    let _res = populations.iter();
    let res1 = populations.iter().rfind(|&&(city, pop)| pop > 1_000_000);
    println!(
        "`rfind()` found a city with population > 500_000: {:?}",
        populations.iter().rfind(|&&(city, pop)| pop > 500_000)
    );
}

// `partition`
// - requires result type to implement `std::default::Default` and `Extend`
fn use_partition() {
    let mut populations = Vec::new();
    populations.push(("Portland", 583776));
    populations.push(("Markham", 553776));
    populations.push(("Fossil", 449));
    populations.push(("Greenhorm", 2));
    populations.push(("Boring", 7762));
    populations.push(("The Dalles", 15340));

    let (big_cities, small_cities): (Vec<(&str, i32)>, Vec<(&str, i32)>) =
        populations.iter().partition(|&&t| t.1 > 500_000);

    println!("Big cities are: {:?}", big_cities);
    println!("Small cities are: {:?}", small_cities);
}

// `for_each` and `try_for_each`
// - simply apply a closure to item of an iterator
// - fallible or exitable earliy with `try_for_each`

pub fn consume_iterators() {
    // use_simple_accumulation();
    // use_max_min();
    // use_comparison();
    // use_position();
    // use_fold();
    // use_nth();
    // use_find();
    use_partition();
}
