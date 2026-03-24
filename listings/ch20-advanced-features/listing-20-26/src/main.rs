fn main() {
    // ANCHOR: here
    type ErtelenenIs = Box<dyn Fn() + Send + 'static>;

    let f: ErtelenenIs = Box::new(|| println!("selam"));

    fn takes_long_type(f: ErtelenenIs) {
        // --snip--
    }

    fn returns_long_type() -> ErtelenenIs {
        // --snip--
        // ANCHOR_END: here
        Box::new(|| ())
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
