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

    pub fn metin_ekle(&mut self, text: &str) {
        self.icerik.push_str(text);
    }

    // ANCHOR: here
    pub fn icerik(&self) -> &str {
        ""
    }
}
// ANCHOR_END: here

trait Durum {}

struct Taslak {}

impl Durum for Taslak {}
