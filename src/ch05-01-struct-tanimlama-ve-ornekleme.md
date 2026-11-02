<!-- Old headings. Do not remove or links may break. -->
<a id="defining-and-instantiating-structs"></a>
<a id="structları-tanımlama-ve-örnekleme-defining-and-instantiating-structs"></a>

## Struct Tanımlama ve Örnekleme

Struct'lar, birbiriyle ilişkili birden fazla değeri bir arada tutmaları bakımından ["Demet Türü"][tuples]<!-- ignore --> bölümünde ele aldığımız demetlere benzer. Tıpkı demetlerde olduğu gibi, bir struct'ın parçaları farklı türlerde olabilir. Ancak demetlerden farklı olarak struct içinde her bir veri parçasına bir ad verirsiniz; böylece değerlerin ne anlama geldiği açıkça anlaşılır. Bu isimlendirme sayesinde struct'lar demetlerden daha esnektir: Bir örneğin değerlerini belirtmek ya da onlara erişmek için verilerin sırasına bağımlı kalmazsınız.

Bir struct tanımlamak için `struct` anahtar kelimesini yazar ve struct'ın tamamına bir ad veririz. Struct'ın adı, bir araya getirilen veri parçalarının ortak anlamını ifade etmelidir. Ardından süslü parantezler içinde, _alan (field)_ olarak adlandırdığımız veri parçalarının adlarını ve türlerini tanımlarız. Örneğin Liste 5-1, bir kullanıcı hesabına ait bilgileri tutan bir struct tanımını göstermektedir.

<Listing number="5-1" file-name="src/main.rs" caption="Bir `Kullanici` struct'ı tanımı">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

</Listing>

Tanımladığımız bir struct'ı kullanmak için, her bir alana somut değerler vererek o struct'ın bir _örneğini (instance)_ oluştururuz. Örnek oluştururken önce struct'ın adını yazar, ardından içine _`anahtar: değer`_ çiftlerini yerleştirdiğimiz süslü parantezler açarız; buradaki anahtarlar alanların adları, değerler ise o alanlarda saklamak istediğimiz verilerdir. Alanları, struct tanımındaki bildirim sırasıyla yazmak zorunda değiliz. Struct tanımı, tür için genel bir şablon gibidir; örnekler ise bu şablonu belirli verilerle doldurarak o türe ait somut değerler üretir. Örneğin Liste 5-2'de gösterildiği gibi belirli bir kullanıcı tanımlayabiliriz.

<Listing number="5-2" file-name="src/main.rs" caption="`Kullanici` struct'ının bir örneğini oluşturma">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

</Listing>

Bir struct içinden belirli bir değeri okumak için nokta gösterimini (*dot notation*) kullanırız. Örneğin bu kullanıcının e-posta adresine erişmek istediğimizde `kullanici1.eposta` yazarız. Eğer örnek değiştirilebilir (`mut`) olarak tanımlanmışsa, nokta gösterimiyle belirli bir alana yeni bir değer atayarak o alanı güncelleyebiliriz. Liste 5-3, değiştirilebilir bir `Kullanici` örneğinin `eposta` alanındaki değerin nasıl değiştirileceğini göstermektedir.

<Listing number="5-3" file-name="src/main.rs" caption="Bir `Kullanici` örneğinin `eposta` alanındaki değeri değiştirme">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

</Listing>

Örneğin tamamının değiştirilebilir olması gerektiğine dikkat edin; Rust yalnızca belirli alanları değiştirilebilir olarak işaretlememize izin vermez. Struct örneği oluşturmak da bir ifade olduğundan, her ifadede olduğu gibi, fonksiyon gövdesinin son ifadesi olarak struct'ın yeni bir örneğini oluşturabilir ve bu örneği örtük olarak döndürebiliriz.

