// ANCHOR: here
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
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
    let tamponlu_okuyucu = BufReader::new(&akis);
    let http_istegi: Vec<_> = tamponlu_okuyucu
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let durum_satiri = "HTTP/1.1 200 OK";
    let icerik = fs::read_to_string("merhaba.html").unwrap();
    let uzunluk = icerik.len();

    let yanit =
        format!("{durum_satiri}\r\nContent-Length: {uzunluk}\r\n\r\n{icerik}");

    akis.write_all(yanit.as_bytes()).unwrap();
}
// ANCHOR_END: here
