trait Visible {
    fn show(&self) {
        todo!();
    }
}

// `trait Creature: Visible` means that every type that implement `Creature` must also implement `Visible`
trait Creature: Visible {
    fn position(&self) {
        todo!();
    }
}

struct Broom {
    shape: String,
    length: u32,
}

// We can impl in arbitrary order but both of `Creature` and `Visible`
impl Creature for Broom {
    fn position(&self) {
        todo!();
    }
}

impl Visible for Broom {
    fn show(&self) {
        todo!();
    }
}
