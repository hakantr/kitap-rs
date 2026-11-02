<!-- Old headings. Do not remove or links may break. -->
<a id="refutability-whether-a-pattern-might-fail-to-match"></a>
<a id="çürütülebilirlik-bir-desenin-eşleşememe-ihtimali"></a>
<a id="çürütülebilirlik-bir-desenin-eşleşmeme-ihtimali"></a>

## Çürütülebilirlik (Refutability): Bir Desenin Eşleşmeme İhtimali

Desenler iki biçimde karşımıza çıkar: çürütülebilir ve çürütülemez. Aktarılan
her olası değerle eşleşecek olan desenler _çürütülemez_ (*irrefutable*)
desenlerdir. Buna bir örnek, `let x = 5;` deyimindeki `x` desenidir; çünkü `x`
her şeyle eşleşir ve dolayısıyla eşleşmemesi mümkün değildir. Bazı olası
değerler için eşleşmesi başarısız olabilen desenler ise _çürütülebilir_
(*refutable*) desenlerdir. Buna örnek olarak `if let Some(x) = bir_deger`
ifadesindeki `Some(x)` verilebilir; çünkü `bir_deger` değişkenindeki değer
`Some` yerine `None` olursa `Some(x)` deseni eşleşmeyecektir.

Fonksiyon parametreleri, `let` deyimleri ve `for` döngüleri yalnızca çürütülemez
desenleri kabul edebilir; çünkü değerler eşleşmediğinde programın yapabileceği
anlamlı hiçbir şey yoktur. `if let` ve `while let` ifadeleri ile `let...else`
deyimi ise hem çürütülebilir hem de çürütülemez desenleri kabul eder; ancak
derleyici çürütülemez desenlere karşı uyarı verir. Çünkü bu yapılar tanım gereği
olası başarısızlık durumlarını ele almak için tasarlanmıştır: Bir koşul
yapısının işlevselliği, başarı ya da başarısızlık durumuna göre farklı
davranabilme yeteneğinde yatar.

Genel olarak çürütülebilir ve çürütülemez desenler arasındaki ayrım konusunda
endişelenmenize gerek yoktur; ancak bir hata mesajında gördüğünüzde ne
yapacağınızı bilmek için çürütülebilirlik kavramına aşina olmanız gerekir. Böyle
durumlarda, kodun amaçlanan davranışına bağlı olarak ya deseni ya da deseni
birlikte kullandığınız dil yapısını değiştirmeniz gerekecektir.

Şimdi Rust'ın çürütülemez bir desen zorunlu kıldığı yerde çürütülebilir bir
desen kullanmaya çalıştığımızda — ve bunun tersi durumda — ne olduğuna bir
örnekle bakalım. Liste 19-8 bir `let` deyimi göstermektedir, ancak desen olarak
çürütülebilir bir desen olan `Some(x)` belirtilmiştir. Tahmin edebileceğiniz
gibi bu kod derlenmeyecektir.

<Listing number="19-8" caption="`let` ile çürütülebilir bir desen kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

</Listing>

Eğer `bir_secenek_degeri` bir `None` değeri olsaydı `Some(x)` deseniyle
eşleşemezdi; bu da desenin çürütülebilir olduğu anlamına gelir. Oysa `let`
deyimi yalnızca çürütülemez bir deseni kabul edebilir; çünkü bir `None`
değeriyle kodun yapabileceği geçerli hiçbir işlem yoktur. Derleme zamanında Rust
derleyicisi, çürütülemez bir desenin gerektiği yerde çürütülebilir bir desen
kullanmaya çalıştığımızı belirterek hata verir:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

`Some(x)` deseniyle her geçerli değeri kapsamadığımız (ve zaten
kapsayamayacağımız!) için Rust haklı olarak bir derleyici hatası üretir.

Çürütülemez bir desenin gerektiği yerde elimizde çürütülebilir bir desen varsa,
bunu deseni kullanan kodu değiştirerek düzeltebiliriz: `let` kullanmak yerine
`let...else` kullanabiliriz. Böylece desen eşleşmezse, süslü parantezler
içindeki kod bu durumu ele alır. Liste 19-9, Liste 19-8'deki kodun nasıl
düzeltileceğini göstermektedir.

<Listing number="19-9" caption="Çürütülebilir desenlerle `let` yerine `let...else` ve bir blok kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

</Listing>

> Not: `let...else` deyimindeki `else` bloğu **ıraksayan** (*diverging*, dönüş
> türü `!` — *never type* olan) bir blok olmak zorundadır; yani `return`,
> `break`, `continue` veya `panic!` gibi kontrol akışını mevcut kapsamdan
> çıkaran bir ifadeyle sonlanmalıdır. Normal akışın `else` bloğundan çıkıp alt
> satırlara devam etmesine izin verilmez; böylece `let...else` satırından sonra
> gelen dış kapsamda `x` değişkeninin her zaman ilklenmiş olması derleme
> zamanında garanti edilir.

Koda bir çıkış yolu verdik! Bu kod tamamen geçerlidir; ancak bu aynı zamanda,
bir uyarı almadan çürütülemez bir desen kullanamayacağımız anlamına gelir.
Liste 19-10'da gösterildiği gibi `let...else` yapısına `x` gibi her zaman
eşleşecek bir desen verirsek derleyici bir uyarı üretecektir.

<Listing number="19-10" caption="`let...else` ile çürütülemez bir desen kullanmaya çalışmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

</Listing>

Rust derleyicisi, `else` bloğuna hiçbir zaman ulaşılamayacağı için `let...else`
yapısını çürütülemez bir desenle kullanmanın mantıklı olmadığı konusunda uyarır:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

Bu nedenle `match` kolları — geriye kalan tüm değerlerle çürütülemez bir desen
kullanarak eşleşmesi gereken son kol dışında — çürütülebilir desenler kullanmak
zorundadır. Rust, tek kollu bir `match` ifadesinde çürütülemez bir desen
kullanmamıza izin verir; ancak bu sözdizimi pek kullanışlı değildir ve daha
basit bir `let` deyimiyle değiştirilebilir.

Artık desenlerin nerelerde kullanılacağını ve çürütülebilir ile çürütülemez
desenler arasındaki farkı bildiğinize göre, desen oluşturmak için
kullanabileceğimiz tüm sözdizimlerini ele alalım.
