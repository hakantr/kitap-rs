use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct IsParcacigiHavuzu {
    calisanlar: Vec<Calisan>,
    gonderici: mpsc::Sender<Gorev>,
}

// ANCHOR: here
// --snip--

type Gorev = Box<dyn FnOnce() + Send + 'static>;

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
    pub fn new(boyut: usize) -> IsParcacigiHavuzu {
        assert!(boyut > 0);

        let (gonderici, alici) = mpsc::channel();

        let alici = Arc::new(Mutex::new(alici));

        let mut calisanlar = Vec::with_capacity(boyut);

        for kimlik in 0..boyut {
            calisanlar.push(Calisan::new(kimlik, Arc::clone(&alici)));
        }

        IsParcacigiHavuzu { calisanlar, gonderici }
    }
    // ANCHOR: here

    pub fn calistir<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let gorev = Box::new(f);

        self.gonderici.send(gorev).unwrap();
    }
}

// --snip--
// ANCHOR_END: here

struct Calisan {
    kimlik: usize,
    thread: thread::JoinHandle<()>,
}

impl Calisan {
    fn new(kimlik: usize, alici: Arc<Mutex<mpsc::Receiver<Gorev>>>) -> Calisan {
        let thread = thread::spawn(|| {
            alici;
        });

        Calisan { kimlik, thread }
    }
}
