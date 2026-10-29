pub struct IsParcacigiHavuzu;

// ANCHOR: here
impl IsParcacigiHavuzu {
    /// Yeni bir IsParcacigiHavuzu oluşturur.
    ///
    /// Boyut, havuzdaki thread sayısıdır.
    ///
    /// # Panics
    ///
    /// `new` fonksiyonu, boyut sıfırsa panikler.
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
