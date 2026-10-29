static mut COUNTER: u32 = 0;

/// SAFETY: Bu fonksiyonu aynı anda birden fazla thread'den çağırmak tanımsız
/// davranıştır; bu yüzden aynı anda *yalnızca* tek bir thread'den çağırdığınızı
/// garanti etmelisiniz.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // SAFETY: Bu fonksiyon yalnızca `main` içindeki tek thread'den çağrılıyor.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
