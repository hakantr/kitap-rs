pub trait Ciz {
    fn ciz(&self);
}

// ANCHOR: here
pub struct Ekran<T: Ciz> {
    pub bilesenler: Vec<T>,
}

impl<T> Ekran<T>
where
    T: Ciz,
{
    pub fn calistir(&self) {
        for bilesen in self.bilesenler.iter() {
            bilesen.ciz();
        }
    }
}
// ANCHOR_END: here
