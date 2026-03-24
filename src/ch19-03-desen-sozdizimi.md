## Desen Sözdizimi

Bu bölümde desenlerde geçerli olan temel sözdizimlerini bir araya toplayacağız
ve her birinin ne zaman işe yaradığını göreceğiz.

### Sabit Değerlerle Eşleşmek

6. bölümde gördüğünüz gibi, desenleri sabit değerlerle doğrudan eşleştirebiliriz:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

Bu kod `bir` yazar; çünkü `deger` içindeki sayı `1`'dir. Belirli somut
değerlere göre farklı davranmak istediğinizde bu sözdizimi çok kullanışlıdır.

### Adlandırılmış Değişkenlerle Eşleşmek

Adlandırılmış değişkenler çürütülemez desenlerdir ve her değerle eşleşir.
Kitap boyunca onları çok kullandık. Ama `match`, `if let` ve `while let`
içinde kullanırken dikkat edilmesi gereken bir nokta vardır: Bu yapılar yeni
bir kapsam başlatır. Dolayısıyla desenin içinde tanımlanan değişkenler, dışarıda
aynı ada sahip değişkenleri gölgeler.

19-11 numaralı liste bunun tipik örneğidir.

<Listing number="19-11" file-name="src/main.rs" caption="Var olan `y` değişkenini gölgeleyen yeni değişken tanımlayan `match` kolu">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-11/src/main.rs:here}}
```

</Listing>

Burada dışarıda `y = 10` vardır. Ama `match` içindeki `Some(y)` deseni, dışarıdaki
`y`yi kullanmaz; yeni bir `y` üretir. Bu yeni değişken `Some` içindeki değere
bağlanır. Bu yüzden `Some(5)` durumunda yazılan çıktı `Eşleşti, y = 5` olur;
ama `match` bittikten sonra dışarıdaki `y` yine `10` olarak kalır.

Eğer dışarıdaki `y` ile karşılaştırma yapmak istiyorsak, desen içinde aynı adı
yeniden tanımlamak yerine match guard kullanmamız gerekir. Bunu birazdan
göreceğiz.

### Birden Fazla Deseni Eşleştirmek

`match` içinde `|` kullanarak birden fazla desen yazabilirsiniz. Bu, desenler
arasında _veya_ anlamına gelir:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

Bu kod `bir ya da iki` yazar.

### `..=` ile Değer Aralıklarını Eşleştirmek

`..=` sözdizimi, kapsayıcı aralıklarla eşleşmemizi sağlar:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

Burada değer `1` ile `5` arasındaysa ilk kol çalışır. Bunu `1 | 2 | 3 | 4 | 5`
yerine yazmak çok daha kısadır.

Rust, aralığın boş olup olmadığını yalnızca sayısal türler ve `char` için
derleme zamanında anlayabildiği için, aralık desenleri yalnız bu türlerle
kullanılabilir. `char` örneği şöyle görünür:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

Burada `'c'`, ilk aralığa düştüğü için `erken ASCII harfi` yazılır.

### Ayrıştırarak Değeri Parçalarına Bölmek

Desenleri struct, enum ve demetleri parçalara ayırmak için de kullanabiliriz.

#### Struct'lar

19-12 numaralı listede, `Nokta` struct'ının alanlarını `let` ile ayrı
değişkenlere ayırıyoruz.

<Listing number="19-12" file-name="src/main.rs" caption="Bir struct alanlarını ayrı değişkenlere ayırmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-12/src/main.rs}}
```

</Listing>

Burada alan adları ile desen içindeki değişken adları aynı olmak zorunda
değildir. Ama sık kullanılan kısa biçimde, aynı adı iki kez yazmadan struct alan
adı doğrudan kullanılır. 19-13 numaralı listedeki sürüm aynı işi daha kısa
yapar.

<Listing number="19-13" file-name="src/main.rs" caption="Struct alanı kısaltması kullanarak ayrıştırmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-13/src/main.rs}}
```

</Listing>

Desenin içinde bazı alanları sabit değerle eşleştirip bazılarını değişkene de
bağlayabilirsiniz. 19-14 numaralı liste bunu gösteriyor.

<Listing number="19-14" file-name="src/main.rs" caption="Aynı desende hem ayrıştırmak hem sabit değerlerle eşleştirmek">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-14/src/main.rs:here}}
```

</Listing>

Burada noktanın `x` ekseni üzerinde, `y` ekseni üzerinde ya da hiçbir eksende
olmaması farklı kollarda ele alınıyor.

#### Enum'lar

