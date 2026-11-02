<!-- Old headings. Do not remove or links may break. -->
<a id="working-with-environment-variables"></a>
<a id="çevre-değişkenleriyle-environment-variables-çalışmak"></a>
<a id="cevre-degiskenleriyle-environment-variables-calismak"></a>
<a id="çevre-değişkenleriyle-çalışmak"></a>

## Çevre Değişkenleri ile Çalışma

`minigrep` ikili dosyasına ek bir özellik kazandırarak programımızı geliştireceğiz: kullanıcının bir çevre değişkeni (*environment variable*) aracılığıyla etkinleştirebileceği büyük/küçük harf duyarsız arama seçeneği. Bu özelliği bir komut satırı seçeneği olarak da tasarlayabilir ve kullanıcıların ihtiyaç duydukları her seferinde bunu girmesini zorunlu tutabilirdik; ancak bunun yerine bir çevre değişkeni kullanarak kullanıcılarımızın değişkeni yalnızca bir kez tanımlamasını ve o terminal oturumu boyunca yapacakları tüm aramaların büyük/küçük harfe duyarsız çalışmasını sağlıyoruz.

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-a-failing-test-for-the-case-insensitive-search-function"></a>
<a id="writing-a-failing-test-for-case-insensitive-search"></a>
<a id="büyükküçük-harf-duyarsız-arama-fonksiyonu-i̇çin-başarısız-olan-bir-test-yazmak"></a>
<a id="büyükküçük-harf-duyarsız-arama-fonksiyonu-için-başarısız-olan-bir-test-yazmak"></a>
<a id="buyukkucuk-harf-duyarsiz-arama-fonksiyonu-icin-basarisiz-olan-bir-test-yazmak"></a>

### Büyük/Küçük Harf Duyarsız Arama İçin Başarısız Olan Bir Test Yazma

İlk olarak `minigrep` kütüphanesine, çevre değişkenine bir değer atandığında çağrılacak `buyuk_kucuk_harf_duyarsiz_ara` adlı yeni bir fonksiyon ekleyeceğiz. TDD sürecini izlemeye devam edeceğimiz için ilk adımımız yine başarısız olan bir test yazmak olacak. Liste 12-20'de gösterildiği gibi yeni `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonu için yeni bir test ekleyecek ve iki test arasındaki farkı netleştirmek amacıyla eski testimizin adını `tek_sonuc` yerine `buyuk_kucuk_harf_duyarli` olarak değiştireceğiz.

<Listing number="12-20" file-name="src/lib.rs" caption="Eklemek üzere olduğumuz büyük/küçük harf duyarsız arama fonksiyonu için başarısız olan yeni bir test ekleme">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

</Listing>

Eski testin `icerik` metnini de düzenlediğimize dikkat edin. Büyük/küçük harfe duyarlı arama yaptığımızda `"güven"` sorgusuyla eşleşmemesi gereken, büyük _G_ harfiyle başlayan `"Güven kolay kazanılmaz."` satırını ekledik. Eski testi bu şekilde güncellemek, daha önce geliştirdiğimiz büyük/küçük harf duyarlı arama işlevselliğini yanlışlıkla bozmadığımızdan emin olmamıza yardımcı olur. Bu test şimdi geçmeli ve biz büyük/küçük harf duyarsız arama üzerinde çalışırken de geçmeye devam etmelidir.

Büyük/küçük harfe _duyarsız_ arama için yazdığımız yeni test ise sorgu olarak `"gÜvEn"` ifadesini kullanıyor. Eklemek üzere olduğumuz `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunda `"gÜvEn"` sorgusu, her ikisi de sorgudan farklı büyük/küçük harf kullanımına sahip olsa bile büyük _G_ harfi içeren `"Güven:"` satırıyla ve `"güven kazanılır."` satırıyla eşleşmelidir. Bu bizim başarısız testimizdir ve henüz `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunu tanımlamadığımız için derlenmeyecektir. Testin derlenip başarısız olduğunu görmek isterseniz, Liste 12-16'da `ara` fonksiyonu için yaptığımıza benzer biçimde her zaman boş bir vektör döndüren iskelet bir uygulama ekleyebilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="implementing-the-search_case_insensitive-function"></a>
<a id="buyuk_kucuk_harf_duyarsiz_ara-fonksiyonunu-uygulamak-implementing"></a>
<a id="buyuk_kucuk_harf_duyarsiz_ara-fonksiyonunu-uygulamak"></a>

