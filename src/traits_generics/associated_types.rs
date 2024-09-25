// we are defining a function to print out all values produced by an iterator
// - to use `{:?}` on value produced by iterator, value must implement `Debug`
// -  in trait `Iterator`, the associated type is named `Item`
use std::fmt::Debug;

fn dump<I>(iter: I)
where
    I: Iterator,
    // - this means `Item` of `I` implements `Debug`
    I::Item: Debug,
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

// we can we can limit what types `I` can be
fn dump2<I>(iter: I)
where
    // - `Iterator<Item = String>` can be seen as a subset of `Iterator` filtered by `<Item=String>`
    // - `Iterator<Item = String>` is itself a trait
    I: Iterator<Item = String>,
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

// - `Iterator<Item = String>` is itself a trait, e.g., could be used for trait object

fn dump3(iter: &mut dyn Iterator<Item = String>) {
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}
