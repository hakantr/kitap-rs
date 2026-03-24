## Çevre Değişkenleriyle (Environment Variables) Çalışmak

`minigrep` ikili dosyasına ek bir özellik (feature) ekleyerek onu iyileştireceğiz: kullanıcının bir çevre değişkeni aracılığıyla açabileceği büyük/küçük harf duyarsız arama seçeneği. Bu özelliği bir komut satırı seçeneği (command line option) haline getirebilir ve kullanıcıların bunu her uygulamak istediklerinde girmelerini gerektirebilirdik, ancak bunun yerine onu bir çevre değişkeni haline getirerek, kullanıcılarımızın çevre değişkenini bir kez ayarlamasına ve o terminal oturumundaki tüm aramalarının büyük/küçük harfe duyarsız olmasına izin veriyoruz.

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-a-failing-test-for-the-case-insensitive-search-function"></a>

### Büyük/Küçük Harf Duyarsız Arama Fonksiyonu İçin Başarısız Olan Bir Test Yazmak

İlk olarak `minigrep` kütüphanesine, çevre değişkeninin bir değeri olduğunda çağrılacak olan `buyuk_kucuk_harf_duyarsiz_ara` adlı yeni bir fonksiyon ekliyoruz. TDD (test güdümlü geliştirme) sürecini takip etmeye devam edeceğiz, bu nedenle ilk adım yine başarısız olan bir test yazmaktır. Yeni `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonu için yeni bir test ekleyeceğiz ve iki test arasındaki farkları netleştirmek için Liste 12-20'de gösterildiği gibi eski testimizi `tek_sonuc` (one_result) yerine `buyuk_kucuk_harf_duyarli` (case_sensitive) olarak yeniden adlandıracağız.

<Listing number="12-20" file-name="src/lib.rs" caption="Eklemek üzere olduğumuz büyük/küçük harf duyarsız fonksiyonu için başarısız olan yeni bir test eklemek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

</Listing>

Eski testin `icerik` kısmını da düzenlediğimize dikkat edin. Büyük/küçük harf duyarlı bir şekilde arama yaptığımızda `"güven"` sorgusuyla eşleşmemesi gereken, büyük _G_ kullanan `"Güven kolay kazanılmaz."` metnine sahip yeni bir satır ekledik. Eski testi bu şekilde değiştirmek, daha önce uyguladığımız büyük/küçük harf duyarlı arama işlevselliğini yanlışlıkla bozmadığımızdan emin olmamıza yardımcı olur. Bu test şimdi geçmeli ve biz büyük/küçük harf duyarsız arama üzerinde çalışırken de geçmeye devam etmelidir.

Büyük/küçük harf _duyarsız_ arama için yeni test sorgu olarak `"gÜvEn"` kullanıyor. Eklemek üzere olduğumuz `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunda `"gÜvEn"` sorgusu, büyük _G_ harfi içeren `"Güven:"` satırıyla eşleşmeli ve her ikisi de sorgudan farklı büyük/küçük harf kullanımına sahip olsa bile `"güven kazanılır."` satırıyla da eşleşmelidir. Bu bizim başarısız (failing) testimizdir ve derlenemeyecektir çünkü henüz `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunu tanımlamadık. Testin derlendiğini ve başarısız olduğunu görmek için Liste 12-16'da `ara` fonksiyonu için yaptığımıza benzer şekilde her zaman boş bir vektör döndüren iskelet bir uygulama eklemekten çekinmeyin.

### `buyuk_kucuk_harf_duyarsiz_ara` Fonksiyonunu Uygulamak (Implementing)

Liste 12-21'de gösterilen `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonu, `ara` fonksiyonu ile neredeyse aynı olacaktır. Tek fark, girdi argümanlarının durumu ne olursa olsun, satırın sorguyu barındırıp barındırmadığını (contains) kontrol ettiğimizde aynı durumda olmaları için `sorgu`yu ve her bir `satir`ı küçük harfe dönüştürecek olmamızdır.

<Listing number="12-21" file-name="src/lib.rs" caption="Sorguyu ve satırı karşılaştırmadan önce küçük harfe dönüştürecek `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunu tanımlamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

</Listing>

