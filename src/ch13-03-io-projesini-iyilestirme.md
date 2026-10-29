<!-- Old headings. Do not remove or links may break. -->

<a id="improving-our-io-project"></a>
<a id="gç-io-projemizi-iyileştirmek"></a>
<a id="gc-io-projemizi-iyilestirmek"></a>

## G/Ç (I/O) Projemizi İyileştirme

İteratörler hakkındaki bu yeni bilgilerimizle, kodun çeşitli bölümlerini daha net ve özlü hale getirmek üzere iteratörleri kullanarak Bölüm 12'deki G/Ç projesini iyileştirebiliriz. İteratörlerin `Yapilandirma::olustur` ve `ara` fonksiyonlarının uygulamalarını nasıl iyileştirebileceğine bakalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="removing-a-clone-using-an-iterator"></a>
<a id="bir-yineleyici-kullanarak-cloneu-klonlamayı-kaldırmak"></a>
<a id="bir-yineleyici-kullanarak-cloneu-klonlamayi-kaldirmak"></a>

### Bir İteratör Kullanarak `clone` Çağrısını Kaldırma

Liste 12-6'da, `String` değerlerinden oluşan bir dilim alan ve dilime indeksle erişip değerleri klonlayarak `Yapilandirma` struct'ının kendi değerlerine sahip olan bir örneğini oluşturan kodu eklemiştik. Liste 13-17'de, `Yapilandirma::olustur` fonksiyonunun Liste 12-23'teki uygulamasını yeniden gösteriyoruz.

<Listing number="13-17" file-name="src/main.rs" caption="Liste 12-23'teki `Yapilandirma::olustur` fonksiyonunun yeniden gösterimi">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/main.rs:ch13}}
```

</Listing>

O sırada, verimsiz `clone` çağrıları için endişelenmememiz gerektiğini, çünkü bunları ileride kaldıracağımızı söylemiştik. İşte o an geldi!

Burada `clone` çağrısına ihtiyaç duymamızın nedeni, `argumanlar` parametresinde `String` öğelerinden oluşan bir dilim bulunması, ancak `olustur` fonksiyonunun `argumanlar` değerinin sahibi olmamasıydı. Bir `Yapilandirma` örneğinin sahipliğini döndürebilmek ve bu örneğin kendi verilerine sahip olmasını sağlamak için `Yapilandirma`'nın `sorgu` ve `dosya_yolu` alanlarına atanacak değerleri klonlamak zorunda kalmıştık.

İteratörler hakkındaki yeni bilgilerimizle, `olustur` fonksiyonunu bir dilimi ödünç almak yerine argüman olarak bir iteratörün sahipliğini alacak şekilde değiştirebiliriz. Dilimin uzunluğunu denetleyen ve belirli konumlara indeksle erişen kod yerine iteratör işlevselliğini kullanacağız. Değerlere erişim işini iteratör üstleneceği için bu değişiklik `Yapilandirma::olustur` fonksiyonunun ne yaptığını çok daha net hale getirecektir.

`Yapilandirma::olustur` iteratörün sahipliğini aldığında ve ödünç alma yoluyla çalışan indeksleme işlemlerini bıraktığında, `clone` çağırıp öbekte yeni bir bellek tahsisi yapmak yerine `String` değerlerini doğrudan iteratörden `Yapilandirma` içine taşıyabiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-returned-iterator-directly"></a>
<a id="döndürülen-yineleyiciyi-returned-iterator-doğrudan-kullanmak"></a>
<a id="dondurulen-yineleyiciyi-returned-iterator-dogrudan-kullanmak"></a>

#### Döndürülen İteratörü Doğrudan Kullanma

G/Ç projenizin _src/main.rs_ dosyasını açın; dosya şu şekilde görünmelidir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

Öncelikle Liste 12-24'teki `main` fonksiyonunun başlangıcını, bu kez iteratör kullanan Liste 13-18'deki kodla değiştireceğiz. `Yapilandirma::olustur` fonksiyonunu da güncelleyene kadar bu kod derlenmeyecektir.

<Listing number="13-18" file-name="src/main.rs" caption="`env::args` dönüş değerini `Yapilandirma::olustur` fonksiyonuna aktarma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

</Listing>

`env::args` fonksiyonu zaten bir iteratör döndürür! İteratör değerlerini bir vektörde toplayıp ardından `Yapilandirma::olustur` fonksiyonuna bir dilim geçirmek yerine, artık `env::args` tarafından döndürülen iteratörün sahipliğini doğrudan `Yapilandirma::olustur` fonksiyonuna aktarıyoruz.

Sırada `Yapilandirma::olustur` tanımını güncellemek var. `Yapilandirma::olustur` fonksiyonunun imzasını Liste 13-19'daki gibi değiştirelim. Fonksiyon gövdesini henüz güncellemediğimiz için bu kod şimdilik derlenmeyecektir.

<Listing number="13-19" file-name="src/main.rs" caption="`Yapilandirma::olustur` imzasını bir iteratör bekleyecek şekilde güncelleme">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/main.rs:here}}
```

