pub struct Gonderi {
    icerik: String,
}

pub struct TaslakGonderi {
    icerik: String,
}

impl Gonderi {
    pub fn yeni() -> TaslakGonderi {
        TaslakGonderi {
            icerik: String::new(),
        }
    }

    pub fn icerik(&self) -> &str {
        &self.icerik
    }
}

impl TaslakGonderi {
    pub fn metin_ekle(&mut self, text: &str) {
        self.icerik.push_str(text);
    }

    pub fn inceleme_iste(self) -> IncelemeBekleyenGonderi {
        IncelemeBekleyenGonderi {
            icerik: self.icerik,
        }
    }
}

pub struct IncelemeBekleyenGonderi {
    icerik: String,
}

impl IncelemeBekleyenGonderi {
    pub fn onayla(self) -> Gonderi {
        Gonderi {
            icerik: self.icerik,
        }
    }
}
