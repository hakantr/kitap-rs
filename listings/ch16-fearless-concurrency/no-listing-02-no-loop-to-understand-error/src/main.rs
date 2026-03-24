use std::sync::Mutex;
use std::thread;

fn main() {
    let sayac = Mutex::new(0);
    let mut tutamaclar = vec![];

    let tutamac = thread::spawn(move || {
        let mut sayi = sayac.lock().unwrap();

        *sayi += 1;
    });
    tutamaclar.push(tutamac);

    let tutamac2 = thread::spawn(move || {
        let mut sayi2 = sayac.lock().unwrap();

        *sayi2 += 1;
    });
    tutamaclar.push(tutamac2);

    for tutamac in tutamaclar {
        tutamac.join().unwrap();
    }

    println!("Sonuç: {}", *sayac.lock().unwrap());
}
