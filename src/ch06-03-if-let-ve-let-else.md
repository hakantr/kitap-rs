## `if let` ve `let...else` ile Özlü Kontrol Akışı

`if let` sözdizimi, geri kalanları görmezden gelirken yalnızca bir desenle (pattern) eşleşen değerleri ele almak için `if` ve `let`'i daha az ayrıntılı bir şekilde birleştirmenize olanak tanır. `max_ayari` (config_max) değişkenindeki bir `Option<u8>` değerini eşleştiren ancak yalnızca değer `Some` varyantıysa kod çalıştırmak isteyen Liste 6-6'daki programı düşünün.

<Listing number="6-6" caption="Sadece değer `Some` olduğunda kod çalıştırmayı önemseyen bir `match`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

</Listing>

Eğer değer `Some` ise, değeri desendeki `max` değişkenine bağlayarak `Some` varyantının içindeki değeri yazdırırız. `None` değeriyle hiçbir şey yapmak istemiyoruz. `match` ifadesini tatmin etmek için, yalnızca bir varyantı işledikten sonra `_ => ()` eklemek zorundayız; bu da eklenmesi can sıkıcı ve gereksiz (boilerplate) bir koddur.

Bunun yerine, bunu `if let` kullanarak daha kısa bir şekilde yazabilirdik. Aşağıdaki kod Liste 6-6'daki `match` ile aynı şekilde davranır:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

`if let` sözdizimi, eşittir işaretiyle ayrılmış bir desen ve bir ifade alır. İfadenin `match`'e verildiği ve desenin onun ilk kolu olduğu bir `match` ile aynı şekilde çalışır. Bu durumda desen `Some(max)`'tir ve `max`, `Some` içindeki değere bağlanır. Daha sonra `max`'i, ilgili `match` kolunda kullandığımız gibi `if let` bloğunun gövdesinde de kullanabiliriz. `if let` bloğundaki kod yalnızca değer desenle eşleşirse çalışır.

`if let` kullanmak daha az kod yazmak, daha az girinti (indentation) yapmak ve daha az tekrarlayan (boilerplate) kod kullanmak anlamına gelir. Ancak, `match`'in hiçbir durumu ele almayı unutmamanızı sağlayan kapsamlı denetimini kaybedersiniz. `match` ve `if let` arasında seçim yapmak, bulunduğunuz durumda ne yaptığınıza ve kısalık kazanmanın kapsamlı kontrolü kaybetmek için uygun bir takas olup olmadığına bağlıdır.

Başka bir deyişle, `if let`'i değer bir desenle eşleştiğinde kodu çalıştıran ve diğer tüm değerleri yoksayan bir `match` için pratik bir sözdizimi (syntax sugar) olarak düşünebilirsiniz.

Bir `if let` ile birlikte bir `else` de ekleyebiliriz. `else` ile birlikte giden kod bloğu, `if let` ve `else` ile eşdeğer olan `match` ifadesindeki `_` durumunda gidecek olan kod bloğuyla aynıdır. `YirmiBesKurus` (Quarter) varyantının da bir `Eyalet` (UsState) değeri tuttuğu Liste 6-4'teki `MadeniPara` (Coin) enum tanımını hatırlayın. Gördüğümüz çeyreklik olmayan tüm paraları sayarken aynı zamanda çeyrekliklerin eyaletini duyurmak isteseydik, bunu şöyle bir `match` ifadesi ile yapabilirdik:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

Veya bunun gibi bir `if let` ve `else` ifadesi kullanabilirdik:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

## `let...else` ile "Mutlu Yolda" (Happy Path) Kalmak

