fn main() {
    let dikdortgen1 = (30, 50);

    println!("Dikdörtgenin alanı {} kare pikseldir.", alan(dikdortgen1));
}

fn alan(boyutlar: (u32, u32)) -> u32 {
    boyutlar.0 * boyutlar.1
}
