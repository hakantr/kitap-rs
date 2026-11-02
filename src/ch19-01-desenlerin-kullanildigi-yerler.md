<!-- Old headings. Do not remove or links may break. -->
<a id="all-the-places-patterns-can-be-used"></a>
<a id="desenlerin-kullanılabildiği-tüm-yerler"></a>

## Desenlerin Kullanılabileceği Tüm Yerler

Desenler Rust'ta pek çok yerde karşımıza çıkar ve farkında bile olmadan onları sıkça kullanmışsınızdır! Bu bölümde desenlerin geçerli olduğu tüm yerleri ele alacağız.

<a id="match-arms"></a>

### `match` Kolları

Bölüm 6'da ele aldığımız gibi, `match` ifadelerinin kollarında desenleri kullanırız. Biçimsel olarak `match` ifadeleri; `match` anahtar kelimesi, üzerinde eşleştirme yapılacak bir değer ve her biri bir desen ile değerin o kolun deseniyle eşleşmesi durumunda çalıştırılacak bir ifadeden oluşan bir veya daha fazla `match` kolu olarak tanımlanır:

<!--
  Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre><code>match <em>DEĞER</em> {
    <em>DESEN</em> => <em>İFADE</em>,
    <em>DESEN</em> => <em>İFADE</em>,
    <em>DESEN</em> => <em>İFADE</em>,
}</code></pre>

Örneğin, Liste 6-5'teki `x` değişkeninde bulunan bir `Option<i32>` değeri üzerinde eşleştirme yapan `match` ifadesi şöyledir:

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

Bu `match` ifadesindeki desenler, her okun sol tarafında yer alan `None` ve `Some(i)` kısımlarıdır.

`match` ifadelerinin temel gereksinimlerinden biri _kapsamlı_ (*exhaustive*) olmalarıdır; yani `match` ifadesindeki değerin alabileceği tüm olasılıkların hesaba katılması gerekir. Her olasılığı kapsadığınızdan emin olmanın bir yolu, son kol için tümünü yakalayan (*catch-all*) bir desen kullanmaktır: Örneğin, herhangi bir değerle eşleşen bir değişken adı asla başarısız olmaz ve dolayısıyla geriye kalan tüm durumları kapsar.

Özel `_` deseni her şeyle eşleşir ancak hiçbir zaman bir değişkene bağlanmaz; bu nedenle son `match` kolunda sıklıkla kullanılır. Örneğin, açıkça belirtilmeyen herhangi bir değeri yoksaymak istediğinizde `_` deseni oldukça kullanışlıdır. `_` desenini bu bölümün ilerleyen kısımlarında yer alan [“Desende Değerleri Yoksaymak”][ignoring-values-in-a-pattern]<!-- ignore --> başlığında daha ayrıntılı olarak ele alacağız.

<!-- Old headings. Do not remove or links may break. -->
<a id="let-statements"></a>
<a id="let-ifadeleri"></a>

### `let` Deyimleri

Bu bölümden önce desen kullanımını yalnızca `match` ve `if let` bağlamında açıkça ele almıştık; ancak aslında `let` deyimleri de dahil olmak üzere başka yerlerde de desenleri kullandık. Örneğin, `let` ile yapılan şu doğrudan değişken atamasını düşünün:

```rust
let x = 5;
```

Farkında olmasanız da bu şekilde bir `let` deyimi kullandığınız her seferinde aslında desen kullanıyordunuz! Daha biçimsel olarak bir `let` deyimi şöyle görünür:

<!--
  Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre>
<code>let <em>DESEN</em> = <em>İFADE</em>;</code>
</pre>

*DESEN* yuvasında bir değişken adının yer aldığı `let x = 5;` gibi deyimlerde, değişken adı yalnızca bir desenin son derece basit bir biçimidir. Rust ifadeyi desenle karşılaştırır ve desende bulduğu her ada, o ada karşılık gelen değeri atar. Dolayısıyla `let x = 5;` örneğinde `x`, “burada eşleşen şeyi `x` değişkenine bağla” anlamına gelen bir desendir. `x` adı desenin tamamını oluşturduğundan bu desen fiilen “değer ne olursa olsun her şeyi `x` değişkenine bağla” anlamına gelir.

