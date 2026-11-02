<!-- Old headings. Do not remove or links may break. -->

<a id="defining-an-enum"></a>
<a id="bir-enum-tanimlama"></a>

## Bir Enum Tanımlama

Struct'lar (örneğin `genislik` ve `yukseklik` alanlarına sahip bir `Dikdortgen`) birbiriyle ilişkili alanları ve verileri bir arada gruplandırmanın bir yolunu sunarken, enum'lar bir değerin olası değerler kümesinden yalnızca biri olduğunu ifade etmenin bir yolunu sunar. Örneğin, `Dikdortgen` türünün `Daire` ve `Ucgen`'i de kapsayan olası şekiller kümesinden biri olduğunu belirtmek isteyebiliriz. Rust, bu olasılıkları bir enum olarak kodlamamıza olanak tanır.

Gelin kodda ifade etmek isteyebileceğimiz somut bir duruma bakalım ve bu senaryoda enum'ların neden struct'lardan daha kullanışlı ve uygun olduğunu görelim. Diyelim ki IP adresleriyle çalışmamız gerekiyor. Günümüzde IP adresleri için iki ana standart kullanılmaktadır: sürüm dört ve sürüm altı. Programımızın karşılaşacağı bir IP adresi için yalnızca bu iki olasılık söz konusu olduğundan, olası tüm varyantları tek tek _numaralandırabiliriz (enumerate)_; numaralandırma (*enumeration*) kavramı da adını buradan alır.

Herhangi bir IP adresi ya sürüm dört ya da sürüm altı adres olabilir, ancak aynı anda ikisi birden olamaz. IP adreslerinin bu özelliği `enum` veri yapısını son derece uygun kılar; çünkü bir enum değeri tanımlı varyantlarından yalnızca biri olabilir. Öte yandan hem sürüm dört hem de sürüm altı adresler temelde hâlâ birer IP adresidir; dolayısıyla her tür IP adresi için geçerli durumları işleyen kodlarda her ikisi de aynı tür olarak ele alınmalıdır.

Bu kavramı kodda bir `IpAdresTuru` numaralandırması tanımlayıp bir IP adresinin alabileceği olası türleri — `V4` ve `V6` — listeleyerek ifade edebiliriz. Bunlar enum'ın varyantlarıdır:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAdresTuru`, artık kodumuzun başka yerlerinde kullanabileceğimiz özel bir veri türüdür.

<!-- Old headings. Do not remove or links may break. -->

<a id="enum-values"></a>
<a id="enum-degerleri"></a>

### Enum Değerleri

`IpAdresTuru` enum'ının iki varyantından her birinin örneklerini şu şekilde oluşturabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Enum varyantlarının, enum'ın tanımlayıcısının *ad alanı (namespace)* altında yer aldığına ve ikisini ayırmak için çift iki nokta üst üste (`::`) kullandığımıza dikkat edin. Bu oldukça kullanışlıdır; çünkü hem `IpAdresTuru::V4` hem de `IpAdresTuru::V6` değerleri artık aynı türdendir: `IpAdresTuru`. Böylece örneğin herhangi bir `IpAdresTuru` değeri alan bir fonksiyon tanımlayabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

Ardından bu fonksiyonu her iki varyantla da çağırabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Enum kullanmanın başka avantajları da vardır. IP adresi türümüz üzerinde biraz daha düşündüğümüzde, şu anki hâliyle asıl IP adresi _verisini_ saklamanın bir yolu olmadığını görürüz; yalnızca hangi _türde_ olduğunu biliyoruz. Bölüm 5'te struct'ları yeni öğrendiğiniz için, bu sorunu Liste 6-1'de gösterildiği gibi struct kullanarak çözmeye yönelmeniz oldukça doğaldır.

<Listing number="6-1" caption="Bir IP adresinin verilerini ve `IpAdresTuru` varyantını `struct` kullanarak saklama">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

</Listing>

Burada iki alana sahip bir `IpAdres` struct'ı tanımladık: `IpAdresTuru` (az önce tanımladığımız enum) türünde bir `tur` alanı ve `String` türünde bir `adres` alanı. Bu struct'ın iki örneğini oluşturduk. İlki olan `ev`, `tur` alanı olarak `IpAdresTuru::V4` değerine ve ilişkili adres verisi olarak `127.0.0.1` metnine sahiptir. İkinci örnek olan `geridongu` ise `tur` değeri olarak `IpAdresTuru` enum'ının diğer varyantı olan `V6`'yı ve bununla ilişkili `::1` adresini barındırır. `tur` ve `adres` değerlerini bir arada tutmak için bir struct kullandık; böylece varyant ile adres verisi birbirine bağlanmış oldu.

