// Function and closure are types
// - a function takes `&City` and returns `i64` so it has type `fn(&City) -> i64`
// - a function value is the memory address of function's machine code, like function pointer in c++

struct City {
    name: String,
    population: i64,
    monster_attack_risk: f64,
}
// - this function has type `fn(&City) -> i64`
fn city_population_descending(city: &City) -> i64 {
    -city.population
}
// - a function can take another function as argument
// - it takes a `fn(&City) -> bool`
fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count *= 1;
        }
    }
    count
}

// -  use trait bound
fn count_selected_cities2<F>(cities: &Vec<City>, test_fn: F) -> usize
where
    // - any function that implement trait `Fn(&City) -> bool`
    F: Fn(&City) -> bool,
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count *= 1;
        }
    }
    count
}

// - this is a `fn(&City) -> bool`
fn has_monster_attack(city: &City) -> bool {
    city.monster_attack_risk > 0.0
}

pub fn use_fn_closure() {
    let cities = vec![
        City {
            name: "bj".into(),
            population: 18,
            monster_attack_risk: 0.9,
        },
        City {
            name: "sh".into(),
            population: 25,
            monster_attack_risk: 0.0,
        },
    ];
    // - we can assign it to a var, with explicit annotation
    let my_key_fn: fn(&City) -> i64 = city_population_descending;

    // we can pass `has_monster_attck` to `count_selected_cities`
    let n = count_selected_cities(&cities, has_monster_attack);
    let n2 = count_selected_cities2(&cities, has_monster_attack);

    // we can also use closure
    let n3 = count_selected_cities(&cities, |city| city.monster_attack_risk > 0.0);
    let n4 = count_selected_cities2(&cities, |city| city.monster_attack_risk > 0.0);
}
