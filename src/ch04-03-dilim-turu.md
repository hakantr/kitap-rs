## Dilim Türü (The Slice Type)

_Dilimler_, bir [koleksiyondaki](ch08-00-yaygin-koleksiyonlar.md)<!-- ignore --> bitişik bir eleman dizisine referans vermenizi sağlar. Bir dilim bir tür referanstır, bu nedenle sahipliği yoktur.

İşte küçük bir programlama problemi: Boşluklarla ayrılmış kelimelerden oluşan bir metin (string) alan ve o metinde bulduğu ilk kelimeyi döndüren bir fonksiyon yazın. Eğer fonksiyon metin içinde boşluk bulamazsa, tüm metin tek bir kelime olmalıdır, bu yüzden metnin tamamı döndürülmelidir.

> Not: Dilimleri tanıtmak amacıyla, bu bölümde yalnızca ASCII karakterleri olduğunu varsayıyoruz; UTF-8 kullanımı ile ilgili daha kapsamlı bir tartışma Bölüm 8'deki ["UTF-8 Kodlanmış Metni Stringler ile Saklamak"][strings]<!-- ignore --> bölümünde yer almaktadır.

Dilimlerin çözeceği problemi anlamak için, dilim kullanmadan bu fonksiyonun imzasını nasıl yazacağımız üzerinde çalışalım:

```rust,ignore
fn ilk_kelime(metin: &String) -> ?
```

`ilk_kelime` fonksiyonunun `&String` türünde bir parametresi var. Sahipliğe ihtiyacımız yok, o yüzden bu gayet uygundur. (İdiyomatik Rust'ta, fonksiyonlar ihtiyaç duymadıkça argümanlarının sahipliğini almazlar ve bunun nedenleri ilerledikçe netleşecektir.) Peki ne döndürmeliyiz? Bir metnin *bir kısmından* bahsetmenin gerçekten bir yolu yok. Ancak, bir boşlukla belirtilen kelimenin sonunun indeksini döndürebiliriz. Liste 4-7'de gösterildiği gibi bunu deneyelim.

<Listing number="4-7" file-name="src/main.rs" caption="`String` parametresine karşılık bir bayt indeksi değeri döndüren `ilk_kelime` fonksiyonu">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

</Listing>

`String`'i eleman eleman incelememiz ve bir değerin boşluk olup olmadığını kontrol etmemiz gerektiğinden, `as_bytes` metodunu kullanarak `String`'imizi bir bayt dizisine dönüştürüyoruz.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:as_bytes}}
```

Daha sonra, `iter` metodunu kullanarak bayt dizisi üzerinde bir yineleyici oluşturuyoruz:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:iter}}
```

Yineleyicileri [Bölüm 13][ch13]<!-- ignore -->'te daha detaylı tartışacağız. Şimdilik `iter`'in bir koleksiyondaki her elemanı döndüren bir metot olduğunu ve `enumerate`'in `iter` sonucunu sarmaladığını ve bunun yerine her elemanı bir demetin parçası olarak döndürdüğünü bilin. `enumerate`'ten döndürülen demetin ilk elemanı indekstir ve ikinci eleman elemana bir referanstır. Bu, indeksi kendimiz hesaplamaktan biraz daha uygundur.

`enumerate` metodu bir demet döndürdüğü için, o demeti ayrıştırmak (destructure) amacıyla desenleri (patterns) kullanabiliriz. Desenler hakkında [Bölüm 6][ch6]<!-- ignore -->'da daha fazla konuşacağız. `for` döngüsünde, demetteki indeks için `i` ve demetteki tek bayt için `&oge` olan bir desen belirtiyoruz. `.iter().enumerate()`'ten elemanın bir referansını aldığımız için desende `&` kullanırız.

