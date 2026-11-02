# Rust Programlama Dili

_Steve Klabnik, Carol Nichols ve Chris Krycho tarafından, Rust Topluluğunun katkılarıyla yazılmıştır._

Kitabın bu sürümü, Rust 1.99.1 (yayın tarihi: 2026-10-15) veya üzerini kullandığınızı ve tüm projeleri Rust 2024 Sürümü (*Edition*) kurallarını kullanacak şekilde yapılandırmak üzere *Cargo.toml* dosyalarında `edition = "2024"` ayarının bulunduğunu varsayar. Rust'ı kurma veya güncelleme yönergeleri için [Bölüm 1'deki "Kurulum" başlığına][install]<!-- ignore -->, sürümler hakkında bilgi için ise [Ek E'ye][ekler-e]<!-- ignore --> bakabilirsiniz.

Kitabın orijinal HTML sürümü çevrimiçi olarak [https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/) adresinde, `rustup` ile yapılan Rust kurulumlarında ise çevrimdışı olarak yer alır; yerel kopyayı açmak için terminalde `rustup doc --book` komutunu çalıştırabilirsiniz.

Topluluk tarafından hazırlanan çeşitli [çeviriler][translations] de mevcuttur.

Bu metin, İngilizce olarak [No Starch Press'ten karton kapaklı baskı ve e-kitap formatında][nsprust] edinilebilir.

> Not: Bu Türkçe çeviride kavramları ana dilinizde rahatça özümseyebilmeniz için standart Türkçe teknik terimler kullanılmış; derleyicinin (`rustc`) ürettiği İngilizce hata mesajları ve resmî belgelerle kolayca bağ kurabilmeniz adına temel terimlerin İngilizce karşılıkları bölümlerde ilk geçtikleri yerde belirtilmiştir. Ayrıca yukarıda adı geçen *derleyici sürümü* (`1.99.1`, altı haftada bir yayınlanan araç zinciri güncellemesi) ile *Rust Sürümü* (`edition = "2024"`, yaklaşık üç yılda bir sunulan dil kuralları paketi) birbirinden farklı iki kavramdır.

[install]: ch01-01-kurulum.html
[ekler-e]: ekler-05-surumler.html
[nsprust]: https://nostarch.com/rust-programming-language-3rd-edition
[translations]: ekler-06-ceviriler.html

> **🚨 Daha etkileşimli bir öğrenme deneyimi mi arıyorsunuz? Kısa sınavlar, kod vurgulamaları, görselleştirmeler ve daha fazlasını içeren alternatif Rust Kitabı sürümünü (İngilizce) deneyin**: <https://rust-book.cs.brown.edu>
