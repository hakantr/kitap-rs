## Bir Enum Tanımlama

Struct'ların (yapıların) size ilgili alanları ve verileri birlikte gruplandırmanın bir yolunu sunduğu gibi (örneğin `genislik` ve `yukseklik` değerlerine sahip bir `Dikdortgen` gibi), enum'lar da bir değerin olası değerler kümesinden biri olduğunu söylemenin bir yolunu sunar. Örneğin, `Dikdortgen`'in `Daire` ve `Ucgen`'i de içeren olası şekiller kümesinden biri olduğunu söylemek isteyebiliriz. Bunu yapmak için Rust, bu olasılıkları bir enum olarak kodlamamıza (encode) olanak tanır.

Kodda ifade etmek isteyebileceğimiz bir duruma bakalım ve enum'ların neden yararlı olduğunu ve bu durumda struct'lardan neden daha uygun olduğunu görelim. Diyelim ki IP adresleriyle çalışmamız gerekiyor. Şu anda IP adresleri için iki ana standart kullanılıyor: sürüm dört ve sürüm altı. Programımızın karşılaşacağı bir IP adresi için yalnızca bu olasılıklar söz konusu olduğundan, olası tüm varyantları _numaralandırabiliriz_ (enumerate); numaralandırma da adını buradan alır.

Herhangi bir IP adresi sürüm dört veya sürüm altı adres olabilir, ancak aynı anda ikisi birden olamaz. IP adreslerinin bu özelliği enum veri yapısını uygun hale getirir çünkü bir enum değeri varyantlarından yalnızca biri olabilir. Hem sürüm dört hem de sürüm altı adresler temelde hala IP adresleridir, bu nedenle kod herhangi bir IP adresi türüne uygulanan durumları işlerken her ikisine de aynı tür olarak davranılmalıdır.

