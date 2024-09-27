fn call_twice<F>(closure: F)
where
    // Fn() is short for `Fn() -> ()`
    F: Fn(),
{
    closure();
    closure();
}

fn call_twice2<F>(mut closure: F)
where
    F: FnMut(),
{
    closure();
    closure();
}
pub fn closure_safety() {
    let my_str = "hello".to_string();

    // f is a type `impl FnOnce()` which only be called once
    // - becasue it drops value
    let f = || drop(my_str);
    // try call f twice - not working
    // f();
    // f();

    let mut i = 0;
    // incr is `impl FnMut()`, which includes `Fn()`
    let incr = || {
        i += 1;
        println!("i is now: {}", i);
    };

    // try use `call_twice` - not working because `call_twice` takes only `Fn`, but f is `FnOnce()`
    // call_twice(f);

    // try use `call_twice` - not working because `call_twice` takes only `Fn`, but incr is `FnMut()`
    // call_twice(incr);

    // works with `call_twice2` which takes `FnMut`
    call_twice2(incr);
}
