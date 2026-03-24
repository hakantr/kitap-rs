<!-- Old headings. Do not remove or links may break. -->

<a id="traits-defining-shared-behavior"></a>

## Traitler ile Paylaşılan Davranışı Tanımlamak

Bir _trait_, belirli bir türün sahip olduğu ve diğer türlerle paylaşabileceği işlevselliği tanımlar. Paylaşılan davranışı soyut bir şekilde tanımlamak için traitleri kullanabiliriz. Jenerik bir türün belirli bir davranışa sahip herhangi bir tür olabileceğini belirtmek için _trait sınırlarını_ kullanabiliriz.

> Not: Traitler, bazı farklılıkları olsa da, diğer dillerde genellikle _arayüzler_ olarak adlandırılan bir özelliğe benzerdir.

### Bir Trait Tanımlamak

Bir türün davranışı, o tür üzerinde çağırabileceğimiz metotlardan oluşur. Tüm bu türlerde aynı metotları çağırabiliyorsak farklı türler aynı davranışı paylaşır. Trait tanımları, bazı amaçları gerçekleştirmek için gerekli bir dizi davranışı tanımlamak amacıyla metot imzalarını gruplamanın bir yoludur.

Örneğin, çeşitli türlerde ve miktarlarda metin tutan birden fazla struct'ımız (yapımız) olduğunu varsayalım: Belirli bir konumda dosyalanmış bir haber hikayesini tutan `HaberMakalesi` struct'ı ve yeni bir gönderi mi, yeniden paylaşım mı yoksa başka bir gönderiye yanıt mı olduğunu belirten meta verilerle birlikte en fazla 280 karaktere sahip olabilen `SosyalGonderi` struct'ı.

Bir `HaberMakalesi` veya `SosyalGonderi` örneğinde depolanabilecek verilerin özetlerini görüntüleyebilen `aggregator` (toplayıcı) adında bir medya toplayıcı kütüphane crate'i yapmak istiyoruz. Bunu yapmak için, her bir türden bir özete ihtiyacımız var ve bir örnek üzerinde `ozetle` metodu çağırarak bu özeti isteyeceğiz. Liste 10-12 bu davranışı ifade eden açık bir `Ozet` trait'inin tanımını gösterir.

<Listing number="10-12" file-name="src/lib.rs" caption="Bir `ozetle` metodunun sağladığı davranıştan oluşan bir `Ozet` trait'i">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

</Listing>

Burada, `trait` anahtar kelimesini ve ardından bu durumda `Ozet` olan trait adını kullanarak bir trait bildiriyoruz. Ayrıca, birkaç örnekte göreceğimiz gibi, bu crate'e bağlı olan cratelerin de bu trait'ten yararlanabilmesi için trait'i `pub` olarak bildiriyoruz. Süslü parantezlerin (curly brackets) içinde, bu trait'i uygulayan türlerin davranışlarını tanımlayan metot imzalarını bildiririz, bu durumda `fn ozetle(&self) -> String`'dir.

Metot imzasından sonra süslü parantezler içinde bir uygulama sağlamak yerine noktalı virgül kullanırız. Bu trait'i uygulayan her tür, metodun gövdesi için kendi özel davranışını sağlamalıdır. Derleyici, `Ozet` trait'ine sahip olan her türün tam olarak bu imzayla tanımlanmış `ozetle` metoduna sahip olmasını zorunlu kılacaktır.

Bir trait gövdesinde birden fazla metoda sahip olabilir: Metot imzaları her satıra bir tane olacak şekilde listelenir ve her satır noktalı virgülle biter.

### Bir Tür Üzerinde Trait Uygulamak (Implementing)

`Ozet` trait'inin metotlarının istenen imzalarını tanımladığımıza göre, artık onu medya toplayıcımızdaki türler üzerinde uygulayabiliriz. Liste 10-13, `ozetle` metodunun dönüş değerini oluşturmak için manşeti, yazarı ve konumu kullanan `HaberMakalesi` struct'ı üzerinde `Ozet` trait'inin bir uygulamasını gösterir. `SosyalGonderi` struct'ı için, gönderi içeriğinin halihazırda 280 karakterle sınırlı olduğunu varsayarak `ozetle` metodunu kullanıcı adı ve ardından gönderinin tüm metni olarak tanımlıyoruz.

<Listing number="10-13" file-name="src/lib.rs" caption="`HaberMakalesi` ve `SosyalGonderi` türleri üzerinde `Ozet` trait'inin uygulanması">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

</Listing>

