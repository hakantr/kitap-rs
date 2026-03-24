use std::sync::Mutex;
use std::thread;

fn main() {
    let sayac = Mutex::new(0);
    let mut tutamaclar = vec![];

    for _ in 0..10 {
        let tutamac = thread::spawn(move || {
            let mut sayi = sayac.lock().unwrap();

            *sayi += 1;
        });
        tutamaclar.push(tutamac);
    }

    for tutamac in tutamaclar {
        tutamac.join().unwrap();
    }

    println!("Sonuç: {}", *sayac.lock().unwrap());
}