`let` deyiminin desen eşleştirme yönünü daha net görmek için, bir demeti ayrıştırmak (*destructure*) üzere `let` ile desen kullanan Liste 19-1'i inceleyelim.

<Listing number="19-1" caption="Bir demeti ayrıştırmak ve aynı anda üç değişken oluşturmak için desen kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs:here}}
```

</Listing>

Burada bir demeti bir desenle eşleştiriyoruz. Rust `(1, 2, 3)` değerini `(birinci, ikinci, ucuncu)` deseniyle karşılaştırır ve değerin desenle eşleştiğini — yani her ikisindeki öğe sayısının aynı olduğunu — görür; böylece `1` değerini `birinci`, `2` değerini `ikinci` ve `3` değerini `ucuncu` değişkenine bağlar. Bu demet desenini, kendi içinde üç ayrı değişken desenini iç içe barındıran bir yapı olarak düşünebilirsiniz.

Desendeki öğe sayısı demetteki öğe sayısıyla eşleşmezse demetin bütününde tür uyuşmaz ve bir derleyici hatası alırız. Örneğin Liste 19-2, üç öğeli bir demeti iki değişkene ayrıştırma girişimini gösterir; bu kod derlenmeyecektir.

<Listing number="19-2" caption="Demetteki öğe sayısıyla eşleşmeyen değişkenlere sahip hatalı bir desen oluşturmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

</Listing>

Bu kodu derlemeye çalışmak aşağıdaki tür hatasıyla sonuçlanır:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-02/output.txt}}
```

Hatayı düzeltmek için, [“Desende Değerleri Yoksaymak”][ignoring-values-in-a-pattern]<!-- ignore --> bölümünde göreceğiniz gibi `_` veya `..` kullanarak demetteki değerlerden birini ya da birkaçını yoksayabiliriz. Eğer sorun desende çok fazla değişken bulunmasıysa, çözüm değişken sayısını demetteki öğe sayısına eşitleyecek şekilde fazladan değişkenleri kaldırarak türlerin eşleşmesini sağlamaktır.

<a id="conditional-if-let-expressions"></a>

### Koşullu `if let` İfadeleri

Bölüm 6'da `if let` ifadelerini esas olarak yalnızca tek bir durumla eşleşen bir `match` ifadesinin eşdeğerini daha kısa yazmanın bir yolu olarak ele almıştık. İsteğe bağlı olarak `if let`, içindeki desen eşleşmediğinde çalıştırılacak kodu barındıran bir `else` bloğuna da sahip olabilir.

Liste 19-3; `if let`, `else if` ve `else if let` ifadelerini bir arada ve karışık biçimde kullanmanın da mümkün olduğunu göstermektedir. Bunu yapmak, desenlerle karşılaştırmak üzere yalnızca tek bir değeri ifade edebildiğimiz bir `match` ifadesine kıyasla bize daha fazla esneklik sağlar. Ayrıca Rust, bir dizi `if let`, `else if` ve `else if let` kolundaki koşulların birbiriyle ilişkili olmasını da şart koşmaz.

Liste 19-3'teki kod, çeşitli koşullara yönelik bir dizi kontrole dayanarak arka plan renginizin ne olacağını belirler. Bu örnek için, gerçek bir programın kullanıcı girdisinden alabileceği değerleri doğrudan koda sabitlenmiş (*hardcoded*) değişkenler olarak oluşturduk.

