<a id="data-types"></a>

## Veri Türleri

Rust'taki her değer belirli bir _veri türüne (data type)_ sahiptir; bu bilgi Rust'a ne tür bir veriyle çalıştığını ve o veriyi nasıl işleyeceğini söyler. Bu bölümde iki veri türü alt kümesini inceleyeceğiz: skaler ve bileşik türler.

Rust'ın _statik türlemeli (statically typed)_ bir dil olduğunu unutmayın; yani derleyicinin tüm değişkenlerin türlerini derleme zamanında bilmesi gerekir. Derleyici çoğu durumda değere ve onu nasıl kullandığımıza bakarak hangi türü istediğimizi kendiliğinden çıkarabilir. Ancak Bölüm 2'deki ["Tahmini Gizli Sayıyla Karşılaştırmak"][comparing-the-guess-to-the-secret-number]<!-- ignore --> kısmında `parse` metodunu kullanarak bir `String` değerini sayısal bir türe dönüştürdüğümüzde olduğu gibi pek çok farklı türün mümkün olduğu durumlarda, aşağıdaki gibi açık bir *tür bildirimi (type annotation)* eklememiz gerekir:

```rust
let tahmin: u32 = "42".parse().expect("Bir sayı değil!");
```

Yukarıdaki kodda gösterilen `: u32` tür bildirimini eklemezsek Rust aşağıdaki hatayı görüntüler; bu mesaj, derleyicinin hangi türü kullanmak istediğimizi anlamak için bizden daha fazla bilgiye ihtiyaç duyduğunu gösterir:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

