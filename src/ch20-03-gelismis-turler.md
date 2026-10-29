<!-- Old headings. Do not remove or links may break. -->

<a id="advanced-types"></a>

## Gelişmiş Türler

Rust'ın tür sisteminde şimdiye kadar değindiğimiz ancak henüz ayrıntılı olarak ele almadığımız bazı özellikler vardır. İlk olarak newtype'ların (yeni türlerin) birer tür olarak neden yararlı olduğunu inceleyerek genel hatlarıyla newtype kavramını tartışacağız. Ardından newtype'lara benzeyen ancak anlambilimi (*semantics*) biraz farklı olan *tür takma adlarına (type aliases)* geçeceğiz. Ayrıca `!` türünü ve *dinamik boyutlu türleri (dynamically sized types)* de ele alacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-newtype-pattern-for-type-safety-and-abstraction"></a>
<a id="type-safety-and-abstraction-with-the-newtype-pattern"></a>

### Newtype Deseniyle Tür Güvenliği ve Soyutlama

Bu bölüm, daha önceki [“Dış Trait'leri Newtype Deseniyle Uygulamak”][newtype]<!-- ignore --> alt bölümünü okuduğunuzu varsayar. Newtype deseni, şimdiye kadar ele aldıklarımızın ötesindeki görevler için de kullanışlıdır; değerlerin birbiriyle asla karıştırılmamasını derleme zamanında (statik olarak) zorunlu kılmak ve bir değerin birimini belirtmek bunların arasındadır. Birimleri göstermek amacıyla newtype kullanımının bir örneğini Liste 20-16'da görmüştünüz: `Milimetreler` ve `Metreler` struct'larının `u32` değerlerini bir newtype içinde sarmaladığını hatırlayın. Parametresi `Milimetreler` türünde olan bir fonksiyon yazsaydık, bu fonksiyonu yanlışlıkla `Metreler` türünden bir değerle veya düz bir `u32` ile çağırmaya çalışan bir programı derleyemezdik.

Newtype desenini bir türün bazı uygulama ayrıntılarını soyutlamak için de kullanabiliriz: Yeni tür, içteki gizli türün API'sinden farklı bir açık (*public*) API sunabilir.

Newtype'lar iç uygulama ayrıntılarını da gizleyebilir. Örneğin, bir kişinin kimlik numarasını adıyla ilişkilendirerek saklayan `HashMap<i32, String>` yapısını sarmalamak üzere bir `People` türü sağlayabiliriz. `People` kullanan kod yalnızca sunduğumuz açık API ile —örneğin `People` koleksiyonuna bir isim string'i ekleyen bir metotla— etkileşime girer; bu kodun perde arkasında isimlere `i32` türünde bir kimlik numarası atadığımızı bilmesine gerek kalmaz. Newtype deseni, Bölüm 18'deki [“Uygulama Ayrıntılarını Gizleyen Kapsülleme”][encapsulation-that-hides-implementation-details]<!-- ignore --> bölümünde ele aldığımız uygulama ayrıntılarını gizlemeye yönelik kapsüllemeyi elde etmenin hafif bir yoludur.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-type-synonyms-with-type-aliases"></a>
<a id="type-synonyms-and-type-aliases"></a>

### Tür Eşanlamlıları ve Tür Takma Adları

Rust, mevcut bir türe başka bir ad vermek amacıyla bir _tür takma adı (type alias)_ bildirme olanağı sunar. Bunun için `type` anahtar kelimesini kullanırız. Örneğin `i32` için `Kilometreler` takma adını şu şekilde oluşturabiliriz:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

Artık `Kilometreler` takma adı, `i32` için bir _eşanlamlıdır (synonym)_; Liste 20-16'da oluşturduğumuz `Milimetreler` ve `Metreler` türlerinin aksine `Kilometreler` ayrı ve yeni bir tür değildir. `Kilometreler` türüne sahip değerler, `i32` türündeki değerlerle birebir aynı şekilde ele alınır:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

`Kilometreler` ve `i32` aynı tür olduğundan her iki türdeki değerleri birbiriyle toplayabilir ve `i32` parametresi alan fonksiyonlara `Kilometreler` değerleri geçirebiliriz. Ancak bu yöntemi kullandığımızda, az önce ele aldığımız newtype deseninin sağladığı tür denetimi avantajlarını elde edemeyiz. Başka bir deyişle, bir yerde `Kilometreler` ve `i32` değerlerini karıştırırsak derleyici bize hata vermez.

Tür eşanlamlılarının başlıca kullanım amacı tekrarı azaltmaktır. Örneğin elimizde şöyle uzun bir tür olabilir:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

Bu uzun türü kodun her yerinde fonksiyon imzalarına ve tür açıklamalarına yazmak yorucu ve hataya açık olabilir. Projenizin Liste 20-25'teki gibi kodlarla dolu olduğunu hayal edin.

<Listing number="20-25" caption="Birçok yerde uzun bir tür kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-25/src/main.rs:here}}
```

</Listing>

Bir tür takma adı, tekrarı azaltarak bu kodu daha yönetilebilir hâle getirir. Liste 20-26'da bu uzun ve ayrıntılı tür için `ErtelenenIs` adında bir takma ad tanımladık; böylece türün tüm kullanımlarını daha kısa olan `ErtelenenIs` takma adıyla değiştirebiliriz.

<Listing number="20-26" caption="Tekrarı azaltmak için `ErtelenenIs` adlı bir tür takma adı tanımlamak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-26/src/main.rs:here}}
```

</Listing>

Bu kodu okumak ve yazmak çok daha kolaydır! Bir tür takma adı için anlamlı bir isim seçmek niyetinizi karşı tarafa aktarmanıza da yardımcı olur (İngilizce literatürde daha ileri bir zamanda değerlendirilecek kod parçalarını ifade etmek için _thunk_ terimi kullanılır; bizim seçtiğimiz `ErtelenenIs` adı da saklanan bir kapanış için bu amacı doğrudan yansıtan uygun bir isimdir).

Tür takma adları, tekrarı azaltmak için `Result<T, E>` türüyle birlikte de yaygın olarak kullanılır. Standart kütüphanedeki `std::io` modülünü düşünün. G/Ç (I/O) işlemleri, işlemlerin başarısız olduğu durumları ele almak için çoğu zaman bir `Result<T, E>` döndürür. Bu kütüphanede olası tüm G/Ç hatalarını temsil eden bir `std::io::Error` struct'ı bulunur. `Write` trait'indeki şu fonksiyonlarda olduğu gibi, `std::io` içindeki fonksiyonların pek çoğu `E` parametresi `std::io::Error` olan `Result<T, E>` döndürür:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

`Result<..., Error>` ifadesi çok fazla tekrarlanmaktadır. Bu nedenle `std::io` modülü şu tür takma adı bildirimine sahiptir:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

Bu bildirim `std::io` modülü içinde yer aldığından, tam nitelikli `std::io::Result<T>` takma adını —yani `E` parametresi `std::io::Error` olarak doldurulmuş bir `Result<T, E>` türünü— kullanabiliriz. Böylece `Write` trait'inin fonksiyon imzaları şu görünüme kavuşur:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

Tür takma adı iki açıdan yardımcı olur: Kodu yazmayı kolaylaştırır _ve_ tüm `std::io` genelinde bize tutarlı bir arayüz sunar. Bu yalnızca bir takma ad olduğundan özünde bildiğimiz `Result<T, E>` türünden ibarettir; yani `Result<T, E>` üzerinde çalışan tüm metotları ve `?` operatörü gibi özel sözdizimlerini onunla da doğrudan kullanabiliriz.

> Not: **Newtype Deseni (`struct Km(i32);`) ile Tür Takma Adı (`type Km = i32;`) Arasındaki Fark:** Her iki yapı da çalışma zamanında ek bellek veya performans maliyeti getirmez; ancak derleme zamanındaki davranışları birbirinden tamamen farklıdır:
> - **Newtype (`struct Km(i32);`)**, derleyicinin gözünde yepyeni ve bağımsız bir tür oluşturur. Değerlerin yanlışlıkla birbirine karıştırılmasını derleme zamanında engeller, iç alanı gizleyerek kapsülleme sağlar ve yetim kuralına (*orphan rule*) takılmadan dış trait'leri uygulamanıza olanak tanır. Buna karşılık içteki `i32` türünün metotlarını ve `+` gibi operatörlerini kendiliğinden devralmaz (`self.0` üzerinden erişmeniz veya ilgili trait'leri uygulamanız gerekir).
> - **Tür takma adı (`type Km = i32;`)** ise yeni bir tür oluşturmaz; yalnızca mevcut türe okunabilirliği artıran kısa bir eşanlamlı ad verir. Asıl türün tüm metotları, trait uygulamaları ve operatörleri herhangi bir sarmalama veya açma işlemi gerektirmeden doğrudan çalışır; ancak derleyici `Km` ile `i32` arasında hiçbir tür ayrımı yapmaz ve üzerinde ayrıca dış trait uygulamanıza izin vermez.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-never-type-that-never-returns"></a>
<a id="hiç-dönmeyen-never-türü"></a>

