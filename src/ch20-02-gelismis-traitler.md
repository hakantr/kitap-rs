## Gelişmiş Trait'ler

Trait'leri ilk olarak 10. bölümdeki ["Trait'lerle Ortak Davranış Tanımlamak"][traits]<!-- ignore --> kısmında ele almıştık; ancak daha ileri ayrıntılara girmemiştik. Artık Rust hakkında daha fazla şey bildiğinize göre, biraz daha derine inebiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="specifying-placeholder-types-in-trait-definitions-with-associated-types"></a>
<a id="associated-types"></a>

### İlişkili Türlerle Trait Tanımlamak

_İlişkili türler_ (associated types), bir trait içindeki yer tutucu türü trait ile bağlar; böylece trait metodlarının imzalarında bu yer tutucu türler kullanılabilir. Trait'i uygulayan taraf, o uygulama için yer tutucu yerine kullanılacak somut türü belirtir. Böylece, bir trait'in bazı türleri kullanacağını söyleyebilir ama trait uygulanana kadar bu türlerin tam olarak ne olacağını bilmek zorunda kalmayız.

Bu bölümde anlattığımız gelişmiş özelliklerin çoğunu "nadiren gerekli" diye tanımladık. İlişkili türler ise ortada bir yerdedir: kitabın geri kalanındaki ana özelliklerden daha az kullanılırlar ama bu bölümdeki bazı diğer özelliklerden daha yaygındırlar.

İlişkili tür kullanan trait'e örnek olarak standart kütüphanedeki `Iterator` trait'ini verebiliriz. İlişkili türün adı `Item`'dır ve `Iterator` trait'ini uygulayan türün yinelediği değerlerin türünü temsil eder. `Iterator` trait'inin tanımı Liste 20-13'te gösteriliyor.

<Listing number="20-13" caption="`Item` adlı ilişkili tür içeren `Iterator` trait'inin tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-13/src/lib.rs}}
```

</Listing>

Buradaki `Item` bir yer tutucudur. `next` metodunun tanımı da `Option<Self::Item>` döndüreceğini söyler. `Iterator` trait'ini uygulayan türler `Item` için somut türü belirler; `next` de bu somut türden bir değer içeren `Option` döndürür.

İlişkili türler ilk bakışta jeneriklere benziyormuş gibi görünebilir. Çünkü jenerikler de bir fonksiyonun hangi türlerle çalışacağını belirtmeden tanımlanmasını sağlar. Aradaki farkı görmek için `Sayac` adlı bir tür üzerinde `Iterator` uygulamasına bakalım; burada `Item` türü `u32` olarak belirtilmiştir:

<Listing file-name="src/lib.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-22-iterator-on-counter/src/lib.rs:ch19}}
```

</Listing>

Bu sözdizimi jeneriklere benzer görünüyor. O hâlde neden `Iterator` trait'ini doğrudan jeneriklerle tanımlamayalım? Liste 20-14'te bunun varsayımsal bir sürümünü görebilirsiniz.

