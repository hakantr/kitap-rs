pub struct IsParcacigiHavuzu;

// ANCHOR: here
impl IsParcacigiHavuzu {
    // --snip--
    // ANCHOR_END: here
    pub fn new(boyut: usize) -> IsParcacigiHavuzu {
        IsParcacigiHavuzu
    }

    // ANCHOR: here
    pub fn calistir<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
// ANCHOR_END: here
