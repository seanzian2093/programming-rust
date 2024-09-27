use std::char::MAX;

// std::ops::Drop
// - Rust calls Drop::drop on a value before dropping its fields or elements
// - the the value the is being dropped is still fully initialized
// - mostly we don't need to imple drop ourselves
struct Appellation {
    name: String,
    nick_names: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        // we are dropping self, but still can use its fields or elements
        println!("Dropping {}", self.name);
        if !self.nick_names.is_empty() {
            println!(" (AKA {})", self.nick_names.join(", "));
        }
    }
}

fn use_drop() {
    let mut a = Appellation {
        name: "sean".to_string(),
        nick_names: vec![
            "killer".to_string(),
            "murderer".to_string(),
            "psychopath".to_string(),
        ],
    };
    println!("Before assignment");
    a = Appellation {
        name: "hera".to_string(),
        nick_names: vec![],
    };
    println!("at the end of block");
}

// std::marker::Sized
// - a `Sized` type is oen whose values all have the same size in memory
// - `?Sized` is special syntax meaning `not necessarilly Sized`

// std::clone::Clone
// - `Clone` trait is for types that can make copy of themselves
// - allocating copies of anything it owns so is expensive both in time and memory
// - `clone()`
// - `clone_from()` modifies self into a copy of source
// - both takes `&self` as parameter, i.e., clone value from a ref

// std::marker::Copy
// - Rust permits a type to implement `Copy` only if a shalow byte-for-byte copy is all it needs
// - types that own other resources, like heap buffers, operating system handles, cannot implement Copy
// - Any type that implements `Drop` cannnot be `Copy`

// std::default::Default
// - if a struct's all fields implement `Default` we can `#[derive(Default)]`

// AsRef and AsMut
// - Should avoid defining our own `AsFoo` trait when we could just implement `AsRef<Foo>`

// std::convert::From std::convert::Into
// std::convert::TryFrom std::convert::TryInto
// - may be not cheap, i.e., need to allocate, copy or otherwise process
// - for falliable conversion, use Try* to hanle error

fn use_try_from_into() {
    let huge = 2_000_000_000_000i64;
    let smaller = huge as i32;
    println!("{}", smaller);

    // use TryInto - explicit annotation
    let smaller2: i32 = huge.try_into().unwrap_or(i32::MAX);
    println!("{}", smaller2);
    let smaller3: i32 = huge
        .try_into()
        .unwrap_or_else(|_| if huge >= 0 { i32::MAX } else { i32::MIN });
    println!("{}", smaller3);
}

pub fn use_utility_traits() {
    use_drop();
    use_try_from_into();
}

// std::borrow::ToOwned
// - usual way to produce an owned copy from a ref is call `clone` of `Clone` if the type implement it
// - but `clone` `&str` or `&[i32]`, we would get `str` `[i32]` but useless
// - we most likely want to copy to `String` or `Vec<i32>`

// std::borrow::Cow
// - in some cases, we cannot decide whether to borrow or to own until the programming is running
// - Cow<B> either borrows a shared ref to a B, i.e., &B or
// - owns a value from which we could borrow such a ref, &B

// - to get a mutable ref we use `to_mut` to return `&mut B`
// - if Cow<B> borows a shared, then `to_owned` will be called to get a copy of the value
// - if Cow<B> owns the value, it borrows a mutable ref to it

// - Cow implement Deref, we can use it as it were a shared ref

use std::borrow::Cow;
use std::error::Error;
use std::path::PathBuf;
// fn describe(error: &Error) -> Cow<'static, str> {
//     match *error {
//         Error::FileNotFound(ref path) => format!("file not found: {}", path.display()).into(),
//         _ => "Other error".into(),
//     }
// }
