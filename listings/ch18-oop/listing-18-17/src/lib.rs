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
        self.durum.as_ref().unwrap().icerik(self)
    }
    // --snip--
    // ANCHOR_END: here

    pub fn inceleme_iste(&mut self) {
        if let Some(s) = self.durum.take() {
            self.durum = Some(s.inceleme_iste())
        }
    }

    pub fn onayla(&mut self) {
        if let Some(s) = self.durum.take() {
            self.durum = Some(s.onayla())
        }
    }
    // ANCHOR: here
}
// ANCHOR_END: here

trait Durum {
    fn inceleme_iste(self: Box<Self>) -> Box<dyn Durum>;
    fn onayla(self: Box<Self>) -> Box<dyn Durum>;
}

struct Taslak {}

impl Durum for Taslak {
    fn inceleme_iste(self: Box<Self>) -> Box<dyn Durum> {
        Box::new(IncelemeBekleyen {})
    }

    fn onayla(self: Box<Self>) -> Box<dyn Durum> {
        self
    }
}

struct IncelemeBekleyen {}

impl Durum for IncelemeBekleyen {
    fn inceleme_iste(self: Box<Self>) -> Box<dyn Durum> {
        self
    }

    fn onayla(self: Box<Self>) -> Box<dyn Durum> {
        Box::new(Yayinlanmis {})
    }
}

struct Yayinlanmis {}

impl Durum for Yayinlanmis {
    fn inceleme_iste(self: Box<Self>) -> Box<dyn Durum> {
        self
    }

    fn onayla(self: Box<Self>) -> Box<dyn Durum> {
        self
    }
}
