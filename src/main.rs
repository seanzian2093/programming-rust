// No need to use `pub`
// `mod xxx` cause Rust to load either `xxx/mod.rs` or `xxx.rs`
// - `xxx/mod.rs` loads submodules from `xxx/submodule.rs`
mod enums;
mod expression;
mod fundamental_types;
mod ownership_move;
mod references;
mod structs;

use {
    enums::*,
    enums::*,
    expression::*,
    fundamental_types::{
        array_vec_slice::*, bool_type::*, char_type::*, fixed_width_numeric::*, string_types::*,
    },
    // ownership_move::{copy_types::*, moves::*},
    ownership_move::*,
    references::{ref_safety::*, *},
    structs::*,
};
fn main() {
    // convert_integer_in_range();
    // convert_integer_out_of_range();
    // check_arithmetic_methods();
    // wrapping_arithmetic_methods();
    // saturating_arithmetic_methods();
    // overflowed_arithmetic_methods();
    // bool_to_integer();
    // convert_char();
    // use_array();
    // use_vector();
    // use_slice();
    // use_string_literals();
    // strings_in_memory();
    // strings();
    // move_operations();
    // move_control_flow();
    // move_indexed_content();
    // copy_types();
    // ref_to_values();
    // ref_to_ref();
    // ref_to_expr();
    // ref_safety();
    // control_flow_in_loop();
    // interior_mutability();
    match_patterns();
}
