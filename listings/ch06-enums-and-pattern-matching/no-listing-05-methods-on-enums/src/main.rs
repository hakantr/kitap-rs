fn main() {
    enum Mesaj {
        Cik,
        Tasi { x: i32, y: i32 },
        Yaz(String),
        RenkDegistir(i32, i32, i32),
    }

    // ANCHOR: here
    impl Mesaj {
        fn cagir(&self) {
            // metot gövdesi burada tanımlanabilir
        }
    }

    let m = Mesaj::Yaz(String::from("merhaba"));
    m.cagir();
    // ANCHOR_END: here
}
