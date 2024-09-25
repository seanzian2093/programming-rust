use std::fmt::{Debug, Display};
use std::iter;
use std::ops::Add;
use std::vec::IntoIter;

// Return type coulc get messy when we use consolidate type
fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
    let res = v.into_iter().chain(u.into_iter()).cycle();
    res
}

// We could use trait object if we are willing to pay the cost of
// - dynamic dispatch
// - heap allocation because it is a Box

fn cyclical_zip2(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item = u8>> {
    let res = v.into_iter().chain(u.into_iter()).cycle();
    Box::new(res)
}

// Or we could use `impl Trait` feature
// - means return type is any type that implement `Iterator<Item = u8>`
// - statically dispatched - at compile time, return type is has to be determined
// - so compiler know the space to allocation for it
fn cyclical_zip3(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
    let res = v.into_iter().chain(u.into_iter()).cycle();
    Box::new(res)
}

trait Shape {
    fn new() -> Self;
    fn area(&self) -> f64;
}

// - below does not work because the return type has to be determined at compile time, not run time
// fn make_shape(shape: &str) -> impl Shape {
//     match shape {
//         "circle" => Circle::new(),
//         "triangle" => Triangle::new(),
//         "rectangle" => Rectangle::new(),
//         _ => Square::new(),
//     }
// }

// `impl Trait` can also be used in fucntions that take generic arguemnts

fn print<T: Display>(val: T) {
    println!("{}", val);
}
fn print2(val: impl Display) {
    println!("{}", val);
}

// Trait can have associated constatns, similar to struct and enum
// - we can declare without initialize
trait Greet {
    const GREETING: &'static str = "Hello";
    fn greet(&self) -> String;
}

trait Float {
    const ZERO: Self;
    const ONE: Self;
}

// impl for certain type
impl Float for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

// write generic code with those consts
fn add_one<T: Float + Add<Output = T>>(value: T) -> T {
    value + T::ONE
}
