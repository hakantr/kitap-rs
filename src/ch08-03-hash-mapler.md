<!-- Old headings. Do not remove or links may break. -->
<a id="storing-keys-with-associated-values-in-hash-maps"></a>
<a id="i̇lişkili-değerlerle-anahtarları-hash-maplerde-saklamak"></a>
<a id="ilişkili-değerlerle-anahtarları-hash-maplerde-saklamak"></a>

## Hash Map'lerde Anahtarları ve İlişkili Değerleri Saklama

Yaygın koleksiyonlarımızın sonuncusu *hash map*'tir (karma eşlem). `HashMap<K, V>` türü, `K` türündeki anahtarların `V` türündeki değerlerle eşleşmesini bir _karma (özetleme) fonksiyonu_ (*hashing function*) kullanarak saklar; bu fonksiyon, söz konusu anahtarların ve değerlerin belleğe nasıl yerleştirileceğini belirler. Pek çok programlama dili bu tür bir veri yapısını destekler ancak çoğu zaman _hash_, _map_ (eşlem/harita), _object_ (nesne), _hash table_ (karma tablosu), _dictionary_ (sözlük) veya _associative array_ (ilişkisel dizi) gibi farklı adlar kullanır.

Hash map'ler, verilere vektörlerde olduğu gibi sayısal bir indeks kullanarak değil de herhangi bir türde olabilen bir anahtar üzerinden ulaşmak istediğinizde oldukça kullanışlıdır. Örneğin bir oyunda her takımın skorunu, anahtarların takım adları ve değerlerin ise her takımın skoru olduğu bir hash map'te tutabilirsiniz. Bir takımın adı verildiğinde, o takımın skoruna doğrudan ulaşabilirsiniz.

Bu bölümde hash map'lerin temel API'sini inceleyeceğiz; ancak standart kütüphanenin `HashMap<K, V>` üzerinde tanımladığı fonksiyonlar arasında çok daha fazla kullanışlı özellik gizlidir. Her zaman olduğu gibi, daha fazla bilgi için standart kütüphane belgelendirmesine göz atabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-new-hash-map"></a>
<a id="yeni-bir-hash-map-oluşturmak"></a>

### Yeni Bir Hash Map Oluşturma

Boş bir hash map oluşturmanın bir yolu `new` fonksiyonunu kullanmak ve ardından `insert` metoduyla eleman eklemektir. Liste 8-20'de adları _Mavi_ ve _Sarı_ olan iki takımın skorlarını takip ediyoruz. Mavi takım 10 puanla, Sarı takım ise 50 puanla oyuna başlar.

<Listing number="8-20" caption="Yeni bir hash map oluşturma ve bazı anahtarlarla değerler ekleme">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

</Listing>

Öncelikle standart kütüphanenin koleksiyonlar modülünden `HashMap` türünü `use` ile kapsama dahil etmemiz gerektiğine dikkat edin. Üç yaygın koleksiyonumuz arasında en az sıklıkla kullanılanı bu olduğu için, ön tanımlı kapsamda (*prelude*) otomatik olarak kapsama dahil edilen öğeler arasında yer almaz. Hash map'ler standart kütüphaneden daha az yerleşik destek alır; örneğin vektörlerdeki `vec!` gibi bunları oluşturmaya yarayan yerleşik bir makro yoktur.

Tıpkı vektörler gibi hash map'ler de asıl verilerini *öbekte (heap)* saklar
(`HashMap` değerinin tabloyu yöneten sabit boyutlu kısmı ise örneklerimizdeki
yerel değişkenin içinde bulunur). Bu örnekteki `HashMap`, `String` türünde
anahtarlara ve `i32` türünde değerlere sahiptir. Vektörler gibi hash map'ler de
türdeştir (homojendir): tüm anahtarlar kendi içinde aynı türde ve tüm değerler
de kendi içinde aynı türde olmalıdır.

