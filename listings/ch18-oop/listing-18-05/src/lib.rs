pub trait Ciz {
    fn ciz(&self);
}

pub struct Ekran {
    pub bilesenler: Vec<Box<dyn Ciz>>,
}

// ANCHOR: here
impl Ekran {
    pub fn calistir(&self) {
        for bilesen in self.bilesenler.iter() {
            bilesen.ciz();
        }
    }
}
// ANCHOR_END: here