Ancak aynı kavramı yalnızca bir enum kullanarak temsil etmek çok daha özlüdür: Bir struct'ın içine enum yerleştirmek yerine, veriyi doğrudan her bir enum varyantının içine koyabiliriz. `IpAdres` enum'ının bu yeni tanımı, hem `V4` hem de `V6` varyantlarının kendileriyle ilişkili birer `String` değerine sahip olacağını belirtir:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Veriyi doğrudan enum'ın her bir varyantına eklediğimiz için fazladan bir struct tanımlamaya gerek kalmaz. Burada enum'ların çalışma biçimine dair önemli bir ayrıntıyı görmek de kolaylaşır: Tanımladığımız her enum varyantının adı, aynı zamanda o enum'ın bir örneğini oluşturan bir fonksiyon hâline gelir. Yani `IpAdres::V4()`, bir `String` argümanı alan ve `IpAdres` türünde bir örnek döndüren bir fonksiyon çağrısıdır. Enum'ı tanımlamamızın bir sonucu olarak bu *yapıcı (constructor)* fonksiyonlar bizim için otomatik olarak tanımlanır.

Struct yerine enum kullanmanın bir avantajı daha vardır: Her varyant farklı türlerde ve farklı sayıda ilişkili veriye sahip olabilir. Sürüm dört IP adresleri her zaman 0 ile 255 arasında değerler alan dört sayısal bileşenden oluşur. `V4` adreslerini dört adet `u8` değeri olarak saklamak, `V6` adreslerini ise yine tek bir `String` değeri olarak ifade etmek isteseydik bunu tek bir struct ile yapamazdık. Enum'lar bu durumu kolaylıkla çözer:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Sürüm dört ve sürüm altı IP adreslerini saklamak üzere veri yapıları tanımlamanın birkaç farklı yolunu gördük. Ne var ki IP adreslerini saklamak ve hangi türde olduklarını kodlamak o kadar yaygın bir ihtiyaçtır ki, [standart kütüphanede doğrudan kullanabileceğimiz hazır bir tanım bulunur!][IpAddr]<!-- ignore --> Gelin standart kütüphanenin `IpAddr` türünü nasıl tanımladığına bakalım. Bizim tanımladığımız ve kullandığımız enum ile aynı yapıya ve varyantlara (`V4` ve `V6`) sahiptir; ancak adres verilerini varyantların içine, her varyant için ayrı tanımlanmış iki farklı struct biçiminde gömer:

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

Bu kod, bir enum varyantının içine her türden veriyi koyabileceğinizi gösterir: örneğin metinler, sayısal türler veya struct'lar. Hatta bir enum varyantının içine başka bir enum bile yerleştirebilirsiniz! Ayrıca gördüğünüz gibi, standart kütüphanedeki türler çoğu zaman sizin tasarlayabileceğiniz yapılardan çok daha karmaşık değildir.

Standart kütüphane `IpAddr` için bir tanım içerse de, standart kütüphanenin bu tanımını kendi kapsamımıza dahil etmediğimiz için kendi türümüzü (hatta aynı `IpAddr` adıyla bile) oluşturup herhangi bir isim çakışması yaşamadan kullanabileceğimize dikkat edin. Türleri kapsama dahil etme konusunu Bölüm 7'de daha ayrıntılı olarak ele alacağız.

Gelin Liste 6-2'deki bir diğer enum örneğine bakalım: Bu enum'ın varyantlarında birbirinden çok farklı veri türleri yer alır.

<Listing number="6-2" caption="Varyantlarının her biri farklı sayıda ve türde değerler saklayan bir `Mesaj` enum'ı">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

</Listing>

Bu enum, farklı türlere sahip dört varyant içerir:

- `Cik`: Kendisiyle ilişkili hiçbir veri barındırmaz.
- `Tasi`: Tıpkı bir struct gibi isimlendirilmiş alanlara sahiptir.
- `Yaz`: Tek bir `String` içerir.
- `RenkDegistir`: Üç adet `i32` değeri içerir.

