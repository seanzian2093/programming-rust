// There are times when we need a little bit of mutable data inside an otherwise immutable value
// - a mutable field of a immutable struct
// - we can use `Cell` or `RefCell`

use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct Robot {
    hardware_error_count: Cell<u32>,
    log: RefCell<Vec<String>>,
}

impl Robot {
    // `add_hardware_error` borrows an immutable ref to `self`
    fn add_hardware_error(&self) {
        // - use `Cell<T>`'s `.get()` to retrive value that is stored in a `Cell<T>`
        // - `Cell<T>`'s `.get()` copy the value so <T> must be a `Copy` type
        // - while `RefCell<T>`'s `.get()` return a ref to the value
        let n = self.hardware_error_count.get();
        // - use `Cell`'s `.set(new_value)` to assign a value to `Cell`, dropping previous one
        self.hardware_error_count.set(n + 1);
    }

    fn write_log(&self, entry: &str) {
        // `.borrow()` return a ref to the value, panic if already mutably borrowed
        // `.borrow_mut()` return a mutable ref to the value, panic if already borrowed

        // `.try_borrow()` return a Result<a ref to the value, err>
        // `.try_borrow_mut()` return a Result< a mutable ref to the value, err>

        // - the `mut` keyword is still needed
        let mut prev_log = self.log.borrow_mut();
        // prev_log.push("found an hardware error".to_string());
        prev_log.push(entry.to_string());
    }

    fn print_log(&self) {
        let curr_log = self.log.borrow();
        println!("{:?}", curr_log);
    }
}
pub fn interior_mutability() {
    let robot = Robot {
        hardware_error_count: Cell::new(0),
        log: RefCell::new(vec!["Initialized".to_string()]),
    };
    println!("Robot is: {:?}", robot);
    robot.print_log();

    robot.add_hardware_error();
    robot.write_log("found an hardware error");
    println!("\nRobot is: {:?}", robot);
    robot.print_log();
}
