<!-- Old headings. Do not remove or links may break. -->

<a id="treating-smart-pointers-like-regular-references-with-the-deref-trait"></a>
<a id="treating-smart-pointers-like-regular-references-with-deref"></a>
<a id="treating-smart-pointers-like-regular-references"></a>
<a id="akıllı-işaretçilere-normal-referanslar-gibi-davranmak"></a>
<a id="akilli-isaretcilere-normal-referanslar-gibi-davranmak"></a>

## Akıllı İşaretçilere Normal Referanslar Gibi Davranma

`Deref` trait'ini uygulamak, _başvuru çözme operatörü (dereference operator)_ olan `*` sembolünün (çarpma veya glob operatörüyle karıştırılmamalıdır) davranışını özelleştirmenize olanak tanır. `Deref` trait'ini bir akıllı işaretçiye normal bir referans gibi davranılabilecek biçimde uygulayarak, referanslar üzerinde çalışan kodlar yazabilir ve aynı kodu akıllı işaretçilerle de kullanabilirsiniz.

Öncelikle başvuru çözme operatörünün normal referanslarla nasıl çalıştığına bakalım. Ardından `Box<T>` gibi davranan özel bir tür tanımlamayı deneyecek ve başvuru çözme operatörünün yeni tanımladığımız bu tür üzerinde neden bir referanstaki gibi çalışmadığını göreceğiz. `Deref` trait'ini uygulamanın, akıllı işaretçilerin referanslara benzer şekillerde çalışmasını nasıl mümkün kıldığını inceleyeceğiz. Son olarak da Rust'ın _deref zorlaması (deref coercion)_ özelliğine ve bu özelliğin hem referanslarla hem de akıllı işaretçilerle çalışmamıza nasıl imkân tanıdığına göz atacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>
<a id="following-the-pointer-to-the-value"></a>
<a id="following-the-reference-to-the-value"></a>
<a id="referansın-göstediği-değere-gitmek"></a>
<a id="referansin-gosterdigi-degere-gitmek"></a>

### Referansı İzleyerek Değere Ulaşmak

Normal bir referans bir işaretçi türüdür ve bir işaretçiyi düşünmenin yollarından biri, onu başka bir yerde saklanan bir değere uzanan bir ok olarak hayal etmektir. Liste 15-6'da bir `i32` değerine referans oluşturuyor ve ardından referansı izleyip değere ulaşmak için başvuru çözme operatörünü kullanıyoruz.

<Listing number="15-6" file-name="src/main.rs" caption="Bir `i32` değerine işaret eden referansı başvuru çözme operatörüyle izlemek">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

</Listing>

`sayi` değişkeni `5` değerinde bir `i32` tutar. `referans` değişkenini `sayi` değişkenine bir referans olacak şekilde ayarlarız. `sayi` değişkeninin `5`'e eşit olduğunu doğrulayabiliriz. Ancak `referans` içindeki değer hakkında bir doğrulama yapmak istiyorsak, derleyicinin gerçek değeri karşılaştırabilmesi için `*referans` yazarak referansı işaret ettiği değere kadar izlememiz (bu işleme _başvuru çözme_ denir) gerekir. `referans` üzerinde başvuru çözme yaptığımızda, `referans`'ın işaret ettiği tam sayı değerine erişir ve bu değeri `5` ile karşılaştırabiliriz.

Bunun yerine `assert_eq!(5, referans);` yazmayı deneseydik şu derleme hatasını alırdık:

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

Bir sayıyı ve bir sayıya referansı karşılaştırmaya izin verilmez; çünkü bunlar farklı türlerdir (`{integer}` ve `&{integer}`). Derleyicinin `E0277` hatasında belirttiği üzere tam sayı türü için `&{integer}` ile eşitlik karşılaştırması (`PartialEq<&{integer}>`) tanımlı değildir. Referansın işaret ettiği değere ulaşmak için başvuru çözme operatörünü kullanmamız gerekir.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-boxt-like-a-reference"></a>
<a id="boxtyi-referans-gibi-kullanmak"></a>