### `buyuk_kucuk_harf_duyarsiz_ara` Fonksiyonunu Uygulama

Liste 12-21'de gösterilen `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonu, `ara` fonksiyonuyla neredeyse aynı olacaktır. Tek fark şudur: `sorgu`'yu ve her bir `satir`'ı küçük harfe dönüştüreceğiz; böylece girdi argümanları hangi büyük/küçük harf diziliminde olursa olsun, satırın sorguyu içerip içermediğini kontrol ettiğimiz anda her ikisi de küçük harfli olacaktır.

<Listing number="12-21" file-name="src/lib.rs" caption="Karşılaştırmadan önce sorguyu ve satırı küçük harfe dönüştüren `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunu tanımlama">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

</Listing>

İlk olarak `sorgu` string'ini küçük harfe dönüştürüyor ve orijinal `sorgu` değişkenini gölgeleyerek aynı adlı yeni bir değişkende saklıyoruz. Sorgu üzerinde `to_lowercase` metodunu çağırmamız gereklidir; böylece kullanıcının sorgusu `"güven"`, `"GÜVEN"`, `"Güven"` veya `"gÜvEn"` değerlerinden hangisi olursa olsun sorguyu sanki `"güven"` girilmiş gibi ele alır ve büyük/küçük harf farkını yok sayarız. `to_lowercase` temel Unicode dönüşümlerini işleyebilse de yüzde 100 isabetli değildir. Gerçek bir uygulama geliştiriyor olsaydık burada biraz daha kapsamlı bir işlem yapmamız gerekirdi; ancak bu bölümün konusu Unicode değil çevre değişkenleri olduğundan şimdilik bunu bu şekilde bırakacağız.

`sorgu` değişkeninin artık bir string dilimi (`&str`) değil, bir `String` olduğuna dikkat edin; çünkü `to_lowercase` çağrısı mevcut veriye referans vermek yerine yeni veri oluşturur. Örneğin sorgunun `"gÜvEn"` olduğunu varsayalım: Bu string dilimi doğrudan ödünç alabileceğimiz küçük harfli bir `ü` veya `e` barındırmaz; bu nedenle öbekte `"güven"` metnini tutan yeni bir `String` tahsis etmemiz gerekir. Artık `sorgu` değişkenini `contains` metoduna argüman olarak aktarırken başına `&` işareti eklememiz gerekir, çünkü `contains` metodunun imzası bir string dilimi alacak şekilde tanımlanmıştır.

Ardından tüm karakterleri küçük harfe dönüştürmek için her `satir` üzerinde de bir `to_lowercase` çağrısı ekliyoruz. Artık hem `satir` hem de `sorgu` küçük harfe dönüştürüldüğüne göre, sorgunun büyük/küçük harf yazımı ne olursa olsun eşleşmeleri bulacağız.

> Not: Burada hem bellek yönetimi hem de Türkçe metin işleme açısından dikkat çekici iki incelik vardır:
>
> 1. **Bellek Tahsisi ve Dilim Ömrü**: `sorgu.to_lowercase()` çağrısını `for` döngüsünün _dışında_ yalnızca bir kez yaparak her satır için sorguyu tekrar tekrar öbekte tahsis etmekten kaçınıyoruz. Döngü içinde `satir.to_lowercase()` karşılaştırma amacıyla geçici bir `String` üretip satır sonunda düşürülse de, eşleşme sağlandığında `sonuclar.push(satir)` ifadesiyle bu geçici kopyayı değil, doğrudan `icerik` içindeki orijinal `&'a str` dilimini döndürüyoruz; böylece sonuçlarda satırın özgün büyük/küçük harf yazımı korunur.
> 2. **Türkçede `'İ'` ve `'I'` Harflerinin Unicode Davranışı**: Standart kütüphanedeki `str::to_lowercase` metodu yerel ayardan (*locale*) bağımsız genel Unicode kurallarını uygular. Bu nedenle Türkçe büyük noktalı `'İ'` (`U+0130`, 2 bayt) üzerinde çağrıldığında tek bir `'i'` yerine `'i'` harfi ve birleştirici üst nokta karakteri üretir (`"i\u{307}"`, iki `char` ve 3 bayt — bu durum `to_lowercase` metodunun neden aynı bayt uzunluğunu koruyamayıp yeni bir `String` döndürmek zorunda olduğunun da somut bir örneğidir). Benzer şekilde noktasız `'I'` harfini Türkçe `'ı'` yerine İngilizce kurallarıyla `'i'` harfine dönüştürür. Sonuç olarak `"İSTANBUL".to_lowercase().contains("istanbul")` ve `"Işık".to_lowercase().contains("ışık")` ifadeleri `false` döndürür! Gerçek dünya Türkçe metin arama uygulamalarında karşılaştırma öncesinde `'İ' -> 'i'` ve `'I' -> 'ı'` dönüşümünü açıkça uygulamak veya ICU4X ekosistemindeki `icu_casemap` gibi yerel ayar duyarlı (*locale-aware*) crate'lerden yararlanmak gerekir.

