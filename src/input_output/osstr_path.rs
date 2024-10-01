// All `str`, `OsStr`, and `Path` implement a common trait `AsRef<Path>`
// - we can use it to define generic function that accepts any of three types
// - owned, growable and heap-allcoated equivalent - `String`, `OsString`, `PathBuf`
// - covert to owned type by `to_string`, `to_os_string`, and `to_path_buf`

// - `str` for actual Unicode strings,
// - `OsStr` for operating system
// - `std::path:Path` for filenames

// Path and PathBuf
// - `Path::new(str)` - does not copy, returns a `&Path` pointing to the same address as `&str`
// - `OsStr::new(str)` is similar

// - `path.parent()` - return `Option<&Path>`
// - `path.file_name()` - return `Option<&OsStr>`, last component of `path`
// - `path.is_absolute()`, `path.is_relative()`
// - `path.join(path2)`
// - `path.components()` - return an iterator over the components of the given path, left to right,
// - return type `std::path::Component`, `Prefix`, `RootDir`, `CurDir`, `ParentDir`, `Normal`

// - `path.ancestors()` - returns an iterator that walks from path up to the root, each item is a `Path`

// - `path.to_str()` - return `Option<&str>`, if `path` is not valid utf-8, return None
// - `path.display()` - works with `println`, `format!`

// `std::fs`
// - `create_dir(path)` - mkdir
// - `create_dir_all(path)` - mkdir -p
// - `remove_dir(path)` - rm
// - `remove_dir_all(path)` - rm -r
// - `remove_file(path)`
// - `copy(src_path, dest_path)`
// - `rename(src_path, dest_path)`
// - `read_dir(path)` - similar to `path.read_dir()` - return type is `std::fs::DirEntry`
// - `entry.file_name()`
// - `entry.path()`
// - `entry.file_type()` - return`std::io::Result<FileType>` which has `is_file` ,`is_dir` and `is_symlink` methods

use std::ffi::OsStr;
use std::path::Path;
fn use_path() {
    let string = "/home/fwolfe";
    let home_dir = Path::new(string);
    println!("{}", string);

    let home_dir2 = OsStr::new(string);
    println!("{}", string);
}

pub fn use_osstr_path() {
    use_path();
}