### `Box<T>`'yi Referans Gibi Kullanmak

Liste 15-6'daki kodu referans yerine bir `Box<T>` kullanacak şekilde yeniden yazabiliriz; Liste 15-7'de `Box<T>` üzerinde kullanılan başvuru çözme operatörü, Liste 15-6'da referans üzerinde kullanılan başvuru çözme operatörüyle birebir aynı şekilde çalışır.

<Listing number="15-7" file-name="src/main.rs" caption="Bir `Box<i32>` üzerinde başvuru çözme operatörünü kullanmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

</Listing>

Liste 15-7 ile Liste 15-6 arasındaki temel fark, burada `kutu` değişkenini `sayi` değerine işaret eden bir referans yerine, `sayi` değerinin öbekteki bir kopyasına işaret eden bir kutu örneği olarak ayarlamamızdır. Son doğrulamada, Liste 15-6'da `referans` değişkeniyle yaptığımız gibi kutunun işaretçisini izlemek için başvuru çözme operatörünü (`*kutu`) kullanabiliriz. Şimdi kendi kutu türümüzü tanımlayarak, `Box<T>`'yi başvuru çözme operatörünü kullanmamıza olanak tanıyacak şekilde özel kılan şeyin ne olduğunu keşfedelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-our-own-smart-pointer"></a>
<a id="kendi-akıllı-işaretçimizi-tanımlamak"></a>
<a id="kendi-akilli-isaretcimizi-tanimlamak"></a>

### Kendi Akıllı İşaretçimizi Tanımlamak

Akıllı işaretçi türlerinin varsayılan olarak referanslardan nasıl farklı davrandığını deneyimlemek için standart kütüphanenin sunduğu `Box<T>` türüne benzer sarmalayıcı bir tür oluşturalım. Ardından başvuru çözme operatörünü kullanma yeteneğini nasıl ekleyeceğimize bakacağız.

> Not: Birazdan oluşturacağımız `BenimKutu<T>` türü ile gerçek `Box<T>` arasında
> büyük bir fark vardır: Bizim sürümümüz verisini öbekte saklamayacaktır. Bu
> örnekte `Deref` trait'ine odaklandığımız için, verinin gerçekte nerede
> saklandığı işaretçi benzeri davranıştan daha az önemlidir.

`Box<T>` türü özünde tek elemanlı bir demet struct'ı olarak tanımlanır; bu yüzden Liste 15-8 de `BenimKutu<T>` türünü aynı şekilde tanımlar. Ayrıca `Box<T>` üzerinde tanımlı `new` fonksiyonuna karşılık gelecek bir `yeni` fonksiyonu da tanımlayacağız.

<Listing number="15-8" file-name="src/main.rs" caption="Bir `BenimKutu<T>` türü tanımlamak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

</Listing>

Türümüzün her türden değeri tutabilmesini istediğimiz için `BenimKutu` adında bir struct tanımlıyor ve jenerik bir `T` parametresi bildiriyoruz. `BenimKutu` türü, `T` türünde tek bir elemana sahip bir demet struct'ıdır. `BenimKutu::yeni` fonksiyonu `T` türünde bir parametre alır ve aktarılan değeri tutan bir `BenimKutu` örneği döndürür.

Liste 15-7'deki `main` fonksiyonunu Liste 15-8'e eklemeyi ve `Box<T>` yerine tanımladığımız `BenimKutu<T>` türünü kullanacak şekilde değiştirmeyi deneyelim. Liste 15-9'daki kod derlenmeyecektir; çünkü Rust, `BenimKutu` üzerinde nasıl başvuru çözme yapacağını bilmez.

<Listing number="15-9" file-name="src/main.rs" caption="`BenimKutu<T>` türünü referansları ve `Box<T>`'yi kullandığımız şekilde kullanmayı denemek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

