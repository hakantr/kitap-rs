pub trait Ciz {
    fn ciz(&self);
}

pub struct Ekran {
    pub bilesenler: Vec<Box<dyn Ciz>>,
}

impl Ekran {
    pub fn calistir(&self) {
        for bilesen in self.bilesenler.iter() {
            bilesen.ciz();
        }
    }
}

// ANCHOR: here
pub struct Dugme {
    pub genislik: u32,
    pub yukseklik: u32,
    pub etiket: String,
}

impl Ciz for Dugme {
    fn ciz(&self) {
        // bir dugmeyi gercekten cizmek icin kod
    }
}
// ANCHOR_END: here
