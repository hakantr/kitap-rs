<!-- Old headings. Do not remove or links may break. -->
<a id="storing-utf-8-encoded-text-with-strings"></a>
<a id="stringlerle-utf-8-kodlanmış-metin-depolamak"></a>

## String'lerle UTF-8 Kodlanmış Metin Saklama

Bölüm 4'te string'lerden (dizgilerden) bahsetmiştik; şimdi onları daha derinlemesine inceleyeceğiz. Rust'a yeni başlayanlar genellikle üç nedenin bir araya gelmesi yüzünden string'lerde takılır: Rust'ın olası hataları erkenden açığa çıkarma eğilimi, string'lerin pek çok programcının sandığından çok daha karmaşık bir veri yapısı olması ve UTF-8 kodlaması. Başka programlama dillerinden geliyorsanız bu etkenlerin birleşimi ilk başta zor görünebilir.

String'leri koleksiyonlar bağlamında ele alıyoruz; çünkü string'ler bir bayt koleksiyonu ve bu baytlar metin olarak yorumlandığında faydalı işlevler sunan metotlar bütünü olarak uygulanmıştır. Bu bölümde, her koleksiyon türünde bulunan oluşturma, güncelleme ve okuma gibi `String` üzerindeki temel işlemleri inceleyeceğiz. Ayrıca `String`'in diğer koleksiyonlardan ayrıldığı yönleri, özellikle de insanların ve bilgisayarların `String` verisini yorumlama biçimleri arasındaki farklar yüzünden bir `String` içine indeksleme yapmanın neden karmaşık olduğunu ele alacağız.

<!-- Old headings. Do not remove or links may break. -->
<a id="what-is-a-string"></a>
<a id="defining-strings"></a>
<a id="stringleri-tanımlamak"></a>

### String'leri Tanımlama

Öncelikle _string_ terimiyle ne kastettiğimizi tanımlayalım. Rust'ın çekirdek dilinde yalnızca tek bir string türü vardır: genellikle ödünç alınmış biçimi olan `&str` şeklinde gördüğümüz `str` string dilimi. Bölüm 4'te, başka bir yerde saklanan UTF-8 kodlu metin verilerine referans veren string dilimlerinden bahsetmiştik. Örneğin string sabitleri (*string literals*) programın ikili dosyasında saklanır ve bu nedenle birer string dilimidir.

Çekirdek dile yerleşik olmak yerine Rust'ın standart kütüphanesi tarafından sağlanan `String` türü ise büyüyebilen, değiştirilebilir, sahipli (*owned*) ve UTF-8 kodlu bir string türüdür (verisini öbekte, yani *heap* üzerinde tutar). Rust geliştiricileri Rust'taki "string'lerden" söz ettiğinde, bu türlerden yalnızca birini değil, hem `String` hem de string dilimi `&str` türlerini kastediyor olabilir. Bu bölüm büyük ölçüde `String` üzerine odaklanacak olsa da her iki tür de Rust'ın standart kütüphanesinde yoğun biçimde kullanılır ve hem `String` hem de string dilimleri UTF-8 kodludur.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-new-string"></a>
<a id="yeni-bir-string-oluşturmak"></a>

### Yeni Bir String Oluşturma

`Vec<T>` ile kullanılabilen işlemlerin birçoğu `String` için de geçerlidir; çünkü `String`, ek birtakım güvenceler, kısıtlamalar ve yeteneklerle donatılmış bir bayt vektörü sarmalayıcısı olarak uygulanmıştır. `Vec<T>` ve `String` ile aynı şekilde çalışan fonksiyonlara güzel bir örnek, Liste 8-11'de gösterildiği gibi yeni bir örnek oluşturmak için kullanılan `new` fonksiyonudur.

<Listing number="8-11" caption="Yeni ve boş bir `String` oluşturma">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

</Listing>

Bu satır, daha sonra içine veri yükleyebileceğimiz `s` adında yeni ve boş bir string oluşturur. Çoğu zaman string'i başlatmak istediğimiz bir başlangıç verimiz olur. Bunun için, string sabitlerinin de uyguladığı `Display` trait'ini uygulayan her tür üzerinde kullanılabilen `to_string` metodundan yararlanırız. Liste 8-12 iki örnek göstermektedir.

<Listing number="8-12" caption="Bir string sabitinden `String` oluşturmak için `to_string` metodunu kullanma">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

</Listing>

