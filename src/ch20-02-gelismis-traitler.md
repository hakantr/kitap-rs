<!-- Old headings. Do not remove or links may break. -->

<a id="advanced-traits"></a>

## Gelişmiş Trait'ler

Trait'leri ilk olarak Bölüm 10'daki [“Trait'lerle Ortak Davranış Tanımlama”][traits]<!-- ignore --> bölümünde ele almıştık; ancak daha ileri düzey ayrıntılara girmemiştik. Artık Rust hakkında çok daha fazla şey bildiğinize göre işin inceliklerine inebiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="specifying-placeholder-types-in-trait-definitions-with-associated-types"></a>
<a id="associated-types"></a>
<a id="defining-traits-with-associated-types"></a>

### İlişkili Türlerle Trait Tanımlamak

_İlişkili türler (associated types)_, bir tür yer tutucusunu bir trait ile ilişkilendirir; böylece trait metot tanımları kendi imzalarında bu yer tutucu türleri kullanabilir. Bir trait'i uygulayan taraf, söz konusu uygulama için yer tutucu türün yerine kullanılacak somut türü belirler. Bu sayede, trait uygulanana kadar bu türlerin tam olarak ne olduğunu bilmemize gerek kalmadan bazı türleri kullanan bir trait tanımlayabiliriz.

Bu bölümde ele aldığımız gelişmiş özelliklerin çoğunu nadiren ihtiyaç duyulan yapılar olarak tanımladık. İlişkili türler ise ikisinin ortasında bir yerde durur: Kitabın geri kalanında açıklanan özelliklere kıyasla daha seyrek, ancak bu bölümde tartışılan diğer pek çok özelliğe kıyasla daha yaygın biçimde kullanılırlar.

İlişkili türe sahip bir trait'e örnek olarak standart kütüphanenin sunduğu `Iterator` trait'ini verebiliriz. Buradaki ilişkili türün adı `Item`'dır ve `Iterator` trait'ini uygulayan türün üzerinde ilerlediği değerlerin türünü temsil eder. `Iterator` trait'inin tanımı Liste 20-13'te gösterilmektedir.

<Listing number="20-13" caption="`Item` adlı ilişkili türe sahip `Iterator` trait'inin tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-13/src/lib.rs}}
```

</Listing>

Buradaki `Item` türü bir yer tutucudur ve `next` metodunun tanımı, `Option<Self::Item>` türünde değerler döndüreceğini gösterir. `Iterator` trait'ini uygulayanlar `Item` için somut türü belirler ve `next` metodu da bu somut türden bir değer içeren bir `Option` döndürür.

İlişkili türler, hangi türleri işleyebileceğini baştan belirtmeden bir fonksiyon tanımlamamıza olanak tanıyan jeneriklere benzer bir kavram gibi görünebilir. İki kavram arasındaki farkı incelemek için, `Item` türünü `u32` olarak belirten `Sayac` adlı bir tür üzerindeki `Iterator` trait uygulamasına bakalım:

<Listing file-name="src/lib.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-22-iterator-on-counter/src/lib.rs:ch19}}
```

</Listing>

Bu sözdizimi jeneriklerinkine oldukça benzer görünüyor. Öyleyse `Iterator` trait'ini neden Liste 20-14'te gösterildiği gibi doğrudan jeneriklerle tanımlamayalım?

<Listing number="20-14" caption="`Iterator` trait'inin jenerikler kullanılarak yazılmış varsayımsal bir tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-14/src/lib.rs}}
```

</Listing>

Aradaki fark şudur: Liste 20-14'te olduğu gibi jenerikleri kullandığımızda, her uygulamada türleri açıkça belirtmemiz gerekir; çünkü `Sayac` için `Iterator<String>` veya başka herhangi bir tür uygulamasını da yazabileceğimizden, `Sayac` için birden fazla `Iterator` uygulamasına sahip olabiliriz. Başka bir deyişle, bir trait jenerik bir parametreye sahip olduğunda, jenerik tür parametrelerinin somut türleri her seferinde değiştirilerek tek bir tür için birden fazla kez uygulanabilir. `Sayac` üzerinde `next` metodunu kullandığımızda ise hangi `Iterator` uygulamasını kullanmak istediğimizi belirtmek için tür açıklamaları eklememiz gerekirdi.

İlişkili türlerle ise tür açıklamaları eklememize gerek kalmaz; çünkü bir trait'i bir tür üzerinde birden fazla kez uygulayamayız. İlişkili türleri kullanan Liste 20-13'teki tanımda, `Sayac` için yalnızca tek bir `impl Iterator for Sayac` olabileceğinden `Item` türünün ne olacağını yalnızca bir kez seçebiliriz. `Sayac` üzerinde `next` çağırdığımız her yerde `u32` değerlerinden oluşan bir iteratör istediğimizi belirtmek zorunda kalmayız.

İlişkili türler aynı zamanda trait'in sözleşmesinin bir parçası hâline gelir: Trait'i uygulayanlar, ilişkili tür yer tutucusunun yerine geçecek bir tür sağlamak zorundadır. İlişkili türler genellikle türün nasıl kullanılacağını açıklayan bir ada sahip olur ve API dokümantasyonunda ilişkili türü belgelendirmek iyi bir pratiktir.

> Not: Kendi trait'inizi tasarlarken ilişkili tür ile jenerik tür parametresi arasında seçim yapmak için şu pratik ölçütü kullanabilirsiniz:
> - Trait'i uygulayan her tür için **yalnızca tek bir doğal çıktı türü** olması mantıklıysa **ilişkili tür** seçin (`Iterator::Item`, `Future::Output` veya `Deref::Target` gibi). Böylece trait'i kullanan kodu her çağrıda tür açıklaması yazma yükünden kurtarırsınız.
> - Aynı türün bu trait'i **farklı girdi veya dönüşüm türleriyle birden fazla kez** uygulayabilmesi gerekiyorsa **jenerik tür parametresi** seçin (`From<T>` ile bir türün hem `&str` hem `i32` değerlerinden dönüştürülebilmesi veya birazdan göreceğimiz `Add<Rhs>` ile farklı sağ taraf türleriyle toplanabilmesi gibi).

<!-- Old headings. Do not remove or links may break. -->

<a id="default-generic-type-parameters-and-operator-overloading"></a>
<a id="using-default-generic-parameters-and-operator-overloading"></a>

### Varsayılan Jenerik Parametreler ve Operatör Aşırı Yükleme

Jenerik tür parametreleri kullandığımızda, jenerik tür için varsayılan bir somut tür belirtebiliriz. Bu, varsayılan tür iş görüyorsa trait'i uygulayanların somut bir tür belirtme zorunluluğunu ortadan kaldırır. Jenerik bir türü bildirirken varsayılan türü `<YerTutucuTur=SomutTur>` sözdizimiyle belirtirsiniz.

Bu tekniğin yararlı olduğu harika bir durum, belirli koşullarda bir operatörün (`+` gibi) davranışını özelleştirdiğiniz _operatör aşırı yüklemedir (operator overloading)_.

Rust kendi operatörlerinizi oluşturmanıza veya rastgele operatörleri aşırı yüklemenize izin vermez. Ancak `std::ops` modülünde listelenen işlemleri ve bunlara karşılık gelen trait'leri, operatörle ilişkili trait'i uygulayarak aşırı yükleyebilirsiniz. Örneğin Liste 20-15'te, iki `Nokta` örneğini birbiriyle toplamak için `+` operatörünü aşırı yüklüyoruz. Bunu bir `Nokta` struct'ı üzerinde `Add` trait'ini uygulayarak yapıyoruz.

<Listing number="20-15" file-name="src/main.rs" caption="`Nokta` örnekleri için `+` operatörünü aşırı yüklemek üzere `Add` trait'ini uygulamak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-15/src/main.rs}}
```

</Listing>

`add` metodu, yeni bir `Nokta` oluşturmak için iki `Nokta` örneğinin `x` değerlerini ve iki `Nokta` örneğinin `y` değerlerini toplar. `Add` trait'i, `add` metodundan döndürülecek türü belirleyen `Output` adında bir ilişkili türe sahiptir.

Bu koddaki varsayılan jenerik tür, `Add` trait'inin içinde yer alır. Tanımı şöyledir:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

Bu kod genel hatlarıyla tanıdık gelmelidir: Tek bir metoda ve bir ilişkili türe sahip bir trait. Yeni olan kısım `Rhs=Self` ifadesidir: Bu sözdizimine _varsayılan tür parametreleri (default type parameters)_ denir. `Rhs` jenerik tür parametresi (“sağ taraf” anlamındaki *right-hand side* ifadesinin kısaltmasıdır), `add` metodundaki `rhs` parametresinin türünü tanımlar. `Add` trait'ini uygularken `Rhs` için somut bir tür belirtmezsek, `Rhs` parametresinin türü varsayılan olarak `Self`, yani üzerinde `Add` uyguladığımız tür olur.

`Nokta` için `Add` uyguladığımızda iki `Nokta` örneğini toplamak istediğimizden `Rhs` için varsayılan türü kullandık. Şimdi `Add` trait'ini uygularken varsayılanı kullanmak yerine `Rhs` türünü özelleştirmek istediğimiz bir örneğe bakalım.

Farklı birimlerde değerler tutan `Milimetreler` ve `Metreler` adında iki struct'ımız var. Mevcut bir türün başka bir struct içinde bu şekilde ince bir katmanla sarmalanması _newtype deseni_ olarak bilinir; bunu [“Dış Trait'leri Newtype Deseniyle Uygulamak”][newtype]<!-- ignore --> bölümünde daha ayrıntılı olarak ele alacağız. Milimetre cinsinden değerleri metre cinsinden değerlerle toplamak ve `Add` uygulamasının birim dönüşümünü doğru biçimde yapmasını istiyoruz. Liste 20-16'da gösterildiği gibi `Rhs` olarak `Metreler` türünü vererek `Milimetreler` için `Add` uygulayabiliriz.

