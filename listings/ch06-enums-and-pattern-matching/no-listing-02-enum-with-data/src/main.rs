fn main() {
    // ANCHOR: here
    enum IpAdres {
        V4(String),
        V6(String),
    }

    let ev = IpAdres::V4(String::from("127.0.0.1"));

    let geridongu = IpAdres::V6(String::from("::1"));
    // ANCHOR_END: here
}