Bu kavramı, kodda bir `IpAdresTuru` (IpAddrKind) numaralandırması tanımlayıp bir IP adresinin olabileceği olası türleri olan `V4` ve `V6`'yı listeleyerek ifade edebiliriz. Bunlar enum'ın varyantlarıdır:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAdresTuru`, artık kodumuzun başka yerlerinde kullanabileceğimiz özel bir veri türüdür.

### Enum Değerleri

`IpAdresTuru`'nün iki varyantının her birinin örneklerini şu şekilde oluşturabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Enum varyantlarının kendi tanımlayıcısı altında adlandırıldığına ve ikisini ayırmak için çift iki nokta üst üste (::) kullandığımıza dikkat edin. Bu faydalıdır çünkü artık `IpAdresTuru::V4` ve `IpAdresTuru::V6` değerlerinin her ikisi de aynı türdendir: `IpAdresTuru`. Daha sonra, örneğin, herhangi bir `IpAdresTuru` alan bir fonksiyon tanımlayabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

Ve bu fonksiyonu her iki varyantla da çağırabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Enum kullanmanın daha da fazla avantajı vardır. IP adresi türümüz hakkında daha fazla düşünürsek, şu anda asıl IP adresi _verisini_ depolamak için bir yolumuz yok; sadece hangi _tür_ olduğunu biliyoruz. Bölüm 5'te struct'ları yeni öğrendiğiniz için, Liste 6-1'de gösterildiği gibi bu sorunu struct'larla çözmek isteyebilirsiniz.

<Listing number="6-1" caption="Bir IP adresinin verilerini ve `IpAdresTuru` varyantını `struct` kullanarak saklama">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

</Listing>

Burada, iki alana sahip bir `IpAdres` (IpAddr) struct'ı tanımladık: `IpAdresTuru` (daha önce tanımladığımız enum) türünde bir `tur` (kind) alanı ve `String` türünde bir `adres` alanı. Bu struct'ın iki örneği var. İlki `ev`'dir ve `tur` değeri olarak `IpAdresTuru::V4`'e, ilişkili adres verisi olarak da `127.0.0.1` değerine sahiptir. İkinci örnek ise `geridongu`'dur (loopback). Bu örnek, `tur` değeri olarak `IpAdresTuru`'nün diğer varyantı olan `V6`'ya sahiptir ve bununla ilişkili `::1` adresini barındırır. `tur` ve `adres` değerlerini bir araya toplamak için bir struct kullandık, böylece artık varyant değerle ilişkilendirilmiş oldu.

Bununla birlikte, aynı kavramı sadece bir enum kullanarak ifade etmek daha kısa ve özlüdür: Bir struct'ın içine bir enum koymak yerine, verileri doğrudan her bir enum varyantının içine koyabiliriz. `IpAdres` enum'ının bu yeni tanımı, hem `V4` hem de `V6` varyantlarının ilişkili `String` değerlerine sahip olacağını belirtir:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Verileri doğrudan enum'ın her bir varyantına ekliyoruz, bu nedenle fazladan bir struct'a gerek kalmaz. Burada, enum'ların nasıl çalıştığına dair başka bir ayrıntıyı görmek de daha kolaydır: Tanımladığımız her enum varyantının adı, aynı zamanda o enum'ın bir örneğini oluşturan bir fonksiyon haline gelir. Yani, `IpAdres::V4()`, bir `String` argümanı alan ve `IpAdres` türünde bir örnek döndüren bir fonksiyon çağrısıdır. Enum'ı tanımladığımız için bu kurucu (constructor) fonksiyon otomatik olarak tanımlanmış olarak gelir.

Struct yerine enum kullanmanın başka bir avantajı daha vardır: Her varyant farklı türlerde ve miktarlarda ilişkili verilere sahip olabilir. Sürüm dört IP adresleri her zaman 0 ile 255 arasında değerlere sahip dört sayısal bileşene sahip olacaktır. `V4` adreslerini dört `u8` değeri olarak depolamak isteyip `V6` adreslerini yine de tek bir `String` değeri olarak ifade etmek isteseydik, bunu bir struct ile yapamazdık. Enum'lar bu durumu kolaylıkla halleder:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Sürüm dört ve sürüm altı IP adreslerini depolamak üzere veri yapılarını tanımlamanın birkaç farklı yolunu gösterdik. Ancak ortaya çıktığı üzere, IP adreslerini depolamak ve hangi türde olduklarını kodlamak o kadar yaygındır ki, [standart kütüphanede kullanabileceğimiz bir tanım vardır!][IpAddr]<!-- ignore --> Gelin standart kütüphanenin `IpAddr`'ı nasıl tanımladığına bakalım. Bizim tanımladığımız ve kullandığımız enum ve varyantların aynısına sahiptir, ancak adres verilerini varyantların içine her varyant için farklı şekilde tanımlanan iki farklı struct biçiminde gömer:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Bu kod, bir enum varyantının içine her tür veriyi koyabileceğinizi göstermektedir: örneğin string'ler, sayısal türler veya struct'lar. Hatta içine başka bir enum bile ekleyebilirsiniz! Ayrıca, standart kütüphane türleri genellikle sizin bulacağınızdan çok daha karmaşık değildir.

Standart kütüphane `IpAddr` için bir tanım içerse de, standart kütüphanenin tanımını kapsamımıza dahil etmediğimiz için yine de kendi tanımımızı oluşturup çakışma olmadan kullanabileceğimizi unutmayın. Türleri kapsama getirme konusunu Bölüm 7'de daha ayrıntılı olarak ele alacağız.

Liste 6-2'de başka bir enum örneğine bakalım: Bunun varyantlarına gömülü çok çeşitli türleri vardır.

<Listing number="6-2" caption="Varyantlarının her biri farklı miktarlarda ve türlerde değerler depolayan bir `Mesaj` enum'ı">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

</Listing>

Bu enum'ın farklı türlere sahip dört varyantı vardır:

- `Cik`: Kendisiyle ilişkili hiçbir veriye sahip değildir.
- `Tasi`: Tıpkı bir struct'ta olduğu gibi isimlendirilmiş alanlara sahiptir.
- `Yaz`: Tek bir `String` içerir.
- `RenkDegistir`: Üç adet `i32` değeri içerir.

Liste 6-2'deki gibi varyantlara sahip bir enum tanımlamak, farklı türlerde struct tanımları tanımlamaya benzer, tek fark enum'ın `struct` anahtar kelimesini kullanmaması ve tüm varyantların `Mesaj` türü altında birlikte gruplandırılmasıdır. Aşağıdaki struct'lar önceki enum varyantlarının tuttuğu aynı verileri tutabilirdi:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Ancak her biri kendi türüne sahip olan bu farklı struct'ları kullansaydık, bu mesaj türlerinden herhangi birini alacak bir fonksiyonu, tek bir tür olan Liste 6-2'deki `Mesaj` enum'ı ile tanımladığımız kadar kolay tanımlayamazdık.

Enum'lar ve struct'lar arasında bir benzerlik daha vardır: Tıpkı struct'larda `impl` kullanarak metotlar tanımlayabildiğimiz gibi, enum'larda da metotlar tanımlayabiliriz. İşte `Mesaj` enum'ımızda tanımlayabileceğimiz `cagir` adlı bir metot:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

Metodun gövdesi, metodu çağırdığımız değeri elde etmek için `self` kullanacaktır. Bu örnekte, `Mesaj::Yaz(String::from("merhaba"))` değerine sahip bir `m` değişkeni oluşturduk; `m.cagir()` çalıştığında `cagir` metodunun gövdesinde `self`'in değeri bu olacaktır.

Standart kütüphanede çok yaygın ve kullanışlı olan başka bir enum'a bakalım: `Option`.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-option-enum-and-its-advantages-over-null-values"></a>

### `Option` Enum'ı

Bu bölüm, standart kütüphane tarafından tanımlanan başka bir enum olan `Option` için bir örnek olay incelemesi yapmaktadır. `Option` türü, bir değerin bir şey olabileceği veya hiçbir şey olamayacağı son derece yaygın senaryoyu kodlar.

Örneğin, boş olmayan bir listedeki ilk öğeyi isterseniz, bir değer alırsınız. Boş bir listedeki ilk öğeyi isterseniz, hiçbir şey alamazsınız. Bu kavramı tür sistemi açısından ifade etmek, derleyicinin ele almanız gereken tüm durumları ele alıp almadığınızı kontrol edebileceği anlamına gelir; bu işlevsellik, diğer programlama dillerinde son derece yaygın olan hataları (bug) önleyebilir.

Programlama dili tasarımı genellikle hangi özellikleri dahil ettiğiniz açısından düşünülür, ancak hariç tuttuğunuz özellikler de önemlidir. Rust, diğer birçok dilin sahip olduğu null (boş) özelliğine sahip değildir. _Null_, orada hiçbir değer olmadığı anlamına gelen bir değerdir. Null içeren dillerde değişkenler her zaman iki durumdan birinde olabilir: null veya null olmayan (not-null).

Null değerinin mucidi Tony Hoare, 2009'daki "Null Referanslar: Milyar Dolarlık Hata" ("Null References: The Billion Dollar Mistake") adlı sunumunda şunları söylemiştir:

> Buna benim milyar dolarlık hatam diyorum. O zamanlar nesne yönelimli (object-oriented) bir dilde referanslar için ilk kapsamlı tür sistemini tasarlıyordum. Amacım, derleyici tarafından otomatik olarak gerçekleştirilen kontrollerle, referansların tüm kullanımlarının kesinlikle güvenli olmasını sağlamaktı. Ancak, uygulaması çok kolay olduğu için null referans ekleme cazibesine karşı koyamadım. Bu, muhtemelen son kırk yılda bir milyar dolarlık acıya ve hasara neden olan sayısız hataya, güvenlik açığına ve sistem çökmesine yol açtı.

Null değerleriyle ilgili sorun, null değerini null olmayan bir değer olarak kullanmaya çalışırsanız, bir tür hata almanızdır. Bu null veya null olmama özelliği yaygın olduğundan (pervasive), bu tür bir hata yapmak son derece kolaydır.

Ancak null'un ifade etmeye çalıştığı kavram hala yararlıdır: Null, şu anda herhangi bir nedenle geçersiz olan veya olmayan (absent) bir değerdir.

Sorun aslında kavramda değil, belirli uygulamadadır. Bu nedenle Rust'ta null yoktur, ancak bir değerin var veya yok olması kavramını kodlayabilen bir enum'a sahiptir. Bu enum `Option<T>`'dir ve [standart kütüphane tarafından][option]<!-- ignore --> şu şekilde tanımlanır:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` enum'ı o kadar faydalıdır ki prelude (başlangıç) kütüphanesine bile dahil edilmiştir; açıkça (explicitly) kapsama dahil etmenize gerek yoktur. Varyantları da prelude kütüphanesine dahildir: `Some` ve `None`'ı `Option::` öneki olmadan doğrudan kullanabilirsiniz. `Option<T>` enum'ı hala sadece normal bir enum'dır ve `Some(T)` ve `None` hala `Option<T>` türünün varyantlarıdır.