Bu kod, `başlangıç içeriği` metnini barındıran bir string oluşturur.

Bir string sabitinden `String` oluşturmak için `String::from` fonksiyonunu da kullanabiliriz. Liste 8-13'teki kod, `to_string` kullanan Liste 8-12'deki kodla tamamen eşdeğerdir.

<Listing number="8-13" caption="Bir string sabitinden `String` oluşturmak için `String::from` fonksiyonunu kullanma">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

</Listing>

String'ler çok çeşitli amaçlarla kullanıldığından, string'lerle çalışmak için bize pek çok seçenek sunan farklı jenerik API'ler mevcuttur. Bunların bazıları ilk bakışta fazlalık gibi görünebilir, ancak hepsinin kendine özgü bir kullanım yeri vardır! Bu örnekte `String::from` ve `to_string` birebir aynı işi yapar; hangisini seçeceğiniz tamamen stil ve okunabilirlik tercihinize bağlıdır.

String'lerin UTF-8 kodlu olduğunu unutmayın; dolayısıyla Liste 8-14'te gösterildiği gibi uygun biçimde kodlanmış her türlü metin verisini içlerinde saklayabiliriz.

<Listing number="8-14" caption="Farklı dillerdeki selamlamaları string'lerde saklama">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

</Listing>

Bunların tamamı geçerli `String` değerleridir.

<!-- Old headings. Do not remove or links may break. -->
<a id="updating-a-string"></a>
<a id="bir-stringi-güncellemek"></a>

### Bir String'i Güncelleme

Bir `String`'in boyutu büyüyebilir ve tıpkı `Vec<T>` türünde olduğu gibi, içine daha fazla veri ekledikçe içeriği değişebilir. Ek olarak, `String` değerlerini birleştirmek için `+` operatörünü veya `format!` makrosunu rahatlıkla kullanabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="appending-to-a-string-with-push_str-and-push"></a>
<a id="appending-with-push_str-or-push"></a>
<a id="push_str-veya-push-ile-ekleme-yapmak"></a>

#### `push_str` veya `push` ile Ekleme Yapma

Liste 8-15'te gösterildiği gibi, bir string dilimi eklemek üzere `push_str` metodunu kullanarak bir `String`'i büyütebiliriz.

<Listing number="8-15" caption="`push_str` metoduyla bir `String`'e string dilimi ekleme">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

</Listing>

Bu iki satırdan sonra `s` değişkeni `foobar` değerini barındıracaktır. `push_str` metodu bir string dilimi alır; çünkü eklediğimiz parametrenin sahipliğini almak istemeyebiliriz. Örneğin Liste 8-16'daki kodda, `s2`'nin içeriğini `s1`'e ekledikten sonra da `s2`'yi kullanabilmek istiyoruz.

<Listing number="8-16" caption="İçeriğini bir `String`'e ekledikten sonra string dilimini kullanmaya devam etme">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

</Listing>

Eğer `push_str` metodu `s2`'nin sahipliğini alsaydı, son satırda değerini yazdıramazdık. Oysa bu kod tam beklediğimiz gibi çalışır!

`push` metodu ise parametre olarak tek bir karakter (`char`) alır ve onu `String`'in sonuna ekler. Liste 8-17, `push` metodunu kullanarak bir `String`'e _l_ harfini eklemektedir.

<Listing number="8-17" caption="`push` kullanarak bir `String` değerine tek bir karakter ekleme">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

</Listing>

Bunun sonucunda `s` değişkeni `lol` değerini içerecektir.

<!-- Old headings. Do not remove or links may break. -->
<a id="concatenation-with-the--operator-or-the-format-macro"></a>
<a id="concatenating-with--or-format"></a>
<a id="-veya-format-ile-birleştirmek"></a>
<a id="veya-format-ile-birleştirmek"></a>

#### `+` veya `format!` ile Birleştirme

Çoğu zaman mevcut iki string'i birleştirmek istersiniz. Bunu yapmanın yollarından biri, Liste 8-18'de gösterildiği gibi `+` operatörünü kullanmaktır.

<Listing number="8-18" caption="İki `String` değerini yeni bir `String` değerinde birleştirmek için `+` operatörünü kullanma">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

</Listing>