<Listing number="20-16" file-name="src/lib.rs" caption="`Milimetreler` ile `Metreler` değerlerini toplamak için `Milimetreler` üzerinde `Add` trait'ini uygulamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-16/src/lib.rs}}
```

</Listing>

`Milimetreler` ve `Metreler` değerlerini toplamak için, `Self` varsayılanını kullanmak yerine `Rhs` tür parametresinin değerini belirlemek üzere `impl Add<Metreler>` yazarız.

Varsayılan tür parametrelerini başlıca iki şekilde kullanırsınız:

1. Mevcut kodu bozmadan bir türü genişletmek için
2. Çoğu kullanıcının ihtiyaç duymayacağı özel durumlarda özelleştirmeye olanak tanımak için

Standart kütüphanenin `Add` trait'i ikinci amaca örnektir: Genellikle birbirine benzer iki türü toplarsınız, ancak `Add` trait'i bunun ötesinde özelleştirme yapabilme olanağı sunar. `Add` trait tanımında varsayılan bir tür parametresi kullanmak, çoğu zaman fazladan parametreyi belirtmek zorunda kalmamanız anlamına gelir. Başka bir deyişle, uygulama sırasında tekrarlayan kalıp kodlara biraz daha az ihtiyaç duyulur ve bu da trait'in kullanımını kolaylaştırır.

İlk amaç ikinciye benzer, ancak ters yönde işler: Mevcut bir trait'e bir tür parametresi eklemek isterseniz, mevcut uygulama kodlarını bozmadan trait'in işlevselliğinin genişletilmesine olanak tanımak için bu parametreye bir varsayılan tür verebilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name"></a>
<a id="disambiguating-between-methods-with-the-same-name"></a>
<a id="disambiguating-between-identically-named-methods"></a>
<a id="aynı-adlı-metodlar-arasında-ayrım-yapmak"></a>

### Aynı Adlı Metotlar Arasında Ayrım Yapmak

Rust'ta hiçbir kural bir trait'in başka bir trait'in metoduyla aynı ada sahip bir metot barındırmasını engellemez; her iki trait'i tek bir tür üzerinde uygulamanıza da engel olmaz. Ayrıca trait'lerdeki metotlarla aynı ada sahip bir metodu doğrudan türün kendisi üzerinde uygulamak da mümkündür.

Aynı ada sahip metotları çağırırken Rust'a hangisini kullanmak istediğinizi söylemeniz gerekir. Her ikisi de `fly` adında bir metoda sahip `Pilot` ve `Buyucu` adlı iki trait tanımladığımız Liste 20-17'deki kodu ele alalım. Ardından her iki trait'i, üzerinde halihazırda `fly` adlı bir metot uygulanmış olan `Insan` türü üzerinde uyguluyoruz. Her bir `fly` metodu farklı bir iş yapıyor.

<Listing number="20-17" file-name="src/main.rs" caption="`fly` metoduna sahip olacak şekilde tanımlanıp `Insan` türü üzerinde uygulanan iki trait ve doğrudan `Insan` üzerinde uygulanan bir `fly` metodu">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-17/src/main.rs:here}}
```

