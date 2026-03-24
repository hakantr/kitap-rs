pub struct Gonderi {
    durum: Option<Box<dyn Durum>>,
    icerik: String,
}

impl Gonderi {
    pub fn yeni() -> Gonderi {
        Gonderi {
            durum: Some(Box::new(Taslak {})),
            icerik: String::new(),
        }
    }

    pub fn metin_ekle(&mut self, text: &str) {
        self.icerik.push_str(text);
    }

    pub fn icerik(&self) -> &str {
        self.durum.as_ref().unwrap().icerik(self)
    }

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
}

// ANCHOR: here
trait Durum {
    // --snip--
    // ANCHOR_END: here
    fn inceleme_iste(self: Box<Self>) -> Box<dyn Durum>;
    fn onayla(self: Box<Self>) -> Box<dyn Durum>;

    // ANCHOR: here
    fn icerik<'a>(&self, post: &'a Gonderi) -> &'a str {
        ""
    }
}

// --snip--
// ANCHOR_END: here

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

// ANCHOR: here
struct Yayinlanmis {}

impl Durum for Yayinlanmis {
    // --snip--
    // ANCHOR_END: here
    fn inceleme_iste(self: Box<Self>) -> Box<dyn Durum> {
        self
    }

    fn onayla(self: Box<Self>) -> Box<dyn Durum> {
        self
    }

    // ANCHOR: here
    fn icerik<'a>(&self, post: &'a Gonderi) -> &'a str {
        &post.icerik
    }
}
// ANCHOR_END: here