Liste 6-2'deki gibi varyantlara sahip bir enum tanımlamak, farklı struct türleri tanımlamaya benzer; aradaki fark, enum'ın `struct` anahtar kelimesini kullanmaması ve tüm varyantların tek bir `Mesaj` türü altında bir arada gruplandırılmasıdır. Aşağıdaki struct'lar, önceki enum varyantlarının tuttuğu verilerin birebir aynısını tutabilirdi:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Ancak her biri kendi ayrı türüne sahip olan bu farklı struct'ları kullansaydık, bu mesaj türlerinden herhangi birini kabul edecek bir fonksiyonu, tek bir tür olan Liste 6-2'deki `Mesaj` enum'ı ile tanımladığımız kadar kolay tanımlayamazdık.

Enum'lar ile struct'lar arasında bir benzerlik daha vardır: Tıpkı struct'lar üzerinde `impl` kullanarak metotlar tanımlayabildiğimiz gibi, enum'lar üzerinde de metotlar tanımlayabiliriz. İşte `Mesaj` enum'ımız üzerinde tanımlayabileceğimiz `cagir` adlı bir metot:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

Metodun gövdesi, metodun üzerinde çağrıldığı değeri elde etmek için `self` parametresini kullanır. Bu örnekte `Mesaj::Yaz(String::from("merhaba"))` değerine sahip bir `m` değişkeni oluşturduk; `m.cagir()` çalıştığında `cagir` metodunun gövdesindeki `self` tam olarak bu değeri temsil edecektir.

Şimdi standart kütüphanede çok yaygın ve son derece kullanışlı olan başka bir enum'a bakalım: `Option`.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-option-enum-and-its-advantages-over-null-values"></a>
<a id="the-option-enum"></a>
<a id="option-enum"></a>
<a id="option-enumi"></a>

### `Option` Enum'ı

Bu alt bölümde, standart kütüphane tarafından tanımlanan bir diğer enum olan `Option` türünü örnek olay olarak inceleyeceğiz. `Option` türü, bir değerin var olabileceği (bir şey olduğu) ya da yok olabileceği (hiçbir şey olmadığı) çok yaygın senaryoyu kodlar.

Örneğin, boş olmayan bir listedeki ilk öğeyi isterseniz bir değer elde edersiniz. Boş bir listedeki ilk öğeyi isterseniz hiçbir şey elde edemezsiniz. Bu kavramı doğrudan tür sistemi üzerinden ifade etmek, ele almanız gereken tüm durumları gerçekten ele alıp almadığınızı derleyicinin denetleyebilmesi anlamına gelir; bu özellik, diğer programlama dillerinde son derece sık karşılaşılan yazılım hatalarını daha derleme aşamasında önleyebilir.

Programlama dili tasarımı çoğu zaman hangi özellikleri dile dahil ettiğiniz üzerinden düşünülür; ancak dışarıda bıraktığınız özellikler de bir o kadar önemlidir. Rust, diğer pek çok dilde bulunan *null* özelliğine sahip değildir. _Null_, ortada hiçbir değer bulunmadığını belirten özel bir değerdir. Null kavramına sahip dillerde değişkenler her zaman iki durumdan birinde olabilir: null veya null olmayan.

Null kavramının mucidi Tony Hoare, 2009 yılındaki “Null Referanslar: Milyar Dolarlık Hata” (*“Null References: The Billion Dollar Mistake”*) başlıklı sunumunda bu durumu şöyle anlatmıştır:

> Buna kendi *milyar dolarlık hatam (billion-dollar mistake)* diyorum. O dönemde nesne yönelimli bir dilde referanslar için ilk kapsamlı tür sistemini tasarlıyordum. Amacım, derleyici tarafından otomatik olarak yürütülen denetimlerle tüm referans kullanımlarının kesinlikle güvenli olmasını sağlamaktı. Ne var ki uygulaması çok kolay olduğu için sisteme bir null referans ekleme cazibesine karşı koyamadım. Bu tercih; son kırk yılda muhtemelen bir milyar dolarlık zarara ve sıkıntıya mal olan sayısız hataya, güvenlik açığına ve sistem çökmesine yol açtı.

Null değerleriyle ilgili temel sorun şudur: Null olan bir değeri sanki geçerli (null olmayan) bir değermiş gibi kullanmaya çalışırsanız bir tür çalışma zamanı hatasıyla karşılaşırsınız. Null veya null olmama durumu dilin her yerine yayılmış olduğundan, bu tür bir hatayı yapmak son derece kolaydır.

Bununla birlikte null'ın ifade etmeye çalıştığı kavram hâlâ yararlıdır: Herhangi bir nedenle o an geçersiz ya da mevcut olmayan bir değeri temsil etmek.

