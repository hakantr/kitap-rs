## Desenlerin Kullanılabildiği Tüm Yerler

Desenler Rust'ta pek çok yerde karşımıza çıkar ve farkında olmadan onları sıkça
kullanmışsınızdır. Bu bölüm, desenlerin geçerli olduğu yerleri tek tek ele alır.

### `match` Kolları

6. bölümde konuştuğumuz gibi, `match` ifadelerinin kollarında desen kullanırız.
Biçimsel olarak bir `match` ifadesi, `match` anahtar sözcüğü, eşleştirilecek bir
değer ve her biri bir desen ile o desene uyunca çalışacak ifadeden oluşan bir
veya daha fazla koldan oluşur:

<pre><code>match <em>DEGER</em> {
    <em>DESEN</em> => <em>IFADE</em>,
    <em>DESEN</em> => <em>IFADE</em>,
    <em>DESEN</em> => <em>IFADE</em>,
}</code></pre>

Örneğin 6. bölümdeki `Option<i32>` eşleştirmesinde desenler `None` ve `Some(i)`
idi. Okun solunda kalan bu kısımlar, her kolun desen kısmıdır.

`match` ifadelerinin önemli bir şartı, _kapsamlı_ olmalarıdır. Yani `match`
edilen değer için bütün olasılıklar ele alınmış olmalıdır. Bunu sağlamanın bir
yolu, son kola her şeyi yakalayan bir desen koymaktır. Herhangi bir değerle
eşleşen değişken adı desenleri buna örnek verilebilir.

Özel `_` deseni de her şeyle eşleşir; ama hiçbir değeri bağlamaz. Bu yüzden
özellikle son kolda sık kullanılır. `_` desenini bölümün ilerleyen kısmında
daha ayrıntılı göreceğiz.

### `let` İfadeleri

Bu bölüme kadar desenlerin `match` ve `if let` içinde kullanıldığını açıkça
konuşmuştuk; ama aslında `let` ifadelerinde de desen kullanıyoruz. Örneğin şu
çok sıradan görünen atama bile bir desen içerir:

```rust
let x = 5;
```

Biçimsel olarak bir `let` ifadesi şöyledir:

<pre><code>let <em>DESEN</em> = <em>IFADE</em>;</code></pre>

Buradaki `x`, "buraya gelen değeri `x` adına bağla" diyen basit bir desendir.

Desen eşleştirme yönünü daha net görmek için 19-1 numaralı listeye bakalım.
Burada `let`, bir demeti ayrıştırmak için kullanılıyor.

<Listing number="19-1" caption="Bir demeti ayrıştırmak ve aynı anda üç değişken oluşturmak için desen kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs:here}}
```

</Listing>

Burada `(1, 2, 3)` değeri `(birinci, ikinci, ucuncu)` desenine karşı
eşleştiriliyor. Öğe sayısı aynı olduğu için eşleşme başarılı oluyor ve sırasıyla
`1`, `2`, `3` değerleri bu adlara bağlanıyor.

Eğer desendeki öğe sayısı ile değerdeki öğe sayısı uyuşmazsa, türler de
uyuşmaz ve derleyici hata verir. 19-2 numaralı liste bunun başarısız örneğini
gösteriyor.

<Listing number="19-2" caption="Demetteki öğe sayısıyla uyuşmayan değişken sayısına sahip yanlış desen">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

</Listing>

Bu kodu derlemeye çalıştığınızda tür hatası alırsınız:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-02/output.txt}}
```

Bu tür durumlarda çözüm ya `_` veya `..` ile bazı değerleri yok saymak ya da
desendeki değişken sayısını değerin yapısına uygun hale getirmektir.

### Koşullu `if let` İfadeleri

6. bölümde `if let` yapısını, tek durumu ilgilendiren kısa `match` biçimi
olarak görmüştük. İsterseniz buna `else` veya `else if let` kolları da
ekleyebilirsiniz.

19-3 numaralı liste, `if let`, `else if`, `else if let` ve `else` yapılarını
birlikte kullanmanın mümkün olduğunu gösteriyor.

