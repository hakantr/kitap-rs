use merhaba_makro::MerhabaMakro;

struct Krepler;

impl MerhabaMakro for Krepler {
    fn merhaba_makro() {
        println!("Merhaba, Makro! Benim adım Krepler!");
    }
}

fn main() {
    Krepler::merhaba_makro();
}
