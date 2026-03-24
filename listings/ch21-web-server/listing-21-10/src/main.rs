// ANCHOR: here
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
// --snip--
// ANCHOR_END: here

fn main() {
    let dinleyici = TcpListener::bind("127.0.0.1:7878").unwrap();

    for akis in dinleyici.incoming() {
        let akis = akis.unwrap();

        baglantiyi_isle(akis);
    }
}
// ANCHOR: here

fn baglantiyi_isle(mut akis: TcpStream) {
    // --snip--

    // ANCHOR_END: here
    let tamponlu_okuyucu = BufReader::new(&akis);
    let istek_satiri = tamponlu_okuyucu.lines().next().unwrap().unwrap();

    // ANCHOR: here
    let (durum_satiri, dosya_adi) = match &istek_satiri[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "merhaba.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "merhaba.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // --snip--
    // ANCHOR_END: here

    let icerik = fs::read_to_string(dosya_adi).unwrap();
    let uzunluk = icerik.len();

    let yanit =
        format!("{durum_satiri}\r\nContent-Length: {uzunluk}\r\n\r\n{icerik}");

    akis.write_all(yanit.as_bytes()).unwrap();
    // ANCHOR: here
}
// ANCHOR_END: here
