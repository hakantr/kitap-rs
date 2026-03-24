fn koordinatlari_yazdir(&(x, y): &(i32, i32)) {
    println!("Güncel konum: ({x}, {y})");
}

fn main() {
    let nokta = (3, 5);
    koordinatlari_yazdir(&nokta);
}
