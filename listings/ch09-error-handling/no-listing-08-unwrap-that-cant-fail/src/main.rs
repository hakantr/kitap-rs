fn main() {
    // ANCHOR: here
    use std::net::IpAddr;

    let ev: IpAddr = "127.0.0.1"
        .parse()
        .expect("Sabit kodlanmış IP adresi geçerli olmalıdır");
    // ANCHOR_END: here
}
