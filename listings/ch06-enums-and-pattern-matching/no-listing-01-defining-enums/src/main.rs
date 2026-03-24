// ANCHOR: def
enum IpAdresTuru {
    V4,
    V6,
}
// ANCHOR_END: def

fn main() {
    // ANCHOR: instance
    let dort = IpAdresTuru::V4;
    let alti = IpAdresTuru::V6;
    // ANCHOR_END: instance

    // ANCHOR: fn_call
    yonlendir(IpAdresTuru::V4);
    yonlendir(IpAdresTuru::V6);
    // ANCHOR_END: fn_call
}

// ANCHOR: fn
fn yonlendir(ip_turu: IpAdresTuru) {}
// ANCHOR_END: fn
