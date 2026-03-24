## İlişkili Değerlerle Anahtarları Hash Maplerde Saklamak

Yaygın koleksiyonlarımızın sonuncusu hash map'tir (karma harita). `HashMap<K, V>` türü, `K` türündeki anahtarları `V` türündeki değerlerle eşleştirmeyi depolamak için bir _karma fonksiyonu_ kullanır ve bu fonksiyon, bu anahtar ve değerleri belleğe nasıl yerleştireceğini belirler. Birçok programlama dili bu tür bir veri yapısını destekler, ancak genellikle _hash_, _map_ (harita), _object_ (nesne), _hash table_ (karma tablosu), _dictionary_ (sözlük) veya _associative array_ (ilişkisel dizi) gibi birkaç farklı isim kullanırlar.

Hash mapler, verileri vektörlerde olduğu gibi bir indeks kullanarak değil de herhangi bir türde olabilen bir anahtar kullanarak aramak istediğinizde kullanışlıdır. Örneğin bir oyunda, her takımın skorunu bir hash mapte tutabilirsiniz; burada her anahtar bir takımın adıdır ve değerler her takımın skorudur. Bir takımın adı verildiğinde, skorunu alabilirsiniz.

Bu bölümde hash maplerin temel API'sini inceleyeceğiz, ancak standart kütüphane tarafından `HashMap<K, V>` üzerinde tanımlanan fonksiyonlarda daha birçok güzel şey saklıdır. Her zaman olduğu gibi, daha fazla bilgi için standart kütüphane belgelerini kontrol edin.

### Yeni Bir Hash Map Oluşturmak

Boş bir hash map oluşturmanın bir yolu `new` kullanmak ve `insert` (ekle) ile eleman eklemektir. Liste 8-20'de isimleri _Mavi_ ve _Sarı_ olan iki takımın skorlarını takip ediyoruz. Mavi takım 10 puanla, Sarı takım ise 50 puanla başlar.

<Listing number="8-20" caption="Yeni bir hash map oluşturmak ve bazı anahtarlar ve değerler eklemek">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

</Listing>

Önce standart kütüphanenin koleksiyonlar kısmından `HashMap`'i `use` ile kullanıma dahil etmemiz gerektiğine dikkat edin. Üç yaygın koleksiyonumuzdan bu, en az sıklıkla kullanılanıdır, bu nedenle otomatik olarak kapsama dahil edilen özellikler arasında yer almaz. Hash mapler ayrıca standart kütüphaneden daha az destek alır; örneğin, bunları oluşturmak için yerleşik bir makro yoktur.

Tıpkı vektörler gibi, hash mapler de verilerini yığında depolar. Bu `HashMap`, `String` türünde anahtarlara ve `i32` türünde değerlere sahiptir. Vektörler gibi, hash mapler de homojendir: Tüm anahtarlar aynı türde olmalı ve tüm değerler aynı türde olmalıdır.

### Bir Hash Mapteki Değerlere Erişmek

Liste 8-21'de gösterildiği gibi anahtarını `get` (al) metoduna sağlayarak hash map'ten bir değer alabiliriz.

<Listing number="8-21" caption="Hash mapte depolanan Mavi takımın skoruna erişmek">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

</Listing>

Burada `skor`, Mavi takımla ilişkili değere sahip olacak ve sonuç `10` olacaktır. `get` metodu bir `Option<&V>` döndürür; hash mapte o anahtar için bir değer yoksa `get`, `None` döndürecektir. Bu program `Option`'ı, `Option<&i32>` yerine `Option<i32>` almak için `copied` (kopyalanmış) çağrısı yaparak ve ardından eğer `skorlar` bu anahtar için bir girdiye sahip değilse `skor`'u sıfıra ayarlamak için `unwrap_or` çağrısı yaparak ele alır.

Bir `for` döngüsü kullanarak, vektörlerde yaptığımıza benzer bir şekilde bir hash map'teki her anahtar-değer çifti üzerinde yineleme yapabiliriz:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Bu kod her bir çifti rastgele bir sırada yazdıracaktır:

```text
Sarı: 50
Mavi: 10
```

<!-- Old headings. Do not remove or links may break. -->

<a id="hash-maps-and-ownership"></a>

### Hash Maplerde Sahipliği Yönetmek

`i32` gibi `Copy` (kopyalama) trait'ini uygulayan türler için değerler hash map'e kopyalanır. `String` gibi sahiplenilmiş değerler için değerler taşınacak ve Liste 8-22'de gösterildiği gibi hash map bu değerlerin sahibi olacaktır.

<Listing number="8-22" caption="Anahtarların ve değerlerin, eklendiklerinde (inserted) hash map'in mülkiyetine geçtiğini göstermek">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

</Listing>

`alan_adi` ve `alan_degeri` değişkenlerini, `insert` çağrısıyla hash map'e taşındıktan sonra artık kullanamayız.

