// ANCHOR: here
use std::thread;

pub struct IsParcacigiHavuzu {
    calisanlar: Vec<Calisan>,
}

impl IsParcacigiHavuzu {
    // --snip--
    // ANCHOR_END: here
    /// Yeni bir IsParcacigiHavuzu oluşturur.
    ///
    /// Boyut, havuzdaki thread sayısıdır.
    ///
    /// # Panics
    ///
    /// `new` fonksiyonu, boyut sıfırsa panikler.
    // ANCHOR: here
    pub fn new(boyut: usize) -> IsParcacigiHavuzu {
        assert!(boyut > 0);

        let mut calisanlar = Vec::with_capacity(boyut);

        for kimlik in 0..boyut {
            calisanlar.push(Calisan::new(kimlik));
        }

        IsParcacigiHavuzu { calisanlar }
    }
    // --snip--
    // ANCHOR_END: here

    pub fn calistir<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
    // ANCHOR: here
}

struct Calisan {
    kimlik: usize,
    thread: thread::JoinHandle<()>,
}

impl Calisan {
    fn new(kimlik: usize) -> Calisan {
        let thread = thread::spawn(|| {});

        Calisan { kimlik, thread }
    }
}
// ANCHOR_END: here
