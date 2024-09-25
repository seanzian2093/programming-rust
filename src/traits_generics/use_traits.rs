// A writer without caring about its type can be written like below
use std::fs::File;
use std::io::Write;
// `&mut dyn Write` means a mutable ref to any value that implements the `Write` trait
// so `out` is trait object so this function is a plain function as opposed to generic function
// multiple traits like `&mut dyn Write + Debug + Hash` is not supported
fn say_hello_p(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

// A generic function - use `where` clause or not
// fn say_hello_g<W: Write>(out: &mut W) -> std::io::Result<()> {
fn say_hello_g<W>(out: &mut W) -> std::io::Result<()>
where
    W: Write,
{
    out.write_all(b"hello world\n")?;
    out.flush()
}

// Generics is preferred -
// - easy to bound a generic type parameter with multiple traits at once
// - fast because types are specified, explicitly or inferred, at compile time
// - not every trait supports trait object

// A trait can sue `Self` as type
// - but would be incompatible with trait object
trait Splicable {
    fn splice(&self, other: &Self) -> Self;
}
// - not working, since `Splicable` can be use as trait object
// fn splice_anthing(left: &dyn Splicable, right: &dyn Splicable) {
//     todo!()
// }

// Or we could design as below
trait MegaSplicable {
    fn splice(&self, other: &dyn MegaSplicable) -> Box<dyn MegaSplicable>;
}
fn splice_anthing(left: &dyn MegaSplicable, right: &dyn MegaSplicable) {
    todo!()
}

pub fn use_traits() -> std::io::Result<()> {
    // Rust automatically convert ordinary ref to trait object
    let mut local_file = File::create("hello.txt")?;
    say_hello_p(&mut local_file)?;
    say_hello_g(&mut local_file)?;

    let mut bytes = vec![];
    say_hello_p(&mut bytes)?;
    assert_eq!(bytes, b"hello world\n");
    say_hello_g(&mut bytes)?;

    // Rust does not permit var of type `dyn Write` because its size is unknown at compile time
    // let writer: dyn Write = bytes;
    // so we must use ref, which size is known at compile time
    let writer: &mut dyn Write = &mut bytes;
    writer.write_all(b"hello again\n");

    println!("{:?}", bytes);

    Ok(())
}
