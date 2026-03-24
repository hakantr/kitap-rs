pub trait Ciz {
    fn ciz(&self);
}

// ANCHOR: here
pub struct Ekran {
    pub bilesenler: Vec<Box<dyn Ciz>>,
}
// ANCHOR_END: here
