// A set never contains multiple copies of same value

// - `HashSet::new()`
// - `iter.collect()`
// - `HashSet::with_capacity(n)`
// - `set.len()`
// - `set.is_empty()`
// - `set.contains(&value)` - much faster than `vec.contains(&value)`
// - `set.insert(value)` - return true if value was added and false if already a member
// - `set.remove(&value)`
// - `set.retain(test)`

// Set iteration
// - `for v in set` and `for v in &set`
// - `set.iter()`

// When equal values are different
// - `set.get(&value)` - return a ref to member that is equal to `value`
// - `set.take(&value)` - similar to remove, but return removed value if any
// - `set.replace(value)` - similar to insert but if set already contains a value that equal to `value`, replaces and return the old one

// Whole-set operations
// - `set1.intersection(&set2)` - returns an iterator of all values in both `set1` and `set2`
// - `set1.union(&set2)`
// - `set1.difference(&set2)` - return an iterator of values in `set1` but not `set2`
// - `set1.symmetirc_difference(&set2)` - return an iterator of values either in `set1` or `set2` but not both
// - `set1.is_disjoint(set2)`
// - `set1.is_subset(set2)`
// - `set1.is_superset(set2)`