Kitap boyunca enum ayrıştırması yaptık; ama şimdi bunu açıkça adlandırıyoruz.
Bir enum varyantını ayrıştıran desen, o varyantın nasıl tanımlandığına karşılık
gelir. 19-15 numaralı listede `Mesaj` enum'u üzerinde bunu görüyoruz.

<Listing number="19-15" file-name="src/main.rs" caption="Farklı türlerde değer taşıyan enum varyantlarını ayrıştırmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-15/src/main.rs}}
```

</Listing>

`Cik` gibi veri taşımayan varyantlarda daha fazla ayrıştırma yapamayız.
Struct benzeri varyantlarda süslü parantezli, demet benzeri varyantlarda ise
demet desenlerine benzeyen sözdizimi kullanırız.

#### İç İçe Geçmiş Struct ve Enum'lar

Desenler bir seviyeden daha derine de inebilir. 19-16 numaralı listede `Mesaj`
içindeki `Renk` enum'unu ayrıştırıyoruz.

<Listing number="19-16" caption="İç içe enum'lar üzerinde eşleştirme yapmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-16/src/main.rs}}
```

</Listing>

Burada tek bir `match` içinde hem dış enum varyantını hem de onun içindeki enum
değerini ayırabiliyoruz.

#### Struct ve Demetleri Karıştırmak

Ayrıştırma desenleri daha da karmaşık biçimde iç içe kullanılabilir:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

Bu tür desenler, yalnız ilgilendiğiniz parçaları elde edip geri kalanı bir
arada bırakmamak için çok kullanışlıdır.

### Desende Değerleri Yoksaymak

Bazen desendeki bazı değerler ilgimizi çekmez. Bunun için birkaç yol vardır:
tek başına `_`, desende iç içe `_`, başında alt çizgi olan adlar ve `..`.

#### Tüm Bir Değeri `_` ile Yoksaymak

`_`, her şeyle eşleşen ama değeri bağlamayan joker desendir. 19-17 numaralı
liste fonksiyon parametresinde kullanımını gösteriyor.

<Listing number="19-17" file-name="src/main.rs" caption="Fonksiyon imzasında `_` kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-17/src/main.rs}}
```

</Listing>

Bu kod ilk parametreyi tamamen yok sayar ve yalnızca `y` değerini kullanır.

#### İç İçe `_` ile Değerin Bir Parçasını Yoksaymak

Bir değerin yalnızca bazı parçalarını önemsiyorsanız, `_`yi başka desenlerin
içinde de kullanabilirsiniz. 19-18 numaralı listedeki örnek bunu gösteriyor.

<Listing number="19-18" caption="`Some` içindeki gerçek değeri kullanmadan, yalnızca varyantın kendisiyle eşleşmek için `_` kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-18/src/main.rs:here}}
```

</Listing>

Burada `ayar_degeri` ile `yeni_ayar_degeri` ikisi de `Some` ise, içlerindeki
gerçek sayıları umursamıyoruz; yalnızca ikisinin de `Some` olmasını önemsiyoruz.

19-19 numaralı liste ise bir demette birden fazla öğeyi yoksaymayı gösteriyor.

<Listing number="19-19" caption="Bir demetin birden fazla parçasını yoksaymak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-19/src/main.rs:here}}
```

</Listing>

#### Başına `_` Koyarak Kullanılmayan Değişkeni Susturmak

Bir değişken oluşturup henüz kullanmıyorsanız Rust normalde uyarı verir.
Ama değişken adını `_` ile başlatırsanız bu uyarıyı susturabilirsiniz.
19-20 numaralı liste bunu gösteriyor.

<Listing number="19-20" file-name="src/main.rs" caption="Kullanılmayan değişken uyarısını önlemek için adın başına `_` koymak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-20/src/main.rs}}
```

</Listing>

Burada `_x` için uyarı gelmez, ama `y` için gelir.

Fakat tek başına `_` ile `_ad` arasında önemli fark vardır: `_ad` yine de
değeri bağlar; `_` ise bağlamaz. Bu fark sahiplik açısından önemlidir.
19-21 numaralı liste bunu gösterir.

<Listing number="19-21" caption="Başında `_` olan kullanılmayan değişken yine de değeri bağlar ve sahipliği alabilir">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-21/src/main.rs:here}}
```

</Listing>

Burada `s` değeri `_s` içine taşındığı için daha sonra yeniden kullanılamaz.
Buna karşılık 19-22 numaralı listedeki gibi yalnızca `_` kullanırsanız değer
bağlanmaz:

<Listing number="19-22" caption="Tek başına `_` kullanmak değeri bağlamaz">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-22/src/main.rs:here}}
```

