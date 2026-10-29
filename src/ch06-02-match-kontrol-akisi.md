<!-- Old headings. Do not remove or links may break. -->

<a id="the-match-control-flow-operator"></a>
<a id="the-match-control-flow-construct"></a>
<a id="match-kontrol-akisi-yapisi"></a>

## `match` Kontrol Akışı Yapısı

Rust, bir değeri bir dizi *desenle (pattern)* karşılaştırmanıza ve hangi desenin eşleştiğine bağlı olarak ilgili kodu çalıştırmanıza olanak tanıyan `match` adında son derece güçlü bir kontrol akışı yapısına sahiptir. Desenler; sabit değerlerden, değişken adlarından, joker desenlerden (*wildcards*) ve daha pek çok bileşenden oluşabilir; [Bölüm 19][ch19-00-patterns]<!-- ignore --> tüm farklı desen türlerini ve ne işe yaradıklarını ayrıntılı olarak ele alır. `match` yapısının gücü, desenlerin yüksek ifade yeteneğinden ve derleyicinin olası tüm durumların eksiksiz biçimde ele alındığını doğrulamasından gelir.

Bir `match` ifadesini bozuk para ayırma makinesi gibi düşünebilirsiniz: Madeni paralar, boyunca farklı boyutlarda delikler bulunan bir raydan aşağı kayar ve her para, karşısına çıkan ve içine sığabildiği ilk delikten aşağı düşer. Aynı şekilde değerler de bir `match` ifadesindeki her bir desenden sırayla geçer ve değerin "uyduğu" ilk desende, çalıştırılmak üzere o desenle ilişkili kod bloğuna düşer.

Madeni paralardan söz etmişken gelin `match` kullanımı için onları örnek alalım! Liste 6-3'te gösterildiği gibi, belirsiz bir ABD madeni parasını alan ve tıpkı para sayma makinesi gibi hangi madeni para olduğunu belirleyip değerini kuruş (sent) cinsinden döndüren bir fonksiyon yazabiliriz.

<Listing number="6-3" caption="Bir enum ve bu enum'ın varyantlarını desen olarak kullanan bir `match` ifadesi">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

</Listing>

Gelin `kurus_degeri` fonksiyonundaki `match` ifadesini adım adım inceleyelim. İlk olarak `match` anahtar kelimesini ve ardından bir ifadeyi yazarız; bu örnekte söz konusu ifade `para` değeridir. Bu yapı `if` ile kullanılan koşul ifadesine çok benzer, ancak aralarında büyük bir fark vardır: `if` yapısında koşulun bir Boole (`bool`) değeri üretmesi gerekirken, burada ifade herhangi bir türde olabilir. Bu örnekteki `para` değerinin türü, ilk satırda tanımladığımız `MadeniPara` enum'ıdır.

Ardından *`match` kolları (match arms)* gelir. Bir kol iki bölümden oluşur: bir desen ve çalıştırılacak kod. Buradaki ilk kolun deseni `MadeniPara::Kurus` değeridir; hemen ardından deseni ve çalıştırılacak kodu birbirinden ayıran `=>` operatörü gelir. Bu koldaki kod yalnızca `1` değeridir. Her kol bir sonrakinden virgülle ayrılır.

`match` ifadesi çalıştırıldığında, elde edilen değeri her bir kolun deseniyle sırasıyla karşılaştırır. Bir desen değerle eşleşirse o desenle ilişkili kod çalıştırılır. Desen değerle eşleşmezse, tıpkı bozuk para ayırma makinesinde olduğu gibi akış bir sonraki kola geçer. İhtiyaç duyduğumuz kadar kol tanımlayabiliriz: Liste 6-3'te `match` ifademizin dört kolu vardır.

Her kolla ilişkili kod bir *ifadedir (expression)* ve eşleşen koldaki ifadenin ürettiği değer, `match` ifadesinin tamamı için döndürülen değer olur.

Liste 6-3'te olduğu gibi her kolun yalnızca tek bir değer ürettiği kısa durumlarda `match` kollarında genellikle süslü parantez kullanmayız. Bir `match` kolunda birden fazla satır kod çalıştırmak istiyorsanız süslü parantez kullanmanız gerekir; bu durumda kolu takip eden virgül isteğe bağlıdır. Örneğin aşağıdaki kod, fonksiyon her `MadeniPara::Kurus` değeriyle çağrıldığında ekrana `"Şanslı kuruş!"` yazdırır, ancak yine de bloğun son ifadesi olan `1` değerini döndürür:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

