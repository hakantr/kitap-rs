<!-- Old headings. Do not remove or links may break. -->

<a id="appendix-e-editions"></a>
<a id="ek-e-sürümler"></a>

## E - Sürümler

Bölüm 1'de `cargo new` komutunun _Cargo.toml_ dosyanıza bir sürüm (*edition*) bilgisi eklediğini görmüştünüz. Bu ekte bu kavramın ne anlama geldiğini ele alıyoruz!

Rust dili ve derleyicisi altı haftalık bir yayın döngüsüne sahiptir; yani kullanıcılar sürekli olarak yeni özelliklere kavuşur. Diğer pek çok programlama dili daha büyük değişiklikleri daha seyrek aralıklarla yayınlarken, Rust daha küçük güncellemeleri daha sık yayınlar. Bir süre sonra tüm bu küçük iyileştirmeler üst üste birikir. Yine de bir yayından diğerine geriye dönüp baktığınızda, “Vay canına, Rust 1.10 ile Rust 1.31 arasında dil ne kadar değişmiş!” diyebilmek her zaman kolay olmayabilir.

Yaklaşık her üç yılda bir Rust ekibi yeni bir Rust _sürümü_ (*edition*) yayınlar. Her sürüm, o zamana dek dile eklenen özellikleri eksiksiz güncellenmiş belgelendirme ve araçlarla birlikte net bir paket hâlinde bir araya getirir. Yeni sürümler de olağan altı haftalık yayın sürecinin bir parçası olarak kullanıcılara sunulur.

Sürümler farklı kitleler için farklı amaçlara hizmet eder:

- Aktif Rust kullanıcıları için yeni bir sürüm, artımlı değişiklikleri anlaşılması kolay tek bir pakette toplar.
- Henüz Rust kullanmayanlar için yeni bir sürüm, Rust'a yeniden göz atmaya değecek önemli ilerlemelerin kaydedildiğinin sinyalini verir.
- Rust'ı geliştirenler için yeni bir sürüm, projenin bütünü adına ortak bir odaklanma noktası oluşturur.

Bu satırların yazıldığı sırada dört Rust sürümü mevcuttur: Rust 2015, Rust 2018, Rust 2021 ve Rust 2024. Bu kitap, Rust 2024 sürümünün idiyomları kullanılarak yazılmıştır.

_Cargo.toml_ dosyasındaki `edition` anahtarı, derleyicinin kodunuz için hangi sürümü kullanması gerektiğini belirtir. Bu anahtar mevcut değilse Rust, geriye dönük uyumluluk amacıyla sürüm değeri olarak varsayılan `2015`'i kullanır.

Her proje, varsayılan 2015 sürümünden farklı bir sürümü açıkça tercih edebilir. Sürümler, mevcut koddaki tanımlayıcılarla çakışabilecek yeni bir anahtar kelimenin eklenmesi gibi geriye dönük uyumsuz değişiklikler barındırabilir. Ancak siz bu yeni sürümü açıkça seçmediğiniz sürece, kullandığınız Rust derleyicisini en güncel sürüme yükseltseniz bile kodunuz sorunsuzca derlenmeye devam eder.

Tüm Rust derleyici sürümleri, o derleyicinin yayınlanmasından önce var olan tüm sürümleri (*editions*) destekler ve desteklenen herhangi bir sürümle yazılmış crate'leri birbirine sorunsuzca bağlayabilir. Sürüm değişiklikleri yalnızca derleyicinin kaynak kodu ön aşamada nasıl ayrıştırdığını ve yorumladığını etkiler. Bu nedenle siz projenizde Rust 2015 kullanıyorsanız ve bağımlılıklarınızdan biri Rust 2018 (veya Rust 2024) kullanıyorsa projeniz derlenecek ve o bağımlılığı kullanabilecektir. Projenizin daha yeni bir sürümü, bir bağımlılığınızın ise Rust 2015 sürümünü kullandığı tersi durum da aynı şekilde çalışır.

Netleştirmek gerekirse: Yeni özelliklerin büyük çoğunluğu tüm sürümlerde kullanılabilir. Hangi Rust sürümünü kullanırsa kullansın tüm geliştiriciler, yeni kararlı derleyici sürümleri yayınlandıkça iyileştirmelerden yararlanmaya devam eder. Yalnızca bazı özel durumlarda —özellikle yeni anahtar kelimeler eklendiğinde veya mevcut bir sözdiziminin varsayılan davranışı iyileştirildiğinde— belirli özellikler yalnızca yeni sürümlerde etkinleştirilir. Bu tür özelliklerden yararlanmak istediğinizde projenizin `edition` ayarını yükseltmeniz gerekir.

> Not: **Rust Sürümlerinin Kısa Özeti ve Otomatik Yükseltme (`cargo fix --edition`):**
> - **Rust 2018:** Modül sisteminde `extern crate` zorunluluğunun kalkması, sözcüksel olmayan ömürler (*non-lexical lifetimes* — NLL) ve `async`/`await` anahtar kelimeleri.
> - **Rust 2021:** Diziler (`[T; N]`) için doğrudan değer üzerinden `IntoIterator`, kapanışlarda alan bazlı (*disjoint*) yakalama ve ön tanımlı kapsama (`prelude`) `TryInto`, `TryFrom`, `FromIterator` eklenmesi.
> - **Rust 2024:** Ön tanımlı kapsama `Future` ve `IntoFuture` eklenmesi, `-> impl Trait` dönüş türlerinde ömür yakalama kurallarının sadeleşmesi, `if let` ve blok sonu ifadelerinde geçici değer düşürme kapsamlarının iyileştirilmesi, `unsafe extern` ve `unsafe fn` içi güvenli olmayan işlem uyarıları, ayrılmış `gen` anahtar kelimesi ve Cargo'da MSRV duyarlı `resolver = "3"`.
> - Mevcut bir crate'i yeni sürüme yükseltmek için önce `cargo fix --edition` komutunu çalıştırıp kodunuzu uyumlu hâle getirebilir, ardından _Cargo.toml_ içinde `edition = "2024"` yazıp `cargo test` ile doğrulayabilirsiniz.

Daha fazla ayrıntı için [_Rust Sürüm Rehberi_][edition-guide] kitabına başvurabilirsiniz. Bu rehber, sürümler arasındaki tüm farkları tek tek sıralayan ve `cargo fix` aracılığıyla kodunuzu yeni bir sürüme otomatik olarak nasıl yükselteceğinizi adım adım açıklayan kapsamlı bir kaynaktır.

[edition-guide]: https://hakantr.github.io/rust-surum/
