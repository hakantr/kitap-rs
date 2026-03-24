## Gelişmiş Türler

Rust'ın tür sisteminde şimdiye kadar adını anıp ayrıntısına girmediğimiz bazı özellikler var. Önce newtype deseninin neden yararlı olduğunu anlatarak başlayacağız. Sonra newtype'a benzeyen ama anlamı biraz farklı olan tür takma adlarına geçeceğiz. Ardından `!` türünü ve dinamik boyutlu türleri ele alacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-newtype-pattern-for-type-safety-and-abstraction"></a>

### Newtype Deseniyle Tür Güvenliği ve Soyutlama

Bu bölüm, daha önceki ["Dış Trait'leri Newtype Deseniyle Uygulamak"][newtype]<!-- ignore --> kısmını okuduğunuzu varsayar. Newtype deseni, şimdiye kadar ele aldığımızın ötesinde işler için de kullanışlıdır. Bunların arasında değerlerin birbiriyle karışmasını derleme zamanında önlemek ve bir değerin birimini açıkça göstermek de vardır. Liste 20-16'da bunun bir örneğini görmüştük: `Milimetreler` ve `Metreler` yapıları, `u32` değerlerini newtype olarak sarmalıyordu. Eğer bir fonksiyon `Milimetreler` parametresi alıyorsa, ona yanlışlıkla `Metreler` ya da doğrudan `u32` vermeye çalışan bir program derlenmez.

Newtype desenini, bir türün bazı uygulama ayrıntılarını gizlemek için de kullanabiliriz. Yeni tür, içteki gizli türden farklı bir açık API sunabilir.

Newtype'lar iç uygulamayı da gizleyebilir. Örneğin, isimlerle kimlik numaralarını eşleyen `HashMap<i32, String>` yapısını sarmalayan `People` adlı bir tür sunabiliriz. `People` kullanan kod yalnızca dışarı sunduğumuz açık API ile, örneğin koleksiyona isim ekleyen bir metodla etkileşir. İçeride isimlere `i32` kimlik atadığımızı bilmesi gerekmez. Newtype deseni, 18. bölümdeki ["Uygulama Ayrıntılarını Gizleyen Kapsülleme"][encapsulation-that-hides-implementation-details]<!-- ignore --> kısmında anlattığımız kapsüllemeyi elde etmenin hafif bir yoludur.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-type-synonyms-with-type-aliases"></a>

### Tür Eşanlamlıları ve Tür Takma Adları

Rust, var olan bir türe başka bir ad vermek için _tür takma adı_ tanımlamanıza izin verir. Bunun için `type` anahtar kelimesini kullanırız. Örneğin `i32` için `Kilometreler` adında bir takma ad oluşturabiliriz:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

Artık `Kilometreler`, `i32` için bir _eşanlamlıdır_. Liste 20-16'da oluşturduğumuz `Milimetreler` ve `Metreler` türlerinden farklı olarak `Kilometreler` yeni ve ayrı bir tür değildir. `Kilometreler` türündeki değerler `i32` ile tamamen aynı kabul edilir:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

`Kilometreler` ile `i32` aynı tür olduğu için her iki türden değerleri toplayabilir, `i32` bekleyen fonksiyonlara `Kilometreler` geçebiliriz. Ama bu yaklaşım, newtype desenindeki tür denetimi avantajlarını vermez. Yani bir yerde `Kilometreler` ile `i32` değerlerini karıştırırsak derleyici hata vermez.

Tür eşanlamlılarının başlıca kullanım nedeni tekrarları azaltmaktır. Örneğin şöyle uzun bir türümüz olabilir:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

Böyle uzun bir türü fonksiyon imzalarında ve tür açıklamalarında tekrar tekrar yazmak yorucu ve hataya açıktır. Kodu bu şekilde dolu bir proje hayal edin; Liste 20-25 buna örnektir.

<Listing number="20-25" caption="Birçok yerde uzun bir tür kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-25/src/main.rs:here}}
```

</Listing>

Tür takma adı, tekrarları azaltarak bu kodu daha yönetilebilir hâle getirir. Liste 20-26'da ayrıntılı tür için `ErtelenenIs` adlı bir takma ad tanımlıyoruz ve bütün kullanımları buna çeviriyoruz.

<Listing number="20-26" caption="Tekrarı azaltmak için `ErtelenenIs` adlı bir tür takma adı tanıtmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-26/src/main.rs:here}}
```

</Listing>

Bu sürüm okumayı ve yazmayı çok kolaylaştırır. Takma ad için anlamlı bir isim seçmek de niyetinizi daha iyi anlatır. Örneğin _thunk_, daha sonra değerlendirilecek kod parçası anlamında kullanılan yerleşik bir terimdir; bu yüzden saklanan bir kapanış için uygun addır.

