use std::thread;

fn main() {
    let liste = vec![1, 2, 3];
    println!("Before defining closure: {liste:?}");

    thread::spawn(move || println!("From thread: {liste:?}"))
        .join()
        .unwrap();
}
