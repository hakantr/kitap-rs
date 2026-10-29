pub struct Gonderi {
    durum: Option<Box<dyn Durum>>,
    icerik: String,
}

// ANCHOR: here
impl Gonderi {
    // --snip--
    // ANCHOR_END: here
    pub fn yeni() -> Gonderi {
        Gonderi {
            durum: Some(Box::new(Taslak {})),
            icerik: String::new(),
        }
    }

    // ANCHOR: here
    pub fn metin_ekle(&mut self, metin: &str) {
        self.icerik.push_str(metin);
    }
}
// ANCHOR_END: here

trait Durum {}

struct Taslak {}

impl Durum for Taslak {}