`for` döngüsünün içinde, bayt literali (byte literal) sözdizimini kullanarak boşluğu temsil eden baytı ararız. Eğer bir boşluk bulursak, o konumu döndürürüz. Aksi takdirde, `metin.len()` kullanarak metnin uzunluğunu döndürürüz.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:inside_for}}
```

Artık metindeki ilk kelimenin sonunun indeksini bulmanın bir yoluna sahibiz, ancak bir sorun var. Tek başına bir `usize` döndürüyoruz, ancak bu yalnızca `&String` bağlamında anlamlı bir sayıdır. Başka bir deyişle, `String`'den ayrı bir değer olduğu için gelecekte de geçerli kalacağının garantisi yoktur. Liste 4-7'deki `ilk_kelime` fonksiyonunu kullanan Liste 4-8'deki programı inceleyelim.

<Listing number="4-8" file-name="src/main.rs" caption="`ilk_kelime` fonksiyonunu çağırmaktan dönen sonucu saklayıp ardından `String` içeriğini değiştirmek">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-08/src/main.rs:here}}
```

</Listing>

Bu program hiçbir hata olmadan derlenir ve `metin.clear()` çağırdıktan sonra `kelime`'yi kullansaydık da derlenirdi. `kelime` `metin`'in durumuna hiç bağlı olmadığı için, `kelime` hala `7` (veya `"merhaba"` kelimesinin uzunluğu) değerini içerir. İlk kelimeyi çıkarmayı denemek için `metin` değişkeni ile birlikte bu `7` değerini kullanabilirdik, ancak bu bir bug olurdu çünkü `kelime`'de `7`'yi kaydettiğimizden bu yana `metin`'in içerikleri değişmiştir.

`kelime`'deki indeksin `metin`'deki verilerle senkronizasyonunun (uyumunun) bozulması konusunda endişelenmek yorucu ve hataya açıktır! Eğer bir `ikinci_kelime` fonksiyonu yazarsak bu indeksleri yönetmek daha da kırılgan (brittle) bir hal alır. İmzası şuna benzerdi:

```rust,ignore
fn ikinci_kelime(metin: &String) -> (usize, usize) {
```

Artık bir başlangıç _ve_ bir bitiş indeksini izliyoruz; belirli bir durumdaki verilerden hesaplanmış ancak o duruma hiçbir şekilde bağlı olmayan daha da fazla değerimiz var. Etrafta dolaşan ve senkronize tutulması gereken, birbiriyle ilgisiz üç değişkenimiz var.

Neyse ki Rust'ın bu soruna bir çözümü var: string dilimleri.

### String Dilimleri (String Slices)

Bir _string dilimi_, bir `String`'in elemanlarının bitişik bir dizisine referanstır ve şuna benzer:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-17-slice/src/main.rs:here}}
```

Tüm `String`'e referans vermek yerine, `merhaba`, ekstra `[0..7]` bölümünde belirtilen `String`'in bir kısmına referanstır. Dilimleri `[başlangıç_indeksi..bitiş_indeksi]` belirterek köşeli parantezler içinde bir aralık kullanarak oluştururuz; burada _`başlangıç_indeksi`_ dilimdeki ilk konumdur ve _`bitiş_indeksi`_ dilimdeki son konumun bir fazlasıdır. Dahili olarak, dilim veri yapısı dilimin başlangıç konumunu ve uzunluğunu depolar; bu uzunluk _`bitiş_indeksi`_ eksi _`başlangıç_indeksi`_ değerine karşılık gelir. Yani, `let dunya = &metin[8..13];` durumunda `dunya`, `metin`'in 8. indeksindeki bayta bir işaretçi ve `5` uzunluk değeri içeren bir dilim olacaktır.

Şekil 4-7 bunu bir diyagram halinde göstermektedir.

<img alt="Üç tablo: metin (s) için stack verilerini temsil eden bir tablo, heap üzerindeki &quot;merhaba dünya&quot; string verileri tablosunda 0. indeksteki bayta işaret eder. Üçüncü tablo ise dunya (world) diliminin stack verilerini temsil eder; bu tablo 5 uzunluk değerine sahiptir ve heap veri tablosunun 8. baytına işaret eder."
src="img/trpl04-07.svg" class="center" style="width: 50%;" />

<span class="caption">Şekil 4-7: `String`'in bir parçasına atıfta bulunan bir string dilimi</span>

Rust'ın `..` aralık sözdizimi ile, eğer indeks 0'dan başlamak istiyorsanız, iki noktadan önceki değeri bırakabilirsiniz. Başka bir deyişle, bunlar birbirine eşittir:

```rust
let metin = String::from("merhaba");