</Listing>

Bir `Insan` örneği üzerinde `fly` metodunu çağırdığımızda derleyici, Liste 20-18'de gösterildiği gibi varsayılan olarak doğrudan türün üzerinde uygulanmış olan metodu çağırır.

<Listing number="20-18" file-name="src/main.rs" caption="Bir `Insan` örneği üzerinde `fly` metodunu çağırmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-18/src/main.rs:here}}
```

</Listing>

Bu kodu çalıştırmak ekrana `*çılgınca kollarını çırpıyor*` yazdıracak ve Rust'ın doğrudan `Insan` üzerinde uygulanmış `fly` metodunu çağırdığını gösterecektir.

Gerek `Pilot` trait'inden gerekse `Buyucu` trait'inden gelen `fly` metotlarını çağırmak için hangi `fly` metodunu kastettiğimizi belirten daha açık bir sözdizimi kullanmamız gerekir. Liste 20-19 bu sözdizimini göstermektedir.

<Listing number="20-19" file-name="src/main.rs" caption="Hangi trait'in `fly` metodunu çağırmak istediğimizi belirtmek">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-19/src/main.rs:here}}
```

</Listing>

Metot adından önce trait adını belirtmek, Rust'a hangi `fly` uygulamasını çağırmak istediğimizi netleştirir. Liste 20-19'da kullandığımız `person.fly()` ifadesine eşdeğer olan `Insan::fly(&person)` biçimini de yazabilirdik; ancak belirsizliği gidermemiz gerekmiyorsa bunu yazmak biraz daha uzundur.

