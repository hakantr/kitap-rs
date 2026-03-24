use std::env;

fn main() {
    let argumanlar: Vec<String> = env::args().collect();
    dbg!(argumanlar);
}
