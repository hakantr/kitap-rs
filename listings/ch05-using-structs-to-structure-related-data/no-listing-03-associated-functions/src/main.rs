#[derive(Debug)]
struct Dikdortgen {
    genislik: u32,
    yukseklik: u32,
}

// ANCHOR: here
impl Dikdortgen {
    fn kare(boyut: u32) -> Self {
        Self {
            genislik: boyut,
            yukseklik: boyut,
        }
    }
}
// ANCHOR_END: here

fn main() {
    let k = Dikdortgen::kare(3);
}
