use std::net::TcpListener;

fn main() {
    let dinleyici = TcpListener::bind("127.0.0.1:7878").unwrap();

    for akis in dinleyici.incoming() {
        let akis = akis.unwrap();

        println!("Bağlantı kuruldu!");
    }
}
