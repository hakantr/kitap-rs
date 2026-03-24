<!-- Old headings. Do not remove or links may break. -->

<a id="treating-smart-pointers-like-regular-references-with-the-deref-trait"></a>
<a id="treating-smart-pointers-like-regular-references-with-deref"></a>

## Akıllı İşaretçilere Normal Referanslar Gibi Davranmak

`Deref` trait'ini uygulamak, `*` _başvuru çözme operatörünün (dereference
operator)_ davranışını özelleştirmenizi sağlar. Bu trait'i, akıllı işaretçi
normal referans gibi davranacak şekilde uygularsanız, referanslarla çalışan
kodu akıllı işaretçilerle de kullanabilirsiniz.

Önce başvuru çözme operatörünün normal referanslarla nasıl çalıştığına bakalım.
Ardından `Box<T>` gibi davranan özel bir tür tanımlamayı deneyeceğiz ve neden
başvuru çözmenin referanslardaki gibi çalışmadığını göreceğiz. Sonra `Deref`
trait'ini uygulamanın akıllı işaretçileri referanslara benzer biçimde
çalıştırmayı nasıl mümkün kıldığına bakacağız. En sonda da Rust'ın _deref
zorlama (deref coercion)_ özelliğini ve bunun referanslarla akıllı işaretçileri
birlikte nasıl kullanmamızı kolaylaştırdığını göreceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>
<a id="following-the-pointer-to-the-value"></a>

### Referansın Göstediği Değere Gitmek

Normal referans bir işaretçi türüdür. İşaretçiyi düşünmenin kolay yollarından
biri, başka yerde tutulan bir değere uzanan ok gibi görmektir. Liste 15-6'da
bir `i32` değerine referans oluşturuyor ve başvuru çözme operatörüyle bu
referansın gösterdiği değere gidiyoruz.

<Listing number="15-6" file-name="src/main.rs" caption="Bir `i32` degerine giden referansi, basvuru cozme operatoruyle izlemek">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

</Listing>

`sayi` değişkeni `5` değerini tutar. `referans`ı `sayi`ya referans olacak
şekilde ayarlarız. `sayi`nın `5` olduğuna dair doğrulama yapabiliriz. Ama
`referans` içindeki değeri doğrulamak istersek, `*referans` yazarak referansın
işaret ettiği değere gitmemiz gerekir; böylece derleyici gerçek değerleri
karşılaştırabilir.

`assert_eq!(5, referans);` yazsaydık derleme hatası alırdık:

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

Sayı ile sayıya referans farklı türler olduğu için doğrudan karşılaştırılamaz.
Referansın işaret ettiği değere gitmek için başvuru çözme operatörünü
kullanmalıyız.

### `Box<T>`yi Referans Gibi Kullanmak

Liste 15-6'daki kodu, referans yerine `Box<T>` kullanacak biçimde
yeniden yazabiliriz. Liste 15-7'de kutu üzerinde kullanılan başvuru çözme
operatörü, Liste 15-6'daki referans üzerinde kullanılanla aynı şekilde çalışır.

<Listing number="15-7" file-name="src/main.rs" caption="`Box<i32>` uzerinde basvuru cozme operatorunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

</Listing>

Fark şu: bu kez `kutu`, `sayi`nın kopyalanmış değerini işaret eden bir kutudur.
Son doğrulamada `*kutu` ile kutunun gösterdiği değere, referansta yaptığımızın
aynı şekilde gidebiliyoruz. Şimdi bunun neden mümkün olduğunu görmek için kendi
kutu türümüzü tanımlayalım.

### Kendi Akıllı İşaretçimizi Tanımlamak

Standart kütüphanenin sunduğu `Box<T>`ye benzer sarmalayıcı bir tür oluşturalım.
Amaç, akıllı işaretçilerin varsayılan olarak referanslardan nasıl farklı
davrandığını deneyimlemek. Ardından başvuru çözmeyi nasıl ekleyebileceğimizi
göreceğiz.

> Not: Birazdan oluşturacağımız `BenimKutu<T>` ile gerçek `Box<T>` arasında
> önemli bir fark var: bizim sürümümüz veriyi öbekte tutmayacak. Burada odak
> `Deref` olduğu için verinin nerede saklandığı değil, işaretçi gibi davranışı
> önemli.

`Box<T>` aslında tek öğeli bir demet struct'ıdır; Liste 15-8 de aynı mantıkla
`BenimKutu<T>` türünü tanımlar. `Box<T>`deki `new` fonksiyonuna benzeyen bir
`yeni` fonksiyonu da tanımlıyoruz.

<Listing number="15-8" file-name="src/main.rs" caption="`BenimKutu<T>` turunu tanimlamak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

</Listing>

`BenimKutu` adında bir struct tanımlayıp jenerik `T` parametresi ekliyoruz;
çünkü her türden değeri tutabilsin istiyoruz. `BenimKutu::yeni`, `T` türünden
bir parametre alır ve bu değeri saklayan `BenimKutu` döndürür.

Şimdi Liste 15-7'deki `main` fonksiyonunu Liste 15-8'e ekleyip `Box<T>`
yerine `BenimKutu<T>` kullanalım. Liste 15-9 derlenmez; çünkü Rust `BenimKutu`
üzerinde başvuru çözmenin nasıl yapılacağını bilmiyor.

<Listing number="15-9" file-name="src/main.rs" caption="`BenimKutu<T>`yi referans ve `Box<T>` gibi kullanmaya calismak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

</Listing>

Hata şöyle olur:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

`BenimKutu<T>` üzerinde bu yeteneği tanımlamadığımız için başvuru çözülemez.
Bunu sağlamak için `Deref` trait'ini uygulamalıyız.

