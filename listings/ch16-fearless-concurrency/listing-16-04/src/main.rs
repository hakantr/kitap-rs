use std::thread;

fn main() {
    let vektor = vec![1, 2, 3];

    let tutamac = thread::spawn(|| {
        println!("İşte bir vektör: {vektor:?}");
    });

    drop(vektor); // eyvah!

    tutamac.join().unwrap();
}
