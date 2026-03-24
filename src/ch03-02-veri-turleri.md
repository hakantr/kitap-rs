## Veri Türleri

Rust'taki her değer, Rust'a ne tür bir veri belirtildiğini söyleyen belirli bir _veri türüne_ (data type) sahiptir, böylece o veriyle nasıl çalışacağını bilir. İki veri türü alt kümesini inceleyeceğiz: skaler (scalar) ve bileşik (compound).

Rust'ın _statik tipli_ (statically typed) bir dil olduğunu aklınızda bulundurun; bu, derleme zamanında tüm değişkenlerin türlerini bilmesi gerektiği anlamına gelir. Derleyici genellikle değere ve onu nasıl kullandığımıza bağlı olarak hangi türü kullanmak istediğimizi çıkarabilir. Bölüm 2'deki ["Tahmini Gizli Sayıyla Karşılaştırmak"][comparing-the-guess-to-the-secret-number]<!-- ignore --> kısmında `parse` kullanarak bir `String`'i sayısal bir türe dönüştürdüğümüz zaman olduğu gibi birçok türün mümkün olduğu durumlarda, şu şekilde bir tür bildirimi eklemeliyiz:

```rust
let tahmin: u32 = "42".parse().expect("Bir sayı değil!");
```

Önceki kodda gösterilen `: u32` tür bildirimini eklemezsek, Rust aşağıdaki hatayı görüntüler; bu da derleyicinin hangi türü kullanmak istediğimizi bilmek için bizden daha fazla bilgiye ihtiyacı olduğu anlamına gelir:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Diğer veri türleri için farklı tür bildirimleri göreceksiniz.

### Skaler Türler (Scalar Types)

Bir _skaler_ tür, tek bir değeri temsil eder. Rust'ın dört ana skaler türü vardır: tam sayılar (integers), kayan noktalı sayılar (floating-point numbers), Boolean'lar ve karakterler. Bunları diğer programlama dillerinden tanıyabilirsiniz. Rust'ta nasıl çalıştıklarına bir göz atalım.

#### Tam Sayı Türleri (Integer Types)

Bir _tam sayı_, kesirli bileşeni olmayan bir sayıdır. Bölüm 2'de bir tam sayı türü kullandık: `u32` türü. Bu tür bildirimi, ilişkilendirildiği değerin 32 bitlik alan kaplayan işaretsiz (unsigned) bir tam sayı olması gerektiğini gösterir (işaretli (signed) tam sayı türleri `u` yerine `i` ile başlar). Tablo 3-1, Rust'taki yerleşik tam sayı türlerini gösterir. Bir tam sayı değerinin türünü bildirmek için bu varyantlardan (variants) herhangi birini kullanabiliriz.

<span class="caption">Tablo 3-1: Rust'taki Tam Sayı Türleri</span>

| Uzunluk | İşaretli | İşaretsiz |
| ------- | -------- | --------- |
| 8-bit   | `i8`     | `u8`      |
| 16-bit  | `i16`    | `u16`     |
| 32-bit  | `i32`    | `u32`     |
| 64-bit  | `i64`    | `u64`     |
| 128-bit | `i128`   | `u128`    |
| Mimariye bağlı | `isize`  | `usize`   |

