// An `iterator` is any value that implement `std::iter::Iterator`
// - a type that implement `std::iter::IntoIterator` can
// - call `into_iter` method to consume iteself and return an iterator

// Many collection types provide `iter` and `iter_mut` that
// - return an iterator of ref/mutable ref to its element

// Use `IntoIterator` in trait bound to restrict the generic type to
// - types that can be iterated over
// - or further require it to produce a target type U by `IntoIterator<Item=U>`

// `std::iter::from_fn` returns an iterator that calls the function to produce its items indefinitely
// - item is an Option
// -  use `take` to take first 100 items
fn use_from_fn() {
    let vec: Vec<f64> = std::iter::from_fn(|| Some(0.1)).take(10).collect();
    println!("{:?}", vec);
}

// use `std::iter::successors` if each item depends on previous item
fn use_successors() {
    let vec: Vec<f64> = std::iter::successors(Some(0.0), |&z| Some(z * z + 0.01))
        .take(10)
        .collect();
    println!("{:?}", vec);
}

// `from_fn` and `successors` also accpet `FnMut` closure which can capture and modify variable from surrounding scopes
fn fibonacci() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    std::iter::from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

// `drain` method
fn use_drain() {
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    println!("outer is now:{}", outer);
    println!("inner is now:{}", inner);
}

pub fn use_iterators() {
    use_from_fn();
    use_successors();

    // use `collect::<Vec<_>>` to let Rust infer the type
    let vec = fibonacci().take(10).collect::<Vec<_>>();
    println!("{:?}", vec);

    use_drain();
}