let dilim = &metin[0..2];
let dilim = &metin[..2];
```

Aynı şekilde, diliminiz `String`'in son baytını içeriyorsa, sondaki sayıyı da bırakabilirsiniz. Bu, şunların eşit olduğu anlamına gelir:

```rust
let metin = String::from("merhaba");

let uzunluk = metin.len();

let dilim = &metin[3..uzunluk];
let dilim = &metin[3..];
```

Tüm metnin bir dilimini almak için her iki değeri de bırakabilirsiniz. Dolayısıyla bunlar da eşittir:

```rust
let metin = String::from("merhaba");

let uzunluk = metin.len();

let dilim = &metin[0..uzunluk];
let dilim = &metin[..];
```

> Not: String dilim aralığı indeksleri geçerli UTF-8 karakter sınırlarında gerçekleşmelidir. Çok baytlı (multibyte) bir karakterin ortasında bir string dilimi oluşturmaya çalışırsanız, programınız bir hatayla çıkış yapacaktır.

Tüm bu bilgileri göz önünde bulundurarak, `ilk_kelime`'yi bir dilim döndürecek şekilde yeniden yazalım. "String dilimi"ni ifade eden tür `&str` olarak yazılır:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

</Listing>

Kelimenin sonu için indeksi Liste 4-7'de yaptığımız gibi, bir boşluğun ilk geçtiği yeri arayarak elde ederiz. Bir boşluk bulduğumuzda, metnin başlangıcını ve boşluğun indeksini sırasıyla başlangıç ve bitiş indeksleri olarak kullanarak bir string dilimi döndürürüz.

Artık `ilk_kelime`'yi çağırdığımızda, temel alınan (underlying) verilere bağlı tek bir değer geri alıyoruz. Bu değer, dilimin başlangıç noktasına yönelik bir referans ile dilimdeki eleman sayısından oluşur.

Bir dilim döndürmek `ikinci_kelime` fonksiyonu için de işe yarayacaktır:

```rust,ignore
fn ikinci_kelime(metin: &String) -> &str {
```

Derleyici `String` içindeki referansların geçerli kalmasını sağlayacağından artık bozması çok daha zor olan basit bir API'ye sahibiz. Liste 4-8'deki programda yer alan ve ilk kelimenin sonunun indeksini aldığımız, ancak daha sonra metni boşalttığımızda indeksimizin geçersiz hale geldiği hatayı hatırlıyor musunuz? O kod mantıksal olarak yanlıştı ancak anında herhangi bir hata göstermemişti. İlk kelime indeksini boşaltılmış bir metinle birlikte kullanmaya devam etseydik, sorunlar daha sonra ortaya çıkacaktı. Dilimler bu hatayı imkansız kılar ve kodumuzla ilgili bir sorunumuz olduğunu çok daha erken bilmemizi sağlar. `ilk_kelime`'nin dilim sürümünü kullanmak bir derleme zamanı hatası fırlatır:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/src/main.rs:here}}
```

</Listing>

İşte derleyici hatası:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/output.txt}}
```

Ödünç alma kurallarını hatırlayın: Eğer bir şeye değiştirilemez bir referansımız varsa, ona ayrıca değiştirilebilir bir referans alamayız. `clear`'ın `String`'i kesmesi (truncate) gerektiği için değiştirilebilir bir referans alması gerekir. `clear` çağrısından sonraki `println!`, `kelime`'deki referansı kullanır, dolayısıyla o noktada değiştirilemez referansın hala aktif olması gerekir. Rust, `clear`'daki değiştirilebilir referans ile `kelime`'deki değiştirilemez referansın aynı anda var olmasına izin vermez ve derleme başarısız olur. Rust yalnızca API'mizi daha kolay kullanılır hale getirmekle kalmadı, aynı zamanda bütün bir hata sınıfını derleme zamanında ortadan kaldırdı!

<!-- Old headings. Do not remove or links may break. -->

<a id="string-literals-are-slices"></a>

#### Dilim Olarak Sabit Metinler (String Literals as Slices)

Sabit metinlerin (string literals) ikili dosya içinde saklandığından bahsettiğimizi hatırlayın. Artık dilimleri bildiğimize göre sabit metinleri uygun bir şekilde anlayabiliriz:

```rust
let metin = "Merhaba, dünya!";
```

Buradaki `metin` değişkeninin türü `&str`'dir: İkili dosyanın belirli bir noktasına işaret eden bir dilimdir. Sabit metinlerin değiştirilemez olmasının nedeni de budur; `&str` değiştirilemez bir referanstır.

#### Parametre Olarak String Dilimleri

Sabitlerin (literals) ve `String` değerlerinin dilimlerini alabileceğinizi bilmek, `ilk_kelime` üzerinde yapacağımız bir iyileştirmeye, yani onun imzasına yönlendirir bizi:

```rust,ignore
fn ilk_kelime(metin: &String) -> &str {
```

Daha tecrübeli bir Rust geliştiricisi bunun yerine Liste 4-9'da gösterilen imzayı yazardı, çünkü bu imza aynı fonksiyonu hem `&String` hem de `&str` değerleri üzerinde kullanmamıza olanak tanır.

<Listing number="4-9" caption="`metin` parametresi için bir string dilimi kullanarak `ilk_kelime` fonksiyonunu iyileştirme">

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:here}}
```

</Listing>

Eğer elimizde bir string dilimi varsa, bunu doğrudan geçirebiliriz. Eğer bir `String`'imiz varsa, bu `String`'in bir dilimini veya `String`'in referansını geçirebiliriz. Bu esneklik, Bölüm 15'teki ["Fonksiyonlarda ve Metotlarda Deref Zorlamalarını Kullanma (Using Deref Coercions in Functions and Methods)"][deref-coercions]<!-- ignore --> bölümünde ele alacağımız deref zorlamaları (deref coercions) özelliğinden yararlanır.

Bir fonksiyonu `String` referansı yerine bir string dilimi alacak şekilde tanımlamak, API'mizi hiçbir işlevselliğini kaybetmeden daha genel ve kullanışlı hale getirir:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:usage}}
```

</Listing>

### Diğer Dilimler (Other Slices)

String dilimleri tahmin edebileceğiniz gibi stringlere özgüdür. Ancak daha genel bir dilim türü de vardır. Şu diziyi (array) ele alalım:

```rust
let d = [1, 2, 3, 4, 5];
```

Tıpkı bir stringin bir kısmına başvurmak (referans vermek) isteyebileceğimiz gibi, bir dizinin bir kısmına da başvurmak isteyebiliriz. Bunu şu şekilde yapardık:

```rust
let d = [1, 2, 3, 4, 5];

let dilim = &d[1..3];

assert_eq!(dilim, &[2, 3]);
```

Bu dilim `&[i32]` türündedir. İlk elemana bir referans ve bir uzunluk depolayarak tıpkı string dilimlerinin çalıştığı gibi çalışır. Bu tür dilimleri her türlü diğer koleksiyon için kullanacaksınız. Bu koleksiyonları, Bölüm 8'de vektörler hakkında konuştuğumuzda ayrıntılı olarak tartışacağız.

## Özet

Sahiplik, ödünç alma ve dilimler kavramları Rust programlarında bellek güvenliğini (memory safety) derleme zamanında sağlar. Rust dili, tıpkı diğer sistem programlama dilleri gibi bellek kullanımınız üzerinde size kontrol verir. Ancak, veri sahibinin kapsam dışına çıktığında o veriyi otomatik olarak temizlemesi, bu kontrolü elde etmek için ekstra kod yazıp ayıklamak zorunda olmadığınız anlamına gelir.

Sahiplik, Rust'ın diğer pek çok parçasının çalışma şeklini etkiler, bu nedenle kitabın geri kalanında bu kavramlar hakkında daha fazla konuşacağız. Şimdi Bölüm 5'e geçelim ve veri parçalarını bir `struct` içinde gruplandırmaya bakalım.

[ch13]: ch13-02-iteratorler.html
[ch6]: ch06-02-match-kontrol-akisi.html#değerlere-bağlanan-desenler
[strings]: ch08-02-stringler.html#stringlerle-utf-8-kodlanmış-metin-depolamak
[deref-coercions]: ch15-02-deref.html#fonksiyon-ve-metotlarda-deref-zorlaması-kullanmak