Yaygın bir örüntü, bir değer mevcut olduğunda bazı hesaplamalar yapmak ve aksi takdirde varsayılan bir değer döndürmektir. `Eyalet` değerine sahip madeni paralar örneğimizle devam edersek, çeyrekliğin üzerindeki eyaletin ne kadar eski olduğuna bağlı olarak komik bir şey söylemek isteseydik, `Eyalet` üzerinde bir eyaletin yaşını kontrol etmek için şöyle bir metot tanıtabilirdik:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:state}}
```

Daha sonra, Liste 6-7'de olduğu gibi koşulun gövdesi içinde bir `eyalet` (state) değişkeni tanıtarak madeni paranın türüyle eşleştirmek için `if let` kullanabiliriz.

<Listing number="6-7" caption="Bir `if let` içine yerleştirilmiş koşullar kullanarak bir eyaletin 1900'de var olup olmadığını kontrol etme">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:describe}}
```

</Listing>

Bu işi halleder, ancak işi `if let` ifadesinin gövdesine iter ve yapılacak iş daha karmaşıksa, üst düzey dalların (branches) birbiriyle tam olarak nasıl ilişkili olduğunu takip etmek zor olabilir. İfadelerin bir değer ürettiği gerçeğinden yararlanarak, Liste 6-8'de olduğu gibi `if let`'ten `eyalet`'i (state) üretebilir veya erken dönüş (return early) yapabiliriz. (Buna benzer bir şeyi `match` ile de yapabilirsiniz.)

<Listing number="6-8" caption="Bir değer üretmek veya erken dönmek için `if let` kullanmak">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-08/src/main.rs:describe}}
```

</Listing>

Yine de bunu takip etmek kendine göre biraz can sıkıcı! `if let`'in bir dalı bir değer üretir ve diğeri fonksiyondan tamamen geri döner.

Bu yaygın kalıbı (pattern) daha güzel ifade etmek için Rust'ta `let...else` vardır. `let...else` sözdizimi, `if let`'e çok benzer şekilde sol tarafta bir desen ve sağ tarafta bir ifade alır, ancak bir `if` dalı yoktur, yalnızca bir `else` dalı vardır. Eğer desen eşleşirse, desendeki değeri dış kapsama (outer scope) bağlar. Eğer desen eşleşmezse, program fonksiyondan dönmesi _gereken_ `else` koluna akacaktır.

Liste 6-9'da, `if let` yerine `let...else` kullanıldığında Liste 6-8'in nasıl göründüğünü görebilirsiniz.

<Listing number="6-9" caption="Fonksiyon boyunca akışı netleştirmek için `let...else` kullanmak">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-09/src/main.rs:describe}}
```

</Listing>

`if let`'in yaptığı gibi iki dal için önemli ölçüde farklı kontrol akışlarına sahip olmadan, fonksiyonun ana gövdesindeki "mutlu yolda" (happy path) kaldığına dikkat edin.

Programınızın `match` kullanarak ifade edilemeyecek kadar uzun veya karmaşık bir mantığa sahip olduğu bir durumla karşılaşırsanız, `if let` ve `let...else` yapılarının da Rust alet çantanızda (toolbox) olduğunu unutmayın.

## Özet

Şimdi, numaralandırılmış (enumerated) değerler kümesinden biri olabilen özel türler oluşturmak için enum'ların nasıl kullanılacağını ele aldık. Standart kütüphanenin `Option<T>` türünün, hataları önlemek için tür sistemini kullanmanıza nasıl yardımcı olduğunu gösterdik. Enum değerlerinin içinde veriler olduğunda, ele almanız gereken durumların sayısına bağlı olarak bu değerleri çıkarmak ve kullanmak için `match` veya `if let` kullanabilirsiniz.

Rust programlarınız artık alanınızdaki (domain) kavramları struct'lar ve enum'lar kullanarak ifade edebilir. API'nizde kullanmak üzere özel türler (custom types) oluşturmak tip güvenliği (type safety) sağlar: Derleyici, fonksiyonlarınızın yalnızca her fonksiyonun beklediği türde değerler almasını garanti eder.

Kullanıcılarınıza kullanımı kolay ve yalnızca tam olarak ihtiyaç duyacakları şeyleri ortaya çıkaran iyi organize edilmiş bir API sağlamak için şimdi Rust'ın modüllerine (modules) dönelim.
