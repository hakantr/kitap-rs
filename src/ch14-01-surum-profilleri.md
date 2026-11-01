<!-- Old headings. Do not remove or links may break. -->
<a id="customizing-builds-with-release-profiles"></a>
<a id="sürüm-profilleriyle-derlemeleri-özelleştirmek"></a>
<a id="surum-profilleriyle-derlemeleri-ozellestirmek"></a>
<a id="surum-profilleriyle-derlemeleri-ozellestirme"></a>

## Sürüm Profilleriyle Derlemeleri Özelleştirme

Rust'ta _sürüm profilleri (release profiles)_, bir programcının kodu derlemeye yönelik çeşitli seçenekler üzerinde daha fazla kontrol sahibi olmasını sağlayan, önceden tanımlanmış ve farklı yapılandırmalara sahip özelleştirilebilir profillerdir. Her profil diğerlerinden bağımsız olarak yapılandırılır.

Cargo'nun iki ana profili vardır: `cargo build` komutunu çalıştırdığınızda Cargo'nun kullandığı `dev` profili ve `cargo build --release` komutunu çalıştırdığınızda kullandığı `release` profili. `dev` profili geliştirme süreci için uygun varsayılan ayarlarla tanımlanmıştır; `release` profili ise sürüm derlemeleri için uygun varsayılan ayarlara sahiptir.

Bu profil adları derleme çıktılarınızdan tanıdık gelebilir:

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

Çıktıdaki `dev` ve `release` ifadeleri, derleyicinin kullandığı bu farklı profilleri gösterir.

Projenin _Cargo.toml_ dosyasına açıkça herhangi bir `[profile.*]` bölümü eklemediğiniz sürece Cargo, her profil için tanımlı varsayılan ayarları uygular. Özelleştirmek istediğiniz herhangi bir profil için `[profile.*]` bölümleri ekleyerek varsayılan ayarların dilediğiniz bir alt kümesini geçersiz kılabilirsiniz. Örneğin `dev` ve `release` profilleri için `opt-level` ayarının varsayılan değerleri şöyledir:

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level` (*optimizasyon düzeyi*) ayarı, Rust'ın kodunuza uygulayacağı optimizasyon sayısını kontrol eder ve 0 ile 3 arasında bir değer alır. Daha fazla optimizasyon uygulamak derleme süresini uzatır; dolayısıyla geliştirme aşamasındaysanız ve kodunuzu sık sık derliyorsanız, ortaya çıkan kod daha yavaş çalışsa bile derlemenin daha hızlı tamamlanması için daha az optimizasyon yapılmasını istersiniz. Bu nedenle `dev` profili için varsayılan `opt-level` değeri `0`'dır. Kodunuzu yayınlamaya hazır olduğunuzda ise derlemeye daha fazla zaman ayırmak en doğrusudur. Sürüm modunda yalnızca bir kez derleme yaparsınız, ancak derlenmiş programı pek çok kez çalıştırırsınız; dolayısıyla sürüm modu, daha hızlı çalışan bir kod elde etmek adına daha uzun derleme süresini göze alır. `release` profili için varsayılan `opt-level` değerinin `3` olmasının nedeni budur.

Varsayılan bir ayarı, _Cargo.toml_ dosyasında o ayar için farklı bir değer belirterek geçersiz kılabilirsiniz. Örneğin geliştirme profilinde optimizasyon düzeyi olarak 1'i kullanmak istersek projemizin _Cargo.toml_ dosyasına şu iki satırı ekleyebiliriz:

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

Bu kod, varsayılan `0` ayarını geçersiz kılar. Artık `cargo build` komutunu çalıştırdığımızda Cargo, `dev` profilinin diğer varsayılan ayarlarıyla birlikte `opt-level` için yaptığımız özelleştirmeyi kullanır. `opt-level` değerini `1` olarak belirlediğimiz için Cargo varsayılandan daha fazla optimizasyon uygular, ancak bu optimizasyonlar bir sürüm derlemesindeki kadar kapsamlı olmaz.

Her profile ait yapılandırma seçeneklerinin ve varsayılan değerlerin tam listesi için [Cargo belgelerine](https://doc.rust-lang.org/cargo/reference/profiles.html) göz atabilirsiniz.

> Not: Üretim ortamına çıkacak gerçek dünya projelerinde, gömülü sistemlerde veya WebAssembly (`wasm32`) hedeflerinde yalnızca sayısal `opt-level` (`0..=3`) ile yetinmeyip çalıştırılabilir dosyanın hızını artıran ya da diskteki boyutunu küçülten başka profil ayarlarıyla da sıkça karşılaşırsınız:
>
> - **Boyut odaklı `opt-level` değerleri (`"s"` ve `"z"`):** `opt-level` ayarı `0..=3` tamsayılarının yanı sıra ikili dosya boyutunu küçültmeye odaklanan `"s"` değerini ve döngü vektörleştirmeyi de kapatarak boyutu daha agresif biçimde küçülten `"z"` değerini alabilir.
> - **`lto` (*Link-Time Optimization* — Bağlama Zamanı Optimizasyonu):** Varsayılan olarak `false` değerindedir. `lto = "thin"` (derleme süresini makul tutarak crate'ler arası satır içi açma ve ölü kod eleme yapar) veya `lto = true` (tüm bağımlılık ağacını tek bütün halinde analiz eder) kullanıldığında hem çalışma hızı artar hem de ikili dosya boyutu küçülür.
> - **`codegen-units = 1`:** Cargo paralel derlemeyi hızlandırmak için bir crate'i varsayılan olarak birden fazla kod üretim birimine böler (`release` profilinde `16`). Bu değeri `1` yapmak paralel kod üretimini devre dışı bırakıp derleme süresini uzatsa da derleyicinin tüm crate genelinde daha bütünsel optimizasyonlar üretmesine olanak tanır.
> - **`panic = "abort"`:** [Bölüm 9'da][ch09-unwinding] gördüğümüz gibi, panik anında yığını geri sarmak (`"unwind"`) yerine programı doğrudan sonlandırır; geri sarma tabloları üretilmediği için ikili dosya küçülür.
> - **`strip = true` (veya `"symbols"` / `"debuginfo"`):** Sembol tablolarını ve hata ayıklama bilgilerini derlenen ikili dosyadan temizleyerek çalışma hızından ödün vermeden dosya boyutunu önemli ölçüde azaltır (ancak panik anındaki *geri izleme (backtrace)* çıktısında fonksiyon adları ve satır numaraları görünmez hale gelir).
>
> Örneğin hem yüksek çalışma performansı hem de küçük ikili dosya boyutu hedefleyen tipik bir üretim yapılandırması ve geliştirme sırasında yalnızca harici bağımlılıkları optimize etme ipucu _Cargo.toml_ içinde şöyle görünebilir:
>
> ```toml
> [profile.release]
> lto = "thin"
> codegen-units = 1
> panic = "abort"
> strip = true
>
> # Geliştirme sırasında kendi kodunuz hızlı derlensin (opt-level = 0),
> # ancak harici crate'ler hızlı çalışsın isterseniz:
> [profile.dev.package."*"]
> opt-level = 2
> ```

[ch09-unwinding]: ch09-01-panic-ile-kurtarilamayan-hatalar.html#unwinding-the-stack-or-aborting-in-response-to-a-panic
