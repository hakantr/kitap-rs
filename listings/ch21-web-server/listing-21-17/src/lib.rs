use std::{sync::mpsc, thread};

pub struct IsParcacigiHavuzu {
    calisanlar: Vec<Calisan>,
    gonderici: mpsc::Sender<Gorev>,
}

struct Gorev;

// ANCHOR: here
impl IsParcacigiHavuzu {
    // --snip--
    // ANCHOR_END: here
    /// Yeni bir IsParcacigiHavuzu olusturur.
    ///
    /// Boyut, havuzdaki is parcacigi sayisidir.
    ///
    /// # Panics
    ///
    /// `new` fonksiyonu, boyut sifirsa panikler.
    // ANCHOR: here
    pub fn new(boyut: usize) -> IsParcacigiHavuzu {
        assert!(boyut > 0);

        let (gonderici, alici) = mpsc::channel();

        let mut calisanlar = Vec::with_capacity(boyut);

        for kimlik in 0..boyut {
            calisanlar.push(Calisan::new(kimlik, alici));
        }

        IsParcacigiHavuzu { calisanlar, gonderici }
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

// --snip--

// ANCHOR_END: here

struct Calisan {
    kimlik: usize,
    thread: thread::JoinHandle<()>,
}

// ANCHOR: here
impl Calisan {
    fn new(kimlik: usize, alici: mpsc::Receiver<Gorev>) -> Calisan {
        let thread = thread::spawn(|| {
            alici;
        });

        Calisan { kimlik, thread }
    }
}
// ANCHOR_END: here