> Not: Derleyicinin ürettiği `error[E0284]: type annotations needed` (*tür bildirimi gerekiyor*) hatası, `"42".parse()` çağrısının `u32`, `i32`, `f64` gibi metinden dönüştürülebilen (`FromStr` trait'ini uygulayan) pek çok farklı türe karşılık gelebileceğini ve tek başına yeterli ipucu bulunmadığını belirtir. Derleyici hatanın altındaki `help` satırında, değişken adının yanına `: /* Type */` biçiminde açık bir tür yazmamızı önerir.

Diğer veri türleri için de farklı tür bildirimleri göreceksiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="scalar-types"></a>
<a id="skaler-türler-scalar-types"></a>

### Skaler Türler

_Skaler (scalar)_ bir tür tek bir değeri temsil eder. Rust'ta dört temel skaler tür bulunur: tam sayılar, kayan noktalı sayılar, Boolean'lar ve karakterler. Bu türleri diğer programlama dillerinden de tanıyor olabilirsiniz. Gelin Rust'ta nasıl çalıştıklarına yakından bakalım.

<!-- Old headings. Do not remove or links may break. -->
<a id="integer-types"></a>
<a id="tam-sayı-türleri-integer-types"></a>

#### Tam Sayı Türleri

_Tam sayı (integer)_, kesirli bileşeni olmayan bir sayıdır. Bölüm 2'de tam sayı türlerinden biri olan `u32` türünü kullanmıştık. Bu tür bildirimi, ilişkili değerin bellekte 32 bitlik alan kaplayan *işaretsiz (unsigned)* bir tam sayı olması gerektiğini belirtir (*işaretli (signed)* tam sayı türleri `u` yerine `i` harfiyle başlar). Tablo 3-1, Rust'taki yerleşik tam sayı türlerini gösterir. Bir tam sayı değerinin türünü bildirmek için bu varyantlardan herhangi birini kullanabiliriz.

<span class="caption">Tablo 3-1: Rust'taki Tam Sayı Türleri</span>

| Uzunluk | İşaretli | İşaretsiz |
| ------- | -------- | --------- |
| 8 bit   | `i8`     | `u8`      |
| 16 bit  | `i16`    | `u16`     |
| 32 bit  | `i32`    | `u32`     |
| 64 bit  | `i64`    | `u64`     |
| 128 bit | `i128`   | `u128`    |
| Mimariye bağlı | `isize`  | `usize`   |

Her tam sayı türünün belirli bir boyutu ve değer aralığı vardır. _İşaretli_ türler negatif sayıları, sıfırı ve pozitif sayıları; _işaretsiz_ türler ise sıfırı ve pozitif sayıları temsil eder. Buradaki “işaret”, sayının kaynak kodda mutlaka `+` veya `-` ile yazılması gerektiğini değil, türün negatif değerleri temsil edip edemediğini belirtir. İşaretli sayılar bellekte [ikiye tümleyen][twos-complement]<!-- ignore --> (*two's complement*) temsili kullanılarak saklanır.

Her işaretli varyant, _n_ ilgili varyantın kullandığı bit sayısı olmak üzere, −(2<sup>n − 1</sup>) ile 2<sup>n − 1</sup> − 1 (sınırlar dahil) aralığındaki sayıları saklayabilir. Dolayısıyla bir `i8`, −(2<sup>7</sup>) ile 2<sup>7</sup> − 1 aralığındaki, yani −128 ile 127 arasındaki sayıları tutabilir. İşaretsiz varyantlar ise 0 ile 2<sup>n</sup> − 1 aralığındaki sayıları saklayabilir; örneğin bir `u8`, 0 ile 2<sup>8</sup> − 1 aralığındaki, yani 0 ile 255 arasındaki sayıları tutabilir.

Ek olarak `isize` ve `usize` türlerinin boyutu, programınızın çalıştığı bilgisayarın mimarisine bağlıdır: 64 bitlik bir mimaride 64 bit, 32 bitlik bir mimaride ise 32 bittir.

Tam sayı sabit değerlerini (*integer literals*) Tablo 3-2'de gösterilen biçimlerden herhangi biriyle yazabilirsiniz. Birden fazla sayısal türe uyabilen sayı sabit değerlerinin, türü açıkça belirlemek amacıyla `57u8` örneğinde olduğu gibi bir tür son eki (*type suffix*) alabileceğine dikkat edin. Ayrıca sayı sabit değerlerinde okunabilirliği artırmak için görsel ayırıcı olarak `_` karakteri kullanılabilir; örneğin `1_000` yazmak, `1000` yazmakla birebir aynı değere karşılık gelir.

<span class="caption">Tablo 3-2: Rust'taki Tam Sayı Sabit Değerleri</span>

| Sayı sabit değerleri | Örnek         |
| -------------------- | ------------- |
| Onluk                | `98_222`      |
| On altılık           | `0xff`        |
| Sekizlik             | `0o77`        |
| İkilik               | `0b1111_0000` |
| Bayt (yalnızca `u8`) | `b'A'`        |

Peki hangi tam sayı türünü kullanacağınızı nasıl bileceksiniz? Emin değilseniz Rust'ın varsayılanları genellikle iyi bir başlangıç noktasıdır: Tam sayı türleri varsayılan olarak `i32` kabul edilir. `isize` veya `usize` kullanacağınız başlıca durum ise bir koleksiyonu indekslemektir.

> <!-- Old headings. Do not remove or links may break. -->
> <a id="integer-overflow"></a>
> <a id="tam-sayı-taşması-integer-overflow"></a>
>
> ##### Tam Sayı Taşması
>
> Diyelim ki 0 ile 255 arasında değerler tutabilen `u8` türünde bir değişkeniniz
> var. Bu değişkenin değerini bu aralığın dışına çıkaracak bir işlem yaparsanız
> (örneğin `255` olan değerini `1` artırarak `256` yapmaya çalışırsanız) _tam
> sayı taşması (integer overflow)_ meydana gelir ve bu durum derleme moduna göre
> iki farklı davranıştan biriyle sonuçlanır. (Doğrudan `let x: u8 = 256;` gibi
> aralık dışı bir sabit değer yazmak ise taşma sayılmaz; derleyici böyle bir
> satırı daha baştan `` literal out of range for `u8` `` hatasıyla reddeder.)
> Hata ayıklama (*debug*) modunda derleme yaptığınızda Rust, tam sayı taşmasını
> yakalayan kontroller ekler ve çalışma zamanında taşma gerçekleşirse
> programınızın _panik_ yapmasına neden olur. Panikleme (*panicking*), normal
> kontrol akışını kesen bir hata mekanizmasıdır; her hata sonucu veya başarısız
> çıkış bir panik değildir. Panikleri Bölüm 9'daki ["`panic!` ile Kurtarılamayan
> Hatalar"][unrecoverable-errors-with-panic]<!-- ignore --> kısmında daha
> derinlemesine inceleyeceğiz.
>
> `--release` bayrağıyla yayın (*release*) modunda derleme yaptığınızda ise
> Rust, varsayılan olarak bu tam sayı taşması kontrollerini _eklemez_. Bu
> davranış profildeki `overflow-checks` ayarıyla değiştirilebilir; burada
> varsayılan profilleri karşılaştırıyoruz. Kontroller kapalıyken taşma
> meydana gelirse Rust _ikiye tümleyen sarması (two's complement wrapping)_ uygular. Kısacası,
> türün tutabileceği en büyük değeri aşan sayılar başa dönerek türün alabileceği
> en küçük değere "sarılır". Bir `u8` için 256 değeri `0`, 257 değeri `1` olur
> ve bu şekilde devam eder. Program paniklemez ancak değişken büyük olasılıkla
> beklemediğiniz bir değer alır. Tam sayı taşmasının bu sarma davranışına
> güvenerek kod yazmak hata olarak kabul edilir.
>
> Taşma olasılığını açıkça ele almak istediğinizde, standart kütüphanenin ilkel sayısal türler için sunduğu şu metot ailelerini kullanabilirsiniz:
>
> - `wrapping_*` metotlarıyla tüm derleme modlarında sarma işlemi yapmak (örneğin `255u8.wrapping_add(1)` çağrısı `0` döndürür).
> - `checked_*` metotlarıyla taşma durumunda `None` değeri döndürmek (örneğin `255u8.checked_add(1)` çağrısı `None` döndürür).
> - `overflowing_*` metotlarıyla hem sarmalanmış değeri hem de taşma olup olmadığını belirten bir `bool` değerini birlikte döndürmek (örneğin `255u8.overflowing_add(1)` çağrısı `(0, true)` döndürür).
> - `saturating_*` metotlarıyla değerin alt veya üst sınırında doygunluğa ulaşarak sabitlenmesini (*saturate*) sağlamak (örneğin `255u8.saturating_add(1)` çağrısı `255` döndürür).

<!-- Old headings. Do not remove or links may break. -->
<a id="floating-point-types"></a>
<a id="kayan-noktalı-floating-point-türler"></a>

#### Kayan Noktalı Türler

Rust'ta ayrıca ondalık basamak içeren _kayan noktalı sayılar (floating-point numbers)_ için iki ilkel (*primitive*) tür bulunur. Rust'ın kayan noktalı türleri sırasıyla 32 bit ve 64 bit boyutundaki `f32` ve `f64`'tür. Varsayılan tür `f64`'tür; çünkü modern işlemcilerde `f32` ile neredeyse aynı hızda çalışırken daha yüksek duyarlılık (*precision*) sunar. Tüm kayan noktalı türler işaretlidir.

İşte kayan noktalı sayıların kullanımını gösteren bir örnek:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Kayan noktalı sayılar IEEE-754 standardına göre temsil edilir. `f32` türü tek duyarlıklı (*single-precision*), `f64` türü ise çift duyarlıklı (*double-precision*) bir kayan noktalı sayıdır.

<a id="numeric-operations"></a>

#### Sayısal İşlemler

Rust, tüm sayı türleri için bekleyeceğiniz temel matematiksel işlemleri destekler: toplama, çıkarma, çarpma, bölme ve kalan (*mod alma*). Tam sayı bölmesi, sonucu sıfıra doğru en yakın tam sayıya yuvarlar; yani kesirli kısmı atar (*truncate*). Aşağıdaki kod, her bir sayısal işlemi bir `let` deyiminde nasıl kullanacağınızı gösterir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Bu deyimlerdeki her bir ifade matematiksel bir operatör kullanır ve değerlendirilerek tek bir değer üretir; ardından bu değer ilgili değişkene bağlanır. [Ek B][appendix_b]<!-- ignore -->, Rust'ın sunduğu tüm operatörlerin tam bir listesini içerir.

> Not: Rust, farklı sayısal türler arasında örtük (otomatik) dönüşüm yapmaz:
> `i32` türündeki bir değeri `i64` türündeki bir değerle doğrudan toplamaya
> çalışırsanız derleyici `` cannot add `i64` to `i32` `` hatası verir. Böyle
> durumlarda değerleri açıkça ortak bir türe dönüştürmeniz gerekir. Örneğin
> `i32` değerini `i64` türüne kayıpsız dönüştürmek için `i64::from(x) + y`
> yazabilirsiniz. `as` ile dönüşüm de mümkündür (`x as i64 + y`); ancak
> daraltıcı tam sayı dönüşümleri veri kaybına yol açabilir. Hedef türe sığmayan
> bir değeri hata olarak ele almak istiyorsanız `TryFrom` veya `TryInto`
> kullanabilirsiniz; bu trait'leri ilerleyen bölümlerde göreceğiz. Ayrıca bir
> tam sayıyı çalışma zamanında sıfıra bölmek, hem hata ayıklama hem de yayın modunda `attempt to
> divide by zero` mesajıyla panik üretir; kayan noktalı sayılarda ise sıfıra
> bölme paniklemez, IEEE-754 kurallarına göre `inf` ya da `NaN` değerini
> döndürür.

<a id="the-boolean-type"></a>

#### Boolean Türü

Diğer pek çok programlama dilinde olduğu gibi Rust'ta da Boolean türünün iki olası değeri vardır: `true` ve `false`. Boolean değerler bellekte bir bayt yer kaplar. Rust'ta Boolean türü `bool` anahtar kelimesiyle belirtilir. Örneğin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Boolean değerlerini kullanmanın başlıca yolu `if` ifadesi gibi koşullu yapılardır. `if` ifadelerinin Rust'ta nasıl çalıştığını ["Kontrol Akışı"][control-flow]<!-- ignore --> bölümünde ele alacağız.

<!-- Old headings. Do not remove or links may break. -->
<a id="the-character-type"></a>
<a id="karakter-character-türü"></a>

#### Karakter Türü

Rust'ın `char` türü, dilin en temel alfabetik veri türüdür. İşte `char` değerleri tanımlamaya dair birkaç örnek:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Çift tırnak kullanan string sabitlerinin (*string literals*) aksine, `char` sabitlerini tek tırnakla belirttiğimize dikkat edin. Rust'ın `char` türü 4 bayt boyutundadır ve bir Unicode skaler değerini temsil eder; bu sayede yalnızca ASCII karakterlerini değil, çok daha fazlasını ifade edebilir. Aksanlı ve noktalı harfler; Çince, Japonca ve Korece karakterler; emojiler ve sıfır genişlikli boşluklar Rust'ta geçerli birer `char` değeridir. Unicode skaler değerleri `U+0000` ile `U+D7FF` ve `U+E000` ile `U+10FFFF` (sınırlar dahil) aralıklarını kapsar. Bununla birlikte "karakter" aslında Unicode standardında doğrudan tanımlı bir kavram değildir; dolayısıyla bir "karakterin" ne olduğuna dair günlük sezgimiz Rust'taki `char` tanımıyla her zaman birebir örtüşmeyebilir. Bu konuyu Bölüm 8'deki ["String'lerle UTF-8 Kodlanmış Metin Saklama"][strings]<!-- ignore --> kısmında ayrıntılı olarak inceleyeceğiz.

> Not: Türkçeye özgü `'ç'`, `'ğ'`, `'ı'`, `'İ'`, `'ö'`, `'ş'`, `'ü'` harfleri ve `'😻'` gibi tek skaler değerden oluşan emojiler geçerli birer `char` sabitidir; her `char` değeri, hangi karakteri tuttuğundan bağımsız olarak bellekte **4 bayt** yer kaplar. Buna karşılık Bölüm 8'de göreceğimiz UTF-8 kodlamalı `String` ve `&str` metinlerinde karakterler değişken uzunlukta saklanır (örneğin ASCII harfler 1 bayt, `'ğ'` ve `'ş'` gibi Türkçe harfler 2 bayt, `'😻'` emojisi ise 4 bayt yer kaplar). Ayrıca birden fazla Unicode skaler değerinin birleşimiyle oluşturulan bazı birleşik emojiler (örneğin `"👨‍👩‍👧‍👦"`) tek bir `char` içine sığmaz.

<!-- Old headings. Do not remove or links may break. -->
<a id="compound-types"></a>
<a id="bileşik-türler-compound-types"></a>

### Bileşik Türler

_Bileşik türler (compound types)_ birden fazla değeri tek bir tür altında gruplayabilir. Rust'ta iki ilkel bileşik tür vardır: demetler (*tuples*) ve diziler (*arrays*).

<!-- Old headings. Do not remove or links may break. -->
<a id="the-tuple-type"></a>
<a id="tuple-türü"></a>

#### Demet Türü

_Demet (tuple)_, farklı türlerde olabilen bir grup değeri tek bir bileşik tür altında bir araya getirmenin genel bir yoludur. Demetlerin sabit bir uzunluğu vardır: Bir kez bildirildikten sonra boyutları büyüyemez veya küçülemez.

Parantez içine virgülle ayrılmış bir değer listesi yazarak bir demet oluştururuz. Demetteki her konumun kendine ait bir türü vardır ve farklı konumlardaki değerlerin türlerinin aynı olması gerekmez. Aşağıdaki örnekte isteğe bağlı tür bildirimlerini de ekledik:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

Bir demet tek bir bileşik öğe olarak kabul edildiğinden `tup` değişkeni demetin tamamına bağlanır. Demetin içindeki tekil değerleri elde etmek için desen eşleştirme (*pattern matching*) kullanarak bir demet değerini aşağıdaki gibi parçalayabiliriz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Bu program önce bir demet oluşturup onu `tup` değişkenine bağlar. Ardından `let` ile birlikte bir desen kullanarak `tup` değerini alır ve üç ayrı değişkene (`x`, `y` ve `z`) ayırır. Tek bir demeti üç parçaya ayırdığı için bu işleme _yapı sökümü (destructuring)_ denir. Son olarak program, `6.4` olan `y` değerini ekrana yazdırır.

Bir demet elemanına, demetin adından sonra bir nokta (`.`) ve erişmek istediğimiz değerin indeksini yazarak doğrudan da ulaşabiliriz. Örneğin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Bu program `x` demetini oluşturur ve ardından her bir elemanına kendi indeksi üzerinden erişir. Çoğu programlama dilinde olduğu gibi bir demetteki ilk indeks `0`'dır.

Hiçbir değer içermeyen boş demetin özel bir adı vardır: _birim (unit)_. Hem bu değer hem de karşılık gelen türü `()` biçiminde yazılır; boş bir değeri veya boş bir dönüş türünü temsil eder. İfadeler, başka bir değer döndürmediklerinde örtük olarak (*implicitly*) birim değerini döndürürler.

> Not: Birim tür `()`, C veya Java gibi dillerdeki `void` kavramına benzer şekilde anlamlı bir değer taşınmadığını belirtir; ancak `void`'den farklı olarak gerçek ve somut bir türdür. Yalnızca tek bir olası değere (`()`) sahip olduğundan derleyici onun çalışma zamanında bellekte yer kaplamasına ihtiyaç duymaz (boyutu **0 bayttır**). Dönüş türü yazılmayan fonksiyonların gerçek dönüş türü `()`'dir.

<!-- Old headings. Do not remove or links may break. -->
<a id="the-array-type"></a>
<a id="array-dizi-türü"></a>

#### Dizi Türü

Birden fazla değerden oluşan bir koleksiyon elde etmenin bir diğer yolu da _dizi (array)_ kullanmaktır. Demetten farklı olarak bir dizinin tüm elemanları aynı türde olmalıdır. Diğer bazı dillerdeki dizilerin aksine, Rust'taki diziler sabit uzunlukludur.

Bir dizideki değerleri köşeli parantezler içinde virgülle ayrılmış bir liste hâlinde yazarız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Diziler, verilerinizin *öbek (heap)* yerine —şimdiye kadar gördüğümüz diğer türlerde olduğu gibi— *yığın (stack)* üzerinde ayrılmasını istediğinizde ([Bölüm 4'te][stack-and-heap]<!-- ignore --> yığın ve öbeği daha ayrıntılı ele alacağız) veya her zaman sabit sayıda elemana sahip olduğunuzdan emin olmak istediğinizde kullanışlıdır. Ancak bir dizi, vektör (*vector*) türü kadar esnek değildir. Vektör, standart kütüphane tarafından sağlanan ve içeriği öbek üzerinde tutulduğu için boyutunun büyümesine veya küçülmesine _izin verilen_ benzer bir koleksiyon türüdür. Dizi mi yoksa vektör mü kullanacağınızdan emin değilseniz büyük olasılıkla bir vektör kullanmalısınız. [Bölüm 8][vectors]<!-- ignore --> vektörleri daha ayrıntılı olarak inceler.

Bununla birlikte, eleman sayısının değişmeyeceğini bildiğiniz durumlarda diziler daha uygundur. Örneğin bir programda ay adlarını kullanıyorsanız, her zaman 12 eleman içereceğini bildiğiniz için büyük olasılıkla vektör yerine bir dizi tercih edersiniz:

```rust
let aylar = ["Ocak", "Şubat", "Mart", "Nisan", "Mayıs", "Haziran", "Temmuz",
             "Ağustos", "Eylül", "Ekim", "Kasım", "Aralık"];
```

Bir dizinin türünü köşeli parantezler içinde her elemanın türünü, ardından bir noktalı virgülü ve dizideki eleman sayısını belirterek yazarız:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Burada `i32` her bir elemanın türüdür. Noktalı virgülden sonraki `5` sayısı ise dizinin beş eleman içerdiğini belirtir.

Tüm elemanları aynı değere sahip olacak bir diziyi, köşeli parantez içinde önce başlangıç değerini, ardından noktalı virgülü ve son olarak dizinin uzunluğunu yazarak da başlatabilirsiniz:

```rust
let a = [3; 5];
```

`a` adındaki bu dizi, başlangıçta tamamı `3` değerine ayarlanmış `5` eleman içerecektir. Bu ifade `let a = [3, 3, 3, 3, 3];` yazmakla birebir aynıdır, ancak çok daha özlüdür.

<!-- Old headings. Do not remove or links may break. -->
<a id="accessing-array-elements"></a>
<a id="array-element-access"></a>
<a id="array-elemanlarına-erişim"></a>

#### Dizi Elemanlarına Erişim

Bir dizi, yığın üzerinde ayrılabilen, boyutu derleme zamanında bilinen sabit büyüklükte tek parça bir bellek bloğudur. İndeksleme kullanarak bir dizinin elemanlarına şu şekilde erişebilirsiniz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

Bu örnekte `ilk` adlı değişken, dizinin `[0]` indeksindeki değer olan `1` değerini alır. `ikinci` adlı değişken ise dizinin `[1]` indeksindeki `2` değerini alır.

<!-- Old headings. Do not remove or links may break. -->
<a id="invalid-array-element-access"></a>
<a id="geçersiz-array-elemanlarına-erişim-invalid-array-element-access"></a>

#### Geçersiz Dizi Elemanlarına Erişim

Bir dizinin sonunu aşan bir indeksle dizi elemanına erişmeye çalışırsanız ne olacağına bakalım. Kullanıcıdan bir dizi indeksi almak için Bölüm 2'deki tahmin oyununa benzer aşağıdaki kodu çalıştırdığınızı düşünelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Bu kod sorunsuzca derlenir. Kodu `cargo run` ile çalıştırıp `0`, `1`, `2`, `3` veya `4` değerlerinden birini girerseniz program dizide o indekse karşılık gelen değeri ekrana yazdırır. Ancak dizinin sınırlarını aşan bir sayı (örneğin `10`) girerseniz şuna benzer bir çıktı görürsünüz:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Program, indeksleme işleminde geçersiz bir değerin kullanıldığı noktada bir çalışma zamanı hatasıyla karşılaştı; bir hata mesajı yazdırarak sonlandı ve en sondaki `println!` deyimini çalıştırmadı. İndeksleme yoluyla bir elemana erişmeye çalıştığınızda Rust, belirttiğiniz indeksin dizi uzunluğundan küçük olup olmadığını kontrol eder. İndeks dizi uzunluğuna eşit veya ondan büyükse Rust panikler. Özellikle bu örnekte olduğu gibi derleyicinin, kodu daha sonra çalıştıracak kullanıcının hangi sayıyı gireceğini derleme zamanında bilmesi imkânsız olduğundan bu kontrolün çalışma zamanında yapılması gerekir.

> Not: Panik çıktısındaki `index out of bounds: the len is 5 but the index is 10` mesajı *"indeks sınırların dışında: dizinin uzunluğu 5 ama istenen indeks 10"* anlamına gelir. Programı çökertmek (`panic!`) yerine sınır aşımını güvenli biçimde kontrol etmek istediğinizde, Bölüm 6 ve Bölüm 8'de göreceğimiz `Option` türünü döndüren `a.get(indeks)` metodunu kullanabilirsiniz.

Bu davranış, Rust'ın bellek güvenliği ilkelerinin iş başındaki somut bir örneğidir. Pek çok alt seviye dilde bu tür bir sınır kontrolü yapılmaz ve yanlış bir indeks verdiğinizde geçersiz bellek bölgelerine erişilebilir. Rust, geçersiz bellek erişimine izin verip çalışmaya devam etmek yerine programı derhal sonlandırarak sizi bu tür güvenlik açıklarına ve belirsiz davranışlara karşı korur. Bölüm 9'da Rust'ın hata yönetimi mekanizmalarını ve ne panikleyen ne de geçersiz bellek erişimine izin veren okunaklı, güvenli kodları nasıl yazabileceğinizi daha ayrıntılı olarak ele alacağız.

[comparing-the-guess-to-the-secret-number]: ch02-00-tahmin-oyunu-programlama.html#tahmini-gizli-sayıyla-karşılaştırmak
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-kontrol-akisi.html#kontrol-akışı-control-flow
[strings]: ch08-02-stringler.html#stringlerle-utf-8-kodlanmış-metin-depolamak
[stack-and-heap]: ch04-01-sahiplik-nedir.html#the-stack-and-the-heap
[vectors]: ch08-01-vektorler.html
[unrecoverable-errors-with-panic]: ch09-01-panic-ile-kurtarilamayan-hatalar.html
[appendix_b]: ekler-02-operatorler.html
