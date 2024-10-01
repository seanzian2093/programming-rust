// Accessing element - works for array, slice, and vector
// - `[]` indexing - panic if out of bound
// - `first` - return Option<&T>, None if caller is empty
// - `last` - return Option<&T>, None if caller is empty
// - `get(index)` - return Option<&T>, Some() if exits None if not
// - `first_mut`, `last_mut`, `get_mut` return Option<&T> instead
// - `to_vec` clone and return a new vector Vec<T> where T: Clone

// Growing and shrinking vecotrs
// - `iter`, `iter_mut`
// - `len`
// - `is_empty`

// `Vec` specific methods
// - `Vec::with_capacity(n)`
// - `vec.capacity()`
// - `vec.reserve(n)` - make sure that vec has at least enough spare capacity for n more elements
// - `vec.reserve_exact(n)` - similar but no extra capacity,i.e. afterward, vec.capacity() = vec.len() + n
// - `vec.shrink_to_fit()` - free up extra memory if any so that capacity = len

// - `vec.push(value)` - add value to end of vec

// - `vec.pop()` - remove last element and return it
// - `vec.insert(index, value)` - insert vaue at index, sliding existing value in vec[index..] to right; panic if index > vec.len()
// - `vec.remove(index)` - remove the value at index, sliding existing value in vec[index+1..] to left; panic if index > vec.len()
// - `insert` and `remove` are slower the more elements to be shifted

// - `vec.resize(new_len, value)` - set vec 's length to new_len, if this increases the length, copies of value are added to fill the new spaces
// - type must implement Clone

// - `vec.resize_with(new_len, closure)` - set vec 's length to new_len, if this increases the length, call closure to construct new values to fill the new spaces
// - type does not need to implement Clone

// - `vec.truncate(new_len)` - reduce length to new_len, dropping any elements in vec[new_len..]
// - `vec.clear()` - remove all elements, equivalaent to `vec.truncate(0)`

// - `vec.extend(iteratable)` - add all items from `iteratable` at the end of `vec`
// - `vec.split_off(index)` - like `vec.truncate(index)` except that
// - it returns a `Vec<T>` containing the values that are removed from the end of `vec`
// - a multicaitve version of `vec.pop()`

// - `vec.append(&mut vec2)` - moves all elements in `vec2` into `vec` where vec2 is same type
// - similar to `vec.extend(vec2)` except `vec2` still exists afterward but empty with unaffected capacity

// - `vec.drain(range)` removes vec[range] from vec and returns an iterator over theremoved elements

// - `vec.retain(test)` - selective removes all elements that don't pass the test; test is a `impl FnMut(&T) -> bool`

// - `vec.dedup()` - remove duplicates where equality is checked by `==`
// - `vec.dedup_by(same)` - remove duplicates where equality is checked by the closure provided same(&mut elem1, &mut elem2)
// - `vec.dedup_by_key(key)` - remove duplicates where equality is checked by the closure provided key(&mut elem1) == key(&mut elem2)

// Joining - works for array, slice, vectors whose elements are array, slice, or vector
// - `slices.concat()` - returns a new vector by concatenating all slices
fn joining_collections() {
    let slices = [[1, 2], [3, 4], [5, 6]];
    let res = slices.concat();
    println!("{:?}.concat() produces: {:?}", slices, res);

    let res = slices.join(&0);
    println!("{:?}.join(&0) produces: {:?}", slices, res);
}

// Splitting
// - `slice.iter()`, `slice.iter_mut()`
// - `slice.iter()`
// - `slice.split_at(n)` - return two parts, first part has n element
// - `slice.split_first()`
// - `slice.split_last()`
// - `slice.split(|&x| x == 0)`
// - `slice.split(2, |&x| x == 0)` - split into 2 parts by first true condition
// - `slice.rsplit(2, |&x| x == 0)` - split into 2 parts by first true condition, from end
// - `slice.chunks(2)` - each chunk has 2 element
// - `slice.windows(4)` - return multiple parts, first is element from [0..4], 2nd is [1..5], etc

// Swapping
// - `slice.swap(i,j)` - swaps two element slice[i] and slice[j]
// - `slice_a.swap(&mut slice_b)` - swaps entire contents of slice_a and slice_b, must be same length
// - `vec.swap_remove(i)` - `vec.remove(i)` except that instead of sliding elements, moves last element to the gap

// Sorting and search
// - `slice.sort()` - sort slice into increasing order; element type must implement `Ord`

// - `slice.sort_by(cmp)` - sort slice by `cmp` which must implement `Fn(&T, &T)-> std::cmp::Ordering`
// - easier to delegate to a `.cmp()` method than hand-implement `cmp`

// - `slice.sort_by_key(key)` - sort to increasing order by key which must implement `Fn(&T) -> K where K: Ord`
// - sort key values are not cached; may be calculated more than n time
// - k cannot return any ref to element of slice

// - `slice.reverse()` reverse clide in place

// - once a slice is sorted, it can be searched efficiently
// - `slice.binary_seach(&value)`
// - `slice.binary_seach_by(&value, cmp)`
// - `slice.binary_seach_by_key(&value, key)`

// - return `Result<usize, usize>`, Ok(index) if slice[index] equals to value under specified sort order
// - return Err(insertion_point) such that inserting the value at insertion_point would preserve the order
// - if slice is not sorted, the return value is arbitrary

// - f64 and f32 have `Nan` values which do not implement `Ord` and cannot be used direcly as sort key
// - use `ord_subset` crate

// - `vec.contains(&value)` searches a vector that is not sorted

// - or more generally use `slice.iter().position(test)` to find index of elements that pass the test

// Comparing slices
// - if type T supports `==` and `!=` operators, i.e. `PartialEq` trait then we can compare array, slice, vector of T
// - `slice.starts_with(other)`
// - `slice.ends_with(other)`

// Random element
// - `slice.choose(&mut rng)`  - rng is a random number generator, could be provide by `rand` crate
// - `slice.shuffle(&mut rng)`

fn sort_collections() {
    #[derive(Debug)]
    struct Student {
        first_name: String,
        last_name: String,
    }

    let mut students = vec![
        Student {
            first_name: "Tom".into(),
            last_name: "Hardy".into(),
        },
        Student {
            first_name: "Sean".into(),
            last_name: "Zian".into(),
        },
        Student {
            first_name: "Tom".into(),
            last_name: "Cruise".into(),
        },
    ];

    println!("Before sorting, students is: {:?}", students);
    // use `.cmp` method
    students.sort_by(|a, b| a.first_name.cmp(&b.first_name));
    println!("Aftersorting by first_name, students is: {:?}", students);

    // use a second field as a tiebreaker
    students.sort_by(|a, b| {
        let a_key = (&a.first_name, &a.last_name);
        let b_key = (&b.first_name, &b.last_name);
        a_key.cmp(&b_key)
    });
    println!(
        "Aftersorting by first_name and last_name, students is: {:?}",
        students
    );
}
pub fn use_collections() {
    joining_collections();
    sort_collections();
}
