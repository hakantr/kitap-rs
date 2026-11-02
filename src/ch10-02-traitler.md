<!-- Old headings. Do not remove or links may break. -->

<a id="traits-defining-shared-behavior"></a>
<a id="defining-shared-behavior-with-traits"></a>
<a id="traitler-ile-paylaşılan-davranışı-tanımlamak"></a>
<a id="traitler-ile-paylasilan-davranisi-tanimlamak"></a>

## Trait'lerle Ortak Davranış Tanımlama

Bir _trait_, belirli bir türün sahip olduğu ve diğer türlerle paylaşabileceği işlevselliği tanımlar. Ortak davranışı soyut bir biçimde tanımlamak için trait'leri kullanabiliriz. Jenerik bir türün, belirli bir davranışa sahip herhangi bir tür olabileceğini belirtmek için ise _trait sınırlarını (trait bounds)_ kullanabiliriz.

> Not: Trait'ler, aralarında bazı farklar bulunsa da diğer dillerde genellikle _arayüz (interface)_ olarak adlandırılan özelliğe benzer.

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-a-trait"></a>
<a id="bir-trait-tanımlamak"></a>
<a id="bir-trait-tanimlamak"></a>

### Bir Trait Tanımlama

Bir türün davranışı, o tür üzerinde çağırabileceğimiz metotlardan oluşur. Farklı türlerin hepsi üzerinde aynı metotları çağırabiliyorsak bu türler aynı davranışı paylaşıyor demektir. Trait tanımları, belirli bir amacı gerçekleştirmek için gereken davranış kümesini tanımlamak üzere metot imzalarını bir araya getirmenin yoludur.

Örneğin, farklı tür ve uzunluklarda metinler tutan birden fazla struct'ımız olduğunu varsayalım: Belirli bir yerden bildirilen bir haber metnini tutan `HaberMakalesi` struct'ı ve en fazla 280 karakterlik bir metnin yanı sıra bu metnin yeni bir gönderi mi, yeniden paylaşım mı yoksa başka bir gönderiye yanıt mı olduğunu gösteren üst verileri barındıran `SosyalGonderi` struct'ı.

Bir `HaberMakalesi` veya `SosyalGonderi` örneğinde saklanabilecek verilerin özetlerini görüntüleyebilen `toplayici` adında bir medya toplayıcı (*media aggregator*) kütüphane crate'i geliştirmek istiyoruz. Bunun için her türden bir özet almamız gerekiyor ve bu özeti bir örnek üzerinde `ozetle` metodunu çağırarak isteyeceğiz. Liste 10-12, bu davranışı ifade eden dışarıya açık (`pub`) bir `Ozet` trait'inin tanımını göstermektedir.

<Listing number="10-12" file-name="src/lib.rs" caption="`ozetle` metodunun sağladığı davranıştan oluşan bir `Ozet` trait'i">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

</Listing>

Burada `trait` anahtar kelimesini ve ardından trait'in adını —bu örnekte `Ozet`— yazarak bir trait bildiriyoruz. Ayrıca birkaç örnek sonra göreceğimiz üzere, bu crate'e bağımlı olan diğer crate'lerin de bu trait'ten yararlanabilmesi için trait'i `pub` olarak işaretliyoruz. Süslü parantezlerin içinde ise bu trait'i uygulayan türlerin sahip olacağı davranışları betimleyen metot imzalarını —bu örnekte `fn ozetle(&self) -> String`— bildiriyoruz.

Metot imzasından sonra süslü parantezler içinde bir gövde tanımlamak yerine noktalı virgül koyuyoruz. Bu trait'i uygulayan her tür, metodun gövdesi için kendi özel davranışını sağlamak zorundadır. Derleyici, `Ozet` trait'ine sahip her türün tam olarak bu imzayla tanımlanmış bir `ozetle` metoduna sahip olmasını zorunlu kılacaktır.

Bir trait, gövdesinde birden fazla metot barındırabilir: Metot imzaları her satıra bir tane gelecek şekilde alt alta yazılır ve her satır noktalı virgülle biter.

<!-- Old headings. Do not remove or links may break. -->

