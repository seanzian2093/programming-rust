pub fn use_array() {
    // Annotate type and length - provide elements directly
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    assert_eq!(lazy_caterer[3], 7);

    // Infer - provide elements by repeating
    let a = [9; 2];

    // Let compiler infer
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(taxonomy.len(), 3);

    // Useful methods
    let mut chaos = [2, 5, 4, 1, 3];

    // `sort` takes argument of `&slice` but compiler converts `array` to `&slice` implicitly
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

pub fn use_vector() {
    // Use `vec!` macro to instantiate - provide elements directly
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    // Vector is growable
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    // Use `vec!` macro to instantiate - provide elements by repeating
    let v = vec![1; 9];
    assert_eq!(v.iter().sum::<i32>(), 9);

    // Useful methods - methods on slice
    // - compiler implicitly borrows a `&mut[&T]` from the vector and call `reverse`
    primes.reverse();
    println!("{:?}", primes);

    // `len()` vs `capacity()`
    println!(
        "{:?} has {} elements, but can hold {}",
        primes,
        primes.len(),
        primes.capacity()
    );

    // `insert` and `remove` - shift all elemetns so could be slow if vector is long
    primes.insert(0, 1);
    println!("\nAfter insert, primes is now {:?}", primes);

    primes.remove(1);
    println!("\nAfter remove, primes is now {:?}", primes);

    // `pop` remove last element and return an `Option<T>` - Some(last element) or None
}

pub fn use_slice() {
    // [T] without a length, is a region of an array or vector
    // - can be any length so can't be stored direcly in a var or passed as function argument
    // - always passed by ref, i.e., &[T], e.g., &[str]

    // a ref to a slice is a fat pointer - one word for pointer to its first element, another for length

    // function takes a slice ref as argument can apply to vector or array
    let a = [1, 2, 3, 4];
    let v = vec![1, 2, 3, 4];

    fn print(n: &[i32]) {
        for elt in n {
            println!("{}", elt);
        }
    }
    print(&a);
    print(&v);
}