`s3` string'i `Merhaba, dünya!` metnini barındıracaktır. Birleştirme işleminden sonra `s1`'in neden artık geçerli olmadığını ve `s2` için neden bir referans kullandığımızı, `+` operatörünü kullandığımızda çağrılan metodun imzası açıklar. `+` operatörü, imzası şuna benzeyen `add` metodunu çağırır:

```rust,ignore
fn add(self, s: &str) -> String {
```

Standart kütüphanede `add` metodunun jenerikler ve ilişkili türler kullanılarak tanımlandığını görürsünüz. Burada jeneriklerin yerine, bu metodu `String` değerleriyle çağırdığımızda ortaya çıkan somut türleri yazdık. Jenerikleri Bölüm 10'da ele alacağız. Bu imza, `+` operatörünün incelikli kısımlarını anlamamız için gereken ipuçlarını verir.

İlk olarak, `s2`'nin başında `&` vardır; yani ilk string'e ikinci string'in bir referansını ekliyoruz. Bunun nedeni `add` fonksiyonundaki `s` parametresidir: Bir `String`'e yalnızca bir string dilimi (`&str`) ekleyebiliriz; iki `String` değerini doğrudan birbirine ekleyemeyiz. Ama bir dakika — `&s2` ifadesinin türü, `add`'in ikinci parametresinde belirtildiği gibi `&str` değil, `&String` türüdür. Öyleyse Liste 8-18 neden derleniyor?

`add` çağrısında `&s2`'yi kullanabilmemizin nedeni, derleyicinin `&String` argümanını `&str` türüne zorlayabilmesidir. `add` metodunu çağırdığımızda Rust, burada `&s2` ifadesini `&s2[..]` dilimine dönüştüren bir *deref zorlaması (deref coercion)* uygular. Deref zorlamasını Bölüm 15'te daha derinlemesine inceleyeceğiz. `add` metodu `s` parametresinin sahipliğini almadığı için `s2`, bu işlemden sonra da geçerli bir `String` olarak kalır.

İkinci olarak, imzada `self` parametresinin başında `&` _bulunmadığı_ için `add` metodunun `self`'in sahipliğini aldığını görebiliriz. Bu durum, Liste 8-18'deki `s1` değişkeninin `add` çağrısının içine taşınacağı ve sonrasında artık geçerli olmayacağı anlamına gelir. Dolayısıyla `let s3 = s1 + &s2;` deyimi her iki string'i kopyalayıp yeni bir string oluşturacakmış gibi görünse de aslında `s1`'in sahipliğini alır, `s2`'nin içeriğinin bir kopyasını onun sonuna ekler ve ardından sonuç değerinin sahipliğini döndürür. Başka bir deyişle, çok sayıda kopyalama yapıyormuş gibi görünse de yapmaz; bu uygulama kopyalamaktan çok daha verimlidir.

Birden fazla string'i birleştirmemiz gerektiğinde `+` operatörünün kullanımı hantal bir hâl alır:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

Bu noktada `s` değişkeninin değeri `tic-tac-toe` olur. Ancak bunca `+` ve `"` karakteri arasında ne olup bittiğini anlamak güçtür. String'leri daha karmaşık biçimlerde birleştirmek için bunun yerine `format!` makrosunu kullanabiliriz:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

Bu kod da `s` değişkenini `tic-tac-toe` olarak ayarlar. `format!` makrosu tıpkı `println!` gibi çalışır; ancak çıktıyı ekrana yazdırmak yerine içeriği barındıran yeni bir `String` döndürür. `format!` kullanan sürüm çok daha kolay okunur; üstelik `format!` makrosunun ürettiği kod referansları kullandığından parametrelerinin hiçbirinin sahipliğini almaz.

> Not: `+` operatörü (`fn add(self, s: &str) -> String`) ile `format!` makrosunun bellek ve sahiplik davranışını şöyle özetleyebiliriz: `let s3 = s1 + &s2;` deyiminde sol taraftaki `s1` (`self`) sahipliğiyle birlikte `add` metoduna taşınır; yığındaki (*stack*) işaretçi, uzunluk ve kapasite bilgisi devralınarak öbekteki mevcut bellek tamponu yeniden kullanılır (kapasite yetmezse tampon büyütülür) ve `s1` işlemden sonra geçersiz hâle gelir. Sağ taraftaki `&s2` ise deref zorlaması sayesinde `&str` dilimine (`&s2[..]`) dönüştürülerek yalnızca ödünç alınır; böylece `s2` geçerliliğini korur. Buna karşılık `format!` makrosu argümanların tamamını referansla okur, hiçbirinin sahipliğini tüketmez ve sonucu öbekte ayrılmış yeni bir `String` olarak döndürür.