Bir tür üzerinde bir trait uygulamak, normal metotları uygulamaya benzer. Aradaki fark, `impl` kelimesinden sonra uygulamak istediğimiz trait adını koymamız, ardından `for` anahtar kelimesini kullanmamız ve sonra trait'i uygulamak istediğimiz türün adını belirtmemizdir. `impl` bloğunun içine, trait tanımında belirtilen metot imzalarını koyarız. Her imzanın sonuna noktalı virgül eklemek yerine süslü parantezler kullanırız ve trait'in metotlarının o belirli tür için sahip olmasını istediğimiz belirli davranışla metot gövdesini doldururuz.

Kütüphane `HaberMakalesi` ve `SosyalGonderi` üzerinde `Ozet` trait'ini uyguladığına göre, crate kullanıcıları normal metotları çağırdığımız şekilde `HaberMakalesi` ve `SosyalGonderi` örneklerinde trait metotlarını çağırabilir. Tek fark, kullanıcının türlerin yanı sıra trait'i de kapsama dahil etmesi gerektiğidir. İşte bir ikili crate'in `aggregator` kütüphane crate'imizi nasıl kullanabileceğine dair bir örnek:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

Bu kod `1 yeni gönderi: horse_ebooks: elbette, muhtemelen zaten bildiğiniz gibi, insanlar` yazdırır.

`aggregator` crate'ine bağlı olan diğer crateler de `Ozet` trait'ini kendi türleri üzerinde uygulamak için `Ozet` trait'ini kapsama dahil edebilirler. Dikkat edilmesi gereken bir kısıtlama, bir tür üzerinde bir trait'i ancak trait veya türden biri ya da her ikisi bizim crate'imiz için yerel ise uygulayabilmemizdir. Örneğin, `SosyalGonderi` türü bizim `aggregator` crate'imiz için yerel olduğundan, `aggregator` crate işlevselliğimizin bir parçası olarak `SosyalGonderi` gibi özel bir tür üzerinde `Display` (Göster) gibi standart kütüphane traitlerini uygulayabiliriz. Ayrıca, `Ozet` trait'i `aggregator` crate'imiz için yerel olduğundan, `aggregator` crate'imizde `Vec<T>` üzerinde `Ozet` uygulayabiliriz.

Ancak harici traitleri harici türler üzerinde uygulayamayız. Örneğin, `aggregator` crate'imiz içerisinde `Vec<T>` üzerinde `Display` trait'ini uygulayamayız, çünkü hem `Display` hem de `Vec<T>` standart kütüphanede tanımlanmıştır ve `aggregator` crate'imiz için yerel değildir. Bu kısıtlama, _tutarlılık_ adı verilen bir özelliğin ve daha spesifik olarak ebeveyn türün mevcut olmaması nedeniyle bu adı alan _yetim kuralının_ bir parçasıdır. Bu kural, başkalarının kodunun sizin kodunuzu, sizin kodunuzun da başkalarının kodunu bozamamasını sağlar. Kural olmasaydı, iki crate aynı tür için aynı trait'i uygulayabilirdi ve Rust hangi uygulamayı kullanacağını bilemezdi.

<!-- Old headings. Do not remove or links may break. -->

<a id="default-implementations"></a>

### Varsayılan Uygulamaları (Default Implementations) Kullanmak

Bazen her türdeki tüm metotlar için uygulama gerektirmek yerine bir trait'teki metotların bazılarında veya tümünde varsayılan bir davranışa sahip olmak yararlıdır. Böylece, trait'i belirli bir tür üzerinde uygularken, her metodun varsayılan davranışını koruyabilir veya geçersiz kılabiliriz.

Liste 10-14'te, Liste 10-12'de yaptığımız gibi yalnızca metot imzasını tanımlamak yerine `Ozet` trait'inin `ozetle` metodu için varsayılan bir string (dizgi) belirliyoruz.

<Listing number="10-14" file-name="src/lib.rs" caption="`ozetle` metodunun varsayılan bir uygulamasıyla birlikte `Ozet` trait'ini tanımlamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

</Listing>

`HaberMakalesi` örneklerini özetlerken varsayılan bir uygulama kullanmak için `impl Ozet for HaberMakalesi {}` şeklinde boş bir `impl` bloğu belirtiriz.

Artık doğrudan `HaberMakalesi` üzerinde `ozetle` metodunu tanımlamıyor olsak da, varsayılan bir uygulama sağladık ve `HaberMakalesi`'nin `Ozet` trait'ini uyguladığını belirttik. Sonuç olarak, `HaberMakalesi` örneğinde `ozetle` metodunu hala şu şekilde çağırabiliriz:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