<Listing number="19-3" file-name="src/main.rs" caption="`if let`, `else if`, `else if let` ve `else` ifadelerini bir arada kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs}}
```

</Listing>

Kullanıcı favori bir renk belirtirse arka plan olarak o renk kullanılır. Favori bir renk belirtilmemişse ve bugün salıysa arka plan rengi yeşil olur. Aksi takdirde, kullanıcı yaşını bir string olarak belirtir ve biz bunu başarıyla bir sayı olarak ayrıştırabilirsek (*parse*), sayının değerine bağlı olarak renk mor veya turuncu olur. Bu koşulların hiçbiri geçerli değilse arka plan rengi mavi olur.

Bu koşullu yapı karmaşık gereksinimleri desteklememize olanak tanır. Burada kullandığımız sabitlenmiş değerlerle bu örnek ekrana `Arka plan rengi olarak mor kullanılıyor` yazdıracaktır.

`if let` ifadesinin de tıpkı `match` kolları gibi mevcut değişkenleri gölgeleyen (*shadowing*) yeni değişkenler tanımlayabildiğini görebilirsiniz: `if let Ok(yas) = yas` satırı, `Ok` varyantının içindeki değeri tutan yeni bir `yas` değişkeni tanımlayarak mevcut `yas` değişkenini gölgeler. Bu da `if yas > 30` koşulunu o bloğun içine yerleştirmemiz gerektiği anlamına gelir: Rust 2021 ve daha eski sürümlerde bu iki koşulu `if let Ok(yas) = yas && yas > 30` şeklinde birleştiremeyiz; çünkü 30 ile karşılaştırmak istediğimiz yeni `yas` değişkeni, süslü parantezle yeni kapsam başlayana kadar geçerli değildir. (Rust 2024 Sürümü bu kısıtlamayı kaldırmıştır; aşağıdaki nota bakın.)

> Not: Kitabın bu bölümündeki anlatım geleneksel kapsam kuralını gösterse de,
> **Rust 1.88** (ve **Rust 2024 Sürümü** / `edition = "2024"`) itibarıyla
> **zincirleme `if let` (*`let` chains*)** özelliği kararlı hâle getirilmiştir!
> Dolayısıyla `edition = "2024"` kullanan güncel Rust derleyicisinde `if let` ve
> `while let` koşullarını `&&` operatörüyle normal mantıksal ifadelerle veya
> başka `let` desenleriyle doğrudan birleştirebilirsiniz:
>
> ```rust
> let yas: Result<u8, _> = "34".parse();
>
> if let Ok(yas) = yas && yas > 30 {
>     println!("Arka plan rengi olarak mor kullanılıyor");
> }
> ```
>
> Bu sözdiziminde `let` deseninin bağladığı yeni `yas` değişkeni hem `&&`
> sonrasındaki koşulda hem de süslü parantez bloğunun içinde geçerlidir; böylece
> yalnızca bu koşulun sağlandığı durumu ele almak için iç içe `if` gerekmez.
> Ancak Liste 19-3'teki iç `if`/`else` yapısını tek bir zincirle değiştirmek
> aynı davranışı korumaz: Ayrıştırılan yaş `30` veya daha küçükse özgün kod
> turuncuyu seçerken zincir başarısız olur ve dıştaki sonraki kola geçilir.
> Her iki yaş grubunu ayrı işlemek istiyorsanız özgün iç `if`/`else` yapısını
> koruyun veya ikinci durumu ayrıca ele alın.

`if let` ifadelerini kullanmanın dezavantajı, `match` ifadelerinin aksine derleyicinin kapsamlılık (*exhaustiveness*) denetimi yapmamasıdır. Son `else` bloğunu atlarsak ve bu nedenle bazı durumları ele almayı kaçırırsak, derleyici bizi olası mantık hatasına karşı uyarmaz.

<a id="while-let-conditional-loops"></a>

### Koşullu `while let` Döngüleri

Yapıca `if let`'e benzeyen koşullu `while let` döngüsü, bir desenin eşleşmeye devam ettiği sürece bir `while` döngüsünün çalışmasına olanak tanır. Liste 19-4'te thread'ler arasında gönderilen mesajları bekleyen, ancak bu kez bir `Option` yerine bir `Result` değerini kontrol eden bir `while let` döngüsü gösteriyoruz.

<Listing number="19-4" caption="`alici.recv()` çağrısı `Ok` döndürdüğü sürece değerleri yazdırmak için bir `while let` döngüsü kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

</Listing>

Bu örnek sırasıyla `1`, `2` ve ardından `3` yazdırır. `recv` metodu kanalın alıcı (*receiver*) ucundan ilk mesajı alır ve bir `Ok(deger)` döndürür. Bölüm 16'da `recv` metodunu ilk gördüğümüzde dönen `Result` değerini doğrudan `unwrap` ile açmış ya da bir `for` döngüsü kullanarak kanalla bir iteratör gibi etkileşime geçmiştik. Ancak Liste 19-4'ün gösterdiği gibi `while let` de kullanabiliriz; çünkü `recv` metodu gönderici var olduğu sürece her mesaj ulaştığında bir `Ok` döndürür, gönderici taraf bağlantıyı kestiğinde ise bir `Err` üretir.

<a id="for-loops"></a>

### `for` Döngüleri

Bir `for` döngüsünde `for` anahtar kelimesinden hemen sonra gelen kısım bir desendir. Örneğin `for x in y` ifadesinde `x` bir desendir. Liste 19-5, bir demeti `for` döngüsünün bir parçası olarak ayrıştırmak — yani parçalarına ayırmak — için `for` döngüsünde nasıl desen kullanılacağını göstermektedir.

<Listing number="19-5" caption="Bir demeti ayrıştırmak için `for` döngüsünde desen kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

</Listing>

Liste 19-5'teki kod aşağıdaki çıktıyı yazdıracaktır:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

`enumerate` metodunu kullanarak bir iteratörü, her değeri ve o değerin indeksini bir demet içinde üretecek biçimde uyarlıyoruz. Üretilen ilk değer `(0, 'a')` demetidir. Bu değer `(indeks, deger)` deseniyle eşleştirildiğinde `indeks` değişkeni `0`, `deger` değişkeni ise `'a'` olur ve çıktının ilk satırı yazdırılır.

<a id="function-parameters"></a>

### Fonksiyon Parametreleri

Fonksiyon parametreleri de birer desen olabilir. `i32` türünde `deger` adlı tek bir parametre alan `bir_fonksiyon` adında bir fonksiyon bildiren Liste 19-6'daki kod artık size tanıdık geliyor olmalıdır.

<Listing number="19-6" caption="Parametrelerinde desen kullanan bir fonksiyon imzası">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-06/src/main.rs:here}}
```

