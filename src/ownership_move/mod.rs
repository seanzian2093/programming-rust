// we do not need these mods to be pub since we have re-export all its children to `ownership_move`
mod copy_types;
mod moves;

// so in `main`` we can use `ownership_move::*` as if those are defined in `ownership_move` to shorten import
pub use copy_types::*;
pub use moves::*;
