use std::sync::mpsc;
use std::thread;

fn main() {
    let (gonderici, alici) = mpsc::channel();

    thread::spawn(move || {
        let deger = String::from("merhaba");
        gonderici.send(deger).unwrap();
        println!("değer şu: {deger}");
    });

    let alinan = alici.recv().unwrap();
    println!("Alındı: {alinan}");
}
