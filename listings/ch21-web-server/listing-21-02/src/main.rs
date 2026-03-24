use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let dinleyici = TcpListener::bind("127.0.0.1:7878").unwrap();

    for akis in dinleyici.incoming() {
        let akis = akis.unwrap();

        baglantiyi_isle(akis);
    }
}

fn baglantiyi_isle(mut akis: TcpStream) {
    let tamponlu_okuyucu = BufReader::new(&akis);
    let http_istegi: Vec<_> = tamponlu_okuyucu
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("İstek: {http_istegi:#?}");
}
