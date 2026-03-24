## Özel Komutlarla Cargo'yu Genişletmek

Cargo, kendisini değiştirmeden yeni alt komutlarla genişletebileceğiniz şekilde
tasarlanmıştır. `$PATH` içinde adı `cargo-birsey` olan bir ikili dosya varsa,
onu `cargo birsey` yazarak sanki Cargo'nun kendi alt komutlarından biriymiş
gibi çalıştırabilirsiniz. Böyle özel komutlar, `cargo --list` çıktısında da
listelenir. Eklentileri `cargo install` ile kurup ardından yerleşik Cargo
araçlarıyla aynı şekilde çalıştırabilmek, Cargo tasarımının çok kullanışlı bir
yanıdır.

## Özet

Cargo ve [crates.io](https://crates.io/)<!-- ignore --> ile kod paylaşmak, Rust
ekosistemini pek çok farklı iş için kullanışlı kılan şeylerden biridir. Rust'ın
standart kütüphanesi küçük ve kararlıdır; ama crate'leri paylaşmak, kullanmak
ve dilden bağımsız bir hızda geliştirmek kolaydır. İşinize yarayan kodu
[crates.io](https://crates.io/)<!-- ignore --> üzerinde paylaşmaktan çekinmeyin;
büyük olasılıkla başka birinin de işine yarayacaktır.