> Not: Rust'ta değiştirilebilirlik (`mut`), struct tanımındaki tekil alanların değil, örneği tutan değişken bağlamasının (veya `&mut` referansının) bir özelliğidir; bu nedenle `struct Kullanici { mut eposta: String }` gibi alan bazlı bir sözdizimi geçerli değildir. Bu tasarım sayesinde aynı `Kullanici` türünü kodun bir yerinde tamamen değiştirilemez (`let kullanici1 = ...`), başka bir yerinde ise değiştirilebilir (`let mut kullanici2 = ...`) olarak güvenle kullanabilirsiniz. Değiştirilemez bir referans arkasındayken yalnızca belirli bir alanı güncellemenin gerektiği özel senaryolar içinse Bölüm 15'te *içsel değiştirilebilirlik (interior mutability)* desenini (`Cell<T>` ve `RefCell<T>`) inceleyeceğiz.

Liste 5-4, verilen e-posta ve kullanıcı adıyla bir `Kullanici` örneği döndüren `kullanici_olustur` fonksiyonunu göstermektedir. `aktif` alanına `true`, `giris_sayisi` alanına ise `1` değeri atanır.

<Listing number="5-4" file-name="src/main.rs" caption="E-posta ve kullanıcı adı alıp bir `Kullanici` örneği döndüren `kullanici_olustur` fonksiyonu">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

</Listing>

Fonksiyon parametrelerine struct alanlarıyla aynı adları vermek mantıklıdır; ancak `eposta` ve `kullanici_adi` adlarını hem alan adı hem de değişken olarak ikişer kez yazmak biraz zahmetlidir. Struct daha fazla alana sahip olsaydı her bir adı bu şekilde yinelemek daha da can sıkıcı hâle gelirdi. Neyse ki bunun için kullanışlı bir kısayol vardır!

<!-- Old headings. Do not remove or links may break. -->
<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>
<a id="using-the-field-init-shorthand"></a>
<a id="alan-başlatma-kısaltmasını-kullanmak-using-the-field-init-shorthand"></a>
<a id="alan-başlatma-kısaltmasını-kullanma"></a>

### Alan Başlatma Kısayolunu Kullanma

Liste 5-4'teki parametre adları ile struct alan adları birebir aynı olduğundan, _alan başlatma kısayolu (field init shorthand)_ sözdizimini kullanarak `kullanici_olustur` fonksiyonunu yeniden yazabiliriz. Liste 5-5'teki yeni sürüm öncekiyle aynı şekilde çalışır, ancak `kullanici_adi` ve `eposta` adlarını tekrar etmez.

<Listing number="5-5" file-name="src/main.rs" caption="`kullanici_adi` ve `eposta` parametreleri struct alanlarıyla aynı ada sahip olduğu için alan başlatma kısayolunu kullanan `kullanici_olustur` fonksiyonu">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

</Listing>

Burada, `eposta` adında bir alanı bulunan `Kullanici` struct'ının yeni bir örneğini oluşturuyoruz. `eposta` alanının değerini, `kullanici_olustur` fonksiyonunun `eposta` parametresindeki değere eşitlemek istiyoruz. Alan adı ile parametre adı aynı olduğu için `eposta: eposta` yerine yalnızca `eposta` yazmamız yeterlidir.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-instances-from-other-instances-with-struct-update-syntax"></a>
<a id="creating-instances-with-struct-update-syntax"></a>
<a id="struct-güncelleme-sözdizimi-ile-örnekler-oluşturmak-creating-instances-with-struct-update-syntax"></a>

### Struct Güncelleme Sözdizimi ile Örnekler Oluşturma

Aynı türdeki mevcut bir örneğin değerlerinin çoğunu koruyup yalnızca birkaçını değiştirerek yeni bir struct örneği oluşturmak çoğu zaman işe yarar. Bunu _struct güncelleme sözdizimi (struct update syntax)_ ile yapabilirsiniz.

Öncelikle Liste 5-6'da, güncelleme sözdizimini kullanmadan sıradan yöntemle `kullanici2` içinde yeni bir `Kullanici` örneğini nasıl oluşturacağımızı görelim. `eposta` alanı için yeni bir değer belirliyor, geri kalan alanlarda ise Liste 5-2'de oluşturduğumuz `kullanici1` örneğindeki değerlerin aynısını kullanıyoruz.

