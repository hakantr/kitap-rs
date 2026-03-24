## Stringlerle UTF-8 Kodlanmış Metin Depolamak

Bölüm 4'te stringlerden (dizgilerden) bahsetmiştik, ancak şimdi onları daha derinlemesine inceleyeceğiz.
Yeni Rustacean'lar genellikle üç nedenin birleşimi yüzünden stringlerde takılırlar: Rust'ın olası hataları açığa çıkarma eğilimi, stringlerin birçok programcının sandığından daha karmaşık bir veri yapısı olması ve UTF-8. Bu faktörler, başka programlama dillerinden geldiğinizde zor görünebilecek bir şekilde birleşir.

Stringleri koleksiyonlar bağlamında tartışıyoruz çünkü stringler bir bayt koleksiyonu ve o baytlar metin olarak yorumlandığında yararlı işlevsellik sağlamak için bazı metotlar olarak uygulanmıştır. Bu bölümde, her koleksiyon türünün sahip olduğu oluşturma, güncelleme ve okuma gibi `String` üzerindeki işlemlerden bahsedeceğiz. Ayrıca `String`'in diğer koleksiyonlardan farklı olduğu yolları, yani insanların ve bilgisayarların `String` verilerini yorumlama biçimleri arasındaki farklar nedeniyle bir `String`'e indeksleme yapmanın ne kadar karmaşık olduğunu da tartışacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="what-is-a-string"></a>

### Stringleri Tanımlamak

Öncelikle _string_ terimiyle ne kastettiğimizi tanımlayacağız. Rust, çekirdek dilinde yalnızca bir string türüne sahiptir: genellikle ödünç alınmış biçiminde `&str` olarak görülen string dilimi `str`. Bölüm 4'te, başka bir yerde depolanan bazı UTF-8 kodlanmış string verilerine referanslar olan string dilimlerinden bahsettik. Örneğin, string sabitleri programın ikili dosyasında saklanır ve bu nedenle string dilimleridirler.

Çekirdek dile kodlanmak yerine Rust'ın standart kütüphanesi tarafından sağlanan `String` türü, büyütülebilir, değiştirilebilir, sahiplenilmiş, UTF-8 kodlamalı bir string türüdür. Rustacean'lar Rust'ta "stringlerden" bahsettiklerinde, bu türlerden sadece birini değil, `String` veya string dilimi `&str` türlerinden herhangi birini kastediyor olabilirler. Bu bölüm büyük ölçüde `String` ile ilgili olsa da, her iki tür de Rust'ın standart kütüphanesinde yoğun bir şekilde kullanılır ve hem `String` hem de string dilimleri UTF-8 ile kodlanmıştır.

### Yeni Bir String Oluşturmak

`Vec<T>` ile kullanılabilen aynı işlemlerin birçoğu `String` ile de mevcuttur, çünkü `String` aslında bazı ekstra garantiler, kısıtlamalar ve yeteneklerle bir bayt vektörünün etrafında bir sarmalayıcı olarak uygulanmıştır. `Vec<T>` ve `String` ile aynı şekilde çalışan bir fonksiyona örnek, Liste 8-11'de gösterildiği gibi, bir örnek oluşturmak için kullanılan `new` fonksiyonudur.

<Listing number="8-11" caption="Yeni, boş bir `String` oluşturmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

</Listing>

Bu satır `s` adında yeni, boş bir string oluşturur ve daha sonra içine veri yükleyebiliriz. Genellikle string'i başlatmak istediğimiz bazı başlangıç verilerimiz olacaktır. Bunun için, string sabitlerinin yaptığı gibi `Display` trait'ini uygulayan herhangi bir türde kullanılabilen `to_string` metodunu kullanırız. Liste 8-12 iki örnek gösterir.

<Listing number="8-12" caption="Bir string sabitinden `String` oluşturmak için `to_string` metodunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

</Listing>

Bu kod, `başlangıç içeriği` içeren bir string oluşturur.

Ayrıca bir string sabitinden `String` oluşturmak için `String::from` fonksiyonunu da kullanabiliriz. Liste 8-13'teki kod, `to_string` kullanan Liste 8-12'deki kodla eşdeğerdir.