Dolayısıyla asıl sorun kavramın kendisinde değil, bu kavramın uygulanış biçimindedir. Bu nedenle Rust'ta null yoktur; ancak bir değerin var veya yok olma durumunu tür düzeyinde kodlayan bir enum vardır. Bu enum `Option<T>` olarak adlandırılır ve [standart kütüphanede][option]<!-- ignore --> şu şekilde tanımlanır:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` enum'ı o kadar kullanışlıdır ki her Rust programında kendiliğinden kapsamda bulunan öğeler kümesi olan *prelude* (ön tanımlı kapsam) içine bile dahil edilmiştir; yani onu `use` ile açıkça kapsama dahil etmenize gerek yoktur. Üstelik varyantları da prelude içindedir: `Some` ve `None` varyantlarını `Option::` öneki yazmadan doğrudan kullanabilirsiniz. Yine de `Option<T>` sıradan bir enum'dır; `Some(T)` ve `None` da hâlâ `Option<T>` türünün varyantlarıdır.

`<T>` sözdizimi, Rust'ın henüz ele almadığımız bir özelliğidir. Bu, *jenerik (generic)* bir tür parametresidir; jenerik türleri Bölüm 10'da ayrıntılı olarak inceleyeceğiz. Şimdilik bilmeniz gereken tek şey şudur: `<T>`, `Option` enum'ının `Some` varyantının herhangi bir türden tek bir veri tutabileceği ve `T` yerine geçen her somut türün `Option<T>` türünün tamamını farklı bir tür hâline getirdiği anlamına gelir. Sayı ve karakter (`char`) türlerini tutmak için `Option` değerlerinin kullanıldığı bazı örneklere bakalım:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

`bir_sayi` değişkeninin türü `Option<i32>`'dir. `bir_karakter` değişkeninin türü ise bundan farklı bir tür olan `Option<char>`'dır. `Some` varyantının içine somut bir değer verdiğimiz için Rust bu türleri kendiliğinden çıkarabilir. `olmayan_sayi` değişkeninde ise Rust bizden `Option` türünün tamamını açıkça belirtmemizi ister: Derleyici, yalnızca bir `None` değerine bakarak karşılık gelen `Some` varyantının hangi türde değer tutacağını çıkaramaz. Burada `olmayan_sayi` değişkeninin `Option<i32>` türünde olmasını istediğimizi Rust'a açıkça bildiririz.

Elimizde bir `Some` değeri olduğunda, ortada bir değer bulunduğunu ve bu değerin `Some` içinde tutulduğunu biliriz. Elimizde bir `None` değeri olduğunda ise bu, bir bakıma null ile aynı anlama gelir: Geçerli bir değerimiz yoktur. Öyleyse `Option<T>` kullanmak, null'a sahip olmaktan neden daha iyidir?

Kısaca söylemek gerekirse, `Option<T>` ile `T` (burada `T` herhangi bir tür olabilir) birbirinden farklı türler olduğu için derleyici, bir `Option<T>` değerini sanki kesinlikle geçerli bir `T` değeriymiş gibi doğrudan kullanmamıza izin vermez. Örneğin aşağıdaki kod, bir `i8` değerine doğrudan bir `Option<i8>` eklemeye çalıştığı için derlenmeyecektir:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Bu kodu çalıştırdığımızda şuna benzer bir hata mesajı alırız:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

İlk bakışta oldukça yoğun bir çıktı! Özünde bu hata mesajı (`` cannot add `Option<i8>` to `i8` ``), `i8` ile `Option<i8>` farklı türler olduğu için Rust'ın bu iki değeri nasıl toplayacağını bilmediğini (yani `i8` türü için `Option<i8>` ile toplama işlemini tanımlayan `Add<Option<i8>>` trait'inin uygulanmadığını) belirtir. Rust'ta `i8` gibi bir türde değere sahip olduğumuzda, derleyici her zaman geçerli bir değere sahip olduğumuzu garanti eder. Bu değeri kullanmadan önce null olup olmadığını kontrol etmek zorunda kalmadan güvenle ilerleyebiliriz. Yalnızca elimizde bir `Option<i8>` (veya üzerinde çalıştığımız değerin türü her neyse) olduğunda bir değerin bulunmama ihtimalini düşünmemiz gerekir; derleyici de değeri kullanmadan önce bu durumu ele aldığımızdan emin olur.

Başka bir deyişle, bir `Option<T>` üzerinde `T` türüne ait işlemleri gerçekleştirebilmek için önce onu bir `T` değerine dönüştürmeniz gerekir. Bu zorunluluk, null kaynaklı en yaygın hatalardan birini yakalamaya yardımcı olur: Gerçekte null olan bir değerin null olmadığını varsaymak.