<!-- Old headings. Do not remove or links may break. -->

<a id="treating-a-type-like-a-reference-by-implementing-the-deref-trait"></a>

### `Deref` Trait'ini Uygulamak

10. bölümdeki [“Bir Tür Üzerinde Trait Uygulamak”][impl-trait]<!-- ignore -->
başlığında gördüğümüz gibi, trait uygulamak için gerekli yöntemleri yazmamız
gerekir. Standart kütüphanedeki `Deref` trait'i, `self`i ödünç alan ve içteki
veriye referans döndüren `deref` adlı tek bir yöntem ister. Liste 15-10 bu
uygulamayı gösterir.

<Listing number="15-10" file-name="src/main.rs" caption="`BenimKutu<T>` uzerinde `Deref` uygulamak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

</Listing>

`type Target = T;` sözdizimi, `Deref` trait'inin kullanacağı ilişkili türü
tanımlar. İlişkili türler jenerik parametre bildirmeye benzer ama biraz farklı
bir mekanizmadır; ayrıntısını 20. bölümde göreceğiz.

`deref` yönteminin gövdesine `&self.0` yazıyoruz; böylece `*` ile erişmek
istediğimiz değere referans dönüyor. 5. bölümdeki demet struct'larında olduğu
gibi `.0`, ilk alanı seçer. Artık Liste 15-9'daki `*kutu` kullanan `main`
derlenir ve doğrulamalar geçer.

`Deref` olmadan derleyici yalnızca `&` referanslarını çözebilir. `deref`
yöntemi, `Deref` uygulayan herhangi bir türü alıp referansa çevirebilme
yeteneği kazandırır.

Liste 15-9'da `*kutu` yazdığımızda, arka planda Rust şu kodu çalıştırır:

```rust,ignore
*(kutu.deref())
```

Rust, `*` operatörünü `deref` çağrısıyla ve ardından normal başvuru çözmeyle
değiştirir; böylece bunu her seferinde elle düşünmemiz gerekmez.

`deref`in doğrudan değeri değil de referans döndürmesinin sebebi sahiplik
sistemidir. Değeri doğrudan döndürseydi, değer `self`in içinden taşınmış olurdu.
Biz çoğu durumda akıllı işaretçinin içindeki değerin sahipliğini almak istemeyiz.

### Fonksiyon ve Metotlarda Deref Zorlaması Kullanmak

_Deref zorlama_, `Deref` trait'ini uygulayan bir türe ait referansı başka bir
türe ait referansa dönüştürür. Örneğin `String`, `Deref` uygulayıp `&str`
döndürdüğü için `&String`, `&str`e dönüştürülebilir. Bu özellik fonksiyon ve
metot çağrılarında, verdiğiniz argüman türü parametre türüyle tam eşleşmediğinde
otomatik çalışır.

Bu özellik, fonksiyon ve metot çağrısı yazarken `&` ve `*` ile açıkça referans
alma ve çözme ihtiyacını azaltır. Ayrıca hem referanslarla hem akıllı
işaretçilerle çalışabilen daha esnek kod yazmanızı sağlar.

Bunu görmek için, Liste 15-8'de tanımladığımız `BenimKutu<T>` ve Liste 15-10'da
eklediğimiz `Deref` uygulamasını kullanalım. Liste 15-11, parametresi `&str`
olan `merhaba` fonksiyonunu gösterir.

<Listing number="15-11" file-name="src/main.rs" caption="Parametresi `&str` olan `merhaba` fonksiyonu">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

</Listing>

`merhaba("Rust");` gibi bir çağrı zaten çalışır. Ama deref zorlama sayesinde
`BenimKutu<String>` referansı da geçebiliriz; Liste 15-12 bunu gösterir.

<Listing number="15-12" file-name="src/main.rs" caption="Deref zorlama sayesinde `merhaba`yi `BenimKutu<String>` referansiyla cagirmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

</Listing>

Burada `merhaba`ya `&kutu` geçiriyoruz; bu bir `BenimKutu<String>` referansıdır.
`BenimKutu<T>` üzerinde `Deref` uyguladığımız için Rust bunu önce `&String`e
çevirir. Standart kütüphane de `String` için `Deref` uygulayıp `&str`
döndürdüğünden, ikinci kez `deref` çağrısı yapılarak `&str` elde edilir.
Sonuçta `merhaba`nın beklediği türle eşleşmiş oluruz.

Rust'ta deref zorlama olmasaydı, Liste 15-12 yerine Liste 15-13'teki gibi
açık kod yazmamız gerekirdi:

<Listing number="15-13" file-name="src/main.rs" caption="Rust'ta deref zorlama olmasaydi yazmamiz gerekecek kod">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

</Listing>

`(*kutu)[..]` ifadesi hem `BenimKutu<String>`yi `String`e çözer hem de tam
dizgiyi kapsayan bir dizgi dilimi üretir. Deref zorlama bu işi bizim yerimize
otomatik yaptığı için kod daha kısa ve okunur olur.

Rust üç durumda deref zorlama yapar:

- `T: Deref<Target=U>` ise `&T`, `&U`ye çevrilir.
- `T: DerefMut<Target=U>` ise `&mut T`, `&mut U`ye çevrilir.
- `T: Deref<Target=U>` ise `&mut T`, `&U`ye de çevrilebilir.

Üçüncü durum, değiştirilebilir referansın değiştirilemez referans da olabilmesi
nedeniyle mümkündür. Tersi doğru değildir: değiştirilemez referansla
değiştirilebilir referans elde edilemez.
