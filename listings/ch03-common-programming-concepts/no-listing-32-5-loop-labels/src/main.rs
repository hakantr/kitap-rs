fn main() {
    let mut ileri_sayim = 0;
    'yukari_sayma: loop {
        println!("İleri ayım = {ileri_sayim}");
        let mut geri_sayim = 10;

        loop {
            println!("Geri sayım = {geri_sayim}");
            if geri_sayim == 9 {
                break;
            }
            if ileri_sayim == 2 {
                break 'yukari_sayma;
            }
            geri_sayim -= 1;
        }

        ileri_sayim += 1;
    }
    println!("Son İlerleme = {ileri_sayim}");
}
