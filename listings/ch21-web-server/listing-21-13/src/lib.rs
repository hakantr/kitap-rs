pub struct IsParcacigiHavuzu;

// ANCHOR: here
impl IsParcacigiHavuzu {
    /// Yeni bir IsParcacigiHavuzu olusturur.
    ///
    /// Boyut, havuzdaki is parcacigi sayisidir.
    ///
    /// # Panics
    ///
    /// `new` fonksiyonu, boyut sifirsa panikler.
    pub fn new(boyut: usize) -> IsParcacigiHavuzu {
        assert!(boyut > 0);

        IsParcacigiHavuzu
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