> Not: Tıpkı Bölüm 3'te gördüğümüz `if` gibi `match` de bir *ifadedir*. Dolayısıyla ürettiği sonucu doğrudan bir değişkene atayabilir (`let tutar = match para { ... };`) veya burada olduğu gibi fonksiyonun dönüş değeri olarak kullanabilirsiniz. Derleyicinin derleme zamanında tek ve kesin bir tür belirleyebilmesi için **tüm `match` kollarının aynı türü üretmesi** zorunludur; aksi halde `E0308` (`match arms have incompatible types` — *`match` kolları uyumsuz türlere sahip*) hatası alırsınız. Özellikle süslü parantezli kollarda son satırdaki `1` ifadesinin sonuna yanlışlıkla noktalı virgül (`;`) koymamaya dikkat edin: `1;` yazarsanız o kol birim türü (`()`) üretirken diğer kollar `u8` üreteceği için kod derlenmeyecektir.

<!-- Old headings. Do not remove or links may break. -->

<a id="patterns-that-bind-to-values"></a>
<a id="degerlere-baglanan-desenler"></a>

### Değerlere Bağlanan Desenler

`match` kollarının bir diğer kullanışlı özelliği, desenle eşleşen değerlerin içindeki parçalara bağlanabilmeleridir. Enum varyantlarının barındırdığı değerleri bu sayede dışarı çıkarabiliriz.

Örnek olarak enum varyantlarımızdan birini içinde veri tutacak şekilde değiştirelim. Amerika Birleşik Devletleri, 1999'dan 2008'e kadar 50 eyaletin her biri için bir yüzünde farklı tasarımlar bulunan çeyreklikler (25 kuruşluk/sentlik madeni paralar) bastı. Başka hiçbir madeni parada eyalet tasarımı yer almadığından bu ek bilgi yalnızca çeyrekliklerde bulunur. Bu bilgiyi Liste 6-4'te yaptığımız gibi `YirmiBesKurus` varyantını içinde bir `Eyalet` değeri barındıracak şekilde değiştirerek `enum` tanımımıza ekleyebiliriz.

<Listing number="6-4" caption="`YirmiBesKurus` varyantının ayrıca bir `Eyalet` değeri tuttuğu `MadeniPara` enum'ı">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

</Listing>

Bir arkadaşımızın 50 eyalet çeyrekliğinin tamamını biriktirmeye çalıştığını düşünelim. Bozuk paralarımızı türlerine göre ayırırken her çeyreklikle ilişkili eyaletin adını da yüksek sesle söyleyeceğiz; böylece arkadaşımızın koleksiyonunda henüz olmayan bir eyalet çıkarsa onu koleksiyonuna ekleyebilecek.

Bu kodun `match` ifadesinde, `MadeniPara::YirmiBesKurus` varyantının değerleriyle eşleşen desene `eyalet` adında bir değişken ekliyoruz. Bir `MadeniPara::YirmiBesKurus` değeri eşleştiğinde `eyalet` değişkeni o çeyrekliğin eyalet değerine bağlanır. Ardından bu koldaki kod içinde `eyalet` değişkenini aşağıdaki gibi kullanabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

`kurus_degeri(MadeniPara::YirmiBesKurus(Eyalet::Alaska))` çağrısı yaptığımızda `para` parametresi `MadeniPara::YirmiBesKurus(Eyalet::Alaska)` değerini alır. Bu değeri her bir `match` koluyla sırayla karşılaştırdığımızda `MadeniPara::YirmiBesKurus(eyalet)` koluna ulaşana kadar hiçbir desen eşleşmez. Bu noktada `eyalet` değişkeni `Eyalet::Alaska` değerine bağlanır. Böylece bu değişkeni `println!` ifadesinde kullanabilir ve `MadeniPara` enum'ının `YirmiBesKurus` varyantı içindeki eyalet verisini elde edebiliriz.

> Not: Sahipliği sizde olan bir enum üzerinde doğrudan `match para` yaptığınızda, `Eyalet` veya `String` gibi `Copy` trait'ini uygulamayan iç veriler desendeki değişkene (`eyalet`) **taşınır (*move*)**. Bu nedenle `match` bloğundan sonra `para` değişkenini tekrar kullanamazsınız. Eğer enum'ın sahipliğini tüketmeden yalnızca içindeki veriyi okumak istiyorsanız referans üzerinden eşleştirme (`match &para`) yapabilirsiniz. Rust'ın *eşleştirme ergonomisi (match ergonomics)* özelliği sayesinde `match &para` yazdığınızda deseni değiştirmeden yine `MadeniPara::YirmiBesKurus(eyalet)` şeklinde yazabilirsiniz; derleyici `eyalet` değişkenini veriyi taşımak yerine otomatik olarak `&Eyalet` referansı biçiminde bağlar.

<!-- Old headings. Do not remove or links may break. -->

<a id="matching-with-optiont"></a>
<a id="the-optiont-match-pattern"></a>

### `Option<T>` `match` Deseni

