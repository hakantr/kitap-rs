use std::thread;

fn main() {
    let liste = vec![1, 2, 3];
    println!("Kapanışı tanımlamadan önce: {liste:?}");

    thread::spawn(move || println!("Thread'den: {liste:?}"))
        .join()
        .unwrap();
}