Her varyant işaretli veya işaretsiz olabilir ve açık bir boyuta sahiptir. _İşaretli_ ve _işaretsiz_ terimleri, sayının negatif olmasının mümkün olup olmadığını ifade eder; diğer bir deyişle, sayının bir işaret alması gerekip gerekmediğini (işaretli) veya sadece pozitif olup bu nedenle işaretsiz olarak temsil edilip edilemeyeceğini (işaretsiz) belirtir. Tıpkı kağıt üzerine sayı yazmak gibidir: İşaret önemli olduğunda bir sayı artı veya eksi işaretiyle gösterilir; ancak sayının pozitif olduğunu varsaymak güvenliyse, herhangi bir işaret olmadan gösterilir. İşaretli sayılar [ikiye tümleyen][twos-complement]<!-- ignore --> (two's complement) temsili kullanılarak saklanır.

Her işaretli varyant −(2<sup>n − 1</sup>) ile 2<sup>n − 1</sup> − 1 (dahil) arasında sayılar saklayabilir; buradaki _n_, o varyantın kullandığı bit sayısıdır. Dolayısıyla, bir `i8`, −(2<sup>7</sup>) ile 2<sup>7</sup> − 1 arasında sayılar saklayabilir, bu da -128 ile 127 arasına eşittir. İşaretsiz varyantlar ise 0 ile 2<sup>n</sup> − 1 arasında sayılar saklayabilir, yani bir `u8`, 0 ile 2<sup>8</sup> − 1 arasında sayılar saklayabilir ki bu da 0 ile 255 arasına eşittir.

Ek olarak, `isize` ve `usize` türleri, programınızın çalıştığı bilgisayarın mimarisine bağlıdır: 64 bitlik bir mimarideyseniz 64 bit ve 32 bitlik bir mimarideyseniz 32 bittir.

Tam sayı sabitlerini (literals) Tablo 3-2'de gösterilen formlardan herhangi birinde yazabilirsiniz. Birden fazla sayısal tür olabilen sayı sabitlerinin, türü belirlemek için `57u8` gibi bir tür son ekine (suffix) izin verdiğini unutmayın. Sayı sabitleri, sayıyı okumayı kolaylaştırmak için görsel bir ayırıcı olarak `_` kullanabilir; örneğin `1_000`, `1000` belirtmişsiniz gibi aynı değere sahip olacaktır.

<span class="caption">Tablo 3-2: Rust'taki Tam Sayı Sabitleri</span>

| Sayı sabitleri   | Örnek         |
| ---------------- | ------------- |
| Onluk (Decimal)  | `98_222`      |
| Onaltılık (Hex)  | `0xff`        |
| Sekizlik (Octal) | `0o77`        |
| İkilik (Binary)  | `0b1111_0000` |
| Bayt (sadece `u8`) | `b'A'`        |

Peki hangi tür tam sayıyı kullanacağınızı nasıl bileceksiniz? Eğer emin değilseniz, Rust'ın varsayılanları genellikle başlamak için iyi yerlerdir: Tam sayı türleri varsayılan olarak `i32`'dir. `isize` veya `usize` kullanacağınız ana durum, bir çeşit koleksiyonu indekslediğiniz zamandır.

> ##### Tam Sayı Taşması (Integer Overflow)
>
> Diyelim ki 0 ile 255 arasında değerler alabilen `u8` türünde bir değişkeniniz var. Değişkeni bu aralığın dışındaki bir değere (örneğin 256'ya) değiştirmeye çalışırsanız, _tam sayı taşması_ (integer overflow) meydana gelir ve bu da iki davranıştan biriyle sonuçlanabilir. Hata ayıklama modunda derleme yaptığınızda Rust, tam sayı taşması için kontroller içerir ve bu davranış gerçekleştiğinde programınızın çalışma zamanında _panik_ yapmasına neden olur. Rust, bir program bir hatayla çıktığında (exits) _panikleme_ (panicking) terimini kullanır; panikleri Bölüm 9'daki ["`panic!` ile Kurtarılamaz Hatalar"][unrecoverable-errors-with-panic]<!-- ignore --> kısmında daha derinlemesine tartışacağız.
>
> `--release` bayrağıyla yayın (release) modunda derleme yaptığınızda, Rust paniğe neden olan tam sayı taşması kontrollerini _içermez_. Bunun yerine, taşma meydana gelirse Rust _ikiye tümleyerek sarma_ (two's complement wrapping) işlemi gerçekleştirir. Kısacası, türün alabileceği maksimum değerden daha büyük değerler, türün alabileceği değerlerin minimumuna "sarılır" (wrap around). Bir `u8` durumunda, 256 değeri 0 olur, 257 değeri 1 olur ve bu böyle devam eder. Program panik yapmaz, ancak değişkenin muhtemelen olmasını beklediğiniz gibi olmayan bir değeri olur. Tam sayı taşmasının sarma davranışına güvenmek bir hata olarak kabul edilir.
>
> Taşma olasılığını açıkça (explicitly) yönetmek için, ilkel sayısal türler için standart kütüphane tarafından sağlanan şu metot ailelerini kullanabilirsiniz:
>
> - `wrapping_add` gibi `wrapping_*` metotlarıyla tüm modlarda sarma.
> - Taşma varsa `checked_*` metotlarıyla `None` değerini döndürme.
> - `overflowing_*` metotlarıyla değeri ve taşma olup olmadığını gösteren bir Boolean döndürme.
> - `saturating_*` metotlarıyla değerin minimum veya maksimum değerlerinde doygunluğa ulaşma (saturate).

#### Kayan Noktalı (Floating-Point) Türler

Rust ayrıca ondalık noktaları olan _kayan noktalı sayılar_ için iki ilkel (primitive) türe sahiptir. Rust'ın kayan noktalı türleri, sırasıyla 32 bit ve 64 bit boyutunda olan `f32` ve `f64`'tür. Varsayılan tür `f64`'tür çünkü modern CPU'larda kabaca `f32` ile aynı hızdadır ancak daha fazla hassasiyete (precision) sahiptir. Tüm kayan noktalı türler işaretlidir (signed).

İşte kayan noktalı sayıları iş başında gösteren bir örnek:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Kayan noktalı sayılar IEEE-754 standardına göre temsil edilir.

#### Sayısal İşlemler

Rust, tüm sayı türleri için beklediğiniz temel matematiksel işlemleri destekler: toplama, çıkarma, çarpma, bölme ve kalan (remainder). Tam sayı bölmesi (integer division) en yakın tam sayıya doğru (sıfıra doğru) keser (truncate). Aşağıdaki kod, `let` ifadesinde her bir sayısal işlemi nasıl kullanacağınızı gösterir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Bu ifadelerdeki (statements) her bir ifade matematiksel bir operatör kullanır ve tek bir değere dönüşür (evaluates), bu değer daha sonra bir değişkene bağlanır. [Ek B][appendix_b]<!-- ignore -->, Rust'ın sağladığı tüm operatörlerin bir listesini içerir.

#### Boolean Türü

Çoğu diğer programlama dilinde olduğu gibi, Rust'taki Boolean türünün iki olası değeri vardır: `true` ve `false`. Boolean'ların boyutu bir bayttır. Rust'taki Boolean türü `bool` kullanılarak belirtilir. Örneğin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Boolean değerlerini kullanmanın ana yolu `if` ifadesi gibi koşullu yapılardır (conditionals). `if` ifadelerinin Rust'ta nasıl çalıştığını ["Kontrol Akışı"][control-flow]<!-- ignore --> bölümünde ele alacağız.

#### Karakter (Character) Türü

Rust'ın `char` türü, dilin en temel alfabetik türüdür. İşte `char` değerlerini bildirmeye ilişkin bazı örnekler:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Çift tırnak kullanan metin sabitlerinin (string literals) aksine, `char` sabitlerini tek tırnakla belirttiğimize dikkat edin. Rust'ın `char` türü 4 bayt boyutundadır ve bir Unicode skaler değerini temsil eder; bu da onun sadece ASCII'den çok daha fazlasını temsil edebileceği anlamına gelir. Aksanlı harfler; Çince, Japonca ve Korece karakterler; emojiler; ve sıfır genişlikli boşluklar (zero-width spaces) Rust'ta geçerli `char` değerleridir. Unicode skaler değerleri `U+0000` ile `U+D7FF` ve `U+E000` ile `U+10FFFF` (dahil) arasında değişir. Ancak, "karakter" aslında Unicode'da tam olarak karşılığı olan bir kavram değildir, dolayısıyla bir "karakterin" ne olduğuna dair insani sezginiz (intuition) Rust'taki bir `char` ile örtüşmeyebilir. Bu konuyu Bölüm 8'deki ["Metinleri (Strings) Kullanarak UTF-8 Kodlu Metinleri Saklamak"][strings]<!-- ignore --> kısmında ayrıntılı olarak tartışacağız.

### Bileşik Türler (Compound Types)

_Bileşik türler_ birden fazla değeri tek bir türde gruplayabilir. Rust'ın iki ilkel bileşik türü vardır: tuple'lar ve array'ler (diziler).

#### Tuple Türü

Bir _tuple_, çeşitli türlere sahip bir dizi değeri tek bir bileşik türde (compound type) bir arada gruplamanın genel bir yoludur. Tuple'ların sabit bir uzunluğu vardır: Bir kez bildirildiklerinde boyutları büyüyemez veya küçülemez.

Değerlerin virgülle ayrılmış bir listesini parantez içine yazarak bir tuple oluştururuz. Tuple'daki her pozisyonun (konum) bir türü vardır ve tuple'daki farklı değerlerin türlerinin aynı olması gerekmez. Bu örnekte isteğe bağlı tür bildirimleri ekledik:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

Bir tuple tek bir bileşik eleman olarak kabul edildiği için `tup` değişkeni tüm tuple'a bağlanır (binds). Bireysel değerleri tuple'dan çıkarmak için, bir tuple değerini parçalamak (destructure) amacıyla desen eşleştirmeyi şu şekilde kullanabiliriz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Bu program önce bir tuple oluşturur ve onu `tup` değişkenine bağlar. Daha sonra `tup`'ı alıp onu üç ayrı değişkene (`x`, `y` ve `z`) dönüştürmek için `let` ile birlikte bir desen kullanır. Tek bir tuple'ı üç parçaya böldüğü için buna _parçalama_ (destructuring) denir. Son olarak program, `6.4` olan `y` değerini yazdırır.

Bir tuple elemanına doğrudan bir nokta (`.`) ve ardından erişmek istediğimiz değerin indeksini (index) kullanarak da erişebiliriz. Örneğin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Bu program `x` tuple'ını oluşturur ve daha sonra kendi indekslerini kullanarak tuple'ın her bir elemanına erişir. Çoğu programlama dilinde olduğu gibi, tuple'daki ilk indeks 0'dır.

Hiçbir değere sahip olmayan tuple'ın özel bir adı vardır, _birim_ (unit). Bu değer ve onun karşılık gelen türü de `()` olarak yazılır ve boş bir değeri veya boş bir dönüş türünü temsil eder. İfadeler (expressions), başka bir değer döndürmezlerse kapalı olarak (implicitly) birim değerini döndürürler.

#### Array (Dizi) Türü

Birden fazla değere sahip bir koleksiyon oluşturmanın başka bir yolu da _array_ (dizi) kullanmaktır. Bir tuple'dan farklı olarak, bir array'in her elemanı aynı türde olmalıdır. Diğer bazı dillerdeki array'lerin aksine, Rust'taki array'lerin sabit bir uzunluğu vardır.

Bir array'deki değerleri köşeli parantezler içinde virgülle ayrılmış bir liste olarak yazarız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Array'ler, verilerinizin şimdiye kadar gördüğümüz diğer türlerdeki gibi heap (yığın) alanı yerine stack (yığıt) üzerinde tahsis edilmesini (allocated) istediğinizde ([Bölüm 4'te][stack-and-heap]<!-- ignore --> stack ve heap'i daha ayrıntılı tartışacağız) veya her zaman sabit sayıda elemana sahip olduğunuzdan emin olmak istediğinizde kullanışlıdır. Ancak bir array, bir vektör (vector) türü kadar esnek değildir. Bir vektör, standart kütüphane tarafından sağlanan ve boyutu büyüyebilen veya küçülebilen benzer bir koleksiyon türüdür çünkü içeriği heap üzerinde yaşar. Array mi yoksa vektör mü kullanacağınızdan emin değilseniz, büyük olasılıkla bir vektör kullanmalısınız. [Bölüm 8][vectors]<!-- ignore --> vektörleri daha detaylı tartışır.

Bununla birlikte, eleman sayısının değişmeyeceğini bildiğiniz durumlarda array'ler daha kullanışlıdır. Örneğin, bir programda ayların isimlerini kullanıyor olsaydınız, her zaman 12 eleman içereceğini bildiğiniz için muhtemelen bir vektör yerine bir array kullanırdınız:

```rust
let aylar = ["Ocak", "Şubat", "Mart", "Nisan", "Mayıs", "Haziran", "Temmuz",
              "Ağustos", "Eylül", "Ekim", "Kasım", "Aralık"];
```

Bir array'in türünü yazarken köşeli parantezler içinde her bir elemanın türünü, ardından noktalı virgül ve dizideki eleman sayısını şu şekilde yazarsınız:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Burada, her elemanın türü `i32`'dir. Noktalı virgülden sonra gelen `5` sayısı, dizinin beş eleman içerdiğini belirtir.

Her bir eleman için aynı değere sahip bir array'i şu şekilde başlatabilirsiniz: önce başlangıç değerini, ardından bir noktalı virgülü ve ardından köşeli parantezler içindeki dizinin uzunluğunu belirtebilirsiniz:

```rust
let a = [3; 5];
```

`a` adındaki dizi, başlangıçta tamamı `3` değerine ayarlanacak olan `5` eleman içerecektir. Bu, `let a = [3, 3, 3, 3, 3];` yazmakla aynı şeydir ancak daha kısa bir yoldur.

<!-- Old headings. Do not remove or links may break. -->
<a id="accessing-array-elements"></a>

#### Array Elemanlarına Erişim

Bir array, stack üzerinde tahsis edilebilen bilinen, sabit boyutlu tek bir bellek yığınıdır (chunk of memory). İndekslemeyi kullanarak array elemanlarına şu şekilde erişebilirsiniz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

Bu örnekte, `ilk` adındaki değişken dizideki `[0]` indeksindeki değer olduğu için `1` değerini alacaktır. `ikinci` adındaki değişken dizideki `[1]` indeksinden `2` değerini alacaktır.

#### Geçersiz Array Elemanlarına Erişim (Invalid Array Element Access)

Bir array'in sonundan daha ileri bir noktasındaki (past the end of the array) bir elemanına erişmeye çalışırsanız ne olacağına bakalım. Kullanıcıdan bir array indeksi almak için Bölüm 2'deki tahmin oyununa benzer şekilde bu kodu çalıştırdığınızı varsayalım:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Bu kod başarıyla derlenir. Bu kodu `cargo run` komutunu kullanarak çalıştırırsanız ve `0`, `1`, `2`, `3` veya `4` değerlerinden birini girerseniz, program array'deki o indekse karşılık gelen değeri yazdıracaktır. Eğer dizinin sonunu geçen bir sayı (örneğin `10`) girerseniz, şöyle bir çıktı göreceksiniz:

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

Program, indeksleme işleminde geçersiz bir değer kullanıldığı noktada bir çalışma zamanı hatası verdi. Program bir hata mesajıyla kapandı ve son `println!` ifadesini çalıştırmadı. İndekslemeyi kullanarak bir elemana erişmeye çalıştığınızda Rust, belirttiğiniz indeksin dizi uzunluğundan daha az olup olmadığını kontrol edecektir. İndeks dizi uzunluğundan büyük veya ona eşitse, Rust paniğe neden olur. Bu kontrolün çalışma zamanında gerçekleşmesi gerekir, özellikle de bu durumda, çünkü derleyicinin bir kullanıcının kodu çalıştırdığında ne değer gireceğini bilmesi imkansızdır.

Bu, Rust'ın bellek güvenliği prensiplerinin iş başındaki bir örneğidir. Birçok alt seviye dilde bu tür bir kontrol yapılmaz ve yanlış bir indeks sağladığınızda geçersiz belleğe erişilebilir. Rust, bellek erişimine izin verip devam etmek yerine hemen programdan çıkarak sizi bu tür hatalara karşı korur. Bölüm 9, Rust'ın hata yönetimini ve panik yaratmayan ya da geçersiz bellek erişimine izin vermeyen okunabilir, güvenli kodu nasıl yazabileceğinizi daha ayrıntılı tartışacaktır.

[comparing-the-guess-to-the-secret-number]: ch02-00-tahmin-oyunu-programlama.html#tahmini-gizli-sayıyla-karşılaştırmak
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-kontrol-akisi.html#kontrol-akışı-control-flow
[strings]: ch08-02-stringler.html#stringlerle-utf-8-kodlanmış-metin-depolamak
[stack-and-heap]: ch04-01-sahiplik-nedir.html#yığın-ve-yığın-bellek-the-stack-and-the-heap
[vectors]: ch08-01-vektorler.html
[unrecoverable-errors-with-panic]: ch09-01-panic-ile-kurtarilamayan-hatalar.html
[appendix_b]: ekler-02-operatorler.md
