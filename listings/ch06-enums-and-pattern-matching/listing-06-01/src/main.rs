fn main() {
    // ANCHOR: here
    enum IpAdresTuru {
        V4,
        V6,
    }

    struct IpAdres {
        tur: IpAdresTuru,
        adres: String,
    }

    let ev = IpAdres {
        tur: IpAdresTuru::V4,
        adres: String::from("127.0.0.1"),
    };

    let geridongu = IpAdres {
        tur: IpAdresTuru::V6,
        adres: String::from("::1"),
    };
    // ANCHOR_END: here
}
