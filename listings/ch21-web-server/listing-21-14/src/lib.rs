// ANCHOR: here
use std::thread;

pub struct IsParcacigiHavuzu {
    is_parcacigis: Vec<thread::JoinHandle<()>>,
}

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

        let mut is_parcacigis = Vec::with_capacity(boyut);

        for _ in 0..boyut {
            // create some is_parcacigis and store them in the vector
        }

        IsParcacigiHavuzu { is_parcacigis }
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