</Listing>

`env::args` fonksiyonunun standart kütüphane dokümantasyonu, döndürdüğü iteratörün türünün `std::env::Args` olduğunu ve bu türün `String` değerleri üretecek şekilde `Iterator` trait'ini uyguladığını gösterir.

`Yapilandirma::olustur` fonksiyonunun imzasını, `argumanlar` parametresi `&[String]` yerine `impl Iterator<Item = String>` trait sınırına sahip jenerik bir tür olacak şekilde güncelledik. Bölüm 10'un ["Trait'leri Parametre Olarak Kullanma"][impl-trait]<!-- ignore --> bölümünde ele aldığımız `impl Trait` sözdiziminin bu kullanımı, `argumanlar` parametresinin `Iterator` trait'ini uygulayan ve `String` öğeleri döndüren herhangi bir tür olabileceği anlamına gelir.

`argumanlar` değerinin sahipliğini aldığımız ve üzerinde ilerleyerek iç durumunu değiştireceğimiz için, `argumanlar` parametresinin tanımına `mut` anahtar kelimesini ekleyerek onu değiştirilebilir hale getirebiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-iterator-trait-methods-instead-of-indexing"></a>
<a id="using-iterator-trait-methods"></a>
<a id="iterator-trait-metotlarını-kullanmak"></a>
<a id="iterator-trait-metotlarini-kullanmak"></a>

#### `Iterator` Trait Metotlarını Kullanma

Şimdi `Yapilandirma::olustur` fonksiyonunun gövdesini düzelteceğiz. `argumanlar` parametresi `Iterator` trait'ini uyguladığı için üzerinde `next` metodunu çağırabileceğimizi biliyoruz! Liste 13-20, Liste 12-23'teki kodu `next` metodunu kullanacak şekilde günceller.

<Listing number="13-20" file-name="src/main.rs" caption="İteratör metotlarını kullanmak üzere `Yapilandirma::olustur` gövdesini değiştirme">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/main.rs:here}}
```

</Listing>

`env::args` dönüş değerindeki ilk öğenin programın adı olduğunu hatırlayın. Bu değeri yoksayıp bir sonrakine geçmek istediğimiz için önce `next` metodunu çağırıyor ve dönüş değeriyle hiçbir şey yapmıyoruz. Ardından `Yapilandirma`'nın `sorgu` alanına yerleştirmek istediğimiz değeri almak için `next` metodunu tekrar çağırıyoruz. `next` çağrısı `Some` döndürürse değeri çıkarmak için `match` ifadesini kullanıyoruz; `None` döndürürse yeterli argüman verilmemiş demektir ve bir `Err` değeriyle fonksiyondan erken dönüyoruz. Aynı işlemi `dosya_yolu` değeri için de uyguluyoruz.

<!-- Old headings. Do not remove or links may break. -->

<a id="making-code-clearer-with-iterator-adapters"></a>
<a id="clarifying-code-with-iterator-adapters"></a>
<a id="yineleyici-adaptörleriyle-iterator-adapters-kodu-netleştirmek"></a>
<a id="yineleyici-adaptorleriyle-iterator-adapters-kodu-netlestirmek"></a>

### İteratör Adaptörleriyle Kodu Netleştirme

G/Ç projemizdeki `ara` fonksiyonunda da iteratörlerden yararlanabiliriz. Fonksiyonun Liste 12-19'daki hali, Liste 13-21'de yeniden gösterilmiştir.

<Listing number="13-21" file-name="src/lib.rs" caption="Liste 12-19'daki `ara` fonksiyonunun uygulaması">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

</Listing>

İteratör adaptörü metotlarını kullanarak bu kodu çok daha özlü bir şekilde yazabiliriz. Üstelik bu yaklaşım, değiştirilebilir bir ara `sonuclar` vektörü tanımlama ihtiyacını da ortadan kaldırır. Fonksiyonel programlama stili, kodu daha anlaşılır kılmak adına değiştirilebilir durum (*mutable state*) miktarını en aza indirmeyi tercih eder. Değiştirilebilir durumu ortadan kaldırmak, `sonuclar` vektörüne eşzamanlı erişimi yönetmek zorunda kalmayacağımız için ileride arama işlemini paralel hale getirmemizi de kolaylaştırabilir. Liste 13-22 bu değişikliği göstermektedir.

<Listing number="13-22" file-name="src/lib.rs" caption="`ara` fonksiyonunun uygulamasında iteratör adaptörü metotlarını kullanma">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

</Listing>

`ara` fonksiyonunun amacının `icerik` içindeki `sorgu` metnini barındıran tüm satırları döndürmek olduğunu hatırlayın. Liste 13-16'daki `filter` örneğine benzer şekilde bu kod, yalnızca `satir.contains(sorgu)` ifadesinin `true` döndürdüğü satırları tutmak için `filter` adaptörünü kullanır. Ardından eşleşen satırları `collect` metoduyla yeni bir vektörde toplarız. Çok daha sade! Aynı değişikliği `buyuk_kucuk_harf_duyarsiz_ara` fonksiyonunda da iteratör metotlarını kullanacak şekilde yapabilirsiniz.

Daha ileri bir iyileştirme olarak, `collect` çağrısını kaldırıp dönüş türünü `impl Iterator<Item = &'a str>` olarak değiştirerek `ara` fonksiyonundan doğrudan bir iteratör döndürebilir ve böylece fonksiyonun kendisini bir iteratör adaptörüne dönüştürebilirsiniz. Bu durumda testleri de güncellemeniz gerekeceğini unutmayın! Davranış farkını gözlemlemek için bu değişikliği yapmadan önce ve yaptıktan sonra `minigrep` aracınızla büyük bir dosyada arama yapmayı deneyin. Değişiklikten önce program tüm eşleşmeleri bir vektörde toplayana dek ekrana hiçbir sonuç yazdırmazken, değişiklikten sonra `calistir` fonksiyonundaki `for` döngüsü iteratörün tembelliğinden (*laziness*) yararlanabildiği için eşleşen her satır bulunur bulunmaz anında yazdırılacaktır.