Bakalım bu uygulama testleri geçecek mi:

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

Harika! Testler geçti. Şimdi yeni `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunu `calistir` fonksiyonunun içinden çağıralım. İlk olarak `Yapilandirma` struct'ına büyük/küçük harfe duyarlı ve duyarsız arama arasında geçiş yapacak bir yapılandırma seçeneği ekleyeceğiz. Bu alanı henüz hiçbir yerde ilklendirmediğimiz için şu aşamada derleyici hatası alacağız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:here}}
```

Mantıksal (`bool`) bir değer tutan `buyuk_kucuk_harf_yoksay` alanını ekledik. Ardından Liste 12-22'de gösterildiği gibi `calistir` fonksiyonunun `buyuk_kucuk_harf_yoksay` alanının değerini kontrol etmesini ve buna göre `ara` fonksiyonunu mu yoksa `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunu mu çağıracağına karar vermesini sağlamamız gerekiyor. Bu kod henüz derlenmeyecektir.

<Listing number="12-22" file-name="src/main.rs" caption="`yapilandirma.buyuk_kucuk_harf_yoksay` içindeki değere bağlı olarak `ara` veya `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunu çağırma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:there}}
```

</Listing>

Son olarak çevre değişkenini kontrol etmemiz gerekiyor. Çevre değişkenleriyle çalışmaya yönelik fonksiyonlar, standart kütüphanenin hâlihazırda _src/main.rs_ dosyasının en üstünde kapsama dahil ettiğimiz `env` modülünde yer alır. Liste 12-23'te gösterildiği gibi, `IGNORE_CASE` adlı çevre değişkenine herhangi bir değer atanıp atanmadığını öğrenmek için `env` modülündeki `var` fonksiyonunu kullanacağız.

<Listing number="12-23" file-name="src/main.rs" caption="`IGNORE_CASE` adlı çevre değişkeninde herhangi bir değer olup olmadığını kontrol etme">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/main.rs:here}}
```

</Listing>

Burada `buyuk_kucuk_harf_yoksay` adında yeni bir değişken oluşturuyoruz. Değerini belirlemek için `env::var` fonksiyonunu çağırıyor ve ona `IGNORE_CASE` çevre değişkeninin adını aktarıyoruz. `env::var` fonksiyonu bir `Result` döndürür: Çevre değişkeni herhangi bir değere ayarlanmışsa bu değişkenin değerini barındıran başarılı `Ok` varyantı, çevre değişkeni tanımlanmamışsa `Err` varyantı döner.

Çevre değişkeninin tanımlanıp tanımlanmadığını öğrenmek için `Result` üzerindeki `is_ok` metodunu kullanıyoruz; değişken tanımlıysa program büyük/küçük harf duyarsız arama yapacaktır. `IGNORE_CASE` çevre değişkeni hiçbir değere ayarlanmamışsa `is_ok` metodu `false` döndürecek ve program büyük/küçük harfe duyarlı arama gerçekleştirecektir. Çevre değişkeninin _değeriyle_ ilgilenmiyoruz; yalnızca tanımlanmış olup olmaması bizim için yeterli olduğundan `unwrap`, `expect` ya da `Result` üzerinde gördüğümüz diğer metotlar yerine `is_ok` metodunu kullanıyoruz.

`buyuk_kucuk_harf_yoksay` değişkenindeki değeri `Yapilandirma` örneğine aktarıyoruz; böylece `calistir` fonksiyonu Liste 12-22'de uyguladığımız şekilde bu değeri okuyup `buyuk_kucuk_harf_duyarsiz_ara` veya `ara` fonksiyonlarından hangisini çağıracağına karar verebilir.

