pub mod copy_types;
pub mod moves;

// so in `main`` we can use `ownership_move::*` as if those are defined in `ownership_move` to shorten import
pub use copy_types::*;
pub use moves::*;