Null olabilecek bir değeri yanlışlıkla geçerli varsayma riskini ortadan kaldırmak, kodunuza çok daha fazla güvenmenizi sağlar. Yok olma ihtimali taşıyan bir değere sahip olmak istediğinizde, o değerin türünü `Option<T>` yaparak bu durumu açıkça tercih etmeniz gerekir. Ardından bu değeri her kullandığınızda, değerin mevcut olmadığı (`None`) durumu açıkça ele almanız zorunludur. Türü `Option<T>` olmayan her yerde ise değerin geçerli olduğunu _güvenle_ varsayabilirsiniz. Bu yaklaşım, null kavramının kontrolsüz yaygınlığını sınırlamak ve Rust kodunun güvenliğini artırmak adına bilinçli olarak alınmış bir tasarım kararıdır.

> Not: C veya C++ gibi dillerden geliyorsanız, `null` işaretçi yerine `Option` enum'ı kullanmanın bellekte fazladan bir varyant etiketi (*discriminant*) yer kaplayıp kaplamadığını merak edebilirsiniz. Rust derleyicisi, **boş işaretçi optimizasyonu (*null pointer optimization*)** (daha genel adıyla *niş optimizasyonu / niche optimization*) sayesinde bu ek maliyeti pek çok tür için tamamen ortadan kaldırır. Rust'ta referanslar (`&T`, `&mut T`), `Box<T>` gibi akıllı işaretçiler ve fonksiyon işaretçileri hiçbir zaman geçerli bir `0` (boş) bellek adresine işaret edemez. Derleyici bu "geçersiz sıfır bit desenini" `None` varyantını temsil etmek için kullanır; böylece örneğin `Option<&T>` bellekte `&T` ile **birebir aynı boyutta** yer kaplar. Başka bir deyişle, C/C++'taki boş olabilen işaretçilerle aynı bellek verimliliğini, sıfır çalışma zamanı ek yükü ve tam derleme zamanı tür güvenliğiyle elde edersiniz.

Peki `Option<T>` türünde bir değere sahip olduğunuzda, içindeki `T` değerini kullanabilmek için onu `Some` varyantından nasıl çıkarırsınız? `Option<T>` enum'ı, farklı senaryolarda işinize yarayacak çok sayıda metoda sahiptir; bunların tamamını [dokümantasyonundan][docs]<!-- ignore --> inceleyebilirsiniz. `Option<T>` üzerindeki metotlara aşina olmak, Rust yolculuğunuzda size büyük kolaylık sağlayacaktır.

> Not: Günlük Rust kodlarında her seferinde uzun `match` ifadeleri yazmak yerine `Option<T>`'nin sunduğu hazır yardımcı metotlardan sıkça yararlanırsınız. Örneğin; değerin varlığını sorgulamak için `is_some()` ve `is_none()`, `None` durumunda varsayılan bir değer sağlamak için `unwrap_or(varsayilan)` (veya varsayılan değeri yalnızca gerektiğinde hesaplamak için Bölüm 13'te göreceğimiz `unwrap_or_else`), içteki değeri `Option` sarmalayıcısından çıkarmadan dönüştürmek için `map(...)`, değerin `None` olması durumunda programı hata mesajıyla durdurmak içinse Bölüm 9'da inceleyeceğimiz `expect("mesaj")` ve `unwrap()` metotları en sık karşılaşacağınız araçlardır.

Genel olarak bir `Option<T>` değerini kullanmak için her bir varyantı ele alacak kodlara sahip olmak istersiniz: Yalnızca elinizde bir `Some(T)` değeri olduğunda çalışacak ve içteki `T` değerini kullanmasına izin verilen bir kod; bir de yalnızca `None` değeriniz olduğunda çalışacak ve elinde kullanılabilir bir `T` değeri bulunmayan başka bir kod. `match` ifadesi, enum'larla birlikte kullanıldığında tam olarak bu işi yapan bir kontrol akışı yapısıdır: Değerin, enum'ın hangi varyantı olduğuna bağlı olarak farklı kod dallarını çalıştırır ve bu kodlar eşleşen değerin içindeki verileri doğrudan kullanabilir.

[IpAddr]: https://doc.rust-lang.org/std/net/enum.IpAddr.html
[option]: https://doc.rust-lang.org/std/option/enum.Option.html
[docs]: https://doc.rust-lang.org/std/option/enum.Option.html