<!-- Old headings. Do not remove or links may break. -->
<a id="indexing-into-strings"></a>
<a id="stringleri-indekslemek"></a>

### String'leri İndeksleme

Diğer pek çok programlama dilinde, bir string içindeki tekil karakterlere indeks numarasıyla başvurarak erişmek geçerli ve yaygın bir işlemdir. Ancak Rust'ta indeksleme sözdizimini kullanarak bir `String`'in parçalarına erişmeye çalışırsanız derleme hatası alırsınız. Liste 8-19'daki geçersiz kodu inceleyelim.

<Listing number="8-19" caption="Bir `String` üzerinde indeksleme sözdizimini kullanmaya çalışma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

</Listing>

Bu kod aşağıdaki hatayla sonuçlanacaktır:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

Hata mesajı durumu açıkça özetliyor: Rust string'leri tam sayıyla indekslemeyi desteklemez (`` the type `str` cannot be indexed by `{integer}` `` — derleyici bunun yerine `.chars().nth()` veya `.bytes().nth()` kullanılabileceğini belirtir). Peki ama neden? Bu soruyu yanıtlamak için Rust'ın string'leri bellekte nasıl sakladığını incelememiz gerekir.

<!-- Old headings. Do not remove or links may break. -->
<a id="internal-representation"></a>
<a id="iç-temsil-internal-representation"></a>

#### İç Temsil

Bir `String`, bir `Vec<u8>` üzerindeki sarmalayıcıdır. Liste 8-14'teki uygun biçimde kodlanmış UTF-8 örnek string'lerimizden bazılarına bakalım. İlk olarak şunu ele alalım:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

Bu durumda uzunluk (`merhaba.len()`) `4` olacaktır; yani `"Hola"` string'ini saklayan vektör 4 bayt uzunluğundadır. UTF-8 ile kodlandığında bu harflerin her biri 1 bayt yer kaplar. Ancak aşağıdaki satır sizi şaşırtabilir (bu string'in `3` rakamıyla değil, büyük Kiril harfi _Ze_ (`З`) ile başladığına dikkat edin):

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

Bu string'in ne kadar uzun olduğu sorulsaydı `12` diyebilirdiniz. Oysa Rust'ın yanıtı `24`'tür: `"Здравствуйте"` metnini UTF-8 ile kodlamak için gereken bayt sayısı budur; çünkü bu string'deki her bir Unicode skaler değeri bellekte 2 bayt yer kaplar. Dolayısıyla string'in baytları üzerindeki bir indeks her zaman geçerli bir Unicode skaler değerine karşılık gelmez. Bunu görmek için şu geçersiz Rust kodunu düşünün:

```rust,ignore,does_not_compile
let merhaba = "Здравствуйте";
let cevap = &merhaba[0];
```

`cevap` değişkeninin ilk harf olan `З` olmayacağını artık biliyorsunuz. UTF-8 ile kodlandığında `З` harfinin ilk baytı `208`, ikincisi ise `151`'dir; dolayısıyla `cevap` değerinin `208` olması gerekirmiş gibi görünür, ancak `208` tek başına geçerli bir karakter değildir. Bu string'in ilk harfini isteyen bir kullanıcı büyük olasılıkla `208` sayısının dönmesini istemeyecektir; fakat Rust'ın bayt indeksi `0`'da sahip olduğu tek veri budur. String yalnızca Latin harfleri içerse bile kullanıcılar genellikle ham bayt değerinin dönmesini istemez: `&"hi"[0]` bayt değerini döndüren geçerli bir kod olsaydı `'h'` değil, `104` döndürürdü.

Sonuç olarak Rust, beklenmedik bir değer döndürmekten ve hemen fark edilemeyecek hatalara yol açmaktan kaçınmak için bu kodu hiç derlemez ve olası yanlış anlaşılmaları geliştirme sürecinin henüz başında engeller.

<!-- Old headings. Do not remove or links may break. -->
<a id="bytes-and-scalar-values-and-grapheme-clusters-oh-my"></a>
<a id="bytes-scalar-values-and-grapheme-clusters"></a>
<a id="baytlar-skaler-değerler-ve-grafem-kümeleri-grapheme-clusters"></a>