### Hiç Dönmeyen Never Türü

Rust'ta, hiçbir değere sahip olmadığı için tür kuramı terminolojisinde _boş tür (empty type)_ olarak bilinen `!` adında özel bir tür vardır. Biz onu _never türü_ olarak adlandırmayı tercih ediyoruz; çünkü bir fonksiyon asla geri dönmeyeceğinde dönüş türünün yerinde durur. İşte bir örnek:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

Bu kod, "`bar` fonksiyonu asla dönmez" şeklinde okunur. Dönüş türü `!` olan (asla dönmeyen) fonksiyonlara _ıraksayan fonksiyonlar (diverging functions)_ denir. `!` türünde değerler oluşturamayacağımız için `bar` fonksiyonunun geri dönmesi hiçbir koşulda mümkün değildir.

Peki hiçbir zaman değer oluşturamayacağınız bir tür ne işe yarar? Sayı tahmin oyununun bir parçası olan Liste 2-5'teki kodu hatırlayın; o kodun küçük bir bölümünü burada Liste 20-27'de yeniden verdik.

<Listing number="20-27" caption="Kollarından biri `continue` ile biten bir `match` ifadesi">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

</Listing>

O sırada bu koddaki bazı ayrıntıları atlamıştık. Bölüm 6'daki [“`match` Kontrol Akışı Yapısı”][the-match-control-flow-construct]<!-- ignore --> bölümünde, tüm `match` kollarının aynı türü döndürmesi gerektiğini ele almıştık. Dolayısıyla örneğin aşağıdaki kod çalışmaz:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