Bu kodu çalıştırmak ekrana aşağıdakileri yazdırır:

```console
{{#include ../listings/ch20-advanced-features/listing-20-19/output.txt}}
```

`fly` metodu bir `self` parametresi aldığından, tek bir _trait'i_ uygulayan iki farklı _türümüz_ olsaydı, Rust `self` parametresinin türüne bakarak trait'in hangi uygulamasını kullanacağını çıkarabilirdi.

Ancak metot olmayan ilişkili fonksiyonların `self` parametresi yoktur. Aynı fonksiyon adına sahip metot olmayan fonksiyonlar tanımlayan birden fazla tür veya trait bulunduğunda, _tam nitelikli sözdizimi (fully qualified syntax)_ kullanmadığınız sürece Rust hangi türü kastettiğinizi her zaman bilemez. Örneğin Liste 20-20'de, tüm yavru köpeklere Karabaş adını vermek isteyen bir hayvan barınağı için bir trait oluşturuyoruz. Metot olmayan ilişkili bir `baby_name` fonksiyonuna sahip `Hayvan` adında bir trait tanımlıyoruz. `Hayvan` trait'i, üzerinde doğrudan metot olmayan ilişkili bir `baby_name` fonksiyonu da sunduğumuz `Kopek` struct'ı için uygulanıyor.