#### Baytlar, Skaler Değerler ve Grafem Kümeleri

UTF-8 ile ilgili bir diğer önemli nokta da Rust açısından string'leri ele almanın aslında üç farklı yolu olmasıdır: *baytlar (bytes)*, *skaler değerler (scalar values)* ve *grafem kümeleri (grapheme clusters)* — yani bizim günlük dilde _harf_ dediğimiz şeye en yakın birimler.

Devanagari alfabesiyle yazılmış Hintçe “नमस्ते” kelimesine bakarsak, bu kelime bellekte şu şekilde bir `u8` değerleri vektörü olarak saklanır:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

Bu 18 bayttır ve bilgisayarların bu veriyi nihayetinde bellekte saklama biçimidir. Aynı veriye Rust'ın `char` türünün karşılığı olan Unicode skaler değerleri olarak bakarsak veri şöyle görünür:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

Burada altı adet `char` değeri vardır; ancak dördüncü ve altıncı öğeler tek başına harf değildir: Bunlar kendi başlarına anlam taşımayan birleştirici ses/aksan işaretleridir (*diacritics*). Son olarak, aynı veriye grafem kümeleri olarak bakarsak bir insanın bu Hintçe kelimeyi oluşturan dört harf diye adlandıracağı birimleri elde ederiz:

```text
["न", "म", "स्", "ते"]
```

Rust, metin hangi insan dilinde olursa olsun her programın kendi ihtiyacına uygun yorumlama biçimini seçebilmesi için bilgisayarların sakladığı ham string verisini yorumlamanın farklı yollarını sunar.

Rust'ın bir karakter elde etmek amacıyla `String` içine indeksleme yapmamıza izin vermemesinin son bir nedeni daha vardır: İndeksleme işlemlerinin her zaman sabit zaman (`O(1)`) alması beklenir. Ancak UTF-8 kodlu bir `String` ile bu performansı garanti etmek mümkün değildir; çünkü Rust'ın kaç adet geçerli karakter bulunduğunu belirlemek için metnin başından ilgili indekse kadar tüm baytları taraması gerekirdi.

<!-- Old headings. Do not remove or links may break. -->
<a id="slicing-strings"></a>
<a id="stringleri-dilimlemek-slicing"></a>

### String'leri Dilimleme

String indeksleme işleminin dönüş türünün ne olması gerektiği — bir bayt değeri mi, bir karakter mi, bir grafem kümesi mi yoksa bir string dilimi mi — net olmadığından bir string içine doğrudan indeksleme yapmak genellikle kötü bir fikirdir. Bu nedenle, string dilimleri oluşturmak için gerçekten indeks kullanmanız gerekiyorsa Rust sizden daha açık olmanızı ister.

`[]` operatörünü tek bir sayıyla kullanmak yerine, belirli baytları içeren bir string dilimi oluşturmak için `[]` operatörünü bir aralık ile kullanabilirsiniz:

```rust
let merhaba = "Здравствуйте";

let s = &merhaba[0..4];
```

Burada `s`, string'in ilk 4 baytını barındıran bir `&str` olacaktır. Bu karakterlerin her birinin 2 bayt olduğundan daha önce bahsetmiştik; dolayısıyla `s` diliminin değeri `Зд` olur.

Bir karakterin baytlarının yalnızca bir kısmını `&merhaba[0..1]` gibi bir ifadeyle dilimlemeye çalışsaydık, tıpkı bir vektörde geçersiz bir indekse erişildiğinde olduğu gibi Rust çalışma zamanında panik üretirdi:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

Panik mesajında da görüldüğü üzere, bitiş bayt indeksi `1` geçerli bir karakter sınırı değildir ve `'З'` karakterinin baytlarının (`0..2`) ortasına denk gelir. Bu nedenle aralık kullanarak string dilimleri oluştururken dikkatli olmalısınız; aksi hâlde programınızın çökmesine yol açabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="methods-for-iterating-over-strings"></a>
<a id="iterating-over-strings"></a>
<a id="stringlerin-üzerinde-yineleme-iterating-yapmak"></a>

### String'ler Üzerinde Yineleme Yapma

String parçaları üzerinde çalışmanın en doğru yolu, karakterleri mi yoksa baytları mı istediğinizi açıkça belirtmektir. Tekil Unicode skaler değerleri için `chars` metodunu kullanın. `"Зд"` üzerinde `chars` metodunu çağırmak metni ayırarak `char` türünde iki değer üretir; her bir elemana erişmek için sonuç üzerinde yineleme yapabilirsiniz:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Bu kod ekrana şunu yazdıracaktır:

```text
З
д
```

Alternatif olarak `bytes` metodu her bir ham baytı döndürür; üzerinde çalıştığınız alana göre bu yöntem daha uygun olabilir:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

Bu kod, bu string'i oluşturan 4 baytı yazdıracaktır:

```text
208
151
208
180
```

Ancak geçerli Unicode skaler değerlerinin 1 bayttan daha fazlasından oluşabileceğini her zaman aklınızda bulundurun.

Devanagari alfabesinde olduğu gibi string'lerden grafem kümeleri elde etmek karmaşık bir işlemdir; bu nedenle bu işlevsellik standart kütüphane tarafından sunulmaz. İhtiyacınız olan işlev buysa [crates.io](https://crates.io/)<!-- ignore --> üzerinde bu amaca yönelik crate'ler (örneğin `unicode-segmentation`) bulabilirsiniz.

> Not: UTF-8 kodlamasında İngilizce ASCII harfler 1 bayt yer kaplarken Türkçeye özgü `'ç'`, `'ğ'`, `'ı'`, `'ö'`, `'ş'`, `'ü'` ve `'İ'` harflerinin her biri bellekte **2 bayt** yer kaplar. Örneğin 3 harfli `"çay"` metni için `"çay".chars().count()` çağrısı `3` (`'ç'`, `'a'`, `'y'`) döndürürken, `"çay".len()` çağrısı `[195, 167, 97, 121]` baytlarına karşılık gelen `4` değerini döndürür. Benzer biçimde 5 harfli `"göğüs"` bellekte `8` bayt, 8 harfli `"İstanbul"` ise `'İ'` harfi 2 bayt olduğu için `9` bayt uzunluğundadır. Dolayısıyla `&"İstanbul"[0..1]` ifadesi `'İ'` karakterinin iki baytını ortadan böldüğü için çalışma zamanında panik üretir; ilk harfi güvenle dilimlemek için geçerli karakter sınırı olan `&"İstanbul"[0..2]` aralığı kullanılmalıdır. Ayrıca standart kütüphanedeki `to_lowercase` ve `to_uppercase` metotları dilden bağımsız Unicode kurallarını uygular: Örneğin `'İ'.to_lowercase()` çağrısı tek bir `char` değil, `'i'` ve birleştirici üst nokta `'\u{307}'` olmak üzere iki Unicode skaler değeri (tek bir grafem kümesi) üreten bir iteratör döndürür. Türkçe dil kurallarına duyarlı (`'İ' -> 'i'`, `'I' -> 'ı'`) büyük-küçük harf dönüşümü gerektiğinde `icu_casemap` gibi yerelleştirme destekli crate'leri tercih edebilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="strings-are-not-so-simple"></a>
<a id="handling-the-complexities-of-strings"></a>
<a id="stringlerin-karmaşıklıklarıyla-başa-çıkmak"></a>

### String'lerin Karmaşıklıklarıyla Başa Çıkma

Özetlemek gerekirse string'ler karmaşıktır. Farklı programlama dilleri, bu karmaşıklığı programcıya nasıl yansıtacakları konusunda farklı tercihler yapar. Rust, `String` verisinin doğru işlenmesini tüm Rust programları için varsayılan davranış hâline getirmeyi seçmiştir; bu da programcıların UTF-8 verisini işleme konusunu en baştan düşünmesi gerektiği anlamına gelir. Bu ödünleşim, string'lerin karmaşıklığını diğer programlama dillerinde görünenden daha fazla göz önüne serer; ancak geliştirme yaşam döngünüzün ilerleyen aşamalarında ASCII dışı karakterlerden kaynaklanan hatalarla uğraşmak zorunda kalmanızı önler.

İyi haber şu ki standart kütüphane, bu karmaşık durumları doğru biçimde ele almanıza yardımcı olmak üzere `String` ve `&str` türleri üzerine inşa edilmiş pek çok işlev sunar. Bir string içinde arama yapmak için `contains` ve bir string'in belirli kısımlarını başka bir string ile değiştirmek için `replace` gibi kullanışlı metotlar için standart kütüphane belgelerine göz atmayı unutmayın.

Şimdi biraz daha az karmaşık bir konuya geçelim: hash map'ler!