Bu kodda `tahmin` değişkeninin türünün aynı anda hem bir tam sayı _hem de_ bir string olması gerekirdi; oysa Rust, `tahmin` değişkeninin yalnızca tek bir türe sahip olmasını şart koşar. Peki `continue` ne döndürür? Liste 20-27'de bir koldan `u32` döndürüp diğer kolu `continue` ile bitirmemize nasıl izin verildi?

Tahmin edebileceğiniz gibi `continue` bir `!` değerine sahiptir. Yani Rust, `tahmin` değişkeninin türünü hesaplarken her iki `match` koluna da bakar: ilki `u32` değerine, ikincisi ise `!` değerine sahiptir. `!` asla bir değere sahip olamayacağından Rust, `tahmin` değişkeninin türünün `u32` olduğuna karar verir.

Bu davranışı tanımlamanın biçimsel yolu, `!` türündeki ifadelerin başka herhangi bir türe zorlanabilmesidir (*coerced*). Bu `match` kolunu `continue` ile bitirmemize izin verilir; çünkü `continue` bir değer döndürmez, bunun yerine kontrol akışını döngünün en başına geri taşır ve dolayısıyla `Err` durumunda `tahmin` değişkenine hiçbir zaman bir değer atamayız.

Never türü `panic!` makrosuyla birlikte de kullanışlıdır. Bir değer üretmek ya da paniklemek için `Option<T>` değerleri üzerinde çağırdığımız `unwrap` fonksiyonunun şu tanımını hatırlayın:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