<Listing number="20-20" file-name="src/main.rs" caption="İlişkili fonksiyona sahip bir trait ile aynı adda ilişkili fonksiyona sahip olan ve aynı zamanda bu trait'i uygulayan bir tür">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-20/src/main.rs}}
```

</Listing>

Tüm yavru köpeklere Karabaş adını verme kodunu, `Kopek` üzerinde tanımlanan `baby_name` ilişkili fonksiyonunda uyguluyoruz. `Kopek` türü ayrıca tüm hayvanların sahip olduğu özellikleri tanımlayan `Hayvan` trait'ini de uygular. Köpek yavrularına yavru köpek denir ve bu durum, `Kopek` üzerindeki `Hayvan` trait uygulamasında `Hayvan` trait'iyle ilişkili `baby_name` fonksiyonu içinde ifade edilmiştir.

`main` içinde, doğrudan `Kopek` üzerinde tanımlanmış ilişkili fonksiyonu çağıran `Kopek::baby_name` fonksiyonunu çağırıyoruz. Bu kod ekrana şunu yazdırır:

```console
{{#include ../listings/ch20-advanced-features/listing-20-20/output.txt}}
```

Bu çıktı bizim istediğimiz sonuç değildir. Biz `Kopek` üzerinde uyguladığımız `Hayvan` trait'inin bir parçası olan `baby_name` fonksiyonunu çağırmak istiyoruz; böylece kod `Bir yavru köpeğe yavru köpek denir` çıktısını üretsin. Liste 20-19'da kullandığımız trait adını belirtme tekniği burada işe yaramaz; `main` fonksiyonunu Liste 20-21'deki koda değiştirirsek bir derleme hatası alırız.

<Listing number="20-21" file-name="src/main.rs" caption="`Hayvan` trait'inden `baby_name` fonksiyonunu çağırmayı denemek; ancak Rust hangi uygulamayı kullanacağını bilemez">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-21/src/main.rs:here}}
```

</Listing>

`Hayvan::baby_name` bir `self` parametresine sahip olmadığından ve `Hayvan` trait'ini uygulayan başka türler de bulunabileceğinden, Rust `Hayvan::baby_name` fonksiyonunun hangi uygulamasını istediğimizi çıkaramaz. Şu derleyici hatasını alırız:

```console
{{#include ../listings/ch20-advanced-features/listing-20-21/output.txt}}
```

Belirsizliği gidermek ve Rust'a başka bir tür için olan `Hayvan` uygulaması yerine `Kopek` için olan `Hayvan` uygulamasını kullanmak istediğimizi söylemek için tam nitelikli sözdizimini kullanmamız gerekir. Liste 20-22 tam nitelikli sözdiziminin nasıl kullanılacağını göstermektedir.

<Listing number="20-22" file-name="src/main.rs" caption="`Kopek` üzerinde uygulandığı hâliyle `Hayvan` trait'indeki `baby_name` fonksiyonunu çağırmak istediğimizi belirtmek için tam nitelikli sözdizimi kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-22/src/main.rs:here}}
```

</Listing>

Açılı parantezler içinde Rust'a bir tür açıklaması sağlıyoruz; bu açıklama, bu fonksiyon çağrısı için `Kopek` türünü bir `Hayvan` olarak ele almak istediğimizi söyleyerek `Kopek` üzerinde uygulandığı hâliyle `Hayvan` trait'indeki `baby_name` metodunu çağırmak istediğimizi belirtir. Bu kod artık istediğimiz çıktıyı yazdıracaktır:

```console
{{#include ../listings/ch20-advanced-features/listing-20-22/output.txt}}
```

Genel olarak tam nitelikli sözdizimi şöyle tanımlanır:

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

Metot olmayan ilişkili fonksiyonlarda bir `receiver` (alıcı, yani `self`) bulunmaz: Yalnızca diğer argümanların listesi yer alır. Fonksiyon veya metot çağırdığınız her yerde tam nitelikli sözdizimini kullanabilirsiniz. Ancak Rust'ın programdaki diğer bilgilerden çıkarabildiği her kısmı bu sözdiziminden çıkarmanıza izin verilir. Bu daha ayrıntılı sözdizimini yalnızca aynı adı kullanan birden fazla uygulamanın bulunduğu ve Rust'ın hangi uygulamayı çağırmak istediğinizi belirlemek için yardıma ihtiyaç duyduğu durumlarda kullanmanız gerekir.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-supertraits-to-require-one-traits-functionality-within-another-trait"></a>
<a id="using-supertraits"></a>

### Üst Trait'leri Kullanmak

Bazen başka bir trait'e bağımlı olan bir trait tanımı yazabilirsiniz: Bir türün ilk trait'i uygulayabilmesi için ikinci trait'i de uygulamasını zorunlu kılmak istersiniz. Bunu, trait tanımınızın ikinci trait'in ilişkili öğelerinden yararlanabilmesi için yaparsınız. Trait tanımınızın dayandığı bu trait'e, trait'inizin _üst trait'i (supertrait)_ denir.

Örneğin, verilen bir değeri yıldız işaretleriyle çerçevelenmiş biçimde yazdıracak `outline_print` metoduna sahip bir `CerceveliYazdir` trait'i yapmak istediğimizi varsayalım. Yani standart kütüphanedeki `Display` trait'ini `(x, y)` sonucu üretecek biçimde uygulayan bir `Nokta` struct'ı verildiğinde, `x` için `1` ve `y` için `3` değerine sahip bir `Nokta` örneği üzerinde `outline_print` çağırdığımızda şunu yazdırmalıdır:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

`outline_print` metodunun uygulamasında `Display` trait'inin işlevselliğini kullanmak istiyoruz. Bu nedenle, `CerceveliYazdir` trait'inin yalnızca `Display` trait'ini de uygulayan ve `CerceveliYazdir`'ın ihtiyaç duyduğu işlevselliği sağlayan türler için çalışacağını belirtmemiz gerekir. Bunu trait tanımında `CerceveliYazdir: Display` belirterek yapabiliriz. Bu teknik, trait'e bir trait sınırı eklemeye benzer. Liste 20-23, `CerceveliYazdir` trait'inin bir uygulamasını göstermektedir.

<Listing number="20-23" file-name="src/main.rs" caption="`Display`'den gelen işlevselliği gerektiren `CerceveliYazdir` trait'ini uygulamak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-23/src/main.rs:here}}
```

</Listing>

`CerceveliYazdir` trait'inin `Display` trait'ini gerektirdiğini belirttiğimiz için, `Display` uygulayan her tür üzerinde otomatik olarak uygulanan `to_string` fonksiyonunu kullanabiliriz. Trait adından sonra iki nokta üst üste ekleyip `Display` trait'ini belirtmeden `to_string` kullanmayı deneseydik, geçerli kapsamda `&Self` türü için `to_string` adında bir metot bulunamadığını söyleyen bir hata alırdık.

`Nokta` struct'ı gibi `Display` uygulamayan bir tür üzerinde `CerceveliYazdir` uygulamaya çalıştığımızda ne olacağını görelim:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

</Listing>

`Display`'in gerekli olduğunu ancak uygulanmadığını belirten bir hata alırız:

```console
{{#include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

Bunu düzeltmek için `Nokta` üzerinde `Display` uygular ve `CerceveliYazdir`'ın gerektirdiği kısıtı şu şekilde karşılarız:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

</Listing>

Ardından `Nokta` üzerinde `CerceveliYazdir` trait'ini uygulamak başarıyla derlenecek ve bir `Nokta` örneğini yıldızlardan oluşan bir çerçeve içinde görüntülemek için üzerinde `outline_print` metodunu çağırabileceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-newtype-pattern-to-implement-external-traits-on-external-types"></a>
<a id="using-the-newtype-pattern-to-implement-external-traits"></a>
<a id="implementing-external-traits-with-the-newtype-pattern"></a>
<a id="dış-traitleri-newtype-deseniyle-uygulamak"></a>

### Dış Trait'leri Newtype Deseniyle Uygulamak

Bölüm 10'daki [“Bir Tür Üzerinde Trait Uygulama”][implementing-a-trait-on-a-type]<!-- ignore --> bölümünde, bir trait'i bir tür üzerinde yalnızca trait'in ya da türün veya her ikisinin birden kendi crate'imize yerel olması durumunda uygulayabileceğimizi belirten yetim kuralından (*orphan rule*) bahsetmiştik. Bir demet struct'ı içinde yeni bir tür oluşturmayı içeren _newtype desenini_ kullanarak bu kısıtlamanın etrafından dolaşmak mümkündür. (Demet struct'larını Bölüm 5'teki [“Demet Struct'ları ile Farklı Türler Oluşturma”][tuple-structs]<!-- ignore --> bölümünde ele almıştık.) Bu demet struct'ı tek bir alana sahip olur ve kendisi için trait uygulamak istediğimiz türün etrafında ince bir sarmalayıcı görevi görür. Böylece sarmalayıcı tür kendi crate'imize yerel olur ve trait'i bu sarmalayıcı üzerinde uygulayabiliriz. _Newtype_, kökeni Haskell programlama diline dayanan bir terimdir. Bu deseni kullanmanın çalışma zamanında hiçbir performans maliyeti yoktur ve sarmalayıcı tür derleme zamanında ortadan kaldırılır.

Örnek olarak `Vec<T>` üzerinde `Display` uygulamak istediğimizi düşünelim; hem `Display` trait'i hem de `Vec<T>` türü crate'imizin dışında tanımlandığı için yetim kuralı bunu doğrudan yapmamızı engeller. Bir `Vec<T>` örneğini tutan bir `Sarmalayici` struct'ı oluşturabiliriz; ardından Liste 20-24'te gösterildiği gibi `Sarmalayici` üzerinde `Display` uygulayabilir ve `Vec<T>` değerini kullanabiliriz.

<Listing number="20-24" file-name="src/main.rs" caption="`Display` uygulamak için `Vec<String>` etrafında bir `Sarmalayici` türü oluşturmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-24/src/main.rs}}
```

</Listing>

`Sarmalayici` bir demet struct'ı olduğu ve `Vec<T>` demetteki 0 indeksli öğe olduğu için `Display` uygulaması içteki `Vec<T>` değerine erişmek üzere `self.0` ifadesini kullanır. Ardından `Sarmalayici` üzerinde `Display` trait'inin işlevselliğini kullanabiliriz.

Bu tekniği kullanmanın dezavantajı, `Sarmalayici`'nın yeni bir tür olması ve dolayısıyla tuttuğu değerin metotlarına sahip olmamasıdır. `Sarmalayici`'ya tıpkı bir `Vec<T>` gibi davranabilmek için `Vec<T>`'nin tüm metotlarını doğrudan `Sarmalayici` üzerinde, işlevi `self.0`'a devredecek şekilde uygulamamız gerekirdi. Yeni türün içteki türün sahip olduğu her metoda sahip olmasını isteseydik, içteki türü döndürecek şekilde `Sarmalayici` üzerinde `Deref` trait'ini uygulamak bir çözüm olurdu (`Deref` trait'ini uygulamayı Bölüm 15'teki [“Akıllı İşaretçilere Normal Referanslar Gibi Davranma”][smart-pointer-deref]<!-- ignore --> bölümünde ele almıştık). Eğer `Sarmalayici` türünün içteki türün tüm metotlarına sahip olmasını istemeseydik —örneğin `Sarmalayici` türünün davranışını kısıtlamak amacıyla— yalnızca gerçekten istediğimiz metotları elle uygulamamız gerekirdi.

Bu newtype deseni, işin içinde trait'ler olmadığında bile kullanışlıdır. Şimdi odağımızı değiştirelim ve Rust'ın tür sistemiyle etkileşim kurmanın bazı gelişmiş yollarına bakalım.

[newtype]: ch20-02-gelismis-traitler.html#dış-traitleri-newtype-deseniyle-uygulamak
[implementing-a-trait-on-a-type]: ch10-02-traitler.html#bir-tür-üzerinde-trait-uygulamak-implementing
[traits]: ch10-02-traitler.html
[smart-pointer-deref]: ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-deref
[tuple-structs]: ch05-01-struct-tanimlama-ve-ornekleme.html#demet-structlar-ile-farklı-türler-oluşturmak-creating-different-types-with-tuple-structs