<a id="implementing-a-trait-on-a-type"></a>
<a id="bir-tür-üzerinde-trait-uygulamak-implementing"></a>
<a id="bir-tur-uzerinde-trait-uygulamak-implementing"></a>
<a id="bir-tür-üzerinde-trait-uygulamak"></a>
<a id="orphan-rule"></a>
<a id="yetim-kuralı"></a>
<a id="yetim-kurali"></a>

### Bir Tür Üzerinde Trait Uygulama

`Ozet` trait'inin metotları için istediğimiz imzaları tanımladığımıza göre, artık bu trait'i medya toplayıcımızdaki türler üzerinde uygulayabiliriz. Liste 10-13, `ozetle` metodunun dönüş değerini oluşturmak için manşeti, yazarı ve konumu kullanan `HaberMakalesi` struct'ı üzerindeki `Ozet` trait uygulamasını gösterir. `SosyalGonderi` struct'ı için ise gönderi içeriğinin zaten 280 karakterle sınırlandırıldığını varsayarak `ozetle` metodunu kullanıcı adı ve ardından gönderinin tüm metnini döndürecek şekilde tanımlıyoruz.

<Listing number="10-13" file-name="src/lib.rs" caption="`HaberMakalesi` ve `SosyalGonderi` türleri üzerinde `Ozet` trait'inin uygulanması">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

</Listing>

Bir tür üzerinde trait uygulamak, normal metotları tanımlamaya çok benzer. Aradaki fark, `impl` anahtar kelimesinden sonra uygulamak istediğimiz trait'in adını yazmamız, ardından `for` anahtar kelimesini kullanmamız ve sonrasında trait'i hangi tür için uygulayacağımızı belirtmemizdir. `impl` bloğunun içine trait tanımında yer alan metot imzalarını yazarız. Ancak her imzanın sonuna noktalı virgül koymak yerine süslü parantez açar ve metot gövdesini, trait metotlarının o türe özgü olarak sergilemesini istediğimiz davranışla doldururuz.

Kütüphanemiz `HaberMakalesi` ve `SosyalGonderi` üzerinde `Ozet` trait'ini uyguladığına göre, crate kullanıcıları bu türlerin örnekleri üzerinde trait metotlarını tıpkı normal metotları çağırır gibi çağırabilir. Tek fark, kullanıcının türlerin yanı sıra trait'i de kapsama dahil etmesinin gerekmesidir. Bir ikili crate'in `toplayici` kütüphane crate'imizi nasıl kullanabileceğine dair bir örnek aşağıda verilmiştir:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

Bu kod ekrana `1 yeni gönderi: horse_ebooks: elbette, muhtemelen zaten bildiğiniz gibi, insanlar` yazdırır.

