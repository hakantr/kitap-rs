use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

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

    let yanit = "HTTP/1.1 200 OK\r\n\r\n";

    akis.write_all(yanit.as_bytes()).unwrap();
}
// ANCHOR_END: here