<Listing number="20-14" caption="`Iterator` trait'inin jeneriklerle yazılmış varsayımsal tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-14/src/lib.rs}}
```

</Listing>

Fark şu: jenerik kullandığımızda, Liste 20-14'te olduğu gibi, her uygulamada türleri ayrıca açıklamamız gerekir. Çünkü `Sayac` için istersek `Iterator<String>`, istersek başka bir `Iterator<T>` uygulaması daha yazabiliriz. Yani trait jenerik parametre alıyorsa, aynı trait aynı tür için birden çok kez uygulanabilir; her seferinde jenerik parametrelerin somut türleri değişebilir. Böyle olsaydı, `Sayac` üzerinde `next` çağırırken hangi `Iterator` uygulamasını kastettiğimizi ayrıca belirtmemiz gerekirdi.

İlişkili türlerde buna gerek kalmaz; çünkü aynı trait'i aynı tür için birden fazla kez uygulayamayız. Liste 20-13'te ilişkili tür kullanan tanımda `Item` türünü yalnızca bir kez seçebiliriz; çünkü `Sayac` için ancak tek bir `impl Iterator` yazılabilir. Böylece `Sayac` üzerinde `next` çağırdığımız her yerde bunun `u32` değerleri döndüren yineleyici olduğunu tekrar tekrar belirtmemiz gerekmez.

İlişkili türler, trait'in sözleşmesinin de bir parçasıdır: trait'i uygulayanlar ilişkili tür yer tutucusuna karşılık gelecek bir tür vermek zorundadır. Bu yüzden ilişkili türlere genellikle kullanımını anlatan anlamlı adlar verilir; API belgelerinde bunları ayrıca açıklamak iyi bir pratiktir.

<!-- Old headings. Do not remove or links may break. -->

<a id="default-generic-type-parameters-and-operator-overloading"></a>

### Varsayılan Jenerik Parametreler ve Operatör Aşırı Yükleme

Jenerik tür parametreleri kullandığımızda, o jenerik tür için varsayılan bir somut tür de belirtebiliriz. Böylece varsayılan tür iş görüyorsa trait'i uygulayanların ayrıca tür belirtmesine gerek kalmaz. Varsayılan türü `<YerTutucuTur=SomutTur>` sözdizimiyle tanımlarız.

Bu tekniğin faydalı olduğu güzel örneklerden biri _operatör aşırı yükleme_ dir; yani belirli durumlarda bir operatörün (`+` gibi) davranışını özelleştirmek.

Rust kendi operatörlerinizi tanımlamanıza ya da rastgele operatörleri aşırı yüklemenize izin vermez. Ancak `std::ops` içindeki işlemleri ve bunlara karşılık gelen trait'leri uygulayarak özelleştirebilirsiniz. Örneğin Liste 20-15'te iki `Nokta` örneğini toplamak için `+` operatörünü aşırı yüklüyoruz. Bunu `Nokta` struct'ı üzerinde `Add` trait'ini uygulayarak yapıyoruz.

<Listing number="20-15" file-name="src/main.rs" caption="`Nokta` örnekleri için `+` operatörünü aşırı yüklemek üzere `Add` trait'ini uygulamak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-15/src/main.rs}}
```

</Listing>

`add` metodu iki `Nokta` örneğinin `x` değerlerini ve `y` değerlerini toplayıp yeni bir `Nokta` üretir. `Add` trait'inde `Output` adlı ilişkili tür vardır; `add` metodunun döndüreceği türü bu belirler.

Bu örnekteki varsayılan jenerik tür, `Add` trait'inin tanımı içindedir:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

