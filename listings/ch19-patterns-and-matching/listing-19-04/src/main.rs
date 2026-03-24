fn main() {
    // ANCHOR: here
    let (gonderici, alici) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for deger in [1, 2, 3] {
            gonderici.send(deger).unwrap();
        }
    });

    while let Ok(deger) = alici.recv() {
        println!("{deger}");
    }
    // ANCHOR_END: here
}
