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
}