</Listing>

Ortaya çıkan derleme hatası şöyledir:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

`BenimKutu<T>` türümüz üzerinde bu yeteneği henüz uygulamadığımız için başvuru çözme işlemi yapılamaz (`type 'BenimKutu<{integer}>' cannot be dereferenced`). `*` operatörüyle başvuru çözmeyi etkinleştirmek için `Deref` trait'ini uygularız.

<!-- Old headings. Do not remove or links may break. -->

<a id="treating-a-type-like-a-reference-by-implementing-the-deref-trait"></a>
<a id="implementing-the-deref-trait"></a>
<a id="deref-traitini-uygulamak"></a>

### `Deref` Trait'ini Uygulamak

Bölüm 10'daki [“Bir Tür Üzerinde Trait Uygulama”][impl-trait]<!-- ignore --> başlığında ele aldığımız gibi, bir trait'i uygulamak için trait'in zorunlu kıldığı metotların uygulamalarını sağlamamız gerekir. Standart kütüphane tarafından sunulan `Deref` trait'i, `self`'i ödünç alan ve içteki veriye bir referans döndüren `deref` adında tek bir metodu uygulamamızı ister. Liste 15-10, `BenimKutu<T>` tanımına eklenecek `Deref` uygulamasını içerir.

<Listing number="15-10" file-name="src/main.rs" caption="`BenimKutu<T>` üzerinde `Deref` trait'ini uygulamak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

</Listing>

`type Target = T;` sözdizimi, `Deref` trait'inin kullanması için bir _ilişkili tür (associated type)_ tanımlar. İlişkili türler jenerik bir parametre bildirmenin biraz farklı bir yoludur, ancak şimdilik bunlar için endişelenmenize gerek yoktur; ilişkili türleri Bölüm 20'de daha ayrıntılı olarak ele alacağız.

`deref` metodunun `*` operatörüyle erişmek istediğimiz değere bir referans döndürmesi için metot gövdesini `&self.0` ile dolduruyoruz; Bölüm 5'teki [“Demet Struct'ları ile Farklı Türler Oluşturma”][tuple-structs]<!-- ignore --> başlığından `.0` sözdiziminin bir demet struct'ındaki ilk değere eriştiğini hatırlayın. Liste 15-9'da `BenimKutu<T>` değeri üzerinde `*` çağıran `main` fonksiyonu artık derlenir ve doğrulamalar başarıyla geçer!

`Deref` trait'i olmadan derleyici yalnızca `&` referanslarının başvurusunu çözebilir. `deref` metodu derleyiciye, `Deref` trait'ini uygulayan herhangi bir türdeki değeri alıp `deref` metodunu çağırarak nasıl çözeceğini bildiği bir `&` referansı elde etme yeteneği kazandırır.

Liste 15-9'da `*kutu` yazdığımızda, perde arkasında Rust aslında şu kodu çalıştırdı:

```rust,ignore
*(kutu.deref())
```

Rust, `deref` metodunu çağırmamız gerekip gerekmediğini düşünmek zorunda kalmamamız için `*` operatörünü önce bir `deref` metodu çağrısıyla, ardından da sıradan bir başvuru çözme işlemiyle değiştirir. Rust'ın bu özelliği, elimizde ister normal bir referans ister `Deref` uygulayan bir tür olsun, birebir aynı şekilde çalışan kodlar yazmamızı sağlar.

`deref` metodunun bir değere referans döndürmesinin ve `*(kutu.deref())` ifadesinde parantez dışındaki sıradan başvuru çözme işleminin hâlâ gerekli olmasının nedeni sahiplik sistemiyle ilgilidir. Eğer `deref` metodu değere referans yerine doğrudan değerin kendisini döndürseydi, değer `self`'in dışına taşınmış (*move*) olurdu. Bu örnekte —ve başvuru çözme operatörünü kullandığımız çoğu durumda— `BenimKutu<T>` içindeki değerin sahipliğini devralmak istemeyiz.

