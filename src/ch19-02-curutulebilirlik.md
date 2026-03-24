## Çürütülebilirlik: Bir Desenin Eşleşememe İhtimali

Desenler iki ana biçime ayrılır: çürütülebilir ve çürütülemez. Her olası
değerle eşleşen desenlere _çürütülemez_ denir. Örneğin `let x = 5;`
ifadesindeki `x` böyledir; çünkü her şeyle eşleşir. Bazı olası değerlerde
eşleşmesi başarısız olabilen desenler ise _çürütülebilir_desendir. Örneğin
`if let Some(x) = bir_deger` ifadesindeki `Some(x)` böyledir; çünkü değer
`None` olursa desen eşleşmez.

Fonksiyon parametreleri, `let` ifadeleri ve `for` döngüleri yalnızca
çürütülemez desen kabul eder. Çünkü eşleşme başarısız olduğunda programın o
bağlamda anlamlı biçimde ne yapacağı belli değildir. `if let`, `while let` ve
`let...else` ise çürütülebilir desenlerle çalışmak için tasarlanmıştır.

Genelde bu ayrımı düşünmeniz gerekmez. Ama hata mesajlarında çürütülebilirlik
karşınıza çıkınca, ya deseni ya da onu kullandığınız yapıyı değiştirmeniz
gerektiğini anlamanız önemlidir.

Önce, çürütülebilir bir deseni `let` içinde kullanmaya çalıştığımız bir örneğe
bakalım. 19-8 numaralı liste bunu gösteriyor.

<Listing number="19-8" caption="`let` ile çürütülebilir desen kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

</Listing>

Eğer `bir_secenek_degeri`, `Some` yerine `None` olsaydı, `Some(x)` deseni
eşleşemezdi. Ama `let` yalnızca çürütülemez desen kabul ettiği için derleyici
hata verir:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

Sorun, `Some(x)` deseninin bütün olasılıkları kapsamamasıdır.

Eğer çürütülebilir bir desene gerçekten ihtiyacımız varsa, `let` yerine
`let...else` kullanabiliriz. Böylece desen eşleşmezse süslü parantez içindeki
kod çalışır. 19-9 numaralı liste bunu gösteriyor.

<Listing number="19-9" caption="Çürütülebilir desenlerde `let` yerine `let...else` kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

</Listing>

Bu biçimde desene bir çıkış yolu vermiş oluruz; desen eşleşmezse `else`
kolundaki kod devreye girer.

Ama bu kez de ters durum mümkündür: `let...else` içine zaten her zaman
eşleşecek bir desen koyarsanız, derleyici bunun anlamsız olduğunu söyler.
19-10 numaralı liste buna örnek:

<Listing number="19-10" caption="`let...else` ile çürütülemez desen kullanmaya çalışmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

</Listing>

Burada desen her zaman başarılı olduğu için `else` kolunun hiçbir anlamı kalmaz
ve Rust uyarı verir:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

Bu nedenle `match` kolları genelde çürütülebilir desenler kullanır; son kol ise
genellikle kalan her şeyi yakalayan çürütülemez bir desen olur.

Artık desenlerin nerede kullanılabileceğini ve çürütülebilirlik ayrımını
gördük. Sırada, desen oluştururken kullanabileceğimiz sözdizimlerinin tamamı
var.