> Not: Bu bölümde yaptığımız iki yeniden düzenleme, Rust'ta sahiplik sistemi ile fonksiyonel iteratör zincirlerinin nasıl uyum içinde çalıştığını somut biçimde gösterir:
>
> 1. **Öbek Tahsisinden (`clone`) Kaçınma:** Önceki `Yapilandirma::olustur(&[String])` imzasında argüman dilimi yalnızca ödünç alındığı için her `.clone()` çağrısı öbekte (*heap*) yeni bellek tahsis edip karakter verisini bayt bayt kopyalıyordu. `impl Iterator<Item = String>` ile iteratörün sahipliğini doğrudan tükettiğimizde ise `argumanlar.next()` çağrısı mevcut `String` değerinin sahipliğini (yalnızca yığındaki işaretçi, uzunluk ve kapasite bilgisini) ek öbek tahsisi yapmadan `Yapilandirma` alanlarına taşır. Üstelik parametre somut `std::env::Args` türü yerine `impl Iterator<Item = String>` kabul ettiği için, birim testlerinde gerçek komut satırı argümanlarına ihtiyaç duymadan doğrudan bellek içi bir iteratör geçirerek `Yapilandirma::olustur` fonksiyonunu kolayca test edebilirsiniz.
> 2. **Değiştirilebilir Ara Durumu Kaldırma ve Paralelleştirme:** `ara` fonksiyonunda döngü içinde adım adım büyütülen `let mut sonuclar` vektörü yerine `icerik.lines().filter(...).collect()` yazmak, değiştirilebilir ara durumu (*mutable intermediate state*) tamamen ortadan kaldırır. İleride çok büyük dosyalarda çok çekirdekli arama yapmak istediğinizde, paylaşılan bir `sonuclar` vektörünü kilitlerle (`Mutex`) senkronize etmekle uğraşmak yerine — örneğin ekosistemde yaygın kullanılan `rayon` crate'indeki `par_lines()` adaptörüne geçerek — neredeyse aynı iteratör zinciriyle aramayı güvenli biçimde paralelleştirebilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="choosing-between-loops-or-iterators"></a>
<a id="choosing-between-loops-and-iterators"></a>
<a id="döngüler-loops-ve-yineleyiciler-iterators-arasında-seçim-yapmak"></a>
<a id="donguler-loops-ve-yineleyiciler-iterators-arasinda-secim-yapmak"></a>

### Döngüler ve İteratörler Arasında Seçim Yapma

Sıradaki mantıklı soru, kendi kodunuzda hangi stili ve neden seçmeniz gerektiğidir: Liste 13-21'deki orijinal uygulama mı, yoksa Liste 13-22'deki iteratör kullanan sürüm mü (doğrudan iteratör döndürmek yerine tüm sonuçları toplayıp döndürdüğümüzü varsayarsak)? Çoğu Rust programcısı iteratör stilini kullanmayı tercih eder. İlk başta alışması biraz daha zor olsa da çeşitli iteratör adaptörlerine ve ne işe yaradıklarına aşina olduğunuzda iteratörleri okumak çok daha kolaylaşır. Döngü ayrıntılarıyla ve yeni vektörler oluşturmakla uğraşmak yerine kod, döngünün üst düzey amacına odaklanır. Bu yaklaşım basmakalıp kod kalıplarını soyutlayarak, iteratördeki her öğenin sağlaması gereken filtreleme koşulu gibi bu koda özgü asıl mantığın çok daha net görülmesini sağlar.

Peki bu iki uygulama gerçekten eşdeğer midir? Sezgisel olarak daha alt düzey olan döngünün daha hızlı çalışacağı düşünülebilir. Şimdi performans konusunu ele alalım.

[impl-trait]: ch10-02-traitler.html#traitleri-parametre-olarak-kullanma