Bu kod `Yeni makale mevcut! (Devamını oku...)` yazdırır.

Varsayılan bir uygulama oluşturmak, Liste 10-13'teki `SosyalGonderi` üzerindeki `Ozet` uygulamasında herhangi bir şeyi değiştirmemizi gerektirmez. Bunun nedeni, varsayılan bir uygulamayı geçersiz kılma sözdiziminin, varsayılan bir uygulaması olmayan bir trait metodunu uygulama sözdizimiyle aynı olmasıdır.

Varsayılan uygulamalar, diğer metotların varsayılan bir uygulaması olmasa bile aynı trait'teki diğer metotları çağırabilir. Bu şekilde, bir trait çok fazla faydalı işlevsellik sağlayabilir ve uygulayıcılardan sadece küçük bir kısmını belirtmelerini isteyebilir. Örneğin, uygulamasının gerekli olduğu `yazari_ozetle` metodu olan bir `Ozet` trait'i tanımlayabiliriz ve ardından `yazari_ozetle` metodunu çağıran varsayılan bir uygulamaya sahip `ozetle` metodu tanımlayabiliriz:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

Bu `Ozet` sürümünü kullanmak için, trait'i bir tür üzerinde uygularken sadece `yazari_ozetle` metodunu tanımlamamız gerekir:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

`yazari_ozetle`'yi tanımladıktan sonra, `SosyalGonderi` struct'ının örnekleri üzerinde `ozetle`'yi çağırabiliriz ve `ozetle`'nin varsayılan uygulaması, sağladığımız `yazari_ozetle` tanımını çağıracaktır. `yazari_ozetle` uygulamasını gerçekleştirdiğimiz için `Ozet` trait'i bize daha fazla kod yazmamıza gerek kalmadan `ozetle` metodunun davranışını verdi. İşte şöyle görünür:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

Bu kod `1 yeni gönderi: (@horse_ebooks yazarından daha fazlasını okuyun...)` yazdırır.

Aynı metodun geçersiz kılınan bir uygulamasından varsayılan uygulamayı çağırmanın mümkün olmadığını unutmayın.

<!-- Old headings. Do not remove or links may break. -->

<a id="traits-as-parameters"></a>

### Traitleri Parametre Olarak Kullanmak

Artık traitlerin nasıl tanımlanacağını ve uygulanacağını bildiğinize göre, birçok farklı türü kabul eden fonksiyonları tanımlamak için traitleri nasıl kullanacağınızı keşfedebiliriz. `Ozet` trait'ini uygulayan bir türde olan `oge` parametresi üzerinde `ozetle` metodunu çağıran `bildir` fonksiyonunu tanımlamak için Liste 10-13'te `HaberMakalesi` ve `SosyalGonderi` türleri üzerinde uyguladığımız `Ozet` trait'ini kullanacağız. Bunu yapmak için, `impl Trait` sözdizimini şu şekilde kullanırız:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

`oge` parametresi için somut bir tür (concrete type) yerine `impl` anahtar kelimesini ve trait adını belirtiyoruz. Bu parametre, belirtilen trait'i uygulayan herhangi bir türü kabul eder. `bildir`'in gövdesinde, `oge` üzerinde `Ozet` trait'inden gelen `ozetle` gibi metotları çağırabiliriz. `bildir` çağırabiliriz ve herhangi bir `HaberMakalesi` veya `SosyalGonderi` örneği geçebiliriz. Fonksiyonu `String` veya `i32` gibi herhangi bir türle çağıran kod derlenmeyecektir çünkü bu türler `Ozet` trait'ini uygulamazlar.

<!-- Old headings. Do not remove or links may break. -->

<a id="fixing-the-largest-function-with-trait-bounds"></a>

#### Trait Sınırı (Trait Bound) Sözdizimi

`impl Trait` sözdizimi basit durumlar için işe yarar ancak aslında _trait sınırı_ (trait bound) olarak bilinen daha uzun bir formun sözdizimsel şekeri gibidir; şu şekilde görünür:

```rust,ignore
pub fn bildir<T: Ozet>(oge: &T) {
    println!("Son dakika haberi! {}", oge.ozetle());
}
```

Bu uzun form, önceki bölümdeki örneğe eşdeğerdir ancak daha uzundur. Jenerik tür parametresinin bildirimi ile birlikte iki nokta üst üsteden sonra ve açılı parantezlerin içine trait sınırlarını yerleştiririz.