Kodumuzda her `*` kullandığımızda, `*` operatörünün bir `deref` metodu çağrısı ve ardından yalnızca bir kez `*` operatörü çağrısı ile değiştirildiğine dikkat edin. `*` operatörünün bu dönüşümü sonsuza dek özyineli olarak tekrarlanmadığı için sonunda Liste 15-9'daki `assert_eq!` makrosunda yer alan `5` ile eşleşen `i32` türünde veriyi elde ederiz.

> Not: `*(kutu.deref())` dönüşümünün sahiplik açısından sağladığı korumayı somut
> olarak görmek için `kutu` değişkeninin `Copy` olmayan bir `BenimKutu<String>`
> olduğunu düşünün. `kutu.deref()` çağrısı `&self` üzerinden ödünç alarak
> `&String` döndürür; dıştaki `*` operatörü ise değeri taşımak yerine bellekteki
> veriyi temsil eden bir _konum ifadesi (place expression)_ üretir. Böylece
> `&*kutu` ile referans alabilir veya içteki değerin metotlarını sahipliği
> bozmadan çağırabilirsiniz. Buna karşılık `let s = *kutu;` yazarak içteki
> `String`'i doğrudan dışarı taşımaya kalkışırsanız derleyici `error[E0507]:
> cannot move out of dereference of BenimKutu<String>` (*`BenimKutu<String>`
> başvuru çözmesinden dışarı taşıma yapılamaz*) hatasıyla akıllı işaretçinin
> bütünlüğünü korur. Liste 15-9'da `*kutu` ifadesinin doğrudan çalışmasının
> nedeni ise `i32` türünün `Copy` trait'ini uygulamasıdır; derleyici konumdan
> değeri taşımak yerine, bit düzeyinde kopyalayarak okur.
>
> Gerçek `Box<T>` ise bu kuralın derleyiciye yerleşik tek istisnasıdır: Kutu,
> içindeki değerin tek sahibi olduğundan `let s = *kutu;` yazarak değeri
> sahipliğiyle birlikte dışarı taşıyabilirsiniz. `Deref` uygulayan başka hiçbir
> tür (`Rc<T>` dahil) bu olanağı sunmaz.

<!-- Old headings. Do not remove or links may break. -->

<a id="implicit-deref-coercions-with-functions-and-methods"></a>
<a id="using-deref-coercions-in-functions-and-methods"></a>
<a id="using-deref-coercion-in-functions-and-methods"></a>
<a id="fonksiyon-ve-metotlarda-deref-zorlaması-kullanmak"></a>
<a id="fonksiyon-ve-metotlarda-deref-zorlamasi-kullanmak"></a>

### Fonksiyon ve Metotlarda Deref Zorlaması Kullanmak

_Deref zorlaması_, `Deref` trait'ini uygulayan bir türe ait referansı başka bir türe ait referansa dönüştürür. Örneğin `String` türü, `&str` döndürecek şekilde `Deref` trait'ini uyguladığı için deref zorlaması `&String`'i `&str`'ye dönüştürebilir. Deref zorlaması, Rust'ın fonksiyon ve metot argümanları üzerinde gerçekleştirdiği bir kolaylıktır ve yalnızca `Deref` trait'ini uygulayan türler üzerinde çalışır. Belirli bir türdeki değere ait referansı, fonksiyon veya metot tanımındaki parametre türüyle eşleşmeyen bir argüman olarak aktardığımızda otomatik olarak devreye girer. Arka arkaya yapılan bir dizi `deref` metodu çağrısı, sağladığımız türü parametrenin ihtiyaç duyduğu türe dönüştürür.

Deref zorlaması Rust'a, fonksiyon ve metot çağrıları yazan programcıların `&` ve `*` ile çok sayıda açık referans alma ve başvuru çözme işlemi eklemek zorunda kalmaması için dahil edilmiştir. Deref zorlaması özelliği ayrıca hem referanslarla hem de akıllı işaretçilerle çalışabilen daha fazla kod yazmamıza olanak tanır.

Deref zorlamasını çalışırken görmek için Liste 15-8'de tanımladığımız `BenimKutu<T>` türünü ve Liste 15-10'da eklediğimiz `Deref` uygulamasını kullanalım. Liste 15-11, string dilimi parametresine sahip bir fonksiyonun tanımını göstermektedir.

<Listing number="15-11" file-name="src/main.rs" caption="`&str` türünde `isim` parametresine sahip bir `merhaba` fonksiyonu">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

</Listing>

`merhaba` fonksiyonunu argüman olarak örneğin `merhaba("Rust");` şeklinde bir string dilimiyle çağırabiliriz. Deref zorlaması ise Liste 15-12'de gösterildiği gibi `merhaba` fonksiyonunu `BenimKutu<String>` türündeki bir değere referansla çağırmayı mümkün kılar.

<Listing number="15-12" file-name="src/main.rs" caption="Deref zorlaması sayesinde `merhaba` fonksiyonunu bir `BenimKutu<String>` değerine referansla çağırmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

</Listing>

Burada `merhaba` fonksiyonunu, bir `BenimKutu<String>` değerine referans olan `&kutu` argümanıyla çağırıyoruz. Liste 15-10'da `BenimKutu<T>` üzerinde `Deref` trait'ini uyguladığımız için Rust, `deref` metodunu çağırarak `&BenimKutu<String>` türünü `&String` türüne dönüştürebilir. Standart kütüphane, `String` üzerinde string dilimi döndüren bir `Deref` uygulaması sunar ve bu durum `Deref`'in API belgelerinde yer alır. Rust, `&String`'i `&str`'ye dönüştürmek için `deref` metodunu bir kez daha çağırır ve böylece `merhaba` fonksiyonunun tanımıyla eşleşme sağlanır.

Eğer Rust deref zorlamasını uygulamasaydı, `merhaba` fonksiyonunu `&BenimKutu<String>` türünde bir değerle çağırmak için Liste 15-12'deki kod yerine Liste 15-13'teki kodu yazmamız gerekirdi.

<Listing number="15-13" file-name="src/main.rs" caption="Rust'ta deref zorlaması olmasaydı yazmamız gerekecek kod">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

</Listing>

`(*kutu)` ifadesi `BenimKutu<String>` değerinin başvurusunu çözerek bir `String` elde eder. Ardından `&` ve `[..]`, `merhaba` fonksiyonunun imzasıyla eşleşmek üzere `String`'in tamamına eşit bir string dilimi alır. Deref zorlaması olmadan yazılan bu kodu tüm bu sembollerle okumak, yazmak ve anlamak daha güçtür. Deref zorlaması, Rust'ın bu dönüşümleri bizim adımıza otomatik olarak ele almasını sağlar.

İlgili türler için `Deref` trait'i tanımlandığında Rust, türleri analiz eder ve parametrenin türüyle eşleşecek bir referans elde etmek için gerektiği kadar `Deref::deref` kullanır. Kaç kez `Deref::deref` çağrısı ekleneceği derleme zamanında çözümlenir; dolayısıyla deref zorlamasından yararlanmanın hiçbir çalışma zamanı maliyeti yoktur!

> Not: Deref zorlaması yalnızca fonksiyon argümanlarında değil, metot çağrısı
> sözdiziminde de iş başındadır. `kutu.len()` yazdığınızda Rust, `len` adında
> bir metot bulana dek `BenimKutu<String>`, `String` ve `str` türlerini sırayla
> dener; bu yüzden değiştirilebilir bir `Box<Vec<i32>>` üzerinde doğrudan
> `.push(1)` çağırabilir ya da bir `Rc<String>` üzerinde `.len()`
> kullanabilirsiniz. Bölüm 5'te gördüğünüz otomatik referans alma ve başvuru
> çözme mekanizması, perde arkasında işte bu `Deref` zincirini izler.

<!-- Old headings. Do not remove or links may break. -->

<a id="how-deref-coercion-interacts-with-mutability"></a>
<a id="handling-deref-coercion-with-mutable-references"></a>

### Değiştirilebilir Referanslarla Deref Zorlamasını Ele Almak

Değiştirilemez referanslar üzerinde `*` operatörünü geçersiz kılmak için `Deref` trait'ini kullandığınız gibi, değiştirilebilir referanslar üzerinde `*` operatörünü geçersiz kılmak için de `DerefMut` trait'ini kullanabilirsiniz.

Rust, aşağıdaki üç durumda uygun türleri ve trait uygulamalarını bulduğunda deref zorlaması gerçekleştirir:

1. `T: Deref<Target=U>` olduğunda `&T`'den `&U`'ya
2. `T: DerefMut<Target=U>` olduğunda `&mut T`'den `&mut U`'ya
3. `T: Deref<Target=U>` olduğunda `&mut T`'den `&U`'ya

İlk iki durum, ikincisinin değiştirilebilirlik içermesi dışında birbiriyle aynıdır. İlk durum, elinizde bir `&T` varsa ve `T` türü bir `U` türüne `Deref` uyguluyorsa şeffaf biçimde bir `&U` elde edebileceğinizi belirtir. İkinci durum, aynı deref zorlamasının değiştirilebilir referanslar için de gerçekleştiğini ifade eder.

Üçüncü durum ise daha inceliklidir: Rust, değiştirilebilir bir referansı da değiştirilemez bir referansa zorlayarak dönüştürebilir. Ancak bunun tersi mümkün _değildir_: Değiştirilemez referanslar asla değiştirilebilir referanslara dönüştürülemez. Ödünç alma kuralları gereği, elinizde değiştirilebilir bir referans varsa bu referans söz konusu veriye olan tek referans olmak zorundadır (aksi takdirde program derlenmezdi). Tek bir değiştirilebilir referansı tek bir değiştirilemez referansa dönüştürmek ödünç alma kurallarını asla ihlal etmez. Buna karşılık değiştirilemez bir referansı değiştirilebilir bir referansa dönüştürmek, başlangıçtaki değiştirilemez referansın o veriye olan tek değiştirilemez referans olmasını gerektirirdi; oysa ödünç alma kuralları bunu garanti etmez. Bu nedenle Rust, değiştirilemez bir referansı değiştirilebilir bir referansa dönüştürmenin mümkün olduğu varsayımında bulunamaz.

> Not: Derleme zamanında sıfır çalışma zamanı maliyetiyle çözümlenen referans
> dönüşümü kurallarını aşağıdaki tabloyla özetleyebiliriz (`DerefMut` trait'i,
> üst trait olarak `Deref`'i zorunlu kılar; yani bir tür üzerinde `DerefMut`
> uygulamak için önce `Deref` uygulanmalıdır):
>
> | Kaynak Referans | Hedef Referans | Gerekli Trait Sınırı | İzin Durumu ve Nedeni |
> | --- | --- | --- | --- |
> | `&T` | `&U` | `T: Deref<Target = U>` | Geçerli — paylaşımlı referanstan paylaşımlı referansa dönüşüm |
> | `&mut T` | `&mut U` | `T: DerefMut<Target = U>` | Geçerli — tekil (münhasır) erişim korunur |
> | `&mut T` | `&U` | `T: Deref<Target = U>` | Geçerli — tekil referans güvenle paylaşımlı referansa daraltılabilir |
> | `&T` | `&mut U` | — | **Geçersiz** — aynı anda başka `&T` referansları da bulunabileceğinden ödünç alma kurallarını bozar |

[impl-trait]: ch10-02-traitler.html#implementing-a-trait-on-a-type
[tuple-structs]: ch05-01-struct-tanimlama-ve-ornekleme.html#creating-different-types-with-tuple-structs
