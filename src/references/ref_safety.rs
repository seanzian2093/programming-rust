// `static` modifier creates a variable that is global in lifetime, not visibility

// - `static` must be initialized
static mut STASH: &i32 = &128;
static WORTH_POINTING_AT: i32 = 1000;

// - mutable `static` is not safe so must be used within a unsafe block, which makes `f` unsafe

// - `fn f(p: &i32)`` is a shorthand, with help from Rust, for `fn f<'a> (p: &'a i32)`
// - `static` variable's life time is `static` so p must live as long
// - add lifetime parameter to limit that only those with `static` lifetime arguments are accepted
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

// `a could be anylife that encloses the call of g
fn g<'a>(p: &'a i32) -> i32 {
    p + 1
}

// a function return a ref to the smallese element of a slice
// - when a function takes only one ref and return one ref, compiler assume they have same lifetime if omitted
// - `fn<'a> smallest(v: &'a [i32]) -> &'a i32 {}`
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

pub fn ref_safety() {
    let x = 10;
    // receiving ref as argument
    // - not working, since &x does not live as long as `static` which is required by `f`
    // f(&x);
    f(&WORTH_POINTING_AT);
    unsafe {
        println!("{}", STASH);
    }

    // passing ref to functions
    // - the lifetime `'a` define in `g` must not live long than `x`
    // - shortest choice of lifetime is the lifetime of call of `g`
    g(&x);
}

pub fn return_ref() {
    // returning ref
    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        // `parabola` is dropped here, so ref `&parabola` live not beyond this block
        // `smallest` return value must live as long as `s`
        // lifetime of this block could be that `'a`
        assert_ne!(*s, 0);
    }
    // `smallest` return value must live as long as `s`
    // - not working since no such lifetime `'a` that satisfies both constraints
    // assert_ne!(*s, 0);
}

pub fn struct_containing_ref() {
    // whenver a ref type appears inside another type's definition, we must write out its lifetime
    // - declare `'a` in struct name before use it to fields
    // - constraint: the ref '&i32' > `'a` > `S` where `>` mean operand's lifetime encloses or outlast
    struct S<'a> {
        r: &'a i32,
    }

    let s;
    {
        let x = 10;
        s = S { r: &x };
        assert_eq!(*s.r, 10)
    }
    // x needs to live here for s to be used
    // assert_eq!(*s.r, 10)
}