</Listing>

#### `..` ile Kalan Parçaları Yoksaymak

Bir değerin pek çok parçası varsa, `..` ile kalan her şeyi topluca
yoksayabilirsiniz. 19-23 numaralı liste buna örnektir.

<Listing number="19-23" caption="`Nokta` içindeki yalnızca `x` alanını kullanıp geri kalanını `..` ile yoksaymak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-23/src/main.rs:here}}
```

</Listing>

Burada `x` alanı alınır; `y` ile `z` ise `..` sayesinde tek tek yazılmadan
yoksayılır.

`..`, demetlerde de çalışır:

<Listing number="19-24" file-name="src/main.rs" caption="Bir demette ilk ve son değeri alıp diğerlerini yoksaymak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-24/src/main.rs}}
```

</Listing>

Ama `..` kullanımı belirsiz olmamalıdır. 19-25 numaralı listedeki gibi iki
tarafa birden yaymaya çalışırsanız derleyici hangi öğelerin gerçekten
eşleşeceğini anlayamaz:

<Listing number="19-25" file-name="src/main.rs" caption="`..` sözdizimini belirsiz biçimde kullanma girişimi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-25/src/main.rs}}
```

</Listing>

Bu durumda derleme hatası alırsınız:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-25/output.txt}}
```

### Match Guard ile Ek Koşul Eklemek

_Match guard_, `match` kolundaki desenden sonra yazılan ek `if` koşuludur.
Desen eşleşse bile, kolun seçilebilmesi için bu koşulun da sağlanması gerekir.

19-26 numaralı listede ilk kolun deseni `Some(x)`, guard'ı ise
`if x % 2 == 0` koşuludur.

<Listing number="19-26" caption="Bir desene match guard eklemek">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-26/src/main.rs:here}}
```

</Listing>

Bu örnek `4 sayısı çifttir` yazar. Eğer değer `Some(5)` olsaydı, ilk desen
eşleşse bile guard yanlış olduğu için ikinci kola düşerdi.

Match guard'ın önemli bir kullanımı, gölgelenme sorununu çözmektir.
19-11'de dışarıdaki `y`yi kullanmak istememize rağmen içteki desen onu
gölgeliyordu. 19-27 numaralı liste, bunu guard ile çözüyor.

<Listing number="19-27" file-name="src/main.rs" caption="Dışarıdaki değişkenle eşitliği denemek için match guard kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-27/src/main.rs}}
```

</Listing>

Burada `Some(n) if n == y` deseni, yeni bir `n` tanımlar; ama dışarıdaki `y`
aynı kalır. Böylece gerçekten dış `y` ile karşılaştırma yapabiliriz.

`|` ile birden fazla deseni bir guard ile birleştirdiğinizde, guard hepsine bir
arada uygulanır. 19-28 numaralı liste bunu gösteriyor.

<Listing number="19-28" caption="Birden fazla deseni tek guard ile birleştirmek">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-28/src/main.rs:here}}
```

</Listing>

Buradaki guard, yalnızca `6`ya değil, tüm `4 | 5 | 6` grubuna uygulanır.

### `@` Bağlamaları Kullanmak

`@` operatörü, bir değeri hem desene göre sınayıp hem de o anda bir değişkene
bağlamamızı sağlar. 19-29 numaralı liste bunun örneğidir.

<Listing number="19-29" caption="Bir değeri hem desenle sınayıp hem `@` ile değişkene bağlamak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-29/src/main.rs:here}}
```

</Listing>

Burada `id @ 3..=7`, `id` alanının `3..=7` aralığında olup olmadığını test eder
ve eşleşirse aynı değeri `id` değişkenine bağlar. Böylece hem test hem bağlama
tek desende yapılmış olur.

## Özet

Rust'taki desenler, farklı veri biçimlerini ayırt etmede son derece güçlüdür.
`match` ile kullanıldıklarında Rust, bütün olası değerleri ele almanızı
zorunlu kılar; aksi halde program derlenmez. `let` ifadeleri ve fonksiyon
parametrelerindeki desenler ise değerleri küçük parçalara ayırıp bu parçaları
ayrı değişkenlere bağlamanızı sağlar.

İhtiyacınıza göre çok basit ya da oldukça karmaşık desenler kurabilirsiniz.

Sıradaki bölümde, kitabın sondan bir önceki durağında, Rust özelliklerinin
çeşitli gelişmiş yönlerine bakacağız.