<Listing number="5-6" file-name="src/main.rs" caption="`kullanici1`'deki değerlerden biri hariç tümünü kullanarak yeni bir `Kullanici` örneği oluşturma">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

</Listing>

Struct güncelleme sözdizimini kullanarak, Liste 5-7'de gösterildiği gibi aynı sonucu çok daha az kodla elde edebiliriz. `..` sözdizimi, açıkça değer verilmeyen geri kalan alanların belirtilen örnekteki karşılıklarıyla aynı değere sahip olacağını ifade eder.

<Listing number="5-7" file-name="src/main.rs" caption="Yeni bir `eposta` değeri belirleyip geri kalan alanları `kullanici1` örneğinden almak için struct güncelleme sözdizimini kullanma">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

</Listing>

Liste 5-7'deki kod da `kullanici2` içinde `eposta` alanı farklı, ancak `kullanici_adi`, `aktif` ve `giris_sayisi` alanları `kullanici1` ile aynı değerlere sahip bir örnek oluşturur. Kalan tüm alanların değerlerinin `kullanici1`'deki karşılık gelen alanlardan alınacağını belirtmek için `..kullanici1` en sonda yer almalıdır; ancak struct tanımındaki alan sırasından bağımsız olarak istediğimiz sayıda alana dilediğimiz sırada açıkça değer verebiliriz.

Struct güncelleme sözdiziminin tıpkı bir atama işlemi gibi `=` kullandığına dikkat edin; bunun nedeni, verileri ["Değişkenlerin ve Verilerin Taşıma ile Etkileşimi"][move]<!-- ignore --> bölümünde gördüğümüz şekilde taşımasıdır. Bu örnekte `kullanici2` oluşturulduktan sonra artık `kullanici1` örneğini bir bütün olarak kullanamayız; çünkü `kullanici1`'in `kullanici_adi` alanındaki `String` değeri `kullanici2` içine taşınmıştır. Eğer `kullanici2`'ye hem `eposta` hem de `kullanici_adi` için yeni `String` değerleri verseydik ve böylece `kullanici1` içinden yalnızca `aktif` ile `giris_sayisi` alanlarını alsaydık, `kullanici2` oluşturulduktan sonra `kullanici1` tamamen geçerli kalmaya devam ederdi. Hem `aktif` hem de `giris_sayisi`, `Copy` trait'ini uygulayan türlerde olduğundan ["Yalnızca Yığındaki Veri: Copy"][copy]<!-- ignore --> bölümünde ele aldığımız kurallar devreye girerdi. Ayrıca mevcut örneğimizde `kullanici1.eposta` alanını hâlâ kullanabiliriz; çünkü bu alanın değeri `kullanici1` dışına taşınmamıştır.

> Not: Derleyicinin bir struct örneğindeki bazı alanların taşındığını, bazılarının ise yerinde kaldığını alan bazında takip etmesine _kısmi taşıma (partial move)_ denir. Liste 5-7'de `..kullanici1` sonrasında `kullanici1.kullanici_adi` taşındığı için `kullanici1` değişkenini bir bütün olarak kullanmaya çalışırsanız (örneğin onu değer olarak başka bir fonksiyona aktarırsanız) `error[E0382]: use of partially moved value: 'kullanici1'` (*kısmen taşınmış değerin kullanımı*) hatası alırsınız; bütünü ödünç almaya çalıştığınızda ise hata `borrow of partially moved value` biçiminde olur. Buna karşılık taşınmayan `kullanici1.eposta` alanına veya `Copy` olan `kullanici1.aktif` ve `kullanici1.giris_sayisi` alanlarına tek tek erişmeye devam edebilirsiniz. Eğer `kullanici1` örneğinin tamamının geçerli kalmasını istiyorsanız, `String` alanını açıkça klonlayarak (`kullanici_adi: kullanici1.kullanici_adi.clone()`) yeni örneği oluşturabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="using-tuple-structs-without-named-fields-to-create-different-types"></a>
<a id="creating-different-types-with-tuple-structs"></a>
<a id="demet-structlar-ile-farklı-türler-oluşturmak-creating-different-types-with-tuple-structs"></a>