<Listing number="8-13" caption="Bir string sabitinden `String` oluşturmak için `String::from` fonksiyonunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

</Listing>

Stringler pek çok şey için kullanıldığından, stringler için bize birçok seçenek sunan pek çok farklı jenerik API kullanabiliriz. Bazıları gereksiz (redundant) görünebilir, ancak hepsinin yeri vardır! Bu durumda, `String::from` ve `to_string` aynı şeyi yapar, bu nedenle hangisini seçeceğiniz bir stil ve okunabilirlik meselesidir.

Unutmayın stringler UTF-8 kodludur, bu yüzden Liste 8-14'te gösterildiği gibi uygun şekilde kodlanmış herhangi bir veriyi onlara dahil edebiliriz.

<Listing number="8-14" caption="Farklı dillerdeki selamlamaları stringlerde depolamak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

</Listing>

Bunların hepsi geçerli `String` değerleridir.

### Bir String'i Güncellemek

Bir `String` boyutu büyüyebilir ve tıpkı `Vec<T>` içeriğinde olduğu gibi, içine daha fazla veri iterseniz (push) içeriği değişebilir. Ayrıca, `String` değerlerini birleştirmek için `+` operatörünü veya `format!` makrosunu rahatça kullanabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="appending-to-a-string-with-push_str-and-push"></a>

#### `push_str` veya `push` ile Ekleme Yapmak

Liste 8-15'te gösterildiği gibi bir string dilimi eklemek için `push_str` metodunu kullanarak bir `String`'i büyütebiliriz.

<Listing number="8-15" caption="`push_str` metodu kullanılarak bir `String`'e bir string dilimi eklemek">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

</Listing>

Bu iki satırdan sonra `s` değişkeni `foobar` içerecektir. `push_str` metodu bir string dilimi alır çünkü parametrenin sahipliğini almak istemeyebiliriz. Örneğin, Liste 8-16'daki kodda, `s2`'nin içeriğini `s1`'e ekledikten sonra onu kullanabilmek istiyoruz.

<Listing number="8-16" caption="İçeriğini bir `String`'e ekledikten sonra bir string dilimi kullanmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

</Listing>

Eğer `push_str` metodu `s2`'nin sahipliğini alsaydı, onun değerini son satırda yazdıramazdık. Ancak, bu kod beklediğimiz gibi çalışır!

`push` metodu tek bir karakteri parametre olarak alır ve onu `String`'e ekler. Liste 8-17, `push` metodu kullanarak bir `String`'e _l_ harfini ekler.

<Listing number="8-17" caption="`push` kullanarak bir `String` değerine tek bir karakter eklemek">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

</Listing>

Sonuç olarak `s`, `lol` içerecektir.

<!-- Old headings. Do not remove or links may break. -->

<a id="concatenation-with-the--operator-or-the-format-macro"></a>

#### `+` veya `format!` ile Birleştirmek

Sıklıkla iki mevcut string'i birleştirmek istersiniz. Bunu yapmanın bir yolu Liste 8-18'de gösterildiği gibi `+` operatörünü kullanmaktır.

<Listing number="8-18" caption="İki `String` değerini yeni bir `String` değerinde birleştirmek için `+` operatörünü kullanmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

</Listing>

`s3` string'i `Merhaba, dünya!` içerecektir. Ekledikten sonra `s1`'in artık geçerli olmamasının ve `s2`'ye bir referans kullanmamızın nedeni, `+` operatörünü kullandığımızda çağrılan metodun imzası ile ilgilidir. `+` operatörü, imzası şuna benzeyen `add` metodunu kullanır:

```rust,ignore
fn add(self, s: &str) -> String {
```

Standart kütüphanede, `add`'in jenerikler ve ilişkili türler kullanılarak tanımlandığını göreceksiniz. Burada, somut türleri yerine koyduk ki bu, metodu `String` değerleriyle çağırdığımızda olan şeydir. Jenerikleri Bölüm 10'da tartışacağız. Bu imza, `+` operatörünün karmaşık kısımlarını anlamamız için gereken ipuçlarını bize verir.