Bu kodda da Liste 20-27'deki `match` ifadesiyle aynı şey gerçekleşir: Rust, `val` değişkeninin `T` türünde ve `panic!` ifadesinin `!` türünde olduğunu görür; dolayısıyla `match` ifadesinin genel sonucu `T` olur. Bu kod çalışır çünkü `panic!` bir değer üretmez; programı sonlandırır. `None` durumunda `unwrap` içinden bir değer döndürmeyeceğimiz için bu kod geçerlidir.

`!` türüne sahip son bir ifade de `loop` döngüsüdür:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

Burada döngü hiçbir zaman sona ermez; bu yüzden ifadenin değeri `!` olur. Ancak döngünün içine bir `break` ekleseydik bu geçerli olmazdı; çünkü döngü `break` ifadesine ulaştığında sonlanırdı.

> Not: Standart kütüphanede karşılaştığınız `std::convert::Infallible` (örneğin asla başarısız olmayacak tür dönüşümlerindeki `TryFrom` uygulamalarında kullanılan `Result<T, Infallible>`), boş tür kavramının pratik bir karşılığıdır. `!` türünün her bağlamda tam teşekküllü bir tür (örneğin jenerik tür argümanı) olarak kararlı (*stable*) Rust'ta kullanılması aşamalı olarak stabilize edildiğinden, standart kütüphane hiçbir varyantı bulunmayan `pub enum Infallible {}` tanımını sunmuştur. Hiçbir varyantı olmayan bir enum'ın da tıpkı `!` gibi çalışma zamanında örneği oluşturulamaz; dolayısıyla `Result<T, Infallible>` döndüren bir işlemin `Err` üretmesi derleme zamanında imkânsızdır ve gelecekteki Rust sürümlerinde `Infallible` türünün doğrudan `!` için bir tür takma adına dönüştürülmesi hedeflenmektedir.

<!-- Old headings. Do not remove or links may break. -->

<a id="dynamically-sized-types-and-the-sized-trait"></a>
<a id="dinamik-boyutlu-türler-ve-sized-traiti"></a>

### Dinamik Boyutlu Türler ve `Sized` Trait'i

Rust'ın, belirli bir türdeki bir değer için ne kadar bellek alanı ayıracağı gibi türlerine ilişkin bazı ayrıntıları bilmesi gerekir. Bu gereksinim, tür sisteminin bir köşesini ilk bakışta biraz kafa karıştırıcı hâle getirir: _dinamik boyutlu türler (dynamically sized types)_ kavramı. Bazen _DST'ler_ veya _boyutsuz türler (unsized types)_ olarak da adlandırılan bu türler, boyutunu yalnızca çalışma zamanında bilebileceğimiz değerleri kullanarak kod yazmamıza olanak tanır.

Gelin kitap boyunca kullandığımız `str` adlı dinamik boyutlu türün ayrıntılarına inelim. Evet, yanlış duymadınız: `&str` değil, tek başına `str` bir DST'dir. Kullanıcının girdiği bir metni saklamak gibi pek çok durumda string'in ne kadar uzun olduğunu çalışma zamanına kadar bilemeyiz. Bu durum, `str` türünde bir değişken oluşturamayacağımız ve `str` türünde bir argüman alamayacağımız anlamına gelir. Çalışmayan aşağıdaki kodu inceleyelim:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

Rust, belirli bir türün herhangi bir değeri için ne kadar bellek ayıracağını bilmek zorundadır ve bir türün tüm değerleri aynı miktarda bellek kullanmalıdır. Rust bu kodu yazmamıza izin verseydi, bu iki `str` değerinin bellekte aynı miktarda yer kaplaması gerekirdi. Oysa uzunlukları farklıdır: `s1` için 12 bayt, `s2` için ise 15 bayt depolama alanı gerekir (`"Nasıl gidiyor?"` metnindeki `'ı'` harfi UTF-8 kodlamasında 2 bayt yer kaplar). İşte bu nedenle dinamik boyutlu bir türü doğrudan tutan bir değişken oluşturmak mümkün değildir.