Önce, orijinal `sorgu`yu gölgeleyerek, `sorgu` string'ini (dizgisini) küçük harfe dönüştürüyoruz ve aynı isimli yeni bir değişkende saklıyoruz. Sorgu üzerinde `to_lowercase` (küçük harfe dönüştür) çağırmak gereklidir, böylece kullanıcının sorgusu `"güven"`, `"GÜVEN"`, `"Güven"` veya `"gÜvEn"` olursa olsun sorguyu sanki `"güven"` imiş gibi ele alırız ve büyük/küçük harfe duyarsız oluruz. `to_lowercase` temel Unicode'u işleyebilecek olsa da yüzde 100 doğru olmayacaktır. Gerçek bir uygulama yazıyor olsaydık burada biraz daha fazla iş yapmak isterdik, ancak bu bölüm Unicode değil çevre değişkenleri (environment variables) ile ilgili olduğu için şimdilik bunu bu şekilde bırakacağız.

`sorgu`'nun artık bir string dilimi olmaktan ziyade (rather than) bir `String` olduğuna dikkat edin, çünkü `to_lowercase` çağırmak mevcut verilere referans vermek yerine yeni veriler oluşturur. Örnek olarak sorgunun `"gÜvEn"` olduğunu varsayalım: Bu string dilimi kullanabileceğimiz küçük harfli bir `u` veya `e` barındırmaz, bu nedenle `"güven"` içeren yeni bir `String` tahsis etmemiz gerekir. Şimdi `contains` metoduna argüman olarak `sorgu`'yu aktardığımızda, bir ampersand (ve işareti - `&`) eklememiz gerekir çünkü `contains`'in imzası bir string dilimi alacak şekilde tanımlanmıştır.

Sonra, tüm karakterleri küçük harfe dönüştürmek için her `satir` üzerinde bir `to_lowercase` çağrısı ekliyoruz. Artık `satir` ve `sorgu`yu küçük harfe dönüştürdüğümüze göre sorgunun büyük/küçük harf durumu ne olursa olsun eşleşmeleri bulacağız.

Bakalım bu uygulama testleri geçecek mi:

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

Harika! Geçtiler. Şimdi `calistir` fonksiyonundan yeni `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunu çağıralım. İlk olarak `Yapilandirma` struct'ına, büyük/küçük harfe duyarlı ve duyarsız arama arasında geçiş yapmak için bir yapılandırma seçeneği ekleyeceğiz. Bu alanı eklemek derleyici hatalarına neden olacaktır çünkü henüz hiçbir yerde bu alanı ilklendirmiyoruz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:here}}
```

Bir Boolean (mantıksal değer) barındıran `buyuk_kucuk_harf_yoksay` alanını ekledik. Ardından Liste 12-22'de gösterildiği gibi `calistir` fonksiyonunun `buyuk_kucuk_harf_yoksay` alanının değerini kontrol etmesine ve bunu `ara` fonksiyonunu mu yoksa `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunu mu çağıracağına karar vermek için kullanmasına ihtiyacımız var. Bu henüz derlenmeyecektir.

<Listing number="12-22" file-name="src/main.rs" caption="`yapilandirma.buyuk_kucuk_harf_yoksay` içerisindeki değere bağlı olarak `ara` ya da `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunu çağırmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:there}}
```

</Listing>

Son olarak çevre değişkenini kontrol etmemiz gerekiyor. Çevre değişkenleriyle çalışmak için gerekli fonksiyonlar, halihazırda _src/main.rs_ dosyasının en üstünde kapsama dahil edilmiş olan (in scope) standart kütüphanedeki `env` modülündedir. Liste 12-23'te gösterildiği gibi `IGNORE_CASE` (BÜYÜK KÜÇÜK HARF YOKSAY) adlı bir çevre değişkeni için herhangi bir değer ayarlanıp ayarlanmadığını görmek üzere `env` modülündeki `var` fonksiyonunu kullanacağız.

<Listing number="12-23" file-name="src/main.rs" caption="`IGNORE_CASE` isimli bir çevre değişkeninde herhangi bir değer olup olmadığını kontrol etmek">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/main.rs:here}}
```

</Listing>

Burada `buyuk_kucuk_harf_yoksay` adında yeni bir değişken oluşturuyoruz. Değerini ayarlamak için `env::var` fonksiyonunu çağırıyor ve ona `IGNORE_CASE` çevre değişkeninin adını aktarıyoruz. `env::var` fonksiyonu, çevre değişkeni herhangi bir değere ayarlanmışsa, çevre değişkeninin değerini içeren başarılı `Ok` varyantını (seçeneğini) döndürecek bir `Result` (Sonuç) döndürür. Eğer çevre değişkeni ayarlanmamışsa `Err` (Hata) varyantını döndürecektir.