`impl Trait` sözdizimi kullanışlıdır ve basit durumlarda kodu daha özlü hale getirirken, daha tam olan (fuller) trait sınırı sözdizimi diğer durumlarda daha fazla karmaşıklığı ifade edebilir. Örneğin, `Ozet` uygulayan iki parametremiz olabilir. Bunu `impl Trait` sözdizimi ile yapmak şuna benzer:

```rust,ignore
pub fn bildir(oge1: &impl Ozet, oge2: &impl Ozet) {
```

Eğer bu fonksiyonun `oge1` ve `oge2`'nin farklı türlere sahip olmasına izin vermesini istiyorsak (her iki tür de `Ozet` uyguladığı sürece) `impl Trait` kullanmak uygundur. Ancak her iki parametreyi de aynı türe sahip olmaya zorlamak istersek, şu şekilde bir trait sınırı kullanmalıyız:

```rust,ignore
pub fn bildir<T: Ozet>(oge1: &T, oge2: &T) {
```

`oge1` ve `oge2` parametrelerinin türü olarak belirtilen jenerik tür `T`, fonksiyonu öyle bir kısıtlar ki, `oge1` ve `oge2` için argüman olarak iletilen değerin somut türü (concrete type) aynı olmalıdır.

<!-- Old headings. Do not remove or links may break. -->

<a id="specifying-multiple-trait-bounds-with-the--syntax"></a>

#### `+` Sözdizimi ile Birden Fazla Trait Sınırı

Birden fazla trait sınırı da belirtebiliriz. Diyelim ki `bildir`'in `oge` üzerinde `ozetle` ile birlikte ekran biçimlendirmesini de kullanmasını istiyoruz: `bildir` tanımında `oge`'nin hem `Display` hem de `Ozet` trait'lerini uygulaması gerektiğini belirtiyoruz. Bunu `+` sözdizimini kullanarak yapabiliriz:

```rust,ignore
pub fn bildir(oge: &(impl Ozet + Display)) {
```

`+` sözdizimi jenerik türler üzerindeki trait sınırlarıyla da geçerlidir:

```rust,ignore
pub fn bildir<T: Ozet + Display>(oge: &T) {
```

Belirtilen iki trait sınırı ile `bildir`'in gövdesi `ozetle`'yi çağırabilir ve `oge`'yi formatlamak için `{}` kullanabilir.

#### `where` Cümlecikleriyle Daha Açık Trait Sınırları

Çok fazla trait sınırı kullanmanın dezavantajları vardır. Her jeneriğin kendi trait sınırları vardır, bu nedenle birden fazla jenerik tür parametresine sahip fonksiyonlar, fonksiyonun adı ve parametre listesi arasında çok fazla trait sınırı bilgisi içerebilir, bu da fonksiyon imzasının okunmasını zorlaştırır. Bu nedenle, Rust fonksiyon imzasından sonra bir `where` (nerede/şartıyla) cümleciği içinde trait sınırlarını belirtmek için alternatif bir sözdizimine sahiptir. Yani bunu yazmak yerine:

