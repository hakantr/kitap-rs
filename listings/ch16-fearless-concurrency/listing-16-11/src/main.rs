use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // ANCHOR: here
    // --snip--

    let (gonderici, alici) = mpsc::channel();

    let gonderici1 = gonderici.clone();
    thread::spawn(move || {
        let degerler = vec![
            String::from("merhaba"),
            String::from("olusturulan"),
            String::from("is"),
            String::from("parcacigindan"),
        ];

        for deger in degerler {
            gonderici1.send(deger).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let degerler = vec![
            String::from("daha"),
            String::from("fazla"),
            String::from("mesaj"),
            String::from("sana"),
        ];

        for deger in degerler {
            gonderici.send(deger).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for alinan in alici {
        println!("Alındı: {alinan}");
    }

    // --snip--
    // ANCHOR_END: here
}
