## G/Ç (I/O) Projemizi İyileştirmek

Yineleyiciler hakkındaki bu yeni bilgiyle, koddaki yerleri daha net ve özlü hale getirmek için yineleyiciler kullanarak Bölüm 12'deki G/Ç projesini geliştirebiliriz. Yineleyicilerin `Yapilandirma::olustur` fonksiyonu ve `ara` fonksiyonu uygulamamızı nasıl geliştirebileceğine bakalım.

### Bir Yineleyici Kullanarak `clone`'u (Klonlamayı) Kaldırmak

Liste 12-6'da, `String` değerlerinden oluşan bir dilim alan ve dilimin içine indeksleyip değerleri klonlayarak (cloning) `Yapilandirma` struct'ının bir örneğini oluşturan ve `Yapilandirma` struct'ının bu değerlere sahip olmasını sağlayan bir kod ekledik. Liste 13-17'de `Yapilandirma::olustur` fonksiyonunun Liste 12-23'teki uygulamasını yeniden ürettik.

<Listing number="13-17" file-name="src/main.rs" caption="Liste 12-23'ten `Yapilandirma::olustur` fonksiyonunun yeniden üretimi">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/main.rs:ch13}}
```

</Listing>

O zamanlar, verimsiz `clone` çağrıları için endişelenmememiz gerektiğini, çünkü bunları gelecekte kaldıracağımızı söylemiştik. İşte o zaman geldi!

Burada `clone`'a ihtiyacımız vardı çünkü `argumanlar` parametresinde `String` elemanlarına sahip bir dilimimiz (slice) var ancak `olustur` fonksiyonu `argumanlar`'ın sahibi değil. Bir `Yapilandirma` örneğinin sahipliğini döndürmek için, `Yapilandirma` örneğinin kendi değerlerine sahip olabilmesi amacıyla `Yapilandirma`'nın `sorgu` ve `dosya_yolu` alanlarındaki değerleri klonlamamız gerekiyordu.

Yineleyiciler hakkındaki yeni bilgimiz sayesinde, `olustur` fonksiyonunu bir dilimi ödünç almak yerine argüman olarak bir yineleyicinin sahipliğini alacak şekilde değiştirebiliriz. Dilimin uzunluğunu kontrol eden ve belirli konumlara indeksleyen kod yerine yineleyici işlevselliğini kullanacağız. Yineleyici değerlere erişeceği için bu, `Yapilandirma::olustur` fonksiyonunun ne yaptığını netleştirecektir.

`Yapilandirma::olustur` yineleyicinin sahipliğini aldığında ve ödünç alan indeksleme işlemlerini kullanmayı bıraktığında, `clone` çağrısı yapmak ve yeni bir tahsis yapmak yerine `String` değerlerini yineleyiciden `Yapilandirma` içine taşıyabiliriz.

#### Döndürülen Yineleyiciyi (Returned Iterator) Doğrudan Kullanmak

G/Ç projenizin _src/main.rs_ dosyasını açın, şu şekilde görünmelidir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

Önce Liste 12-24'te sahip olduğumuz `main` fonksiyonunun başlangıcını, bu kez bir yineleyici kullanan Liste 13-18'deki kodla değiştireceğiz. Bu kod, `Yapilandirma::olustur`'u da güncelleyene kadar derlenmeyecektir.

<Listing number="13-18" file-name="src/main.rs" caption="`env::args`'ın dönüş değerini `Yapilandirma::olustur`'a geçirmek (passing)">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

</Listing>

`env::args` fonksiyonu bir yineleyici döndürür! Yineleyici değerlerini bir vektörde toplayıp ardından `Yapilandirma::olustur`'a bir dilim (slice) geçirmek (passing) yerine, artık `env::args`'tan döndürülen yineleyicinin sahipliğini doğrudan `Yapilandirma::olustur`'a iletiyoruz.

Sonra `Yapilandirma::olustur`'un tanımını (definition) güncellemeliyiz. `Yapilandirma::olustur`'un imzasını Liste 13-19'a benzeyecek şekilde değiştirelim. Bu yine de derlenmeyecektir çünkü fonksiyon gövdesini (function body) de güncellememiz gerekiyor.

<Listing number="13-19" file-name="src/main.rs" caption="`Yapilandirma::olustur`'un imzasını bir yineleyici bekleyecek şekilde güncellemek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/main.rs:here}}
```

