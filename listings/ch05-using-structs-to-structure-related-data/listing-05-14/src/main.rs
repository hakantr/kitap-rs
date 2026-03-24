fn main() {
    let dikdortgen1 = Dikdortgen {
        genislik: 30,
        yukseklik: 50,
    };
    let dikdortgen2 = Dikdortgen {
        genislik: 10,
        yukseklik: 40,
    };
    let dikdortgen3 = Dikdortgen {
        genislik: 60,
        yukseklik: 45,
    };

    println!(
        "dikdortgen1, dikdortgen2'yi kapsayabilir mi? {}",
        dikdortgen1.kapsayabilir_mi(&dikdortgen2)
    );
    println!(
        "dikdortgen1, dikdortgen3'ü kapsayabilir mi? {}",
        dikdortgen1.kapsayabilir_mi(&dikdortgen3)
    );
}