Tür takma adları, tekrarları azaltmak amacıyla `Result<T, E>` ile de çok sık kullanılır. Standart kütüphanedeki `std::io` modülünü düşünün. Girdi/çıktı işlemleri, işlem başarısız olabileceği için çoğu zaman `Result<T, E>` döndürür. Bu modülde tüm olası G/Ç hatalarını temsil eden `std::io::Error` yapısı vardır. `std::io` içindeki pek çok fonksiyonun dönüş türü, `E` kısmı `std::io::Error` olan `Result<T, E>` olur. `Write` trait'indeki şu fonksiyonlar buna örnektir:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

Buradaki `Result<..., Error>` ifadesi tekrar tekrar yazılıyor. Bu yüzden `std::io` modülü şu tür takma adını tanımlar:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

Bu bildirim `std::io` modülü içinde olduğundan, `std::io::Result<T>` tam nitelikli takma adını kullanabiliriz; yani `E` kısmı `std::io::Error` ile doldurulmuş bir `Result<T, E>`. Böylece `Write` trait'inin imzaları şu hâle gelir:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

Bu tür takma adı iki açıdan faydalıdır: kodu yazmayı kolaylaştırır ve `std::io` genelinde tutarlı bir arayüz sunar. Sonuçta bu yalnızca bir takma addır; yani aslında yine `Result<T, E>` kullanıyoruz. Bu yüzden `Result<T, E>` üzerinde çalışan metodların hepsini ve `?` gibi özel sözdizimlerini bununla da kullanabiliriz.

### Hiç Dönmeyen Never Türü

Rust'ta `!` adında özel bir tür vardır. Tür kuramında buna _boş tür_ denir; çünkü hiç değeri yoktur. Biz buna _never türü_ demeyi tercih ediyoruz; çünkü bir fonksiyon hiç dönmeyecekse, dönüş türünün yerine bu yazılır. Örnek:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

Bu kodu "`bar` fonksiyonu hiç dönmez" diye okuruz. Hiç dönmeyen fonksiyonlara _ayrışan fonksiyonlar_ denir. `!` türünde değer üretemeyeceğimiz için `bar` hiçbir koşulda gerçekten dönemeyecektir.

Peki hiç değeri olmayan bir tür ne işe yarar? Bunun için sayı tahmin oyunundan Liste 2-5'i hatırlayın; o kodun küçük bir parçasını burada Liste 20-27 olarak yeniden veriyoruz.

<Listing number="20-27" caption="Kollarından biri `continue` ile biten bir `match`">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

</Listing>

O sırada bu kodun bazı ayrıntılarını atlamıştık. 6. bölümdeki ["`match` Kontrol Akışı Yapısı"][the-match-control-flow-construct]<!-- ignore --> kısmında, `match` kollarının aynı türü döndürmesi gerektiğini konuşmuştuk. Örneğin şu kod çalışmaz:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

Burada `guess` değişkeninin aynı anda hem tamsayı hem string olması gerekirdi; ama Rust bir değişkenin tek türü olmasını ister. O hâlde `continue` ne döndürüyor? Liste 20-27'de bir kolda `u32`, diğer kolda `continue` olmasına nasıl izin verildi?

Tahmin ettiğiniz gibi `continue` ifadesinin türü `!`'dir. Rust, `guess` türünü hesaplarken bir kolun `u32`, diğer kolun `!` olduğuna bakar. `!` hiçbir değere sahip olamayacağı için Rust, `guess` türünün `u32` olduğuna karar verir.

Bunun resmi açıklaması şudur: `!` türündeki ifadeler başka herhangi bir türe zorlanabilir. Bu yüzden `match` kolunu `continue` ile bitirebiliriz; çünkü `continue` bir değer döndürmez, denetimi döngünün başına geri taşır. Dolayısıyla `Err` durumunda `guess`'e hiçbir zaman değer atamayız.

Never türü `panic!` makrosunda da işe yarar. `Option<T>` üzerinde çağırdığımız `unwrap` fonksiyonunu hatırlayın; ya içteki değeri üretir ya da panikler. Tanımı şöyledir:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

Burada, Liste 20-27'deki `match` ile aynı durum gerçekleşir: Rust `val` değişkeninin türünü `T`, `panic!` ifadesinin türünü ise `!` olarak görür. Böylece tüm `match` ifadesinin sonucu `T` olur. Bu kod geçerlidir; çünkü `panic!` bir değer üretmez, programı sonlandırır. `None` durumunda `unwrap` içinden değer döndürmeyiz.

`!` türüne sahip son bir ifade daha vardır: döngü.

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

Burada döngü hiç bitmediği için ifadenin değeri `!` olur. Ama `break` koysaydık bu doğru olmazdı; çünkü döngü `break` noktasında sonlanırdı.

### Dinamik Boyutlu Türler ve `Sized` Trait'i

Rust'ın türleri hakkında bazı ayrıntıları bilmesi gerekir; örneğin belli bir türden değer için ne kadar alan ayrılacağı gibi. Bu yüzden tür sisteminin ilk bakışta kafa karıştırıcı görünen bir köşesi vardır: _dinamik boyutlu türler_. Bunlara bazen _DST_ ya da _unsized type_ da denir. Bu türler, boyutunu ancak çalışma zamanında bilebileceğimiz değerlerle çalışabilmemizi sağlar.

