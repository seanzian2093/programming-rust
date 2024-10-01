// - `map.len()`
// - `map.is_empy()`
// - `map.contains_key(&key)`
// - `map.get(&key)`
// - `map.get_mut(&key)`
// - `map.insert(key, value)`
// - `map.extend(iterable)` - (K, V) items of iterable
// - `map.append(&mut map2)`
// - `map.remove(&key)` - return Option<V>
// - `map.remove_entry(&key)` - return Option<(K, V)>
// - `map.retain(test)` - test must implement `FnMut(&K, &mut V) -> bool`
// - `map.clear()`

// - `map.entry(key)` - like a mutable ref to a place within map that is
// - returns a `Entry` type that represents
// - either occupied by a key-value pari
// - or vacant then `entry.or_*()` can insert a record there

// - `map.entry(key).or_insert(value)`
// - `map.entry(key).or_defaut()`
// - `map.entry(key).or_insert_with(fn)` - fn is a function or closure that generates a record

// - `map.entry(key).and_modify(closre)` - modify extant fields

// - `Entry` is an enum has two variants - `Occupied(OccupiedEntry<'a, K, V>)` and `Vacant(VacantEntry<'a, K, V>)`

// Iteration
// - `for (k, v) in map` - produce (K, V) pairs and consume the map
// - `for (k, v) in &map`- produce (&K, &V) pairs and leave the map intact
// - `for (k, v) in &mut map`- produce (&K, &mut V) pairs and have access to modify values, but not keys
// - `map.iter()`
// - `map.iter_mut()`
// - `map.keys()`
// - `map.values()`
// - `map.values_mut()`