Haydi deneyelim! İlk olarak programımızı çevre değişkenini tanımlamadan ve tamamı küçük harflerden oluşan `ne` sorgusuyla çalıştıracağız. Büyük/küçük harfe duyarlı aramada yalnızca küçük harfli _ne_ ifadesini barındıran satırla eşleşmesini bekleriz:

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

Görünüşe göre bu kısım sorunsuz çalışmaya devam ediyor! Şimdi programı `IGNORE_CASE` değişkenini `1` değerine ayarlayarak yine aynı `ne` sorgusuyla çalıştıralım:

```console
$ IGNORE_CASE=1 cargo run -- ne siir.txt
```

PowerShell kullanıyorsanız çevre değişkenini tanımlamayı ve programı çalıştırmayı ayrı komutlar hâlinde yazmanız gerekir:

```console
PS> $Env:IGNORE_CASE=1; cargo run -- ne siir.txt
```

Bu komut `IGNORE_CASE` değişkeninin mevcut kabuk oturumunuzun geri kalanı boyunca kalıcı olmasını sağlar. Değişkeni `Remove-Item` cmdlet'i ile kaldırabilirsiniz:

```console
PS> Remove-Item Env:IGNORE_CASE
```

> Not: Kabuklar (*shells*) arasında çevre değişkeni tanımlama sözdizimi ve değişkenin ömrü farklılık gösterir:
> - **Bash / Zsh (Linux, macOS, Git Bash)**: Komutun hemen önüne yazılan `IGNORE_CASE=1 cargo run ...` ifadesi değişkeni **yalnızca o komut süresince** geçici olarak tanımlar; oturum genelinde kalıcı yapmak için `export IGNORE_CASE=1`, silmek için `unset IGNORE_CASE` kullanılır.
> - **PowerShell**: `$Env:IGNORE_CASE=1` ataması mevcut oturum boyunca kalıcıdır; kaldırmak için `Remove-Item Env:IGNORE_CASE` çalıştırılır.
> - **Windows Komut İstemi (`cmd.exe`)**: `set IGNORE_CASE=1` komutuyla oturum boyunca tanımlanır ve ardından `cargo run -- ne siir.txt` çalıştırılır; değişkeni silmek için `set IGNORE_CASE=` kullanılır (`=` işaretinin çevresinde boşluk bırakmamaya dikkat edin).
>
> Ayrıca kodumuzda değerin içeriğini değil yalnızca varlığını (`env::var("IGNORE_CASE").is_ok()`) denetlediğimiz için, değişken silinmediği sürece `IGNORE_CASE=0` veya `IGNORE_CASE=false` atamalarının da büyük/küçük harf duyarsız aramayı etkin tutacağını unutmayın.

Artık büyük harf barındırabilecek _ne_ eşleşmelerini, yani `Ne` ile başlayan dizeleri de elde etmeliyiz:

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

Mükemmel, `Ne` içeren satırları da aldık! `minigrep` programımız artık bir çevre değişkeniyle kontrol edilen büyük/küçük harf duyarsız arama yapabiliyor. Böylece hem komut satırı argümanları hem de çevre değişkenleri kullanılarak belirlenen seçenekleri nasıl yöneteceğinizi öğrenmiş oldunuz.

Bazı programlar aynı yapılandırma için hem argümanlara _hem de_ çevre değişkenlerine izin verir. Bu gibi durumlarda program, ikisinden birinin öncelikli olmasına karar verir. Kendi başınıza yapabileceğiniz ek bir alıştırma olarak, büyük/küçük harf duyarlılığını hem bir komut satırı argümanıyla hem de bir çevre değişkeniyle kontrol etmeyi deneyin. Program biri büyük/küçük harfe duyarlı, diğeri büyük/küçük harfi yok sayacak biçimde ayarlanarak çalıştırıldığında komut satırı argümanının mı yoksa çevre değişkeninin mi öncelikli olması gerektiğine karar verin.

`std::env` modülü çevre değişkenleriyle çalışmak için daha pek çok kullanışlı özellik barındırır; nelerin mevcut olduğunu görmek için standart kütüphane belgelerine göz atabilirsiniz.