Eğer hash map'e değerlere ait referansları eklersek, değerler hash map'e taşınmaz. Referansların işaret ettiği değerler, en az hash map geçerli olduğu sürece geçerli olmalıdır. Bu konular hakkında Bölüm 10'da ["Referansları Ömürlerle Doğrulamak (Validating References with Lifetimes)"][validating-references-with-lifetimes]<!-- ignore --> kısmında daha fazla konuşacağız.

### Bir Hash Mapi Güncellemek

Anahtar ve değer çiftlerinin sayısı artabilir olsa da her benzersiz anahtarın aynı anda yalnızca bir değerle ilişkisi olabilir (ancak tersi geçerli değildir: Örneğin hem Mavi takım hem de Sarı takım, `skorlar` hash map'inde `10` değerini depolayabilir).

Bir hash mapteki veriyi değiştirmek istediğinizde, bir anahtara halihazırda bir değer atanmışsa bu durumu nasıl ele alacağınıza karar vermelisiniz. Eski değeri tamamen göz ardı edip yerine yeni değeri koyabilirsiniz. Eski değeri koruyup yeni değeri yok sayabilir ve yalnızca anahtarın zaten bir değeri _yoksa_ yeni değeri ekleyebilirsiniz. Veya eski değer ile yeni değeri birleştirebilirsiniz. Her birini nasıl yapacağımıza bakalım!

#### Bir Değerin Üzerine Yazmak (Overwriting)

Eğer bir hash map'e bir anahtar ve bir değer ekler ve daha sonra aynı anahtarı farklı bir değerle eklerseniz, o anahtarla ilişkili değer değiştirilir. Liste 8-23'teki kod `insert` fonksiyonunu iki kez çağırsa bile, hash map yalnızca bir anahtar-değer çifti barındıracaktır çünkü her ikisinde de Mavi takımın anahtarı için değer ekliyoruz.

<Listing number="8-23" caption="Belirli bir anahtarla depolanan bir değeri değiştirmek">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

</Listing>

Bu kod `{"Mavi": 25}` yazdıracaktır. `10` olan orijinal değerin üzerine yazılmıştır.

<!-- Old headings. Do not remove or links may break. -->

<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### Yalnızca Bir Anahtarın Değeri Yoksa Bir Anahtar ve Değer Eklemek

Belirli bir anahtarın bir değer ile hash mapte halihazırda var olup olmadığını kontrol etmek ve ardından şu eylemleri gerçekleştirmek yaygındır: Eğer anahtar hash mapte varsa mevcut değer olduğu gibi kalmalıdır; eğer anahtar yoksa onu ve onun için bir değer ekleyin.

Hash maplerin bunun için kontrol etmek istediğiniz anahtarı parametre olarak alan `entry` (giriş) adında özel bir API'si vardır. `entry` metodunun dönüş değeri, var olabilecek veya olmayabilecek bir değeri temsil eden `Entry` adlı bir enum'dır. Diyelim ki Sarı takımın anahtarının onunla ilişkili bir değeri olup olmadığını kontrol etmek istiyoruz. Eğer yoksa `50` değerini eklemek istiyoruz ve aynısını Mavi takım için de yapmak istiyoruz. `entry` API'si kullanılarak kod, Liste 8-24'teki gibi görünür.

<Listing number="8-24" caption="Sadece anahtar halihazırda bir değere sahip değilse `entry` metodu kullanarak değer eklemek">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

</Listing>

`Entry` üzerindeki `or_insert` metodu, karşılık gelen `Entry` anahtarı varsa o değere değiştirilebilir bir referans (mutable reference) döndürmek üzere tanımlanmıştır ve eğer yoksa, parametreyi bu anahtar için yeni bir değer olarak ekler ve yeni değere değiştirilebilir bir referans döndürür. Bu teknik, mantığı kendi başımıza yazmaktan çok daha temizdir ve ek olarak ödünç alma denetleyicisi ile daha uyumlu çalışır.

Liste 8-24'teki kodun çalıştırılması `{"Sarı": 50, "Mavi": 10}` yazdıracaktır. İlk `entry` çağrısı, Sarı takım için `50` değeri ile birlikte anahtarı ekleyecektir çünkü Sarı takımın halihazırda bir değeri yoktur. İkinci `entry` çağrısı hash map'i değiştirmeyecektir çünkü Mavi takım halihazırda `10` değerine sahiptir.

#### Eski Değere Dayalı Olarak Bir Değeri Güncellemek

Hash mapler için başka bir yaygın kullanım durumu, bir anahtarın değerini aramak ve sonra eski değere dayanarak onu güncellemektir. Örneğin Liste 8-25, bazı metinlerdeki her bir kelimenin kaç kez geçtiğini sayan bir kod gösterir. Anahtarlar olarak kelimeleri içeren bir hash map kullanıyoruz ve o kelimeyi kaç kez gördüğümüzü takip etmek için değeri artırıyoruz. Eğer bir kelimeyi ilk kez görüyorsak ilk olarak `0` değerini ekleyeceğiz.

<Listing number="8-25" caption="Kelimeleri ve sayımları saklayan bir hash map kullanarak kelimelerin ortaya çıkma sıklığını saymak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

</Listing>

Bu kod `{"dünya": 2, "merhaba": 1, "harika": 1}` yazdıracaktır. Aynı anahtar-değer çiftlerinin farklı bir sırada yazdırıldığını görebilirsiniz: ["Bir Hash Mapteki Değerlere Erişmek"][access]<!-- ignore --> bölümünden bir hash map üzerinde yineleme yapmanın rastgele bir düzende gerçekleştiğini hatırlayın.

`split_whitespace` (boşluklardan_ayır) metodu, `metin` içindeki değerin boşluklarla ayrılmış alt dilimleri üzerinde bir yineleyici döndürür. `or_insert` metodu belirtilen anahtar için değere değiştirilebilir bir referans (`&mut V`) döndürür. Burada, bu değiştirilebilir referansı `sayac` değişkeninde depoluyoruz, bu nedenle bu değere atama yapmak için öncelikle yıldız imi (`*`) kullanarak `sayac`'ı referanstan kaldırmalıyız. Değiştirilebilir referans `for` döngüsünün sonunda kapsamın dışına çıkar, bu nedenle tüm bu değişiklikler güvenlidir ve ödünç alma kuralları tarafından izin verilir.

### Karma (Hashing) Fonksiyonları

Varsayılan olarak `HashMap`, karma tablolarını barındıran hizmet reddi (denial-of-service, DoS) saldırılarına karşı direnç sağlayabilen _SipHash_ adlı bir karma fonksiyonu kullanır[^siphash]<!-- ignore -->. Bu mevcut en hızlı karma algoritması değildir, ancak performanstaki düşüşle birlikte gelen daha iyi güvenlik için yapılan takasa değer. Eğer kodunuzun profilini çıkarır ve varsayılan karma fonksiyonunun amaçlarınız için çok yavaş olduğunu fark ederseniz, farklı bir karma oluşturucu belirterek başka bir fonksiyona geçiş yapabilirsiniz. Bir _hasher_ (karma oluşturucu), `BuildHasher` trait'ini uygulayan bir türdür. Trait'ler ve bunların nasıl uygulanacağı hakkında [Bölüm 10][traits]<!-- ignore -->'da konuşacağız. Kendi karma oluşturucunuzu baştan uygulamak zorunda değilsiniz; [crates.io](https://crates.io/)<!-- ignore -->, diğer Rust kullanıcıları tarafından paylaşılan, birçok yaygın karma algoritmasını uygulayan karma oluşturucular sağlayan kütüphanelere sahiptir.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## Özet

Vektörler, stringler ve hash mapler verileri depolamanız, bunlara erişmeniz ve bunları değiştirmeniz gerektiğinde programlarda gereken büyük miktarda işlevselliği sağlayacaktır. İşte artık çözmek için donanımlı olmanız gereken bazı egzersizler:

1. Bir tam sayı listesi verildiğinde bir vektör kullanın ve listenin medyanını (sıralandığında, orta konumdaki değer) ve modunu (en sık oluşan değer; burada bir hash map yardımcı olacaktır) döndürün.
1. Stringleri Pig Latin'e (Domuz Latincesi) dönüştürün. Her kelimenin ilk sessiz harfi kelimenin sonuna taşınır ve _ay_ eklenir, böylece _ilk_ _ilkay-f_ (irst-fay) olur. Sesli harfle başlayan kelimelerin sonuna bunun yerine _hay_ eklenir (_elma_ (apple), _elma-hay_ (apple-hay) olur). UTF-8 kodlamasıyla ilgili detayları aklınızda bulundurun!
1. Bir hash map ve vektörler kullanarak bir kullanıcının bir şirketteki bir departmana çalışan isimleri eklemesini sağlamak için bir metin arayüzü oluşturun; örneğin, "Sally'yi Mühendisliğe Ekle (Add Sally to Engineering)" veya "Amir'i Satışa Ekle (Add Amir to Sales)". Ardından kullanıcının bir departmandaki tüm kişilerin veya şirketteki tüm kişilerin departmana göre alfabetik olarak sıralanmış bir listesini almasına izin verin.

Standart kütüphane API belgeleri, bu egzersizler için yararlı olacak vektörlerin, stringlerin ve hash maplerin sahip olduğu metotları açıklar!

İşlemlerin başarısız olabileceği daha karmaşık programlara giriyoruz, bu yüzden hata yönetimi hakkında tartışmak için mükemmel bir zaman. Sırada bunu yapacağız!

[validating-references-with-lifetimes]: ch10-03-omurler.html#referansları-ömürlerle-lifetimes-doğrulamak
[access]: #bir-hash-mapteki-değerlere-erişmek
[traits]: ch10-02-traitler.html
