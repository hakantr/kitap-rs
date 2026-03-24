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
}

trait Durum {}

struct Taslak {}

impl Durum for Taslak {}
