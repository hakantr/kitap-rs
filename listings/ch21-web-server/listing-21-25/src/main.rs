use merhaba::IsParcacigiHavuzu;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// ANCHOR: here
fn main() {
    let dinleyici = TcpListener::bind("127.0.0.1:7878").unwrap();
    let havuz = IsParcacigiHavuzu::new(4);

    for akis in dinleyici.incoming().take(2) {
        let akis = akis.unwrap();

        havuz.calistir(|| {
            baglantiyi_isle(akis);
        });
    }

    println!("Kapatılıyor.");
}
// ANCHOR_END: here

fn baglantiyi_isle(mut akis: TcpStream) {
    let tamponlu_okuyucu = BufReader::new(&akis);
    let istek_satiri = tamponlu_okuyucu.lines().next().unwrap().unwrap();

    let (durum_satiri, dosya_adi) = match &istek_satiri[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "merhaba.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "merhaba.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let icerik = fs::read_to_string(dosya_adi).unwrap();
    let uzunluk = icerik.len();

    let yanit =
        format!("{durum_satiri}\r\nContent-Length: {uzunluk}\r\n\r\n{icerik}");

    akis.write_all(yanit.as_bytes()).unwrap();
}
