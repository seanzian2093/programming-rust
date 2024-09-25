// Type-associated functions, aka static methods in other languages
// any type that implements `StringSet` must implement all the type-associated functions

trait StringSet {
    // `new` and `from_slice` does not take `&self` or `self` as parameter
    // - similarly in struct
    fn new() -> Self;
    fn from_slice(strings: &[&str]) -> Self;
    // `contains` and `add` take `self` or `&self` as parameter, they are associated functions too
    // - unsimilarly in struct
    fn contains(&self, string: &str) -> bool;
    fn add(&mut self, string: &str);
}

// but above `StringSet` is incompatible with trait object because `new` and `from_slice` use `Self`
// - to remedy add `Sized` to Self
trait StringSet2 {
    fn new() -> Self
    where
        Self: Sized;
    fn from_slice(strings: &[&str]) -> Self
    where
        Self: Sized;
    fn contains(&self, string: &str) -> bool;
    fn add(&mut self, string: &str);
}
fn use_taf() {
    // we cannot instantiate a trait object from `StringSet`
    // let ss: &dyn StringSet;
    // we can instantiate a trait object from `StringSet2`
    let ss2: &dyn StringSet2;
}