Kitap boyunca kullandığımız `str` buna örnektir. Evet, `&str` değil, doğrudan `str` bir DST'dir. Kullanıcının girdiği metin gibi pek çok durumda, string'in uzunluğunu ancak çalışma zamanında bilebiliriz. Bu yüzden `str` türünde bir değişken oluşturamayız ve `str` türünde parametre alamayız. Aşağıdaki kod bu nedenle çalışmaz:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

Rust, herhangi bir türdeki değer için ne kadar bellek ayrılacağını bilmek zorundadır ve aynı türden tüm değerler aynı miktarda bellek kullanmalıdır. Eğer buna izin verseydi, bu iki `str` değeri de aynı kadar yer kaplamak zorunda olurdu. Oysa uzunlukları farklıdır: `s1` için 12 bayt, `s2` için 15 bayt gerekir. Bu yüzden dinamik boyutlu türü doğrudan tutan değişken oluşturamayız.

Peki ne yaparız? Cevabı zaten biliyorsunuz: `s1` ve `s2` türünü `str` yerine string dilimi (`&str`) yaparız. 4. bölümdeki ["String Dilimleri"][string-slices]<!-- ignore --> kısmından hatırlayın: dilim veri yapısı yalnızca başlangıç konumu ve uzunluğu saklar. Dolayısıyla `&T`, `T`'nin bulunduğu adresi tutan tek bir değerken; string dilimi iki değer içerir: `str`'nin adresi ve uzunluğu. Bu yüzden string dilimi değerinin boyutunu derleme zamanında biliriz; her zaman bir `usize`'nin iki katıdır. Başvurduğu string ne kadar uzun olursa olsun, string diliminin boyutu bellidir.

Genel olarak Rust'ta dinamik boyutlu türler böyle kullanılır: dinamik bilginin boyutunu tutan ek meta verilerle birlikte, bir işaretçinin arkasında tutulurlar. Kural şudur: dinamik boyutlu türlerin değerlerini her zaman bir tür işaretçinin arkasına koymalıyız.

`str`'yi farklı işaretçilerle birleştirebiliriz: örneğin `Box<str>` ya da `Rc<str>`. Aslında bunu başka bir DST ile zaten görmüştünüz: trait'ler. Her trait de dinamik boyutlu türdür ve adına bakarak ancak bir işaretçi arkasından başvurabiliriz. 18. bölümdeki ["Trait Nesneleriyle Ortak Davranışı Soyutlamak"][using-trait-objects-to-abstract-over-shared-behavior]<!-- ignore --> kısmında bunu `&dyn Trait`, `Box<dyn Trait>` ve benzer örneklerle görmüştük.

DST'lerle çalışabilmek için Rust, bir türün boyutunun derleme zamanında bilinip bilinmediğini anlamaya yarayan `Sized` trait'ini sunar. Boyutu derleme zamanında bilinen her tür için bu trait otomatik uygulanır. Ayrıca Rust, her jenerik fonksiyona örtük olarak `Sized` sınırı ekler. Yani şu türden bir jenerik fonksiyon:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

aslında şu şekilde ele alınır:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

Varsayılan olarak jenerik fonksiyonlar yalnızca boyutu derleme zamanında bilinen türlerde çalışır. Ama bu kısıtı gevşetmek için özel bir sözdizimi kullanabilirsiniz:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

`?Sized` tür sınırı, "`T`, `Sized` olabilir de olmayabilir de" anlamına gelir. Bu gösterim, jenerik türlerin derleme zamanında bilinen boyuta sahip olma varsayılanını geçersiz kılar. Bu anlamdaki `?Trait` sözdizimi yalnızca `Sized` için vardır; diğer trait'ler için yoktur.

Ayrıca `t` parametresinin türünü `T` yerine `&T` yaptığımıza dikkat edin. Çünkü tür `Sized` olmayabilir; bu yüzden onu bir tür işaretçinin arkasından kullanmamız gerekir. Burada referans seçtik.

Sırada fonksiyonlar ve kapanışlar var!

[encapsulation-that-hides-implementation-details]: ch18-01-nyp-nedir.html#uygulama-ayrıntılarını-gizleyen-kapsülleme
[string-slices]: ch04-03-dilim-turu.html#string-dilimleri-string-slices
[the-match-control-flow-construct]: ch06-02-match-kontrol-akisi.html#match-kontrol-akışı-yapısı
[using-trait-objects-to-abstract-over-shared-behavior]: ch18-02-trait-nesneleri.html#ortak-davranış-üzerinden-soyutlamak-için-trait-nesnelerini-kullanmak
[newtype]: ch20-02-gelismis-traitler.html#dış-traitleri-newtype-deseniyle-uygulamak