> Not: Yukarıdaki `use toplayici::{Ozet, SosyalGonderi};` satırındaki `Ozet` kısmını silerseniz, `SosyalGonderi` türü kapsamda olduğu hâlde `gonderi.ozetle()` çağrısı derlenmez. Derleyici `` error[E0599]: no method named `ozetle` found `` hatasıyla birlikte `items from traits can only be used if the trait is in scope` (*trait'lerden gelen öğeler yalnızca trait kapsamdaysa kullanılabilir*) açıklamasını ve eklemeniz gereken `use` satırını önerir. Bir trait'in metotlarını çağırabilmek için trait'in de kapsamda olması kuralı, farklı crate'lerdeki iki trait'in aynı türe aynı adlı metotlar eklemesi durumunda hangi metodun kastedildiğinin belirsizleşmesini önler. Yalnızca standart kütüphanenin *prelude*'ünde yer alan `Clone`, `ToString` veya `Iterator` gibi trait'ler her kapsamda hazır olduğundan onlar için ek bir `use` satırına gerek yoktur.

`toplayici` crate'ine bağımlı olan diğer crate'ler de `Ozet` trait'ini kendi türleri üzerinde uygulamak üzere kapsama dahil edebilir. Burada dikkat edilmesi gereken önemli bir kısıtlama vardır: Bir trait'i bir tür üzerinde yalnızca trait'in kendisi, türün kendisi veya her ikisi birden kendi crate'imize yerel (*local*) ise uygulayabiliriz. Örneğin, `SosyalGonderi` türü `toplayici` crate'imize yerel olduğu için `toplayici` crate'imizin bir parçası olarak `SosyalGonderi` gibi özel bir tür üzerinde `Display` gibi standart kütüphane trait'lerini uygulayabiliriz. Benzer şekilde, `Ozet` trait'i `toplayici` crate'imize yerel olduğu için `toplayici` crate'i içinde `Vec<T>` üzerinde de `Ozet` trait'ini uygulayabiliriz.

Ancak harici trait'leri harici türler üzerinde uygulayamayız. Örneğin, `toplayici` crate'imiz içinde `Vec<T>` üzerinde `Display` trait'ini uygulayamayız; çünkü hem `Display` hem de `Vec<T>` standart kütüphanede tanımlanmıştır ve `toplayici` crate'imize yerel değildir. Bu kısıtlama, _tutarlılık (coherence)_ adı verilen özelliğin ve daha özelinde —ebeveyn tür mevcut olmadığı için bu adı alan— _yetim kuralının (orphan rule)_ bir parçasıdır. Bu kural, başkalarının kodunun sizin kodunuzu, sizin kodunuzun da başkalarının kodunu bozamamasını güvence altına alır. Bu kural olmasaydı iki farklı crate aynı tür için aynı trait'i uygulayabilir ve Rust hangi uygulamayı kullanacağını bilemezdi.

> Not: Yetim kuralının bağımlılık çakışmalarını nasıl önlediğini somut bir senaryoyla düşünelim: Projenize eklediğiniz iki harici bağımlılığın ikisi de standart kütüphanedeki `Vec<String>` türü için `Display` trait'ini kendince tanımlayabilseydi, bir `Vec<String>` değerini `{}` ile yazdırmak istediğinizde derleyici hangi crate'in uygulamasını çağıracağına karar veremezdi; üstelik bir bağımlılığın yeni sürümüne böyle bir `impl` eklendiğinde çalışan kodunuz bir anda derlenemez hâle gelirdi. Yine de `Vec<String>` gibi harici bir türe `Display` gibi harici bir trait kazandırmanız gereken durumlarda, bu türü `struct Sarmalayici(Vec<String>);` şeklinde tek alanlı yerel bir demet struct'ı içine alarak kendi yerel türünüzü oluşturabilirsiniz. Çalışma zamanında hiçbir ek bellek veya performans maliyeti getirmeyen bu tekniğe _newtype deseni_ denir; ayrıntılarını Bölüm 20'deki [“Dış Trait'leri Newtype Deseniyle Uygulamak”][newtype]<!-- ignore --> bölümünde inceleyeceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="default-implementations"></a>
<a id="using-default-implementations"></a>
<a id="varsayılan-uygulamaları-default-implementations-kullanmak"></a>
<a id="varsayilan-uygulamalari-default-implementations-kullanmak"></a>
<a id="varsayılan-uygulamaları-kullanmak"></a>

### Varsayılan Uygulamaları Kullanma

Bazen her tür üzerindeki tüm metotların tek tek yazılmasını zorunlu kılmak yerine, bir trait içindeki metotların bazıları veya tamamı için _varsayılan uygulamalar (default implementations)_ tanımlamak faydalıdır. Böylece trait'i belirli bir tür üzerinde uygularken her bir metodun varsayılan davranışını olduğu gibi koruyabilir ya da geçersiz kılabiliriz (*override*).

Liste 10-14'te, Liste 10-12'de yaptığımız gibi yalnızca metot imzasını tanımlamak yerine `Ozet` trait'inin `ozetle` metodu için varsayılan bir string belirliyoruz.

<Listing number="10-14" file-name="src/lib.rs" caption="`ozetle` metodunun varsayılan bir uygulamasıyla birlikte `Ozet` trait'ini tanımlama">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

</Listing>

`HaberMakalesi` örneklerini özetlerken bu varsayılan uygulamayı kullanmak için `impl Ozet for HaberMakalesi {}` şeklinde boş bir `impl` bloğu yazarız.

Artık `ozetle` metodunu doğrudan `HaberMakalesi` üzerinde tanımlamıyor olsak da, trait içinde varsayılan bir uygulama sunduk ve `HaberMakalesi` türünün `Ozet` trait'ini uyguladığını belirttik. Sonuç olarak, bir `HaberMakalesi` örneği üzerinde `ozetle` metodunu yine şu şekilde çağırabiliriz:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

Bu kod ekrana `Yeni makale mevcut! (Devamını oku...)` yazdırır.

Varsayılan bir uygulama oluşturmak, Liste 10-13'teki `SosyalGonderi` üzerindeki `Ozet` uygulamasında herhangi bir değişiklik yapmamızı gerektirmez. Bunun nedeni, varsayılan bir uygulamayı geçersiz kılma sözdiziminin, varsayılan uygulaması olmayan bir trait metodunu uygulama sözdizimiyle birebir aynı olmasıdır.

Varsayılan uygulamalar, aynı trait içindeki diğer metotları —o metotların varsayılan bir uygulaması olmasa bile— çağırabilir. Bu sayede bir trait pek çok kullanışlı işlevselliği hazır olarak sunarken, trait'i uygulayan türlerden bunun yalnızca küçük bir parçasını tanımlamalarını isteyebilir. Örneğin `Ozet` trait'ini, uygulanması zorunlu bir `yazari_ozetle` metoduna ve bu `yazari_ozetle` metodunu çağıran varsayılan uygulamalı bir `ozetle` metoduna sahip olacak şekilde tanımlayabiliriz:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

`Ozet` trait'inin bu sürümünü kullanmak için, trait'i bir tür üzerinde uygularken yalnızca `yazari_ozetle` metodunu tanımlamamız yeterlidir:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

`yazari_ozetle` metodunu tanımladıktan sonra `SosyalGonderi` struct'ının örnekleri üzerinde `ozetle` metodunu çağırabiliriz; `ozetle`'nin varsayılan uygulaması da bizim sağladığımız `yazari_ozetle` tanımını çağırır. `yazari_ozetle` metodunu uyguladığımız için `Ozet` trait'i, fazladan kod yazmamıza gerek bırakmadan `ozetle` metodunun davranışını bize kazandırmış olur. Kullanımı şöyle görünür:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

Bu kod ekrana `1 yeni gönderi: (@horse_ebooks yazarından daha fazlasını okuyun...)` yazdırır.

Bir metodun varsayılan uygulamasını, aynı metodun geçersiz kılınan uygulamasının içinden çağırmanın mümkün olmadığına dikkat edin.

<!-- Old headings. Do not remove or links may break. -->

<a id="traits-as-parameters"></a>
<a id="using-traits-as-parameters"></a>
<a id="traitleri-parametre-olarak-kullanmak"></a>
<a id="traitleri-parametre-olarak-kullanma"></a>

### Trait'leri Parametre Olarak Kullanma

Artık trait'lerin nasıl tanımlanacağını ve uygulanacağını bildiğinize göre, pek çok farklı türü kabul edebilen fonksiyonlar tanımlamak için trait'leri nasıl kullanacağımızı inceleyebiliriz. Liste 10-13'te `HaberMakalesi` ve `SosyalGonderi` türleri üzerinde uyguladığımız `Ozet` trait'ini kullanarak, `Ozet` trait'ini uygulayan bir türdeki `oge` parametresi üzerinde `ozetle` metodunu çağıran bir `bildir` fonksiyonu tanımlayacağız. Bunun için `impl Trait` sözdizimini şu şekilde kullanırız:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

`oge` parametresi için somut bir tür yazmak yerine `impl` anahtar kelimesini ve ilgili trait adını belirtiyoruz. Bu parametre, belirtilen trait'i uygulayan herhangi bir türü kabul eder. `bildir` fonksiyonunun gövdesinde, `oge` üzerinde `Ozet` trait'inden gelen `ozetle` gibi her türlü metodu çağırabiliriz. `bildir` fonksiyonunu çağırırken argüman olarak herhangi bir `HaberMakalesi` veya `SosyalGonderi` örneğine referans geçirebiliriz. Fonksiyonu `String` veya `i32` gibi başka bir türle çağıran kod ise bu türler `Ozet` trait'ini uygulamadığından derlenmeyecektir.

<!-- Old headings. Do not remove or links may break. -->

<a id="fixing-the-largest-function-with-trait-bounds"></a>
<a id="trait-bound-syntax"></a>
<a id="trait-sınırı-trait-bound-sözdizimi"></a>
<a id="trait-siniri-trait-bound-sozdizimi"></a>

#### Trait Sınırı Sözdizimi

`impl Trait` sözdizimi doğrudan ve yalın durumlarda gayet kullanışlıdır; ancak aslında _trait sınırı (trait bound)_ olarak bilinen daha uzun bir biçimin sözdizimsel şekeridir (*syntax sugar*) ve şu şekilde görünür:

```rust,ignore
pub fn bildir<T: Ozet>(oge: &T) {
    println!("Son dakika haberi! {}", oge.ozetle());
}
```

Bu uzun biçim önceki bölümdeki örnekle tamamen eşdeğerdir ancak daha ayrıntılıdır. Trait sınırlarını, açılı ayraçlar içindeki jenerik tür parametresi bildiriminde iki nokta üst üsteden (`:`) hemen sonra yazarız.

`impl Trait` sözdizimi pratiktir ve basit durumlarda daha özlü kod yazmamızı sağlar; daha kapsamlı olan trait sınırı sözdizimi ise diğer durumlarda daha karmaşık kısıtlamaları ifade edebilir. Örneğin, `Ozet` trait'ini uygulayan iki parametre almak isteyebiliriz. Bunu `impl Trait` sözdizimiyle yazmak şöyle görünür:

```rust,ignore
pub fn bildir(oge1: &impl Ozet, oge2: &impl Ozet) {
```

Fonksiyonun `oge1` ve `oge2` parametrelerinin —her iki tür de `Ozet` trait'ini uyguladığı sürece— birbirinden farklı somut türlere sahip olmasına izin vermesini istiyorsak `impl Trait` kullanmak uygundur. Ancak her iki parametrenin de birebir aynı somut türe sahip olmasını zorunlu kılmak istiyorsak şu şekilde bir trait sınırı kullanmalıyız:

```rust,ignore
pub fn bildir<T: Ozet>(oge1: &T, oge2: &T) {
```

`oge1` ve `oge2` parametrelerinin türü olarak belirtilen jenerik `T` türü, fonksiyonu öyle kısıtlar ki `oge1` ve `oge2` argümanları olarak geçirilen değerlerin somut türü aynı olmak zorundadır.

<!-- Old headings. Do not remove or links may break. -->

<a id="specifying-multiple-trait-bounds-with-the--syntax"></a>
<a id="multiple-trait-bounds-with-the--syntax"></a>

#### `+` Sözdizimi ile Birden Fazla Trait Sınırı

Birden fazla trait sınırı da belirtebiliriz. Diyelim ki `bildir` fonksiyonunun `oge` üzerinde `ozetle` metodunu çağırmasının yanı sıra `Display` biçimlendirmesini de kullanmasını istiyoruz: `bildir` tanımında `oge`'nin hem `Display` hem de `Ozet` trait'lerini uygulaması gerektiğini belirtiriz. Bunu `+` sözdizimiyle yapabiliriz:

```rust,ignore
pub fn bildir(oge: &(impl Ozet + Display)) {
```

`+` sözdizimi, jenerik türler üzerindeki trait sınırlarında da geçerlidir:

```rust,ignore
pub fn bildir<T: Ozet + Display>(oge: &T) {
```

Bu iki trait sınırı belirtildiğinde, `bildir` fonksiyonunun gövdesi hem `ozetle` metodunu çağırabilir hem de `oge` değerini `{}` ile biçimlendirebilir.

<!-- Old headings. Do not remove or links may break. -->

<a id="clearer-trait-bounds-with-where-clauses"></a>
<a id="where-cümlecikleriyle-daha-açık-trait-sınırları"></a>
<a id="where-cumlecikleriyle-daha-acik-trait-sinirlari"></a>

#### `where` Cümlecikleriyle Daha Açık Trait Sınırları

Çok fazla trait sınırı kullanmanın okunabilirlik açısından dezavantajları vardır. Her jenerik tür parametresinin kendi trait sınırları olduğundan, birden fazla jenerik tür parametresi alan fonksiyonlarda fonksiyon adı ile parametre listesi arasına çok fazla trait sınırı bilgisi girebilir ve bu da fonksiyon imzasının okunmasını güçleştirir. Bu nedenle Rust, trait sınırlarını fonksiyon imzasından sonra gelen bir `where` cümleciği içinde belirtmek için alternatif bir sözdizimi sunar. Dolayısıyla şunu yazmak yerine:

```rust,ignore
fn bir_fonksiyon<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

şu şekilde bir `where` cümleciği kullanabiliriz:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

Böylece fonksiyonun imzası kalabalıktan arınır: Çok sayıda trait sınırı içermeyen sıradan bir fonksiyonda olduğu gibi fonksiyon adı, parametre listesi ve dönüş türü birbirine yakın durur.

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-types-that-implement-traits"></a>
<a id="traitleri-uygulayan-implement-türler-döndürmek"></a>
<a id="traitleri-uygulayan-implement-turler-dondurmek"></a>
<a id="traitleri-uygulayan-türler-döndürmek"></a>

### Trait Uygulayan Türler Döndürme

Bir trait'i uygulayan herhangi bir türdeki değeri döndürmek için dönüş konumunda da `impl Trait` sözdizimini kullanabiliriz:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

Dönüş türü olarak `impl Ozet` kullanarak, `ozetlenebilir_dondur` fonksiyonunun somut türün adını açıkça belirtmeksizin `Ozet` trait'ini uygulayan bir tür döndürdüğünü ifade ederiz. Bu örnekte `ozetlenebilir_dondur` bir `SosyalGonderi` döndürür, ancak fonksiyonu çağıran kodun bu somut türü bilmesi gerekmez.

Dönüş türünü yalnızca uyguladığı trait üzerinden belirtebilme olanağı, özellikle Bölüm 13'te ele alacağımız kapanışlar ve iteratörler bağlamında son derece kullanışlıdır. Kapanışlar ve iteratörler, yalnızca derleyicinin bildiği isimsiz türler ya da açıkça yazılması çok uzun süren karmaşık türler üretir. `impl Trait` sözdizimi, çok uzun bir tür adını tek tek yazmak zorunda kalmadan bir fonksiyonun `Iterator` trait'ini uygulayan bir tür döndürdüğünü özlü biçimde belirtmenizi sağlar.

Bununla birlikte, dönüş konumunda `impl Trait`'i yalnızca **tek bir somut tür** döndürüyorsanız kullanabilirsiniz. Örneğin, dönüş türü `impl Ozet` olarak belirtilen ancak koşula göre bir `HaberMakalesi` ya da bir `SosyalGonderi` döndürmeye çalışan aşağıdaki kod derlenmez:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

`impl Trait` sözdiziminin derleyicide nasıl uygulandığına bağlı kısıtlamalar nedeniyle aynı fonksiyondan hem `HaberMakalesi` hem de `SosyalGonderi` döndürülmesine izin verilmez. Bu davranışa sahip bir fonksiyonun nasıl yazılacağını Bölüm 18'deki [“Ortak Davranışı Soyutlamak İçin Trait Nesnelerini Kullanma”][trait-objects]<!-- ignore --> bölümünde ele alacağız.

> Not: Dönüş konumundaki `-> impl Ozet` ifadesi çalışma zamanında dinamik bir tür seçimi yapmaz; derleyici fonksiyonun gövdesini inceleyerek döndürülen **tek bir somut türü** derleme zamanında saptar ve çağıran tarafta yığın (*stack*) üzerinde tam olarak o türün boyutu kadar yer ayırır (*statik gönderim / static dispatch*). Yukarıdaki kodda `HaberMakalesi` ile `SosyalGonderi` bellekte farklı boyutlara ve yerleşimlere sahip iki ayrı somut tür olduğundan, derleyici `error[E0308]: 'if' and 'else' have incompatible types` (`if` ve `else` kolları uyumsuz türlere sahip) hatası verir. Çalışma zamanındaki bir koşula göre aynı trait'i uygulayan farklı somut türler döndürmeniz gerektiğinde —derleyicinin hata çıktısındaki ipucunda da önerdiği üzere— değerleri öbekte (*heap*) tutup dönüş türünü `Box<dyn Ozet>` şeklinde bir _trait nesnesi_ olarak tanımlayabilirsiniz (`Box::new(HaberMakalesi { ... })` ve `Box::new(SosyalGonderi { ... })`). Bu yaklaşımın nasıl çalıştığını ve dinamik gönderim (*dynamic dispatch*) ödünleşimlerini Bölüm 18'de ayrıntılarıyla göreceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-trait-bounds-to-conditionally-implement-methods"></a>
<a id="metotları-koşullu-conditionally-uygulamak-için-trait-sınırlarını-kullanmak"></a>
<a id="metotlari-kosullu-conditionally-uygulamak-icin-trait-sinirlarini-kullanmak"></a>
<a id="metotları-koşullu-uygulamak-için-trait-sınırlarını-kullanmak"></a>
<a id="blanket-implementations"></a>
<a id="örtü-uygulamaları"></a>
<a id="ortu-uygulamalari"></a>
<a id="kapsayıcı-uygulamalar"></a>

### Trait Sınırlarını Kullanarak Metotları Koşullu Uygulama

Jenerik tür parametreleri kullanan bir `impl` bloğuyla birlikte bir trait sınırı belirterek, yalnızca belirtilen trait'leri uygulayan türler için koşullu olarak metotlar tanımlayabiliriz. Örneğin, Liste 10-15'teki `Cift<T>` türü, yeni bir `Cift<T>` örneği döndüren `new` ilişkili fonksiyonunu her zaman uygular (Bölüm 5'teki [“Metot Sözdizimi”][methods]<!-- ignore --> bölümünden hatırlayacağınız üzere `Self`, içinde bulunulan `impl` bloğunun türü —yani burada `Cift<T>`— için bir tür takma adıdır). Ancak bir sonraki `impl` bloğunda `Cift<T>`, `karsilastir_goster` metodunu yalnızca içindeki `T` türü hem karşılaştırma yapılmasını sağlayan `PartialOrd` trait'ini _hem de_ ekrana yazdırmayı sağlayan `Display` trait'ini uyguluyorsa barındırır.

<Listing number="10-15" file-name="src/lib.rs" caption="Trait sınırlarına bağlı olarak jenerik bir tür üzerinde metotları koşullu uygulama">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

</Listing>

Ayrıca, belirli bir trait'i uygulayan her tür için başka bir trait'i de koşullu olarak uygulayabiliriz. Trait sınırlarını karşılayan herhangi bir tür üzerinde gerçekleştirilen bu tür genel trait uygulamalarına _örtü uygulamaları_ (*blanket implementations*, _kapsayıcı uygulamalar_ olarak da adlandırılır) denir ve bunlar Rust standart kütüphanesinde yaygın biçimde kullanılır. Örneğin standart kütüphane, `Display` trait'ini uygulayan her tür üzerinde `ToString` trait'ini uygular. Standart kütüphanedeki ilgili `impl` bloğu şu koda benzer:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

Standart kütüphanede bu örtü uygulaması bulunduğundan, `Display` trait'ini uygulayan herhangi bir tür üzerinde `ToString` trait'inin tanımladığı `to_string` metodunu çağırabiliriz. Örneğin tam sayılar `Display` trait'ini uyguladığı için bir tam sayıyı karşılık gelen `String` değerine şu şekilde dönüştürebiliriz:

```rust
let s = 3.to_string();
```

Örtü uygulamaları, ilgili trait'in dokümantasyonunda “Implementors” (Uygulayıcılar) başlığı altında listelenir.

Trait'ler ve trait sınırları, kod tekrarını azaltmak için jenerik tür parametreleri kullanan kodlar yazmamızı sağlarken, aynı zamanda derleyiciye bu jenerik türün belirli bir davranışa sahip olmasını istediğimizi belirtmemize de olanak tanır. Derleyici daha sonra bu trait sınırı bilgisini kullanarak kodumuzla birlikte kullanılan tüm somut türlerin doğru davranışı sunduğunu denetler. Dinamik tür sistemine sahip dillerde, bir tür üzerinde o türde tanımlı olmayan bir metodu çağırsaydık hatayla ancak çalışma zamanında karşılaşırdık. Oysa Rust bu hataları derleme zamanına taşır; böylece kodumuz henüz çalışmaya bile başlamadan sorunları düzeltmemizi zorunlu kılar. Üstelik davranışı derleme zamanında zaten doğruladığımız için çalışma zamanında bir türün ilgili davranışı destekleyip desteklemediğini denetleyen ek kodlar yazmamıza da gerek kalmaz. Bu sayede jeneriklerin esnekliğinden ödün vermeden performansı artırmış oluruz.

[newtype]: ch20-02-gelismis-traitler.html#dış-traitleri-newtype-deseniyle-uygulamak
[trait-objects]: ch18-02-trait-nesneleri.html#ortak-davranış-üzerinden-soyutlamak-için-trait-nesnelerini-kullanmak
[methods]: ch05-03-metotlar.html#metot-sözdizimi