İlk olarak, `s2` bir `&`'e sahiptir, bu da ikinci string'in bir referansını ilk string'e eklediğimiz anlamına gelir. Bunun nedeni `add` fonksiyonundaki `s` parametresidir: Bir `String`'e sadece bir string dilimi ekleyebiliriz; iki `String` değerini birbirine ekleyemeyiz. Ama bir dakika, `add`'in ikinci parametresinde belirtildiği gibi `&s2`'nin türü `&str` değil, `&String`'dir. Öyleyse Liste 8-18 neden derleniyor?

`add` çağrısında `&s2`'yi kullanabilmemizin nedeni, derleyicinin `&String` argümanını bir `&str`'ye zorlayabilmesidir. `add` metodunu çağırdığımızda Rust, burada `&s2`'yi `&s2[..]` haline getiren bir deref zorlaması kullanır. Deref zorlamasını Bölüm 15'te daha derinlemesine tartışacağız. `add` fonksiyonu `s` parametresinin sahipliğini almadığı için, `s2` bu operasyondan sonra hala geçerli bir `String` olacaktır.

İkinci olarak, imzada `add` fonksiyonunun `self`'in sahipliğini aldığını görebiliriz, çünkü `self` bir `&`'e sahip _değildir_. Bu, Liste 8-18'deki `s1`'in `add` çağrısının içine taşınacağı ve ondan sonra artık geçerli olmayacağı anlamına gelir. Yani, `let s3 = s1 + &s2;` ifadesi her iki string'i kopyalayıp yeni bir tane oluşturacakmış gibi görünse de, bu ifade aslında `s1`'in sahipliğini alır, `s2`'nin içeriğinin bir kopyasını ekler ve ardından sonucun sahipliğini döndürür. Başka bir deyişle, bir sürü kopya yapıyormuş gibi görünür ama yapmaz; bu uygulama kopyalamadan daha verimlidir.

Eğer birden fazla string'i birleştirmemiz gerekirse, `+` operatörünün davranışı hantal hale gelir:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

Bu noktada `s`, `tic-tac-toe` olacaktır. Tüm o `+` ve `"` karakterleriyle neler olup bittiğini görmek zordur. Stringleri daha karmaşık şekillerde birleştirmek için bunun yerine `format!` makrosunu kullanabiliriz:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

Bu kod da `s`'i `tic-tac-toe` olarak ayarlar. `format!` makrosu `println!` gibi çalışır, ancak çıktıyı ekrana yazdırmak yerine içeriklerle birlikte bir `String` döndürür. `format!` kullanan kod sürümünün okunması çok daha kolaydır ve `format!` makrosu tarafından oluşturulan kod referanslar kullandığından, bu çağrı parametrelerinin hiçbirinin sahipliğini almaz.

### Stringleri İndekslemek

Diğer birçok programlama dilinde, bir string'deki tekil karakterlere indeksleriyle başvurarak erişmek geçerli ve yaygın bir işlemdir. Ancak, Rust'ta indeksleme sözdizimini kullanarak bir `String`'in parçalarına erişmeye çalışırsanız bir hata alırsınız. Liste 8-19'daki geçersiz kodu inceleyelim.

<Listing number="8-19" caption="Bir `String` ile indeksleme sözdizimini kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

</Listing>

Bu kod aşağıdaki hatayla sonuçlanacaktır:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

Hata hikayeyi anlatıyor: Rust stringleri indekslemeyi desteklemiyor. Peki ama neden? Bu soruyu cevaplamak için Rust'ın stringleri bellekte nasıl sakladığını tartışmamız gerekiyor.

#### İç Temsil (Internal Representation)

Bir `String`, bir `Vec<u8>` üzerinde sarmalayıcıdır. Liste 8-14'teki uygun şekilde kodlanmış UTF-8 örnek stringlerimizden bazılarına bakalım. Önce buna:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