Önceki bölümde `Option<T>` kullanırken `Some` varyantının içindeki `T` değerini elde etmek istemiştik; tıpkı `MadeniPara` enum'ında yaptığımız gibi `Option<T>` türünü de `match` kullanarak işleyebiliriz! Bu kez madeni paraları karşılaştırmak yerine `Option<T>` varyantlarını karşılaştıracağız, ancak `match` ifadesinin çalışma mantığı birebir aynı kalır.

Diyelim ki parametre olarak bir `Option<i32>` alan ve içinde bir değer varsa bu değere `1` ekleyen bir fonksiyon yazmak istiyoruz. Eğer içinde bir değer yoksa fonksiyon `None` döndürmeli ve başka hiçbir işlem yapmaya çalışmamalıdır.

`match` sayesinde bu fonksiyonu yazmak son derece kolaydır ve Liste 6-5'teki gibi görünür.

<Listing number="6-5" caption="Bir `Option<i32>` üzerinde `match` ifadesi kullanan bir fonksiyon">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

</Listing>

`bir_ekle` fonksiyonunun ilk çağrılışını daha yakından inceleyelim. `bir_ekle(bes)` çağrısı yaptığımızda `bir_ekle` gövdesindeki `x` değişkeni `Some(5)` değerini alır. Ardından bu değeri her bir `match` koluyla sırayla karşılaştırırız:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

`Some(5)` değeri `None` deseniyle eşleşmez; bu yüzden bir sonraki kola geçeriz:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

`Some(5)` değeri `Some(i)` deseniyle eşleşir mi? Evet, eşleşir! İkisi de aynı varyanttır. `i` değişkeni `Some` içindeki değere bağlanır ve `5` değerini alır. Ardından bu `match` kolundaki kod çalıştırılır: `i` değerine `1` eklenir ve içinde toplam `6` değerini taşıyan yeni bir `Some` varyantı oluşturulup döndürülür.

Şimdi Liste 6-5'te `x` parametresinin `None` olduğu ikinci `bir_ekle` çağrısına bakalım. `match` ifadesine girer ve ilk kolla karşılaştırırız:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Eşleşti! Eklenecek bir değer olmadığından akış burada tamamlanır ve `=>` operatörünün sağ tarafındaki `None` değeri döndürülür. İlk kol eşleştiği için diğer kollar hiç kontrol edilmez.

`match` ile enum'ları bir arada kullanmak pek çok senaryoda son derece işlevseldir. Rust kodlarında şu yapıyı çok sık göreceksiniz: Bir enum üzerinde `match` uygula, içindeki veriyi bir değişkene bağla ve bu veriye dayanarak ilgili kodu çalıştır. İlk başta biraz yabancı gelebilir, ancak bir kez alıştığınızda bu yapının kullandığınız tüm programlama dillerinde olmasını isteyeceksiniz. Rust kullanıcılarının her zaman en sevdiği özelliklerin başında gelir.

<!-- Old headings. Do not remove or links may break. -->

<a id="matches-are-exhaustive"></a>
<a id="eşleşmeler-kapsamlıdır-exhaustive"></a>
<a id="eslesmeler-kapsamlidir"></a>

### Eşleşmeler Kapsamlıdır

`match` ifadelerinin ele almamız gereken önemli bir yönü daha vardır: Kolların desenleri olası tüm durumları kapsamak zorundadır. `bir_ekle` fonksiyonumuzun hata barındıran ve derlenmeyecek olan aşağıdaki sürümüne bakalım:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

Burada `None` durumunu ele almadık; dolayısıyla bu kod hataya yol açacaktır. Neyse ki bu, Rust derleyicisinin anında yakalayabildiği türden bir hatadır. Bu kodu derlemeye çalıştığımızda aşağıdaki hatayı alırız:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

> Not: Derleyicinin ürettiği `error[E0004]: non-exhaustive patterns: None not covered` mesajı, *"kapsamlı olmayan desenler: `None` durumu kapsanmadı"* anlamına gelir. Dikkat ederseniz `help:` satırında derleyici, eksik kalan kolu `None => todo!(),` şeklinde kodunuza nasıl ekleyebileceğinizi bile doğrudan gösterir.

Rust, olası her durumu kapsamadığımızı bilir; üstelik hangi deseni unuttuğumuzu bile tam olarak söyler! Rust'ta eşleşmeler _kapsamlıdır (exhaustive)_: Kodun geçerli olabilmesi için olası her durumu sonuna kadar, eksiksiz biçimde ele almalıyız. Özellikle `Option<T>` söz konusu olduğunda, Rust `None` durumunu açıkça işlemeyi unutmamızı engelleyerek ortada bir değer yokken (`null` benzeri bir durumdayken) sanki geçerli bir değer varmış gibi varsayımda bulunmamızın önüne geçer; böylece önceki bölümde bahsettiğimiz milyar dolarlık hatayı derleme zamanında imkânsız hale getirir.