### Demet Struct'ları ile Farklı Türler Oluşturma

Rust ayrıca demetlere benzeyen ve _demet struct'ları (tuple structs)_ olarak adlandırılan struct'ları da destekler. Demet struct'ları, struct adının kazandırdığı ek anlama sahiptir; ancak alanlarının isimleri yoktur, yalnızca alan türleri belirtilir. Demet struct'ları, demetin tamamına bir ad vererek onu diğer demetlerden ayrı bir tür hâline getirmek istediğinizde ve her bir alanı normal bir struct'taki gibi isimlendirmenin gereksiz yere uzun ya da yinelemeli olacağı durumlarda kullanışlıdır.

Bir demet struct'ı tanımlamak için `struct` anahtar kelimesiyle başlayıp struct adını yazar, ardından parantez içinde demetteki türleri belirtirsiniz. Örneğin aşağıda `Renk` ve `Nokta` adında iki demet struct'ı tanımlayıp kullanıyoruz:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

</Listing>

`siyah` ve `orijin` değerlerinin, farklı demet struct'larının örnekleri oldukları için farklı türlere sahip olduğuna dikkat edin. Tanımladığınız her struct, içerdiği alanlar aynı türlerde olsa bile başlı başına ayrı bir türdür. Örneğin her iki tür de üç adet `i32` değerinden oluşmasına rağmen, `Renk` türünde bir parametre bekleyen bir fonksiyona argüman olarak bir `Nokta` veremezsiniz. Bunun dışında demet struct örnekleri demetlere benzer: Onları tekil parçalarına ayırabilir ve belirli bir değere erişmek için nokta (`.`) işaretinin ardından indeks numarasını yazabilirsiniz. Ancak sıradan demetlerden farklı olarak demet struct'ları, yapı sökümü (*destructuring*) yaparken struct'ın tür adını belirtmenizi gerektirir. Örneğin `orijin` noktasındaki değerleri `x`, `y` ve `z` adlı değişkenlere ayırmak için `let Nokta(x, y, z) = orijin;` yazarız.

> Not: Tek bir alana sahip demet struct'ları (örneğin `struct Metre(f64);` ve `struct Saniye(f64);` ya da `struct KullaniciId(u64);`), Rust ekosisteminde _newtype (yeni tür) deseni_ olarak sıkça kullanılır. Bu sarmalayıcı türler çalışma zamanında içerdikleri veriyle birebir aynı bellek temsiline sahip oldukları için ek bir performans maliyeti getirmez; buna karşılık metre beklenen bir fonksiyona yanlışlıkla saniye gönderilmesi gibi birimleri veya kimlikleri karıştırma hatalarını derleme zamanında yakalamanızı sağlar. Bu desenin gelişmiş kullanımlarını Bölüm 20'de yeniden ele alacağız.

<!-- Old headings. Do not remove or links may break. -->
<a id="unit-like-structs-without-any-fields"></a>
<a id="defining-unit-like-structs"></a>
<a id="birim-benzeri-structları-tanımlamak-defining-unit-like-structs"></a>

### Birim Benzeri Struct'ları Tanımlama

Hiçbir alanı olmayan struct'lar da tanımlayabilirsiniz! Bunlara, ["Demet Türü"][tuples]<!-- ignore --> bölümünde değindiğimiz birim türe (`()`) benzer biçimde davrandıkları için _birim benzeri struct'lar (unit-like structs)_ adı verilir. Birim benzeri struct'lar, bir tür üzerinde bir trait uygulamanız gerektiğinde ancak türün kendisinde saklamak istediğiniz herhangi bir veri bulunmadığında oldukça işe yarar. Trait'leri Bölüm 10'da ele alacağız. Aşağıda `HerZamanEsit` adlı birim benzeri bir struct'ı tanımlayıp örnekliyoruz:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

