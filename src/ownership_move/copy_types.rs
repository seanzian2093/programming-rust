// Any type that needs to do something special when a value of this type is dropped, can not be a `Copy`
// - a Vec needs to free its elements
// - a File nees to close its file handle
// - a MutexGuard needs to unlock its Mutex
// - by default `struct` and `enum` are not `Copy`
pub fn copy_types() {
    // `Label` by default is not a `Copy` type
    struct Label {
        number: i32,
    }

    // `print` takes ownership of its argument
    fn print(l: Label) {
        println!("Stamp: {}", l.number);
    }

    let l = Label { number: 3 };
    // `l` is moved here
    print(l);
    // cannot be used again
    // println!("{}", l.number);

    // if all fields are `Copy` type, e.g., i32, we can make it a `Copy` by adding #[derive(Copy, Clone)]
    #[derive(Copy, Clone)]
    struct Label2 {
        number: i32,
    }

    fn print2(l2: Label2) {
        println!("Stamp: {}", l2.number);
    }

    let l2 = Label2 { number: 3 };
    // `l` is copied here
    print2(l2);
    println!("{}", l2.number);
}
