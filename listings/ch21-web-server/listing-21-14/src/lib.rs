// ANCHOR: here
use std::thread;

pub struct IsParcacigiHavuzu {
    threadler: Vec<thread::JoinHandle<()>>,
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

        let mut threadler = Vec::with_capacity(boyut);

        for _ in 0..boyut {
            // thread'leri oluştur ve vektörde sakla
        }

        IsParcacigiHavuzu { threadler }
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
// ANCHOR_END: here
