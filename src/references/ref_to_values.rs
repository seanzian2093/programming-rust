type Table = std::collections::HashMap<String, Vec<String>>;

// show_into takes ownership
fn show_into(table: Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

// show_into takes ownership
fn show(table: &Table) {
    // iterating over a &HashMap is defined to produce ref to each key and value, i.e., (&key, &value)
    // - so (&String, &Vec<String>), and similarly a iterating over &Vec<String> produces each &String
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

pub fn ref_to_values() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebre Responsoria".to_string(),
        ],
    );

    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The musicians".to_string(),
            "The calling of St. Matthew".to_string(),
        ],
    );

    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    let table2 = table.clone();
    // table is moved
    show_into(table);
    // cannot be used
    // println!("{:?}", table);

    show(&table2);
    // cannot be used
    println!("\n{:?}", table2);
}