<!-- Old headings. Do not remove or links may break. -->

<a id="catch-all-patterns-and-the-_-placeholder"></a>
<a id="hepsini-yakalayan-catch-all-desenler-ve-_-yer-tutucusu"></a>
<a id="tumunu-yakalama-desenleri-ve-_-yer-tutucusu"></a>

### Tümünü Yakalama Desenleri ve `_` Yer Tutucusu

Enum'ları ve `match` yapısını kullanarak belirli birkaç değer için özel işlemler yapabilir, kalan diğer tüm değerler için ise tek bir varsayılan işlem uygulayabiliriz. Şöyle bir oyun geliştirdiğimizi düşünelim: Zar attığınızda `3` gelirse oyuncunuz hareket etmez, bunun yerine süslü yeni bir şapka kazanır. Eğer `7` gelirse oyuncunuz süslü şapkasını kaybeder. Diğer tüm zar değerlerinde ise oyuncunuz oyun tahtasında gelen sayı kadar kare ilerler. Aşağıda bu mantığı uygulayan bir `match` ifadesi yer alıyor; bu örnekte zar atışının sonucu rastgele üretilmek yerine doğrudan koda sabit olarak yazılmış ve diğer işlemlerin ayrıntıları konumuzun dışında olduğundan gövdesi boş fonksiyonlarla temsil edilmiştir:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

İlk iki kolun desenleri `3` ve `7` sabit değerleridir. Kalan olası tüm değerleri kapsayan son kolun deseni ise `diger` adını verdiğimiz değişkendir. `diger` kolunda çalışan kod, yakalanan bu değişkeni `oyuncuyu_tasi` fonksiyonuna argüman olarak aktararak kullanır.

Bir `u8` türünün alabileceği olası tüm değerleri tek tek listelememiş olsak da bu kod sorunsuzca derlenir; çünkü son desen özel olarak listelenmeyen tüm değerlerle eşleşir. Bu *tümünü yakalama deseni (catch-all pattern)*, `match` ifadesinin kapsamlı olma zorunluluğunu karşılar. Desenler sırayla değerlendirildiği için tümünü yakalayan kolu en sona koymamız gerektiğine dikkat edin. Eğer bu kolu daha üst bir sıraya koysaydık ondan sonraki kollar asla çalışmazdı; nitekim tümünü yakalayan bir koldan sonra başka kollar eklerseniz Rust ulaşılamaz desen konusunda sizi uyarır!

Rust ayrıca tümünü yakalayan bir kol istediğimizde ancak yakalanan değeri _kullanmak_ istemediğimizde başvurabileceğimiz özel bir desene sahiptir: `_`, herhangi bir değerle eşleşen fakat o değere bağlanmayan özel bir yer tutucu desendir. Bu desen Rust'a söz konusu değeri kullanmayacağımızı bildirir; böylece Rust ne değeri taşır ne de kullanılmayan bir değişken hakkında uyarı verir.

Gelin oyunun kurallarını değiştirelim: Artık `3` veya `7` dışında bir sayı atarsanız tekrar zar atmanız gerekiyor. Yakalanan değere artık ihtiyacımız olmadığından kodumuzu `diger` adlı değişken yerine `_` kullanacak şekilde güncelleyebiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

Son kolda diğer tüm değerleri açıkça yoksaydığımız için bu örnek de kapsamlılık şartını eksiksiz karşılar; unuttuğumuz hiçbir durum yoktur.

Son olarak oyunun kurallarını bir kez daha değiştirelim: `3` veya `7` dışında bir sayı attığınızda sıranızda başka hiçbir şey olmasın. Bunu, `_` kolunda çalıştırılacak kod olarak birim değerini (["Demet Türü"][tuples]<!-- ignore --> bölümünde bahsettiğimiz boş demet türü `()`) yazarak ifade edebiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Burada Rust'a önceki kolların desenleriyle eşleşmeyen hiçbir değeri kullanmayacağımızı ve bu durumda herhangi bir kod çalıştırmak istemediğimizi açıkça belirtmiş oluyoruz.

Desenler ve eşleştirme hakkında [Bölüm 19][ch19-00-patterns]<!-- ignore -->'da ele alacağımız daha pek çok ayrıntı var. Şimdilik, `match` ifadesinin biraz ayrıntılı ve uzun kalabildiği durumlarda oldukça pratik bir alternatif sunan `if let` sözdizimine geçeceğiz.

[tuples]: ch03-02-veri-turleri.html#demet-türü
[ch19-00-patterns]: ch19-00-desenler.html