```rust,ignore
fn bazi_fonksiyonlar<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

Şu şekilde bir `where` cümleciği kullanabiliriz:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

Bu fonksiyonun imzası daha az karmaşıktır: Fonksiyon adı, parametre listesi ve dönüş türü birbirine yakındır, tıpkı çok fazla trait sınırı olmayan bir fonksiyona benzer şekilde.

### Traitleri Uygulayan (Implement) Türler Döndürmek

`impl Trait` sözdizimini, burada gösterildiği gibi bir trait uygulayan bazı türlerden bir değer döndürmek için dönüş pozisyonunda da kullanabiliriz:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

Dönüş türü olarak `impl Ozet` kullanarak, `ozetlenebilir_dondur` fonksiyonunun somut türün adını vermeden `Ozet` trait'ini uygulayan bir tür döndürdüğünü belirtiyoruz. Bu durumda, `ozetlenebilir_dondur` bir `SosyalGonderi` döndürür ancak bu fonksiyonu çağıran kodun bunu bilmesi gerekmez.

Dönüş türünü sadece uyguladığı trait ile belirtebilme yeteneği, özellikle Bölüm 13'te ele aldığımız kapanışlar ve yineleyiciler bağlamında faydalıdır. Kapanışlar ve yineleyiciler, yalnızca derleyicinin bildiği türler veya belirtilmesi çok uzun olan türler yaratır. `impl Trait` sözdizimi, bir fonksiyonun çok uzun bir tür yazmaya gerek kalmadan `Iterator` trait'ini uygulayan bir tür döndürdüğünü özlü bir şekilde belirtmenizi sağlar.

Ancak, `impl Trait`'i yalnızca tek bir tür döndürüyorsanız kullanabilirsiniz. Örneğin, dönüş türü `impl Ozet` olarak belirtilen ve bir `HaberMakalesi` ya da bir `SosyalGonderi` döndüren bu kod işe yaramayacaktır:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

Derleyicide `impl Trait` sözdiziminin nasıl uygulandığına ilişkin kısıtlamalar nedeniyle `HaberMakalesi` veya `SosyalGonderi` döndürülmesine izin verilmez. Bu davranışa sahip bir fonksiyonun nasıl yazılacağını Bölüm 18'in [“Paylaşılan Davranış Üzerinden Soyutlama Yapmak İçin Trait Nesneleri Kullanmak”][trait-objects]<!-- ignore --> bölümünde ele alacağız.

### Metotları Koşullu (Conditionally) Uygulamak İçin Trait Sınırlarını Kullanmak

Jenerik tür parametreleri kullanan bir `impl` bloğu ile birlikte bir trait sınırı kullanarak, belirtilen traitleri uygulayan türler için koşullu olarak metotlar uygulayabiliriz. Örneğin, Liste 10-15'teki `Cift<T>` türü her zaman yeni bir `Cift<T>` örneği döndüren `new` fonksiyonunu uygular (Bölüm 5'in [“Metot Sözdizimi”][methods]<!-- ignore --> kısmından hatırlayın, `Self`, bu durumda `Cift<T>` olan `impl` bloğunun türü için bir tür takma adıdır). Fakat bir sonraki `impl` bloğunda, `Cift<T>` yalnızca kendi içindeki `T` türü karşılaştırmayı sağlayan `PartialOrd` trait'ini _ve_ yazdırmayı sağlayan `Display` trait'ini uyguluyorsa `karsilastir_goster` metodunu uygular.

<Listing number="10-15" file-name="src/lib.rs" caption="Trait sınırlarına bağlı olarak jenerik bir tür üzerinde koşullu olarak metotları uygulamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

</Listing>

Ayrıca, başka bir trait'i uygulayan herhangi bir tür için koşullu olarak bir trait de uygulayabiliriz. Trait sınırlarını karşılayan herhangi bir tür üzerindeki trait uygulamalarına _kapsayıcı uygulamalar_ denir ve Rust standart kütüphanesinde yaygın olarak kullanılırlar. Örneğin, standart kütüphane `Display` trait'ini uygulayan her tür için `ToString` trait'ini uygular. Standart kütüphanedeki `impl` bloğu bu koda benzemektedir:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

Standart kütüphanede bu kapsayıcı uygulama (blanket implementation) bulunduğundan dolayı, `Display` trait'ini uygulayan herhangi bir tür üzerinde `ToString` trait'i tarafından tanımlanan `to_string` metodunu çağırabiliriz. Örneğin tamsayılar `Display` uyguladığından tamsayıları bu şekilde karşılık gelen `String` değerlerine dönüştürebiliriz:

```rust
let s = 3.to_string();
```

Kapsayıcı uygulamalar, trait'in dokümantasyonunda "Uygulayıcılar" (Implementors) bölümünde yer alır.

Traitler ve trait sınırları, kod tekrarını azaltmak için jenerik tür parametrelerini kullanan kod yazmamızı sağlarken aynı zamanda derleyiciye jenerik türün belirli bir davranışa sahip olmasını istediğimizi belirtmemize de olanak tanır. Derleyici daha sonra trait sınırı bilgisini, kodumuzla birlikte kullanılan tüm somut türlerin doğru davranışı sağladığını kontrol etmek için kullanabilir. Dinamik olarak yazılmış dillerde, bir metodu tanımlamayan bir tür üzerinde metodu çağırsaydık çalışma zamanında bir hata alırdık. Ancak Rust bu hataları derleme zamanına taşır, böylece kodumuz çalışmadan önce sorunları düzeltmeye zorlanırız. Ek olarak, çalışma zamanında davranış için kontrol yapan bir kod yazmamıza gerek kalmaz, çünkü zaten derleme zamanında kontrol etmişizdir. Bunu yapmak jeneriklerin esnekliğinden vazgeçmek zorunda kalmadan performansı artırır.

[trait-objects]: ch18-02-trait-nesneleri.html#ortak-davranış-üzerinden-soyutlamak-için-trait-nesnelerini-kullanmak
[methods]: ch05-03-metotlar.html#metotlar-methods