`<T>` sözdizimi, Rust'ın henüz bahsetmediğimiz bir özelliğidir. Bu genel (generic) bir tür parametresidir ve Bölüm 10'da jenerikleri daha ayrıntılı olarak ele alacağız. Şimdilik bilmeniz gereken tek şey, `<T>`'nin `Option` enum'ının `Some` varyantının herhangi bir türden bir veri tutabileceği anlamına geldiği ve `T` yerine kullanılan her somut (concrete) türün genel `Option<T>` türünü farklı bir tür haline getirdiğidir. Sayı türlerini ve karakter (char) türlerini tutmak için `Option` değerlerinin kullanıldığı bazı örnekler:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

`bir_sayi` değişkeninin türü `Option<i32>`'dir. `bir_karakter` değişkeninin türü ise farklı bir tür olan `Option<char>`'dır. `Some` varyantının içinde bir değer belirttiğimiz için Rust bu türleri çıkarabilir. `olmayan_sayi` için Rust bizden genel `Option` türünü açıklamamızı ister: Derleyici, yalnızca bir `None` değerine bakarak karşılık gelen `Some` varyantının tutacağı türü çıkaramaz. Burada, Rust'a `olmayan_sayi`'nın `Option<i32>` türünde olmasını kastettiğimizi söyleriz.

