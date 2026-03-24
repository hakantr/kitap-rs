use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (gonderici, alici) = mpsc::channel();

    thread::spawn(move || {
        let degerler = vec![
            String::from("merhaba"),
            String::from("olusturulan"),
            String::from("is"),
            String::from("parcacigindan"),
        ];

        for deger in degerler {
            gonderici.send(deger).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for alinan in alici {
        println!("Alındı: {alinan}");
    }
}