> Not: Bir türün hash map anahtarı olarak kullanılabilmesi için `Eq` ve `Hash`
> trait'lerini uygulaması gerekir; `String`, `&str`, tam sayı türleri, `bool`,
> `char` ve yalnızca bu türlerden oluşan demetler bu koşulu hazır olarak sağlar.
> Kayan noktalı sayılar (`f32`, `f64`) ise `NaN` değeri kendisine bile eşit
> olmadığından `Eq` trait'ini uygulamaz ve doğrudan anahtar olarak kullanılamaz.
> Kendi struct veya enum türlerinizin bütün alanları gerekli trait'leri
> uyguluyorsa, Ek C'de ele alınan `#[derive(PartialEq, Eq, Hash)]` özniteliğiyle
> bu trait'leri türetebilirsiniz. Ayrıca anahtarları `String` olan bir hash
> map'te arama yaparken yeni bir `String` oluşturmanıza gerek yoktur:
> `skorlar.get("Mavi")` biçiminde doğrudan bir string sabiti (`&str`)
> verebilirsiniz; `get`, `remove` ve `contains_key` gibi metotlar, `Borrow`
> trait'i sayesinde `&String` yerine `&str` türünde argümanları da kabul eder.

<!-- Old headings. Do not remove or links may break. -->
<a id="accessing-values-in-a-hash-map"></a>
<a id="bir-hash-mapteki-değerlere-erişmek"></a>

### Bir Hash Map'teki Değerlere Erişme

Liste 8-21'de gösterildiği gibi anahtarını `get` metoduna vererek hash map'ten bir değer alabiliriz.

<Listing number="8-21" caption="Hash map'te saklanan Mavi takımın skoruna erişme">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

</Listing>

Burada `skor` değişkeni Mavi takımla ilişkili değeri alacak ve sonuç `10` olacaktır. `get` metodu bir `Option<&V>` döndürür; hash map'te o anahtara karşılık gelen bir değer yoksa `get` metodu `None` döndürür. Bu program `Option` değerini işlemek için önce `Option<&i32>` yerine doğrudan `Option<i32>` elde etmek üzere `copied` metodunu, ardından `skorlar` içinde bu anahtara ait bir girdi bulunmuyorsa `skor` değişkenine `0` değerini atamak üzere `unwrap_or` metodunu çağırır.

Bir `for` döngüsü kullanarak, vektörlerde yaptığımıza benzer biçimde bir hash map'teki her anahtar-değer çifti üzerinde dönebiliriz:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Bu kod her bir çifti rastgele (belirsiz) bir sırayla yazdıracaktır:

```text
Sarı: 50
Mavi: 10
```

<!-- Old headings. Do not remove or links may break. -->
<a id="hash-maps-and-ownership"></a>
<a id="managing-ownership-in-hash-maps"></a>
<a id="hash-maplerde-sahipliği-yönetmek"></a>

### Hash Map'lerde Sahipliği Yönetme

`i32` gibi `Copy` trait'ini uygulayan türler için değerler hash map'in içine kopyalanır. `String` gibi sahipli (*owned*) değerlerde ise değerler taşınır ve Liste 8-22'de gösterildiği gibi hash map bu değerlerin yeni sahibi olur.

<Listing number="8-22" caption="Anahtarların ve değerlerin eklendikten sonra hash map'in sahipliğine geçtiğini gösterme">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

</Listing>

`insert` çağrısıyla hash map'in içine taşındıktan sonra `alan_adi` ve `alan_degeri` değişkenlerini artık kullanamayız.

Hash map'e değerlerin kendisi yerine referanslarını eklersek değerler hash map'e taşınmaz. Ancak referansların işaret ettiği değerlerin, en az hash map geçerli olduğu sürece geçerli kalması gerekir. Bu konuyu Bölüm 10'daki ["Ömürlerle Referansları Doğrulama"][validating-references-with-lifetimes]<!-- ignore --> başlığında daha ayrıntılı ele alacağız.

<!-- Old headings. Do not remove or links may break. -->
<a id="updating-a-hash-map"></a>
<a id="bir-hash-mapi-güncellemek"></a>

### Bir Hash Map'i Güncelleme

