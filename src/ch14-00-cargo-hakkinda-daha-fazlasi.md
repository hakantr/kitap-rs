<!-- Old headings. Do not remove or links may break. -->
<a id="more-about-cargo-and-cratesio"></a>
<a id="cargo-ve-cratesio-hakkinda-daha-fazlasi"></a>

# Cargo ve Crates.io Hakkında Daha Fazlası

Şimdiye kadar kodumuzu derlemek, çalıştırmak ve test etmek için Cargo'nun yalnızca en temel özelliklerini kullandık; ancak Cargo bundan çok daha fazlasını yapabilir. Bu bölümde, aşağıdaki işlemleri nasıl gerçekleştireceğinizi göstermek üzere Cargo'nun daha gelişmiş bazı özelliklerini ele alacağız:

- Sürüm profilleriyle derlemenizi özelleştirme
- Kütüphaneleri [crates.io](https://crates.io/)<!-- ignore --> üzerinde yayınlama
- Büyük projeleri çalışma alanlarıyla düzenleme
- [crates.io](https://crates.io/)<!-- ignore --> üzerinden `cargo install` ile ikili dosyaları kurma
- Özel komutlarla Cargo'yu genişletme

Cargo, bu bölümde ele aldığımız işlevlerden çok daha fazlasını sunabilir; dolayısıyla tüm özelliklerinin eksiksiz bir açıklaması için [Cargo belgelerine](https://doc.rust-lang.org/cargo/) göz atabilirsiniz.

> Not: Pek çok programlama dilinde derleme sistemi, bağımlılık yöneticisi, test çerçevesi ve belgelendirme üreteci birbirinden bağımsız üçüncü parti araçlar olarak geliştiği için projeden projeye farklı yapılandırmalarla karşılaşmak yaygındır. Rust ekosisteminde ise bu görevlerin tamamını tek ve standart bir araç olarak **Cargo** üstlenir:
>
> - _Cargo.toml_ ve _Cargo.lock_ dosyaları üzerinden tekrarlanabilir paket ve bağımlılık yönetimi sağlar.
> - `rustc` derleyicisini sürüm profillerine göre yönlendirir.
> - `cargo test` ile birim, entegrasyon ve belgelendirme testlerini çalıştırır.
> - `cargo doc` ile kaynak koddaki belgelendirme yorumlarından standart HTML API belgeleri üretir.
> - _Çalışma alanı (workspace)_ düzenleyicisi olarak birbiriyle ilişkili birden çok paketi ortak bir bağımlılık ağacı altında yönetir.
>
> Bu bütünleşik yapı sayesinde herhangi bir açık kaynaklı ya da kurumsal Rust projesini indirdiğinizde, ek bir araç zinciri öğrenmeye gerek kalmadan aynı standart komutlarla derleyebilir, test edebilir ve belgelendirebilirsiniz.