</Listing>

`HerZamanEsit` türünü tanımlamak için `struct` anahtar kelimesini, istediğimiz adı ve ardından noktalı virgülü yazarız. Süslü parantezlere veya normal parantezlere gerek yoktur! Ardından `ozne` değişkeninde bir `HerZamanEsit` örneği elde etmek için de benzer şekilde, herhangi bir süslü ya da normal parantez kullanmadan doğrudan tanımladığımız adı yazarız. İleride bu tür için, `HerZamanEsit` türünün her örneğinin diğer tüm türlerin her örneğine daima eşit kabul edileceği bir davranış uygulayacağımızı düşünün; böyle bir davranış, örneğin testlerde sonucu önceden bilinen bir karşılaştırma elde etmek için işe yarayabilir. Böyle bir davranışı uygulamak için struct içinde herhangi bir veri tutmamıza gerek yoktur! Bölüm 10'da trait'leri nasıl tanımlayacağınızı ve birim benzeri struct'lar da dahil olmak üzere herhangi bir tür üzerinde nasıl uygulayacağınızı göreceksiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="ownership-of-struct-data"></a>
<a id="struct-verisinin-sahipliği-ownership-of-struct-data"></a>
<a id="struct-verisinin-sahipliği"></a>

> ### Struct Verilerinin Sahipliği
>
> Liste 5-1'deki `Kullanici` struct tanımında, `&str` string dilimi türü yerine verinin sahibi olan `String` türünü kullandık. Bu bilinçli bir tercihtir; çünkü bu struct'ın her örneğinin kendi verilerinin tamamına sahip olmasını ve bu verilerin, struct'ın kendisi geçerli olduğu sürece geçerli kalmasını istiyoruz.
>
> Struct'ların başka bir değişkene ait verilere referans saklaması da mümkündür; ancak bunun için Bölüm 10'da inceleyeceğimiz bir Rust özelliği olan _ömürlerin (lifetimes)_ kullanılması gerekir. Ömürler, bir struct tarafından referans verilen verilerin struct var olduğu sürece geçerli kalmasını garanti eder. Diyelim ki aşağıdaki *src/main.rs* dosyasında olduğu gibi, ömür belirtmeden bir struct içinde referans saklamayı denediniz; bu kod derlenmez:
>
> <Listing file-name="src/main.rs">
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct Kullanici {
>     aktif: bool,
>     kullanici_adi: &str,
>     eposta: &str,
>     giris_sayisi: u64,
> }
>
> fn main() {
>     let kullanici1 = Kullanici {
>         aktif: true,
>         kullanici_adi: "birkullaniciadi123",
>         eposta: "birisi@example.com",
>         giris_sayisi: 1,
>     };
> }
> ```
>
> </Listing>
>
> Derleyici, ömür belirteçlerinin eksik olduğunu bildiren bir hata verecektir:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:20
>   |
> 3 |     kullanici_adi: &str,
>   |                    ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct Kullanici<'a> {
> 2 |     aktif: bool,
> 3 ~     kullanici_adi: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:13
>   |
> 4 |     eposta: &str,
>   |             ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct Kullanici<'a> {
> 2 |     aktif: bool,
> 3 |     kullanici_adi: &str,
> 4 ~     eposta: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` (bin "structs") due to 2 previous errors
> ```
>
> Bölüm 10'da struct'lar içinde referans saklayabilmeniz için bu hataları (`missing lifetime specifier` — *ömür belirteci eksik*) nasıl gidereceğinizi ayrıntılı olarak ele alacağız; şimdilik bu tür hataları `&str` gibi referanslar yerine `String` gibi sahipli türler kullanarak çözeceğiz.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-veri-turleri.html#the-tuple-type
[move]: ch04-01-sahiplik-nedir.html#variables-and-data-interacting-with-move
[copy]: ch04-01-sahiplik-nedir.html#stack-only-data-copy