Bu durumda, `uzunluk` (len) `4` olacaktır, bu da `"Hola"` stringini saklayan vektörün 4 bayt uzunluğunda olduğu anlamına gelir. UTF-8 ile kodlandığında bu harflerin her biri 1 bayt yer kaplar. Ancak aşağıdaki satır sizi şaşırtabilir (bu string'in 3 rakamıyla değil, büyük Kiril harfi _Ze_ ile başladığına dikkat edin):

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

Eğer size string'in ne kadar uzun olduğu sorulsaydı, 12 diyebilirdiniz. Aslında Rust'ın cevabı 24'tür: Bu, "Здравствуйте" stringini UTF-8'de kodlamak için gereken bayt sayısıdır, çünkü o string'deki her bir Unicode skaler değeri 2 bayt depolama alanı kaplar. Bu nedenle, string'in baytlarındaki bir indeks her zaman geçerli bir Unicode skaler değeri ile ilişkili olmayacaktır. Bunu göstermek için, şu geçersiz Rust kodunu inceleyin:

```rust,ignore,does_not_compile
let merhaba = "Здравствуйте";
let cevap = &merhaba[0];
```

`cevap` değişkeninin ilk harf olan `З` olmayacağını zaten biliyorsunuz. UTF-8 ile kodlandığında `З` harfinin ilk baytı `208`, ikincisi ise `151`'dir, bu yüzden görünüşe göre `cevap` aslında `208` olmalıdır, ancak `208` tek başına geçerli bir karakter değildir. Bir kullanıcı bu string'in ilk harfini istediğinde muhtemelen `208` döndürülmesini istemeyecektir; ancak Rust'ın bayt indeks 0'da sahip olduğu tek veri budur. String sadece Latin harfleri içerse bile kullanıcılar genellikle bayt değerinin döndürülmesini istemezler: Eğer `&"hi"[0]` bayt değerini döndüren geçerli bir kod olsaydı, `h` değil `104` döndürürdü.

O halde cevap, beklenmedik bir değer döndürmekten ve hemen keşfedilemeyecek hatalara neden olmaktan kaçınmak için Rust'ın bu kodu hiç derlememesi ve geliştirme sürecinin başlarında yanlış anlamaları önlemesidir.

<!-- Old headings. Do not remove or links may break. -->

<a id="bytes-and-scalar-values-and-grapheme-clusters-oh-my"></a>

#### Baytlar, Skaler Değerler ve Grafem Kümeleri (Grapheme Clusters)

UTF-8 ile ilgili bir diğer nokta da Rust'ın perspektifinden stringlere bakmanın aslında üç alakalı yolu olmasıdır: baytlar, skaler değerler ve grafem kümeleri (bizim _harfler_ olarak adlandırdığımız şeye en yakın olan şey).

Devanagari alfabesiyle yazılmış Hintçe “नमस्ते” kelimesine bakarsak, şöyle görünen bir `u8` değerleri vektörü olarak saklanır:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

Bu 18 bayttır ve bilgisayarların bu veriyi nihayetinde saklama şeklidir. Onlara Rust'ın `char` türünün karşılığı olan Unicode skaler değerleri olarak bakarsak, bu baytlar şöyle görünür:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

Burada altı tane `char` değeri vardır, ancak dördüncü ve altıncı harf değildir: Onlar tek başlarına anlam ifade etmeyen aksan işaretleridir. Son olarak, onlara grafem kümeleri olarak bakarsak, bir insanın Hintçe kelimeyi oluşturan dört harf olarak adlandıracağı şeyi elde ederiz:

```text
["न", "म", "स्", "ते"]
```

Rust, veri hangi insan dilinde olursa olsun, her programın ihtiyaç duyduğu yorumlamayı seçebilmesi için bilgisayarların sakladığı ham string verilerini yorumlamanın farklı yollarını sunar.

Rust'ın bir karakteri elde etmek için bir `String`'e indeksleme yapmamıza izin vermemesinin son bir nedeni de, indeksleme işlemlerinin her zaman sabit zaman (constant time, O(1)) almasının beklenmesidir. Ancak bir `String` ile bu performansı garanti etmek mümkün değildir, çünkü Rust'ın kaç tane geçerli karakter olduğunu belirlemek için içeriğin başından indekse kadar yürümesi gerekir.

### Stringleri Dilimlemek (Slicing)

String indeksleme işleminin dönüş türünün ne olması gerektiği (bir bayt değeri mi, bir karakter mi, bir grafem kümesi mi yoksa bir string dilimi mi) net olmadığı için bir string içine indeksleme yapmak genellikle kötü bir fikirdir. Bu nedenle, string dilimleri oluşturmak için gerçekten indeksleri kullanmanız gerekiyorsa, Rust sizden daha spesifik olmanızı ister.

`[]` operatörünü tek bir numarayla kullanarak indeksleme yapmak yerine, belirli baytları içeren bir string dilimi oluşturmak için `[]` operatörünü bir aralık (range) ile kullanabilirsiniz:

```rust
let merhaba = "Здравствуйте";

let s = &merhaba[0..4];
```

Burada `s`, stringin ilk 4 baytını barındıran bir `&str` olacaktır. Daha önce bu karakterlerin her birinin 2 bayt olduğundan bahsetmiştik, yani `s`, `Зд` olacaktır.

Eğer bir karakterin baytlarının sadece bir kısmını `&merhaba[0..1]` gibi bir şeyle dilimlemeye çalışsaydık, tıpkı bir vektörde geçersiz bir indekse erişilmiş gibi Rust çalışma zamanında panik yapardı:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

Aralıklar kullanarak string dilimleri oluştururken dikkatli olmalısınız çünkü bunu yapmak programınızı çökertebilir.

<!-- Old headings. Do not remove or links may break. -->

<a id="methods-for-iterating-over-strings"></a>

### Stringlerin Üzerinde Yineleme (Iterating) Yapmak

Stringlerin parçaları üzerinde çalışmanın en iyi yolu, karakter mi yoksa bayt mı istediğinizi açıkça belirtmektir. Bireysel Unicode skaler değerleri için `chars` (karakterler) metodunu kullanın. "Зд" üzerinde `chars` çağrıldığında `char` türünde iki değeri ayırır ve döndürür; her bir elemana erişmek için sonucun üzerinde yineleme yapabilirsiniz:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Bu kod aşağıdakileri yazdıracaktır:

```text
З
д
```

Alternatif olarak `bytes` (baytlar) metodu her bir ham baytı döndürür, ki bu sizin kullanım alanınız için uygun olabilir:

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

Ancak geçerli Unicode skaler değerlerinin 1 bayttan fazlasından oluşabileceğini unutmayın.

Devanagari alfabesinde olduğu gibi stringlerden grafem kümeleri elde etmek karmaşıktır, bu nedenle bu işlev standart kütüphane tarafından sağlanmaz. İhtiyacınız olan işlevsellik buysa [crates.io](https://crates.io/)<!-- ignore --> adresinde crateler mevcuttur.

<!-- Old headings. Do not remove or links may break. -->

<a id="strings-are-not-so-simple"></a>

### Stringlerin Karmaşıklıklarıyla Başa Çıkmak

Özetlemek gerekirse, stringler karmaşıktır. Farklı programlama dilleri, bu karmaşıklığı programcıya nasıl sunacakları konusunda farklı seçimler yaparlar. Rust, `String` verilerinin doğru işlenmesini tüm Rust programları için varsayılan davranış haline getirmeyi seçti; bu da programcıların UTF-8 verilerini en başından ele almak için daha fazla kafa yormaları gerektiği anlamına gelir. Bu takas, stringlerin karmaşıklığının diğer programlama dillerinde görünenden daha fazlasını açığa çıkarır, ancak geliştirme yaşam döngünüzün ilerleyen aşamalarında ASCII olmayan karakterlerle ilgili hataları ele almak zorunda kalmanızı önler.

İyi haber şu ki, standart kütüphane bu karmaşık durumları doğru bir şekilde ele almaya yardımcı olmak için `String` ve `&str` türleri üzerine inşa edilmiş birçok işlev sunuyor. Bir string'de arama yapmak için `contains` (içerir) ve bir string'in parçalarını başka bir string ile değiştirmek için `replace` (değiştir) gibi faydalı metotlar için belgelere göz atmayı unutmayın.

Biraz daha az karmaşık bir şeye geçelim: hash mapler (karma haritalar)!