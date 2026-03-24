use bir_ekle;
use rand;

fn main() {
    let sayi = 10;
    println!(
        "Merhaba, dünya! {} artı bir {} eder!",
        sayi,
        bir_ekle::bir_ekle(sayi)
    );
}
