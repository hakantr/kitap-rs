// ANCHOR: here
struct BenimKutu<T>(T);

impl<T> BenimKutu<T> {
    fn yeni(x: T) -> BenimKutu<T> {
        BenimKutu(x)
    }
}
// ANCHOR_END: here

fn main() {}