<Listing number="19-3" file-name="src/main.rs" caption="`if let`, `else if`, `else if let` ve `else` yapılarını karıştırmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs}}
```

</Listing>

Bu kod, birkaç koşula göre arka plan rengini seçiyor. Kullanıcının favori rengi
varsa onu kullanıyor. Favori renk yoksa ve bugün salıysa yeşil kullanıyor.
Yok, yaş bilgisi dizgi olarak verildiyse ve başarıyla sayıya çevrilebildiyse bu
kez yaşa göre mor veya turuncu seçiyor. Bunların hiçbiri yoksa maviye düşüyor.

Bu yapı bize `match`'ten daha fazla esneklik verir; çünkü tek bir değer üstünde
eşleştirme yapmak zorunda değiliz.

Burada önemli bir ayrıntı da şudur: `if let Ok(yas) = yas` satırı, dışarıdaki
`yas` değişkenini gölgeleyen yeni bir `yas` üretir. Bu yüzden `yas > 30`
kontrolünü o bloğun içinde yapmak zorundayız.

`if let` ifadelerinin dezavantajı, derleyicinin kapsamlılık denetimi yapmamasıdır.
Yani son `else` kolunu unutursanız olası bir mantık hatası hakkında uyarı
almazsınız.

### Koşullu `while let` Döngüleri

`if let` ile benzer biçimde, `while let` de desen eşleştiği sürece döngüyü
devam ettirir. 19-4 numaralı listede, bu yapıyı bir kanaldan gelen iletileri
yazdırmak için kullanıyoruz.

<Listing number="19-4" caption="`alici.recv()` `Ok` döndürdüğü sürece değerleri yazdırmak için `while let` kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

</Listing>

Bu örnek `1`, `2` ve `3` yazar. `recv`, gönderici taraf açık olduğu sürece
`Ok(deger)` döndürür; gönderici kapandığında ise `Err` gelir. `while let`, bu
yapıyı doğal biçimde ifade etmemizi sağlar.

### `for` Döngüleri

`for` döngüsünde `for` anahtar sözcüğünden hemen sonra gelen kısım da bir
desendir. Örneğin `for x in y` ifadesinde `x` bir desendir. 19-5 numaralı
liste, demeti ayrıştırmak için `for` içinde desen kullanımını gösteriyor.

<Listing number="19-5" caption="Bir demeti ayrıştırmak için `for` döngüsünde desen kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

</Listing>

Bu kod şu çıktıyı üretir:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

Burada `enumerate`, yineleyiciyi `(indeks, deger)` demetleri üretecek biçimde
uyarlıyor. Desen de bu demeti iki parçaya ayırıp her birini ayrı adla
kullanabilmemizi sağlıyor.

### Fonksiyon Parametreleri

Fonksiyon parametreleri de desendir. 19-6 numaralı listedeki `foo` fonksiyonuna
bakın:

<Listing number="19-6" caption="Parametrelerde desen kullanan bir fonksiyon imzası">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-06/src/main.rs:here}}
```

</Listing>

Buradaki `deger` de bir desendir. Tıpkı `let` gibi, fonksiyon parametreleri
yerinde de demet ayrıştırması yapabilirsiniz. 19-7 numaralı liste bunu
gösteriyor.

<Listing number="19-7" file-name="src/main.rs" caption="Bir demeti ayrıştıran parametrelere sahip fonksiyon">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-07/src/main.rs}}
```

</Listing>

Bu kod `Güncel konum: (3, 5)` yazar. `&(3, 5)` değeri `&(x, y)` desenine
eşleşir; böylece `x` ile `y` sırasıyla `3` ve `5` olur.

Kapanış parametrelerinde de aynı şekilde desen kullanabilirsiniz; çünkü
kapanışlar bu açıdan fonksiyonlara benzer.

Bu noktada desenlerin birkaç farklı yerde kullanılabildiğini gördünüz. Ancak
her yerde aynı tür desenler kullanılamaz. Bazı yerlerde desenlerin
çürütülemez olması gerekir; bazı yerlerde çürütülebilir desenler de kabul
edilir. Şimdi buna bakalım.

[ignoring-values-in-a-pattern]: ch19-03-desen-sozdizimi.html#desende-değerleri-yoksaymak