</Listing>

`env::args` fonksiyonu için standart kütüphane dokümantasyonu, döndürdüğü yineleyicinin türünün `std::env::Args` olduğunu ve bu türün `Iterator` trait'ini uyguladığını ve `String` değerleri döndürdüğünü gösterir.

`Yapilandirma::olustur` fonksiyonunun imzasını, `argumanlar` parametresinin `&[String]` yerine `impl Iterator<Item = String>` trait sınırlarına sahip jenerik bir tür olacak şekilde güncelledik. Bölüm 10'un ["Traitleri Parametre Olarak Kullanmak"][impl-trait]<!-- ignore --> kısmında tartışılan `impl Trait` sözdiziminin bu kullanımı, `argumanlar`'ın `Iterator` trait'ini uygulayan ve `String` ögeleri döndüren herhangi bir tür olabileceği anlamına gelir.

`argumanlar`'ın sahipliğini aldığımız ve üzerinde yineleme yaparak `argumanlar`'ı değiştireceğimiz (mutating) için, onu değiştirilebilir hale getirmek üzere `argumanlar` parametresinin spesifikasyonuna `mut` anahtar kelimesini ekleyebiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-iterator-trait-methods-instead-of-indexing"></a>

#### `Iterator` Trait Metotlarını Kullanmak

Sonraki adımda, `Yapilandirma::olustur`'un gövdesini düzelteceğiz. `argumanlar` `Iterator` trait'ini uyguladığı için onun üzerinde `next` (sonraki) metodunu çağırabileceğimizi biliyoruz! Liste 13-20, Liste 12-23'teki kodu `next` metodunu kullanacak şekilde günceller.

<Listing number="13-20" file-name="src/main.rs" caption="Yineleyici metotlarını kullanmak için `Yapilandirma::olustur`'un gövdesini değiştirmek">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/main.rs:here}}
```

</Listing>

`env::args` dönüş değerindeki ilk değerin programın adı olduğunu unutmayın. Bunu göz ardı etmek ve bir sonraki değere geçmek istiyoruz, bu yüzden önce `next`'i çağırıyoruz ve dönüş değeriyle hiçbir şey yapmıyoruz. Sonra, `Yapilandirma`'nın `sorgu` alanına (field) koymak istediğimiz değeri elde etmek için `next`'i çağırıyoruz. Eğer `next` `Some` döndürürse, değeri dışarı çıkarmak için bir `match` kullanıyoruz. Eğer `None` döndürürse bu, yeterli argüman (arguments) verilmediği anlamına gelir ve bir `Err` (hata) değeri ile erkenden döneriz (return early). Aynı şeyi `dosya_yolu` değeri için de yapıyoruz.

<!-- Old headings. Do not remove or links may break. -->

<a id="making-code-clearer-with-iterator-adapters"></a>

### Yineleyici Adaptörleriyle (Iterator Adapters) Kodu Netleştirmek

Burada Liste 12-19'da olduğu gibi Liste 13-21'de de çoğaltılan G/Ç projemizdeki `ara` fonksiyonunda yineleyicilerden faydalanabiliriz.

<Listing number="13-21" file-name="src/lib.rs" caption="Liste 12-19'dan `ara` fonksiyonunun uygulaması">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

</Listing>

Yineleyici adaptörü metotlarını kullanarak bu kodu daha özlü bir şekilde yazabiliriz. Bunu yapmak aynı zamanda değiştirilebilir bir ara `sonuclar` vektörüne sahip olmaktan kaçınmamızı sağlar. Fonksiyonel programlama stili, kodu daha net hale getirmek için değiştirilebilir durum (mutable state) miktarını en aza indirmeyi tercih eder. Değiştirilebilir durumu kaldırmak, aramanın paralel olarak gerçekleşmesini sağlayacak gelecekteki bir geliştirmeyi mümkün kılabilir, çünkü `sonuclar` vektörüne eşzamanlı erişimi yönetmek zorunda kalmayız. Liste 13-22 bu değişikliği göstermektedir.

<Listing number="13-22" file-name="src/lib.rs" caption="`ara` fonksiyonunun uygulamasında yineleyici adaptörü metotlarını kullanmak">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

</Listing>

`ara` fonksiyonunun amacının `icerik` içindeki `sorgu`yu barındıran tüm satırları döndürmek (return) olduğunu unutmayın. Liste 13-16'daki `filter` (filtrele) örneğine benzer şekilde bu kod, yalnızca `satir.contains(sorgu)`'nun (`line.contains(query)`) `true` (doğru) döndürdüğü satırları tutmak için `filter` adaptörünü kullanır. Daha sonra eşleşen satırları `collect` (topla) ile başka bir vektörde toplarız. Çok daha basit! Aynı değişikliği `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunda da yineleyici metotlarını kullanmak için yapmaktan çekinmeyin.

