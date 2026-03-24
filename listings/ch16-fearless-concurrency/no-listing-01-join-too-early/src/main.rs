use std::thread;
use std::time::Duration;

fn main() {
    let tutamac = thread::spawn(|| {
        for sira in 1..10 {
            println!("oluşturulan iş parçacığından merhaba sayı {sira}!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    tutamac.join().unwrap();

    for sira in 1..5 {
        println!("ana iş parçacığından merhaba sayı {sira}!");
        thread::sleep(Duration::from_millis(1));
    }
}