Çevre değişkeninin ayarlanıp ayarlanmadığını kontrol etmek için `Result` üzerindeki `is_ok` metodunu kullanıyoruz, bu da programın büyük/küçük harf duyarsız arama yapması gerektiği anlamına gelir. Eğer `IGNORE_CASE` çevre değişkeni herhangi bir değere ayarlanmamışsa, `is_ok` `false` (yanlış) değerini döndürecektir ve program büyük/küçük harf duyarlı arama gerçekleştirecektir. Çevre değişkeninin _değeri_ umurumuzda değil, sadece ayarlanmış veya ayarlanmamış olması önemli, bu nedenle `unwrap`, `expect` veya `Result` üzerinde gördüğümüz diğer metotları kullanmak yerine `is_ok` komutunu kontrol ediyoruz.

`buyuk_kucuk_harf_yoksay` değişkenindeki değeri `Yapilandirma` örneğine geçiriyoruz, böylece Liste 12-22'de uyguladığımız (implemented) gibi `calistir` fonksiyonu o değeri okuyabilir ve `buyuk_kucuk_harf_duyarsiz_ara` mı yoksa `ara` mı çağıracağına karar verebilir.

Haydi bir deneyelim! İlk olarak programımızı, çevre değişkeni ayarlanmadan ve tamamı küçük harflerden oluşan `ne` sorgusuyla çalıştıracağız. Büyük/küçük harfe duyarlı aramada yalnızca gerçekten küçük harfli `ne` geçen satırın dönmesini bekleriz:

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

Görünüşe göre hala çalışıyor! Şimdi programı `IGNORE_CASE` (BÜYÜK KÜÇÜK HARF YOKSAY) `1`'e ayarlanmış olarak fakat aynı `ne` sorgusuyla çalıştıralım:

```console
$ IGNORE_CASE=1 cargo run -- ne siir.txt
```

PowerShell kullanıyorsanız, çevre değişkenini ayarlamanız ve programı ayrı komutlar olarak çalıştırmanız gerekir:

```console
PS> $Env:IGNORE_CASE=1; cargo run -- ne siir.txt
```

Bu `IGNORE_CASE`'in shell (kabuk) oturumunuzun geri kalanı boyunca kalıcı olmasını sağlayacaktır. `Remove-Item` cmdlet'i ile kaldırılabilir:

```console
PS> Remove-Item Env:IGNORE_CASE
```

Büyük/küçük harf duyarsız arama sayesinde `Ne` ile başlayan satırları da elde etmeliyiz:

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
IGNORE_CASE=1 cargo run -- ne siir.txt
can't extract because of the environment variable
-->

```console
Ne hasta bekler sabahı,
Ne taze ölüyü mezar.
Ne de şeytan, bir günahı,
Gelme, artık neye yarar?
```

Mükemmel, artık hem `Ne` ile başlayan satırları hem de küçük harfli `ne` geçen satırı aldık! Bizim `minigrep` programımız artık bir çevre değişkeni tarafından kontrol edilen büyük/küçük harf duyarsız arama yapabiliyor. Artık komut satırı argümanları (command line arguments) veya çevre değişkenleri kullanarak belirlenen seçenekleri nasıl yöneteceğinizi biliyorsunuz.

Bazı programlar aynı yapılandırma için hem argümanlara _hem de_ çevre değişkenlerine izin verir. Böyle durumlarda, programlar birinin veya diğerinin öncelikli olduğuna karar verir. Kendi başınıza yapacağınız bir başka alıştırma olarak büyük/küçük harf duyarlılığını bir komut satırı argümanı veya bir çevre değişkeni aracılığıyla kontrol etmeyi deneyin. Programın biri büyük/küçük harf duyarlı, diğeri büyük/küçük harf yoksay olarak ayarlanmış şekilde çalıştırılması durumunda komut satırı argümanının mı yoksa çevre değişkeninin mi öncelikli olması gerektiğine karar verin.

`std::env` modülü çevre değişkenleriyle ilgilenmek için çok daha yararlı özellikler barındırır: Nelerin mevcut olduğunu görmek için belgesine göz atın.