Bir `Some` değerimiz olduğunda, bir değerin mevcut olduğunu ve değerin `Some` içinde tutulduğunu biliriz. Bir `None` değerimiz olduğunda ise bu, bazı açılardan null ile aynı anlama gelir: Geçerli bir değerimiz yoktur. Peki, `Option<T>`'ye sahip olmak null'a sahip olmaktan neden daha iyidir?

Kısaca, `Option<T>` ve `T` (burada `T` herhangi bir tür olabilir) farklı türler olduğundan, derleyici bir `Option<T>` değerini sanki kesinlikle geçerli bir değermiş gibi kullanmamıza izin vermez. Örneğin, aşağıdaki kod derlenmeyecektir, çünkü bir `Option<i8>`'e bir `i8` eklemeye çalışmaktadır:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Eğer bu kodu çalıştırırsak, şuna benzer bir hata mesajı alırız:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

Sert! Aslında bu hata mesajı, Rust'ın farklı türlerde oldukları için bir `i8` ile bir `Option<i8>`'i nasıl toplayacağını anlamadığı anlamına gelir. Rust'ta `i8` gibi bir türde değere sahip olduğumuzda, derleyici her zaman geçerli bir değere sahip olduğumuzdan emin olur. Bu değeri kullanmadan önce null olup olmadığını kontrol etmek zorunda kalmadan güvenle devam edebiliriz. Yalnızca bir `Option<i8>`'e (veya üzerinde çalıştığımız değerin türü her neyse) sahip olduğumuzda bir değere sahip olmama ihtimalinden endişe duymalıyız; derleyici bu değeri kullanmadan önce bu durumu ele aldığımızdan emin olacaktır.

Başka bir deyişle, bir `Option<T>` ile `T` işlemleri gerçekleştirebilmeniz için onu bir `T`'ye dönüştürmeniz gerekir. Genellikle bu, null ile ilgili en yaygın sorunlardan birini yakalamaya yardımcı olur: Bir şeyin aslında null olduğu halde null olmadığını varsaymak.

Null olmayan (not-null) bir değeri yanlış varsayma riskini ortadan kaldırmak, kodunuza daha fazla güvenmenize yardımcı olur. Null olabilme ihtimali olan bir değere sahip olmak için, o değerin türünü `Option<T>` yaparak bu durumu açıkça (explicitly) belirtmeniz (opt in) gerekir. Daha sonra, o değeri kullandığınızda, değerin null olduğu durumu açıkça ele almanız istenir. Değerin `Option<T>` olmayan bir türe sahip olduğu her yerde, değerin null olmadığını güvenle varsayabilirsiniz. Bu, Rust'ta null'un yaygınlığını sınırlamak ve Rust kodunun güvenliğini artırmak için bilinçli (deliberate) olarak verilmiş bir tasarım kararıydı.

Peki, bu değeri kullanabilmek için `Option<T>` türünde bir değere sahip olduğunuzda `T` değerini bir `Some` varyantından nasıl çıkarırsınız? `Option<T>` enum'ı, çeşitli durumlarda yararlı olan çok sayıda metoda sahiptir; bunları [belgelerinden][docs]<!-- ignore --> inceleyebilirsiniz. `Option<T>` üzerindeki metotlara aşina olmak, Rust yolculuğunuzda son derece yararlı olacaktır.

Genel olarak, bir `Option<T>` değerini kullanmak için, her bir varyantı ele alacak bir koda sahip olmak istersiniz. Yalnızca `Some(T)` değeriniz olduğunda çalışacak bir kod istersiniz ve bu kodun içteki (inner) `T`'yi kullanmasına izin verilir. Yalnızca `None` değeriniz varsa çalışacak başka bir kod istersiniz ve bu kodda kullanılabilecek bir `T` değeri yoktur. `match` ifadesi, enum'larla kullanıldığında tam olarak bunu yapan bir kontrol akışı yapısıdır: Sahip olduğu enum'ın hangi varyantına bağlı olarak farklı kodlar çalıştıracaktır ve bu kod eşleşen değerin içindeki verileri kullanabilir.

[IpAddr]: ../std/net/enum.IpAddr.html
[option]: ../std/option/enum.Option.html
[docs]: ../std/option/enum.Option.html