Daha ileri bir iyileştirme olarak `collect` çağrısını kaldırıp fonksiyonun bir yineleyici adaptörü haline gelmesi için dönüş türünü `impl Iterator<Item = &'a str>` olarak değiştirerek `ara` fonksiyonundan bir yineleyici döndürün. Testleri de güncellemeniz gerekeceğini unutmayın! Davranıştaki farkı gözlemlemek için bu değişikliği yapmadan önce ve yaptıktan sonra `minigrep` aracınızı kullanarak büyük bir dosyada arama yapın. Bu değişiklikten önce program, tüm sonuçları toplayana kadar hiçbir sonucu yazdırmaz ancak değişiklikten sonra, eşleşen her satır bulundukça sonuçlar yazdırılacaktır, çünkü `calistir` fonksiyonundaki `for` döngüsü yineleyicinin tembelliğinden faydalanabilir.

<!-- Old headings. Do not remove or links may break. -->

<a id="choosing-between-loops-or-iterators"></a>

### Döngüler (Loops) ve Yineleyiciler (Iterators) Arasında Seçim Yapmak

Bir sonraki mantıksal soru, kendi kodunuzda hangi stili seçeceğiniz ve nedenidir: Liste 13-21'deki orijinal uygulama mı yoksa Liste 13-22'deki yineleyicileri kullanan versiyon mu (tüm sonuçları döndürmeden önce yineleyici döndürmek yerine biriktirdiğimizi varsayarak). Çoğu Rust programcısı yineleyici stilini kullanmayı tercih eder. Başlangıçta alışması biraz zordur ancak çeşitli yineleyici adaptörlerini (iterator adapters) ve ne yaptıklarını hissetmeye başladığınızda yineleyicilerin anlaşılması daha kolay olabilir. Döngülerin çeşitli kısımlarıyla uğraşmak ve yeni vektörler oluşturmak yerine, kod döngünün üst düzey hedefine odaklanır. Bu, sıradan kodun bir kısmını soyutlayarak, yineleyicideki her ögenin geçmesi gereken filtreleme koşulu gibi bu koda özgü kavramların daha kolay görülmesini sağlar.

Peki bu iki uygulama (implementations) gerçekten eşdeğer midir? Sezgisel varsayım (assumption), daha düşük seviyeli döngünün daha hızlı olacağı yönünde olabilir. Hadi performans hakkında konuşalım.

[impl-trait]: ch10-02-traitler.html#traitleri-parametre-olarak-kullanmak