Anahtar ve değer çiftlerinin sayısı dinamik olarak artabilse de her benzersiz anahtar aynı anda yalnızca tek bir değerle ilişkilendirilebilir (ancak tersi geçerli değildir: örneğin `skorlar` hash map'inde hem Mavi takım hem de Sarı takım `10` değerine sahip olabilir).

Bir hash map'teki veriyi değiştirmek istediğinizde, bir anahtara hâlihazırda bir değer atanmışsa bu durumu nasıl ele alacağınıza karar vermeniz gerekir. Eski değeri tamamen göz ardı ederek yerine yeni değeri yazabilirsiniz. Eski değeri koruyup yeni değeri yok sayabilir ve yeni değeri yalnızca anahtarın henüz bir değeri _yoksa_ ekleyebilirsiniz. Ya da eski değer ile yeni değeri birleştirebilirsiniz. Gelin bu işlemlerin her birini nasıl yapacağımıza bakalım!

<!-- Old headings. Do not remove or links may break. -->
<a id="overwriting-a-value"></a>
<a id="bir-değerin-üzerine-yazmak-overwriting"></a>

#### Bir Değerin Üzerine Yazma

Bir hash map'e bir anahtar ve değer ekledikten sonra aynı anahtarı farklı bir değerle tekrar eklerseniz, o anahtarla ilişkili değer yenisiyle değiştirilir. Liste 8-23'teki kod `insert` metodunu iki kez çağırsa da her iki seferde de Mavi takımın anahtarı için değer eklediğimizden hash map yalnızca tek bir anahtar-değer çifti barındıracaktır.

<Listing number="8-23" caption="Belirli bir anahtarla saklanan bir değeri değiştirme">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

</Listing>

Bu kod ekrana `{"Mavi": 25}` yazdıracaktır; başlangıçtaki `10` değerinin üzerine yazılmıştır.

<!-- Old headings. Do not remove or links may break. -->
<a id="only-inserting-a-value-if-the-key-has-no-value"></a>
<a id="adding-a-key-and-value-only-if-a-key-isnt-present"></a>
<a id="yalnızca-bir-anahtarın-değeri-yoksa-bir-anahtar-ve-değer-eklemek"></a>

#### Yalnızca Anahtar Mevcut Değilse Anahtar ve Değer Ekleme

Belirli bir anahtarın hash map'te bir değerle birlikte hâlihazırda var olup olmadığını kontrol edip şu adımları uygulamak oldukça yaygın bir ihtiyaçtır: Anahtar hash map'te zaten varsa mevcut değer olduğu gibi kalsın; anahtar yoksa anahtar ve onun için verilen değer eklensin.

Hash map'ler bu işlem için, kontrol etmek istediğiniz anahtarı parametre olarak alan `entry` adında özel bir API sunar. `entry` metodunun dönüş değeri, var olabilecek ya da olmayabilecek bir girdiyi temsil eden `Entry` adlı bir enum'dır. Diyelim ki Sarı takımın anahtarıyla ilişkili bir değer olup olmadığını kontrol etmek istiyoruz; eğer yoksa `50` değerini eklemek istiyoruz ve aynı işlemi Mavi takım için de uygulamak istiyoruz. `entry` API'sini kullandığımızda kod Liste 8-24'teki gibi görünür.

<Listing number="8-24" caption="Yalnızca anahtarın henüz bir değeri yoksa ekleme yapmak için `entry` metodunu kullanma">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

</Listing>

`Entry` üzerindeki `or_insert` metodu, ilgili `Entry` anahtarı mevcutsa o anahtarın değerine değiştirilebilir bir referans döndürecek; anahtar mevcut değilse verilen parametreyi bu anahtarın yeni değeri olarak ekleyip yeni değere değiştirilebilir bir referans döndürecek şekilde tanımlanmıştır. Bu teknik, aynı kontrol mantığını kendimiz yazmaktan çok daha temizdir ve ödünç alma denetleyicisiyle çok daha uyumlu çalışır.

Liste 8-24'teki kodu çalıştırdığımızda `{"Sarı": 50, "Mavi": 10}` çıktısını alırız. İlk `entry` çağrısı, Sarı takımın henüz bir değeri olmadığı için Sarı takım anahtarını `50` değeriyle birlikte ekler. İkinci `entry` çağrısı ise Mavi takımın zaten `10` değeri bulunduğu için hash map'te herhangi bir değişiklik yapmaz.

> Not: `entry` API'si, `if !skorlar.contains_key("Sarı") { skorlar.insert(String::from("Sarı"), 50); }` şeklinde iki ayrı çağrı yazmaya kıyasla hem daha ergonomik hem de daha verimlidir. İki ayrı metot çağrısında anahtarın karma (hash) değeri iki kez hesaplanır ve tabloda iki kez arama yapılır; oysa `entry` metodu anahtarın tablodaki konumunu **yalnızca bir kez** bulur ve `Entry` enum'ı (`Occupied` veya `Vacant` varyantı) üzerinden ekleme ya da erişim işlemini doğrudan o konumda gerçekleştirir. Ayrıca tek bir değiştirilebilir ödünç alma üzerinden `&mut V` döndürdüğü için aynı kapsamda okuma ve yazma referanslarının çakışmasını önler. Eklenecek varsayılan değeri hesaplamak maliyetliyse, `or_insert` yerine kapanış alan `or_insert_with(|| ...)` (veya türün `Default` değerini kullanan `or_default()`) metodunu tercih ederek değerin yalnızca anahtar gerçekten yoksa üretilmesini sağlayabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="updating-a-value-based-on-the-old-value"></a>
<a id="eski-değere-dayalı-olarak-bir-değeri-güncellemek"></a>

#### Eski Değere Göre Bir Değeri Güncelleme

Hash map'lerin bir diğer yaygın kullanım alanı, bir anahtarın değerini bulup onu eski değerine göre güncellemektir. Örneğin Liste 8-25, bir metin içinde her kelimenin kaç kez geçtiğini sayan bir kodu göstermektedir. Kelimeleri anahtar olarak tutan bir hash map kullanıyor ve o kelimeyle kaç kez karşılaştığımızı takip etmek için ilişkili değeri artırıyoruz. Bir kelimeyle ilk kez karşılaşıyorsak tabloya önce `0` değerini ekliyoruz.

<Listing number="8-25" caption="Kelimeleri ve geçiş sayılarını tutan bir hash map ile kelime sıklıklarını sayma">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

</Listing>

Bu kod ekrana `{"dünya": 2, "merhaba": 1, "harika": 1}` yazdıracaktır. Aynı anahtar-değer çiftlerinin farklı bir sırada yazdırıldığını da görebilirsiniz: ["Bir Hash Map'teki Değerlere Erişme"][access]<!-- ignore --> bölümünden, bir hash map üzerinde dönerken elemanların rastgele bir sırayla geldiğini hatırlayın.

`split_whitespace` metodu, `metin` içindeki değerin boşluk karakterleriyle ayrılmış alt dilimleri (`&str`) üzerinde dönen bir iteratör döndürür. `or_insert` metodu ise belirtilen anahtara karşılık gelen değere değiştirilebilir bir referans (`&mut V`) döndürür. Burada bu değiştirilebilir referansı `sayac` değişkeninde saklıyoruz; dolayısıyla doğrudan o değere atama yapabilmek için önce yıldız (`*`) başvuru çözme operatörünü kullanarak `sayac` referansını çözmemiz (`*sayac += 1;`) gerekir. Değiştirilebilir referans `for` döngüsünün her adımının sonunda kapsam dışına çıkar; bu nedenle yapılan tüm bu değişiklikler güvenlidir ve ödünç alma kurallarına uygundur.

<!-- Old headings. Do not remove or links may break. -->
<a id="hashing-functions"></a>
<a id="karma-hashing-fonksiyonları"></a>

### Karma Fonksiyonları

Varsayılan olarak `HashMap`, karma tablolarını hedef alan hizmet reddi (*denial-of-service*, DoS) saldırılarına karşı direnç sağlayabilen _SipHash_ adlı bir karma (özetleme) fonksiyonu kullanır[^siphash]<!-- ignore -->. Bu mevcut en hızlı özetleme algoritması değildir; ancak daha yüksek güvenlik karşılığında katlanılan bu küçük performans ödünü buna değerdir. Kodunuzun performans profilini çıkarır ve varsayılan özetleme fonksiyonunun amacınız için fazla yavaş kaldığını tespit ederseniz, farklı bir karma oluşturucu (*hasher*) belirterek başka bir fonksiyona geçiş yapabilirsiniz. _Karma oluşturucu_, `BuildHasher` trait'ini uygulayan bir türdür. Trait'lerden ve bunların nasıl uygulanacağından [Bölüm 10'da][traits]<!-- ignore --> bahsedeceğiz. Kendi karma oluşturucunuzu sıfırdan yazmanız da gerekmez; [crates.io](https://crates.io/)<!-- ignore --> üzerinde diğer Rust kullanıcılarının paylaştığı ve pek çok yaygın özetleme algoritmasını uygulayan hazır karma oluşturucu kütüphaneleri bulunmaktadır.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

<!-- Old headings. Do not remove or links may break. -->
<a id="summary"></a>
<a id="ozet"></a>

## Özet

Vektörler, string'ler ve hash map'ler; programlarınızda veri saklamanız, verilere erişmeniz ve onları değiştirmeniz gerektiğinde ihtiyaç duyacağınız işlevselliğin büyük bir bölümünü sağlar. Artık çözebilecek donanıma sahip olduğunuz birkaç alıştırma aşağıda verilmiştir:

1. Bir tam sayı listesi verildiğinde, bir vektör kullanarak listenin *medyanını* (ortancasını; liste sıralandığında tam ortadaki konumda bulunan değeri) ve *modunu* (tepe değerini; en sık tekrar eden değeri — burada bir hash map işinize yarayacaktır) döndürün.
1. String'leri *Pig Latin* (Domuz Latincesi) biçimine dönüştürün. Ünsüz harfle başlayan her kelimenin ilk ünsüz harfi kelimenin sonuna taşınır ve sonuna _ay_ eklenir; böylece _first_ kelimesi _irst-fay_ (veya Türkçe bir örnekle _kitap_ kelimesi _itap-kay_) olur. Ünlü harfle başlayan kelimelerin sonuna ise bunun yerine doğrudan _hay_ eklenir (_apple_ kelimesi _apple-hay_ veya _elma_ kelimesi _elma-hay_ olur). UTF-8 kodlamasıyla ilgili ayrıntıları göz önünde bulundurmayı unutmayın!
1. Bir hash map ve vektörler kullanarak, kullanıcının bir şirketteki bir departmana çalışan isimleri eklemesine olanak tanıyan metin tabanlı bir arayüz oluşturun; örneğin "Sally'yi Mühendislik bölümüne ekle" veya "Amir'i Satış bölümüne ekle" (`"Add Sally to Engineering"` veya `"Add Amir to Sales"`). Ardından kullanıcının bir departmandaki tüm kişilerin listesini ya da şirketteki tüm kişilerin departmanlara göre gruplanmış ve alfabetik olarak sıralanmış listesini alabilmesini sağlayın.

Standart kütüphane API belgelendirmesi; vektörlerin, string'lerin ve hash map'lerin bu alıştırmalar için çok yararlı olacak metotlarını açıklar!

> Not: Bu üç alıştırmayı çözerken işinizi kolaylaştıracak bazı pratik standart kütüphane ipuçları:
>
> - **Medyan ve Mod**: Vektörü yerinde sıralamak için `sort()` veya tam sayılarda ek bellek ayırmadan daha hızlı çalışan `sort_unstable()` metodunu kullanabilirsiniz. Mod hesabında ise Liste 8-25'teki `entry(sayi).or_insert(0)` deseninden yararlanıp en yüksek sayaca sahip anahtarı bulabilirsiniz.
> - **Pig Latin ve UTF-8**: Türkçedeki `'ç'`, `'ğ'`, `'ı'`, `'ö'`, `'ş'`, `'ü'` gibi karakterler UTF-8'de 2 bayt yer kapladığından `&kelime[0..1]` gibi bayt dilimlemeleri çalışma zamanında panik üretebilir. Bunun yerine `let mut karakterler = kelime.chars();` oluşturup ilk karakteri `karakterler.next()` ile güvenle alabilir, kelimenin geri kalan dilimine ise `karakterler.as_str()` ile ulaşabilirsiniz.
> - **Çalışan ve Departman Arayüzü**: Şirket verisini `HashMap<String, Vec<String>>` olarak modelleyip yeni çalışan eklerken `departmanlar.entry(departman).or_default().push(isim)` çağrısını kullanabilirsiniz. Komut satırından (`std::io::stdin()`) okuduğunuz metni kelimelere ayırmak için `split_whitespace()`, çalışan isimlerini alfabetik sıralamak için `Vec::sort()`, tüm şirketi departman adına göre de sıralı listelemek için anahtarları bir vektöre toplayıp sıralamayı (veya anahtarları doğal olarak sıralı tutan `BTreeMap` yapısını) değerlendirebilirsiniz.

Artık işlemlerin başarısız olabileceği daha karmaşık programlara adım atıyoruz; dolayısıyla hata yönetimini ele almak için en doğru zamandayız. Sırada tam olarak bu var!

[validating-references-with-lifetimes]: ch10-03-omurler.html#referansları-ömürlerle-lifetimes-doğrulamak
[access]: #bir-hash-mapteki-değerlere-erişme
[traits]: ch10-02-traitler.html
