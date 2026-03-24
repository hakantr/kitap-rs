use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// ANCHOR: here
pub struct IsParcacigiHavuzu {
    calisanlar: Vec<Calisan>,
    gonderici: Option<mpsc::Sender<Gorev>>,
}
// --snip--
// ANCHOR_END: here

type Gorev = Box<dyn FnOnce() + Send + 'static>;

// ANCHOR: here
impl IsParcacigiHavuzu {
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
        // --snip--

        // ANCHOR_END: here
        assert!(boyut > 0);

        let (gonderici, alici) = mpsc::channel();

        let alici = Arc::new(Mutex::new(alici));

        let mut calisanlar = Vec::with_capacity(boyut);

        for kimlik in 0..boyut {
            calisanlar.push(Calisan::new(kimlik, Arc::clone(&alici)));
        }

        // ANCHOR: here
        IsParcacigiHavuzu {
            calisanlar,
            gonderici: Some(gonderici),
        }
    }

    pub fn calistir<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let gorev = Box::new(f);

        self.gonderici.as_ref().unwrap().send(gorev).unwrap();
    }
}

impl Drop for IsParcacigiHavuzu {
    fn drop(&mut self) {
        drop(self.gonderici.take());

        for calisan in self.calisanlar.drain(..) {
            println!("Çalışan kapatılıyor: {}", calisan.kimlik);

            calisan.thread.join().unwrap();
        }
    }
}
// ANCHOR_END: here

struct Calisan {
    kimlik: usize,
    thread: thread::JoinHandle<()>,
}

impl Calisan {
    fn new(kimlik: usize, alici: Arc<Mutex<mpsc::Receiver<Gorev>>>) -> Calisan {
        let thread = thread::spawn(move || loop {
            let gorev = alici.lock().unwrap().recv().unwrap();

            println!("Çalışan {kimlik} bir görev aldı; çalıştırılıyor.");

            gorev();
        });

        Calisan { kimlik, thread }
    }
}