</Listing>

Buradaki `deger` kısmı bir desendir! Tıpkı `let` ile yaptığımız gibi, bir fonksiyonun argümanlarındaki bir demeti de desenle eşleştirebiliriz. Liste 19-7, bir demeti fonksiyona aktarırken içindeki değerleri birbirinden ayırır.

<Listing number="19-7" file-name="src/main.rs" caption="Bir demeti ayrıştıran parametrelere sahip bir fonksiyon">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-07/src/main.rs}}
```

</Listing>

Bu kod ekrana `Güncel konum: (3, 5)` yazdırır. `&(3, 5)` değeri `&(x, y)` deseniyle eşleşir; böylece `x` değişkeni `3`, `y` değişkeni ise `5` değerini alır.

Bölüm 13'te ele aldığımız gibi kapanışlar (*closures*) fonksiyonlara benzediğinden, desenleri fonksiyon parametre listelerinde kullandığımız şekilde kapanış parametre listelerinde de kullanabiliriz.

Bu noktaya kadar desenleri kullanmanın çeşitli yollarını gördünüz; ancak desenler onları kullanabildiğimiz her yerde aynı şekilde çalışmaz. Bazı yerlerde desenlerin çürütülemez (*irrefutable*) olması zorunludur; başka durumlarda ise çürütülebilir (*refutable*) olabilirler. Sırada bu iki kavramı ele alacağız.

[ignoring-values-in-a-pattern]: ch19-03-desen-sozdizimi.html#desende-değerleri-yoksaymak