Bu kod size tanıdık gelmiş olmalı: tek metodlu, ilişkili tür içeren bir trait. Yeni kısım `Rhs=Self` ifadesidir. Buna _varsayılan tür parametresi_ denir. `Rhs` jenerik tür parametresi ("sağ taraf" anlamındaki right-hand side'ın kısaltmasıdır), `add` metodundaki `rhs` parametresinin türünü belirler. `Add` trait'ini uygularken `Rhs` için somut tür vermezsek `Self` kullanılır; yani `Add`'i uyguladığımız türün kendisi.

`Nokta` için `Add` uygularken `Rhs` için varsayılanı kullandık; çünkü iki `Nokta` toplamak istiyorduk. Şimdi `Rhs` türünü özelleştirdiğimiz bir örneğe bakalım.

Elimizde farklı birimlerde değer tutan `Milimetreler` ve `Metreler` yapıları olduğunu düşünelim. Var olan bir türü başka bir struct ile ince biçimde sarmalama yaklaşımına _newtype deseni_ denir; bunu birazdan daha ayrıntılı ele alacağız. Diyelim ki milimetre cinsinden değerlerle metre cinsinden değerleri toplamak istiyoruz ve `Add` uygulaması dönüşümü doğru yapsın istiyoruz. Bunun için `Milimetreler` üzerinde, `Rhs` türü `Metreler` olacak şekilde `Add` uygulayabiliriz; Liste 20-16 bunu gösteriyor.

<Listing number="20-16" file-name="src/lib.rs" caption="`Milimetreler` üzerinde `Add` uygulayarak `Milimetreler` ile `Metreler` toplamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-16/src/lib.rs}}
```

</Listing>

`Milimetreler` ile `Metreler` toplayabilmek için `impl Add<Metreler>` yazarız; böylece `Rhs` parametresi `Self` yerine `Metreler` olur.

Varsayılan tür parametrelerini başlıca iki amaçla kullanırsınız:

1. Var olan kodu bozmadan bir türü ya da trait'i genişletmek
2. Çoğu kullanıcının ihtiyaç duymayacağı belirli durumlarda özelleştirme imkânı sunmak

Standart kütüphanedeki `Add` trait'i ikinci amaca güzel örnektir: çoğu zaman aynı türden iki değeri toplarsınız; ama `Add` gerektiğinde bundan fazlasını da yapabilmenizi sağlar. `Add` tanımındaki varsayılan tür parametresi sayesinde, çoğu durumda bu ek parametreyi yazmanız gerekmez. Yani biraz fazladan tekrarlayan koddan kurtulmuş olursunuz.

İlk amaç da buna benzer, ama ters yönde işler: var olan bir trait'e yeni tür parametresi eklemek isterseniz buna varsayılan değer vererek trait'in işlevini genişletebilir, mevcut uygulama kodlarını bozmamış olursunuz.

<!-- Old headings. Do not remove or links may break. -->

<a id="fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name"></a>
<a id="disambiguating-between-methods-with-the-same-name"></a>

### Aynı Adlı Metodlar Arasında Ayrım Yapmak

Rust'ta bir trait'in, başka bir trait'teki metodla aynı adda metod tanımlamasını engelleyen bir kural yoktur. Hatta aynı tür üzerinde bu iki trait'in ikisini birden uygulayabilirsiniz. Ayrıca türün kendisi üzerinde, trait metodlarıyla aynı ada sahip bir metod da tanımlayabilirsiniz.

Aynı isimli metodlar çağrılırken Rust'a hangisini kullanmak istediğinizi söylemeniz gerekir. Liste 20-17'de bunun örneğini görüyoruz: `Pilot` ve `Buyucu` adlı iki trait tanımlıyoruz; ikisinde de `fly` adlı metod var. Sonra her iki trait'i de zaten `fly` metodu olan `Insan` türü üzerinde uyguluyoruz. Her `fly` farklı bir şey yapıyor.

<Listing number="20-17" file-name="src/main.rs" caption="Hem `Pilot` hem `Buyucu` içinde `fly` metodu tanımlanması, bunların `Insan` üzerinde uygulanması ve `Insan` üzerinde ayrıca doğrudan `fly` metodunun bulunması">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-17/src/main.rs:here}}
```

</Listing>

Bir `Insan` örneğinde `fly` çağırdığımızda, derleyici varsayılan olarak türün kendisi üzerinde tanımlı olan metodu seçer. Bunu Liste 20-18'de görebilirsiniz.

<Listing number="20-18" file-name="src/main.rs" caption="Bir `Insan` örneğinde `fly` çağırmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-18/src/main.rs:here}}
```

</Listing>

Bu kod çalıştırıldığında `*waving arms furiously*` yazdırır; yani Rust, doğrudan `Insan` üzerinde uygulanmış `fly` metodunu çağırır.

`Pilot` veya `Buyucu` trait'lerindeki `fly` metodlarını çağırmak için daha açık bir sözdizimi kullanmamız gerekir. Liste 20-19 bunu gösteriyor.

<Listing number="20-19" file-name="src/main.rs" caption="Hangi trait'teki `fly` metodunu çağırmak istediğimizi açıkça belirtmek">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-19/src/main.rs:here}}
```

</Listing>

Metod adından önce trait adını yazmak, Rust'a hangi `fly` uygulamasını istediğimizi açıkça söyler. İstersek `Insan::fly(&person)` da yazabilirdik; bu, `person.fly()` ile eşdeğerdir. Ama ayrım yapmaya ihtiyacımız yoksa daha uzundur.

Bu kodun çıktısı şöyledir:

```console
{{#include ../listings/ch20-advanced-features/listing-20-19/output.txt}}
```

`fly` metodu `self` parametresi aldığı için, aynı trait'i uygulayan iki farklı _tür_ olsa bile Rust `self` türüne bakarak hangi uygulamanın kullanılacağını anlayabilir.

Ama metod olmayan ilişkili fonksiyonlarda `self` parametresi yoktur. Aynı adlı metod olmayan fonksiyonları birden fazla tür ya da trait tanımladığında, tam nitelikli sözdizimi kullanmazsanız Rust her zaman neyi kastettiğinizi anlayamaz. Örneğin Liste 20-20'de, bir hayvan barınağının bütün yavru köpeklere `Karabaş` adını vermek istediğini düşünelim. `Hayvan` adlı bir trait oluşturuyoruz; burada metod olmayan ilişkili fonksiyon `baby_name` var. `Kopek` struct'ı bu trait'i uyguluyor; ayrıca `Kopek` üzerinde doğrudan yine `baby_name` adlı bir ilişkili fonksiyon tanımlıyoruz.

<Listing number="20-20" file-name="src/main.rs" caption="Aynı adlı ilişkili fonksiyona sahip bir trait ve aynı adlı ilişkili fonksiyona sahip, bu trait'i de uygulayan bir tür">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-20/src/main.rs}}
```

</Listing>

Tüm yavru köpeklere `Karabaş` adını verme kodunu, `Kopek` üzerinde tanımlı `baby_name` ilişkili fonksiyonunda yazıyoruz. `Kopek` ayrıca bütün hayvanların ortak özelliklerini anlatan `Hayvan` trait'ini de uygular. Yavru köpeklerin "yavru köpek" diye anılması da, `Hayvan` trait'inin `Kopek` üzerindeki `baby_name` uygulamasında ifade edilir.

`main` içinde `Kopek::baby_name` çağırdığımızda, doğrudan `Kopek` üzerinde tanımlı ilişkili fonksiyon çağrılır. Bu kod şu çıktıyı verir:

```console
{{#include ../listings/ch20-advanced-features/listing-20-20/output.txt}}
```

Ama istediğimiz bu değildir. Biz `Kopek` için uyguladığımız `Hayvan` trait'inin `baby_name` fonksiyonunu çağırmak istiyoruz; böylece kod `Bir yavru köpeğe yavru köpek denir` anlamına gelen çıktıyı üretsin. Liste 20-19'da kullandığımız teknik burada işe yaramaz. `main` fonksiyonunu Liste 20-21'deki gibi değiştirirsek derleme hatası alırız.

<Listing number="20-21" file-name="src/main.rs" caption="`Hayvan` trait'indeki `baby_name` fonksiyonunu çağırmaya çalışmak; ama Rust hangi uygulamanın kullanılacağını bilemez">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-21/src/main.rs:here}}
```

</Listing>

`Hayvan::baby_name` bir `self` parametresi almadığı ve `Hayvan` trait'ini uygulayan başka türler de olabileceği için, Rust hangi uygulamayı kastettiğimizi anlayamaz. Derleyiciden şu hatayı alırız:

```console
{{#include ../listings/ch20-advanced-features/listing-20-21/output.txt}}
```

Rust'a özellikle `Kopek` için uygulanmış `Hayvan` sürümünü kullanmak istediğimizi söylemek için _tam nitelikli sözdizimi_ (fully qualified syntax) kullanmamız gerekir. Liste 20-22 bunu gösteriyor.

<Listing number="20-22" file-name="src/main.rs" caption="`Kopek` üzerinde uygulanmış `Hayvan` trait'indeki `baby_name` fonksiyonunu çağırmak istediğimizi tam nitelikli sözdizimiyle belirtmek">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-22/src/main.rs:here}}
```

</Listing>

Açılı ayraçlar içinde Rust'a bir tür açıklaması veriyoruz; yani bu fonksiyon çağrısı için `Kopek` türüne `Hayvan` gibi davranılmasını istediğimizi söylüyoruz. Böylece tam istediğimiz çıktı elde edilir:

```console
{{#include ../listings/ch20-advanced-features/listing-20-22/output.txt}}
```

Genel biçim şöyledir:

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

Metod olmayan ilişkili fonksiyonlarda `receiver` kısmı bulunmaz; yalnızca diğer argümanlar kalır. Aslında fonksiyon ve metod çağırdığınız her yerde bu sözdizimini kullanabilirsiniz. Ama Rust programdaki diğer bilgilerden neyi kastettiğinizi çıkarabiliyorsa bazı kısımları yazmadan geçmenize izin verir. Bu daha uzun sözdizimine yalnızca aynı adı kullanan birden fazla uygulama olduğunda ihtiyaç duyarsınız.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-supertraits-to-require-one-traits-functionality-within-another-trait"></a>

### Üst Trait'leri Kullanmak

Bazen bir trait tanımınız başka bir trait'e dayanır. Yani bir türün ilk trait'i uygulayabilmesi için ikinci trait'i de uygulamasını şart koşmak istersiniz. Bunu, trait tanımınızın ikinci trait'teki öğeleri kullanabilmesi için yaparsınız. Trait'inizin dayandığı bu trait'e _üst trait_ denir.

Örneğin `CerceveliYazdir` adlı bir trait oluşturmak istediğimizi varsayalım. Bunun `outline_print` metodu, verilen değeri yıldızlarla çerçeveleyerek yazdırsın. Diyelim ki `(x, y)` biçiminde çıktı üreten `Display` uygulamasına sahip bir `Nokta` yapımız var. `x = 1` ve `y = 3` olan bir `Nokta` üzerinde `outline_print` çağrıldığında şu sonucu görmek istiyoruz:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

`outline_print` içinde `Display` trait'inin sunduğu işlevselliği kullanmak istiyoruz. Bu yüzden `CerceveliYazdir` trait'inin yalnızca `Display` uygulayan türlerde çalışacağını belirtmeliyiz. Bunu trait tanımında `CerceveliYazdir: Display` diyerek yaparız. Bu teknik, trait'e trait sınırı eklemeye benzer. Liste 20-23, `CerceveliYazdir` uygulamasını gösteriyor.

<Listing number="20-23" file-name="src/main.rs" caption="`Display` işlevselliğine ihtiyaç duyan `CerceveliYazdir` trait'ini uygulamak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-23/src/main.rs:here}}
```

</Listing>

`CerceveliYazdir` trait'inin `Display` gerektirdiğini belirttiğimiz için, `Display` uygulayan her tür için otomatik olarak gelen `to_string` fonksiyonunu kullanabiliyoruz. Eğer trait adından sonra `: Display` yazmasaydık, `&Self` türü için geçerli kapsamda `to_string` adlı metod bulunamadığına dair hata alırdık.

Şimdi `Display` uygulamayan bir tür, örneğin `Nokta`, üzerinde `CerceveliYazdir` uygulamaya çalıştığımızda ne olacağına bakalım:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

</Listing>

Derleyici bize `Display` gerektiğini ama uygulanmadığını söyler:

```console
{{#include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

Bunu düzeltmek için `Nokta` üzerinde `Display` uygular, böylece `CerceveliYazdir`'in istediği koşulu karşılarız:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

</Listing>

Bundan sonra `Nokta` üzerinde `CerceveliYazdir` uygulamak sorunsuz derlenir; `outline_print` çağrısıyla değeri yıldızlı çerçeve içinde yazdırabiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-newtype-pattern-to-implement-external-traits-on-external-types"></a>
<a id="using-the-newtype-pattern-to-implement-external-traits"></a>

### Dış Trait'leri Newtype Deseniyle Uygulamak

10. bölümdeki ["Bir Trait'i Bir Türe Uygulamak"][implementing-a-trait-on-a-type]<!-- ignore --> kısmında yetim kuralından söz etmiştik: bir trait'i bir tür üzerinde ancak trait ya da türden en az biri kendi crate'imize aitse uygulayabiliriz. Bu kısıtlamayı aşmanın yollarından biri newtype desenidir. Bunun için bir demet struct içinde yeni bir tür oluştururuz. (Demet struct'ları 5. bölümdeki ["Demet Struct'larla Farklı Türler Oluşturmak"][tuple-structs]<!-- ignore --> kısmında görmüştük.) Bu demet struct tek alanlı, ince bir sarmalayıcı olur. Böylece sarmalayıcı tür bizim crate'imize ait olduğundan, trait'i onun üzerinde uygulayabiliriz. _Newtype_ terimi Haskell'den gelir. Bu desenin çalışma zamanında ek bir maliyeti yoktur; sarmalayıcı tür derleme zamanında ortadan kaldırılır.

Örneğin, `Vec<T>` üzerinde `Display` uygulamak istediğimizi düşünelim. Yetim kuralı buna doğrudan izin vermez; çünkü hem `Display` hem de `Vec<T>` bizim crate'imizin dışında tanımlanmıştır. Bunun yerine `Vec<T>` örneğini tutan `Sarmalayici` adlı bir struct oluşturabiliriz. Sonra `Display` trait'ini `Sarmalayici` üzerinde uygular ve içteki `Vec<T>` değerini kullanırız. Liste 20-24 bunu gösteriyor.

<Listing number="20-24" file-name="src/main.rs" caption="`Display` uygulayabilmek için `Vec<String>` etrafında `Sarmalayici` türü oluşturmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-24/src/main.rs}}
```

</Listing>

`Display` uygulaması, `Sarmalayici` bir demet struct olduğu ve `Vec<T>` de demetin 0 numaralı öğesi olduğu için içteki `Vec<T>`'ye `self.0` ile erişir. Böylece `Sarmalayici` üzerinde `Display` işlevselliğini kullanabiliriz.

Bu tekniğin dezavantajı, `Sarmalayici`'ın yeni bir tür olmasıdır; dolayısıyla sardığı değerin metodlarına otomatik olarak sahip değildir. `Sarmalayici`'ın tam olarak `Vec<T>` gibi davranmasını istersek, `Vec<T>` metodlarını `Sarmalayici` üzerinde tek tek yazar ve bunları `self.0`'a yönlendiririz. Yeni türün iç türdeki bütün metodlara sahip olmasını istersek `Deref` trait'ini uygulayıp iç türü döndürmek çözüm olabilir. Bunu 15. bölümdeki ["Akıllı İşaretçileri Normal Referanslar Gibi Ele Almak"][smart-pointer-deref]<!-- ignore --> kısmında incelemiştik. Buna karşılık, `Sarmalayici`'ın iç türün bütün metodlarını görmesini istemiyorsak, yalnızca istediğimiz metodları elle uygularız.

Newtype deseni yalnızca trait'lerle sınırlı değildir. Trait'lerle devam etmeden önce Rust'ın tür sistemiyle etkileşmenin başka gelişmiş yollarına bakalım.

[newtype]: ch20-02-gelismis-traitler.html#dış-traitleri-newtype-deseniyle-uygulamak
[implementing-a-trait-on-a-type]: ch10-02-traitler.html#bir-tür-üzerinde-trait-uygulamak-implementing
[traits]: ch10-02-traitler.html
[smart-pointer-deref]: ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-deref
[tuple-structs]: ch05-01-struct-tanimlama-ve-ornekleme.html#demet-structlar-ile-farklı-türler-oluşturmak-creating-different-types-with-tuple-structs
