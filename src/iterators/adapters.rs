use std::collections::HashMap;
use std::iter::Peekable;
// `map` and `filter` returns `std::iter::Map` and `std::iter::Filter` type
// - we could write `impl Iterator<Item=..,>` instead to inform user what we want
// - calling an adapter on iterator does not consume any item but returns a new iterator, ready to produce
// - only way to make work actually done is to call `next` on the final iterator
// - explicitly or by `collect()` alike

// `filter_map` - transform or drop
// - i.e., return zero or one item per iteration,
// - while `map` return one item per iteration
fn use_filter_map() {
    use std::str::FromStr;
    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        // process if Some() and drop if None
        .filter_map(|w| f64::from_str(w).ok())
    {
        println!("{:4.2}", number.sqrt());
    }
}

// `flat_map` - simiarly but return a sequence of any number of items
fn use_flat_map() {
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("US", vec!["New York", "Boston"]);
    major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);

    let countries = ["Japan", "US", "Brazil"];

    // order is not guaranteed
    let cities = countries
        .iter()
        // in each iteration, an internal city iterator is created and its elements are exhausted one by one
        // - return type of the closure must implement `IntoInterator`, i.e., iteratable
        // - in this case, a Vec<&str>
        .flat_map(|country| &major_cities[country])
        .collect::<Vec<_>>();
    println!("{:?}", cities);
}

// `flatten`
fn use_flatten() {
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("US", vec!["New York", "Boston"]);
    major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);

    let countries = ["Japan", "US", "Brazil"];

    // order is not guaranteed
    // in each iteration, an internal city iterator is created and its elements are exhausted one by one
    // - the element of calling iterator, i.e. `major_cities.values()`, must implement `IntoInterator`, i.e., iteratable
    // - in this case, a Vec<&str>
    let cities = major_cities.values().flatten().collect::<Vec<_>>();
    println!("{:?}", cities);
}

// `flat_map` is equivalent to `map` + `flatten`
fn to_uppercase(s: &str) {
    let res = s
        .chars()
        .map(char::to_uppercase)
        .flatten()
        .collect::<Vec<_>>();

    let res2 = s.chars().flat_map(char::to_uppercase).collect::<Vec<_>>();
    println!("{:?} vs {:?}", res, res2);
}

// `take` and `take_while` -
fn use_take() {
    let vec: Vec<f64> = std::iter::from_fn(|| Some(0.1)).take(10).collect();
    println!("{:?}", vec);
}

fn use_take_while() {
    let vec: Vec<f64> = std::iter::successors(Some(1.0), |&z| Some(z + z * 0.05))
        .take_while(|&n| n < 2.0)
        .collect();
    println!("{:?}", vec);
}

// `skip` and `skip_while` - complement to `take` and `take_while`

// `peekable` - a peekable iterator lets we peek at the next item that will be produced without actually consuming it
fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
where
    I: Iterator<Item = char>,
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n,
        }
        tokens.next();
    }
}
fn use_peek() {
    let mut chars = "226153980,1766319049".chars().peekable();
    println!("{}", parse_number(&mut chars));
    println!("{:?}", chars.next());
    println!("{}", parse_number(&mut chars));
}

// `fuse` takes an iterator and prodoces one that will definitely continue to return `None` once has done so the first time
// - i.e., ends after first None
fn use_fuse() {
    struct Flaky(bool);

    impl Iterator for Flaky {
        type Item = &'static str;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some("totaly the last item")
            } else {
                self.0 = true;
                None
            }
        }
    }

    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totaly the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totaly the last item"));

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totaly the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);
}

// Reversible Iterator and rev
// - `std::iter::DoubleEndedIterator` trait, requiring `Iterator`
// - `next` draws elemetn from the one end
// - `next_back` draws elemetn from the other end
// - if a `Iterator` is also a `DoubleEndedIterator`, we can use `rev` method to reverse it

fn use_reverse() {
    let meals = ["breakfast", "lunch", "dinner"];
    println!("{:?}", meals);
    let rev_meals = meals.iter().rev().collect::<Vec<_>>();
    println!("{:?}", rev_meals);

    let mut rev_meals_iter = rev_meals.iter();
    let mut meals_iter = meals.iter();
    println!(
        "{:?} == {:?}",
        rev_meals_iter.next(),
        meals_iter.next_back()
    );
    println!(
        "{:?} == {:?}",
        rev_meals_iter.next(),
        meals_iter.next_back()
    );
    println!(
        "{:?} == {:?}",
        rev_meals_iter.next(),
        meals_iter.next_back()
    );
}

// `inspect` - apply a closure to a ref to each item of iterator and pass through
// - no modification, only print or assertation etc
fn use_inspect() {
    let upper_case = "Größe"
        .chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("After: {:?}", c))
        .collect::<String>();
}

// `chain` - chain an iterator together with any iterable that prodocues the same item type
// - result is `rev`ersible if both of its underlying iterators are
fn use_chain() {
    let v = (1..4).chain(vec![4, 5, 6, 7]).collect::<Vec<_>>();
    println!("{:?}", v);

    let v2 = (1..4).chain(vec![4, 5, 6, 7]).rev().collect::<Vec<_>>();
    println!("{:?}", v2);
}

// `enumerate`, `zip`
// `by_ref` - adapters take ownership of calling iterator and provide no method to give it back
// - use `by_ref` to borrow a ref to the iterator so we can apply adapters to the ref
// - after the adapter is dropped, we regain access to the original iterator, modified though

// `cloned` and `copied`
// - `cloned` equivalent to `iter.map(|item| item.clone())`, `item` must implement `Clone`
// - `copyd` equivalent to `iter.map(|item| item.copy())`, `item` must implement `Copy`

// `cycle`
// - returns an iterator that endlessly repeats the sequence produced by calling iterator
// - calling iterator must implement `Clone` for `cycle` to resue it

fn use_by_ref() {
    let message = "To: jimb\r\nFrom: id\r\n\r\nOoooh, donuts!!\r\n";

    let mut lines = message.lines();
    println!("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }

    println!("Body:");
    // pick up the `lines` as what is left
    for body in lines {
        println!("{}", body);
    }
}

pub fn use_adapters() {
    use_filter_map();
    use_flat_map();
    use_flatten();
    to_uppercase("abcdef");
    use_take();
    use_take_while();
    use_peek();
    use_fuse();
    use_reverse();
    use_inspect();
    use_chain();
    use_by_ref();
}