Peki bu durumda ne yaparız? Burada cevabı zaten biliyorsunuz: `s1` ve `s2`'nin türünü `str` yerine string dilimi (`&str`) yaparız. Bölüm 4'teki [“String Dilimleri”][string-slices]<!-- ignore --> bölümünden, dilim veri yapısının yalnızca başlangıç konumunu ve dilimin uzunluğunu sakladığını hatırlayın. Dolayısıyla `&T`, `T` değerinin bulunduğu bellek adresini saklayan tek bir değer olmasına rağmen bir string dilimi _iki_ değerden oluşur: `str` verisinin adresi ve uzunluğu. Bu sayede bir string dilimi değerinin boyutunu derleme zamanında bilebiliriz: Boyutu bir `usize` uzunluğunun iki katıdır. Yani başvurduğu string ne kadar uzun olursa olsun, bir string diliminin boyutunu her zaman biliriz. Genel olarak Rust'ta dinamik boyutlu türler bu şekilde kullanılır: Dinamik bilginin boyutunu saklayan fazladan bir parça üstveriye (*metadata*) sahiptirler. Dinamik boyutlu türlerin altın kuralı şudur: Dinamik boyutlu türlerin değerlerini her zaman bir tür işaretçinin arkasına koymalıyız.

`str` türünü her türlü işaretçiyle birleştirebiliriz: örneğin `Box<str>` veya `Rc<str>`. Aslında bunu daha önce farklı bir dinamik boyutlu türle görmüştünüz: trait'ler. Her trait, adını kullanarak başvurabileceğimiz dinamik boyutlu bir türdür. Bölüm 18'deki [“Ortak Davranışı Soyutlamak İçin Trait Nesnelerini Kullanma”][using-trait-objects-to-abstract-over-shared-behavior]<!-- ignore --> bölümünde, trait'leri trait nesneleri olarak kullanabilmek için onları `&dyn Trait` veya `Box<dyn Trait>` gibi bir işaretçinin arkasına koymamız gerektiğinden bahsetmiştik (`Rc<dyn Trait>` de aynı şekilde çalışır).

DST'lerle çalışabilmek için Rust, bir türün boyutunun derleme zamanında bilinip bilinmediğini belirlemek üzere `Sized` trait'ini sağlar. Bu trait, boyutu derleme zamanında bilinen her şey için otomatik olarak uygulanır. Buna ek olarak Rust, her jenerik fonksiyona örtük olarak `Sized` üzerine bir sınır ekler. Yani şöyle bir jenerik fonksiyon tanımı:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

gerçekte sanki şunu yazmışız gibi ele alınır:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

Varsayılan olarak jenerik fonksiyonlar yalnızca derleme zamanında bilinen bir boyuta sahip türler üzerinde çalışır. Ancak bu kısıtlamayı gevşetmek için aşağıdaki özel sözdizimini kullanabilirsiniz:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

`?Sized` üzerindeki bir trait sınırı, "`T` türü `Sized` olabilir de olmayabilir de" anlamına gelir ve bu gösterim, jenerik türlerin derleme zamanında bilinen bir boyuta sahip olması gerektiği varsayılanını geçersiz kılar. Bu anlama sahip `?Trait` sözdizimi yalnızca `Sized` için kullanılabilir; başka hiçbir trait için geçerli değildir.

Ayrıca `t` parametresinin türünü `T`'den `&T`'ye değiştirdiğimize dikkat edin. Tür `Sized` olmayabileceği için onu bir tür işaretçinin arkasında kullanmamız gerekir. Bu örnekte bir referans seçtik.

Sırada fonksiyonlar ve kapanışlar var!

[encapsulation-that-hides-implementation-details]: ch18-01-nyp-nedir.html#uygulama-ayrıntılarını-gizleyen-kapsülleme
[string-slices]: ch04-03-dilim-turu.html#string-dilimleri-string-slices
[the-match-control-flow-construct]: ch06-02-match-kontrol-akisi.html#match-kontrol-akışı-yapısı
[using-trait-objects-to-abstract-over-shared-behavior]: ch18-02-trait-nesneleri.html#ortak-davranış-üzerinden-soyutlamak-için-trait-nesnelerini-kullanmak
[newtype]: ch20-02-gelismis-traitler.html#dış-traitleri-newtype-deseniyle-uygulamak
