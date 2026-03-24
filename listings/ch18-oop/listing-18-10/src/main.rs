use arayuz::Ekran;

fn main() {
    let ekran = Ekran {
        bilesenler: vec![Box::new(String::from("Merhaba"))],
    };

    ekran.calistir();
}
