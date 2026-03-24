## Struct'ları Tanımlama ve Örnekleme (Defining and Instantiating Structs)

Struct'lar, ["Demet Türü (The Tuple Type)"][tuples]<!-- ignore --> bölümünde tartışılan demetlere benzerler, her ikisi de birden fazla ilgili değeri bir arada tutar. Demetler gibi, bir struct'ın parçaları da farklı türlerde olabilir. Demetlerden farklı olarak, bir struct'ta her bir veri parçasını isimlendirirsiniz, böylece değerlerin ne anlama geldiği açıkça belli olur. Bu isimleri eklemek, struct'ların demetlerden daha esnek olduğu anlamına gelir: Bir örneğin değerlerini belirtmek veya onlara erişmek için verilerin sırasına güvenmek zorunda kalmazsınız.

Bir struct tanımlamak için `struct` anahtar kelimesini girer ve tüm struct'a bir isim veririz. Bir struct'ın adı, gruplandırılan veri parçalarının önemini (anlamını) açıklamalıdır. Daha sonra, süslü parantezler içinde, _alanlar (fields)_ olarak adlandırdığımız veri parçalarının isimlerini ve türlerini tanımlarız. Örneğin, Liste 5-1 bir kullanıcı hesabı hakkında bilgi depolayan bir struct'ı göstermektedir.

<Listing number="5-1" file-name="src/main.rs" caption="Bir `Kullanici` struct'ı tanımı">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

</Listing>

Tanımladıktan sonra bir struct'ı kullanmak için, alanların her biri için somut değerler belirterek o struct'ın bir _örneğini_ oluştururuz. Bir örnek oluştururken struct'ın adını belirtiriz ve ardından _`anahtar: değer`_ (_`key: value`_) çiftlerini içeren süslü parantezler ekleriz; burada anahtarlar alanların adlarıdır ve değerler o alanlarda saklamak istediğimiz verilerdir. Alanları, struct'ta tanımladığımız sırayla belirtmek zorunda değiliz. Başka bir deyişle, struct tanımı tür için genel bir şablon gibidir ve örnekler, o türün değerlerini oluşturmak için bu şablonu belirli verilerle doldurur. Örneğin, Liste 5-2'de gösterildiği gibi belirli bir kullanıcı bildirebiliriz.

<Listing number="5-2" file-name="src/main.rs" caption="`Kullanici` struct'ının bir örneğini oluşturmak">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

</Listing>

Bir struct'tan belirli bir değeri almak için nokta gösterimini (dot notation) kullanırız. Örneğin, bu kullanıcının e-posta adresine erişmek için `kullanici1.eposta` kullanırız. Örnek değiştirilebilir ise, nokta gösterimini kullanıp belirli bir alana atama yaparak bir değeri değiştirebiliriz. Liste 5-3, değiştirilebilir bir `Kullanici` örneğinin `eposta` alanındaki değerin nasıl değiştirileceğini göstermektedir.

<Listing number="5-3" file-name="src/main.rs" caption="Bir `Kullanici` örneğinin `eposta` alanındaki değeri değiştirmek">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

</Listing>

Tüm örneğin değiştirilebilir olması gerektiğine dikkat edin; Rust yalnızca belirli alanları değiştirilebilir olarak işaretlememize izin vermez. Herhangi bir ifadede olduğu gibi, bu yeni örneği örtük olarak (implicitly) döndürmek için fonksiyon gövdesindeki son ifade olarak struct'ın yeni bir örneğini oluşturabiliriz.

Liste 5-4, verilen e-posta ve kullanıcı adıyla bir `Kullanici` örneği döndüren bir `kullanici_olustur` fonksiyonunu göstermektedir. `aktif` alanı `true` değerini ve `giris_sayisi` `1` değerini alır.

<Listing number="5-4" file-name="src/main.rs" caption="Bir e-posta ve kullanıcı adı alıp `Kullanici` örneği döndüren bir `kullanici_olustur` fonksiyonu">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

</Listing>

Fonksiyon parametrelerine struct alanlarıyla aynı adı vermek mantıklıdır, ancak `eposta` ve `kullanici_adi` alan adlarını ve değişkenlerini tekrarlamak zorunda kalmak biraz yorucudur. Struct'ın daha fazla alanı olsaydı, her bir adı tekrarlamak daha da sinir bozucu olurdu. Neyse ki, kullanışlı bir kısaltma var!

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### Alan Başlatma Kısaltmasını Kullanmak (Using the Field Init Shorthand)

Liste 5-4'te parametre adları ve struct alan adları tam olarak aynı olduğundan, `kullanici_olustur`'u Liste 5-5'te gösterildiği gibi `kullanici_adi` ve `eposta` tekrarları olmadan tam olarak aynı davranacak şekilde yeniden yazmak için _alan başlatma kısaltması (field init shorthand)_ sözdizimini kullanabiliriz.

<Listing number="5-5" file-name="src/main.rs" caption="`kullanici_adi` ve `eposta` parametreleri struct alanlarıyla aynı ada sahip olduğu için alan başlatma kısaltmasını kullanan bir `kullanici_olustur` fonksiyonu">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

</Listing>

Burada, `eposta` adında bir alanı olan `Kullanici` struct'ının yeni bir örneğini oluşturuyoruz. `eposta` alanının değerini `kullanici_olustur` fonksiyonunun `eposta` parametresindeki değere ayarlamak istiyoruz. `eposta` alanı ve `eposta` parametresi aynı ada sahip olduğundan, `eposta: eposta` yerine sadece `eposta` yazmamız yeterlidir.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-instances-from-other-instances-with-struct-update-syntax"></a>

### Struct Güncelleme Sözdizimi İle Örnekler Oluşturmak (Creating Instances with Struct Update Syntax)

Aynı türdeki başka bir örneğin değerlerinin çoğunu içeren, ancak bazılarını değiştiren yeni bir struct örneği oluşturmak genellikle yararlıdır. Bunu _struct güncelleme sözdizimi (struct update syntax)_ kullanarak yapabilirsiniz.

İlk olarak, Liste 5-6'da güncelleme sözdizimi olmadan, `kullanici2` içinde düzenli bir yolla nasıl yeni bir `Kullanici` örneği oluşturulacağını gösteriyoruz. `eposta` için yeni bir değer belirliyoruz ancak bunun dışında Liste 5-2'de oluşturduğumuz `kullanici1`'deki değerlerin aynısını kullanıyoruz.

<Listing number="5-6" file-name="src/main.rs" caption="`kullanici1`'deki değerlerden biri hariç tümünü kullanarak yeni bir `Kullanici` örneği oluşturmak">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

</Listing>

Struct güncelleme sözdizimini kullanarak, Liste 5-7'de gösterildiği gibi aynı etkiyi daha az kodla elde edebiliriz. `..` sözdizimi, açıkça ayarlanmayan geri kalan alanların verilen örnekteki alanlarla aynı değere sahip olması gerektiğini belirtir.

<Listing number="5-7" file-name="src/main.rs" caption="Bir `Kullanici` örneği için yeni bir `eposta` değeri ayarlamak, ancak geri kalan değerleri `kullanici1`'den kullanmak için struct güncelleme sözdizimini kullanmak">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

</Listing>

Liste 5-7'deki kod, `kullanici2`'de `eposta` için farklı bir değere sahip olan ancak `kullanici1`'den alınan `kullanici_adi`, `aktif` ve `giris_sayisi` alanları için aynı değerlere sahip bir örnek oluşturur. `..kullanici1`, kalan tüm alanların değerlerini `kullanici1`'deki karşılık gelen alanlardan alması gerektiğini belirtmek için en sona gelmelidir, ancak struct tanımındaki alanların sırasına bakılmaksızın istediğimiz kadar alan için herhangi bir sırada değer belirtebiliriz.

Struct güncelleme sözdiziminin tıpkı bir atama (assignment) işlemi gibi `=` kullandığına dikkat edin; bunun nedeni, veriyi ["Değişkenlerin ve Verilerin Move İle Etkileşimi"][move]<!-- ignore --> bölümünde gördüğümüz gibi taşımasıdır (move). Bu örnekte, `kullanici2` oluşturulduktan sonra artık `kullanici1`'i kullanamayız çünkü `kullanici1`'in `kullanici_adi` alanındaki `String` `kullanici2`'ye taşınmıştır. Eğer `kullanici2`'ye hem `eposta` hem de `kullanici_adi` için yeni `String` değerleri verseydik ve dolayısıyla `kullanici1`'den yalnızca `aktif` ve `giris_sayisi` değerlerini kullansaydık, o zaman `kullanici2` oluşturulduktan sonra `kullanici1` hala geçerli olurdu. Hem `aktif` hem de `giris_sayisi`, `Copy` trait'ini uygulayan türlerdir, bu nedenle ["Sadece Stack Verisi: Copy (Stack-Only Data: Copy)"][copy]<!-- ignore --> bölümünde tartıştığımız davranış uygulanacaktır. Ayrıca bu örnekte hala `kullanici1.eposta` kullanabiliriz çünkü değeri `kullanici1`'den dışarı taşınmamıştır (moved out).

<!-- Old headings. Do not remove or links may break. -->

<a id="using-tuple-structs-without-named-fields-to-create-different-types"></a>

### Demet Struct'lar ile Farklı Türler Oluşturmak (Creating Different Types with Tuple Structs)

Rust ayrıca, _demet struct'lar (tuple structs)_ olarak adlandırılan ve demetlere (tuples) benzeyen struct'ları da destekler. Demet struct'lar, struct adının sağladığı ek anlama sahiptir, ancak alanlarıyla ilişkili adları yoktur; bunun yerine, yalnızca alanların türlerine sahiptirler. Demet struct'lar, tüm demete bir isim vermek, demeti diğer demetlerden farklı bir tür yapmak istediğinizde ve her bir alanı normal bir struct'taki gibi isimlendirmenin gereksiz veya fazlalık (verbose) olacağı durumlarda kullanışlıdır.

Bir demet struct'ı tanımlamak için, `struct` anahtar kelimesiyle başlayın ve ardından struct adına ek olarak demetteki türleri belirtin. Örneğin, burada `Renk` ve `Nokta` adında iki demet struct tanımlayıp kullanıyoruz:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

</Listing>

`siyah` ve `orijin` değerlerinin, farklı demet struct'larının örnekleri oldukları için farklı türlerde olduklarına dikkat edin. Tanımladığınız her struct, kendi içinde yer alan alanlar aynı türlerde olsa bile, kendine has (kendi başına) bir türdür. Örneğin, `Renk` türünde bir parametre alan bir fonksiyon argüman olarak bir `Nokta` alamaz, her iki tür de üç `i32` değerinden oluşsa bile. Bunun dışında demet struct örnekleri, bireysel parçalarına ayrıştırılabilmeleri (destructure) ve belirli bir değere erişmek için noktadan (`.`) sonra indeksi kullanılabilmeleri bakımından demetlere benzerler. Demetlerden farklı olarak, demet struct'ları ayrıştırdığınızda (destructure) struct'ın türünü belirtmenizi gerektirir. Örneğin, `orijin` noktasındaki değerleri `x`, `y` ve `z` adlı değişkenlere ayrıştırmak için `let Nokta(x, y, z) = orijin;` yazardık.

<!-- Old headings. Do not remove or links may break. -->

<a id="unit-like-structs-without-any-fields"></a>

### Birim Benzeri Struct'ları Tanımlamak (Defining Unit-Like Structs)

Hiçbir alanı olmayan struct'lar da tanımlayabilirsiniz! Bunlara ["Demet Türü (The Tuple Type)"][tuples]<!-- ignore --> bölümünde bahsettiğimiz birim türe `()` benzer şekilde davrandıkları için _birim benzeri struct'lar (unit-like structs)_ denir. Birim benzeri struct'lar, bazı türler üzerinde bir trait uygulamanız gerektiğinde ancak türün kendisinde saklamak istediğiniz herhangi bir veri olmadığında yararlı olabilir. Trait'leri Bölüm 10'da tartışacağız. İşte `HerZamanEsit` adında bir birim struct tanımlama ve örnekleme (instantiate) örneği:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

</Listing>

`HerZamanEsit`'i tanımlamak için, `struct` anahtar kelimesini, istediğimiz ismi ve ardından bir noktalı virgül kullanırız. Süslü parantezlere veya normal parantezlere gerek yoktur! Daha sonra, benzer bir yolla (herhangi bir süslü veya normal parantez olmadan tanımladığımız adı kullanarak) `ozne` değişkeninde `HerZamanEsit` örneğini alabiliriz. Daha sonra bu tür için bir davranış (behavior) uygulayacağımızı, öyle ki `HerZamanEsit`'in her örneğinin başka herhangi bir türün her örneğine her zaman eşit olacağını hayal edin, belki de test amacıyla bilinen bir sonuca sahip olmak için. Bu davranışı uygulamak için hiçbir veriye ihtiyacımız olmazdı! Bölüm 10'da trait'leri nasıl tanımlayacağınızı ve bunları birim benzeri struct'lar dahil herhangi bir tür üzerinde nasıl uygulayacağınızı göreceksiniz.

> ### Struct Verisinin Sahipliği (Ownership of Struct Data)
>
> Liste 5-1'deki `Kullanici` struct tanımında, `&str` string dilimi türünden ziyade sahipliği olan `String` türünü kullandık. Bu bilinçli bir seçimdir çünkü bu struct'ın her bir örneğinin tüm verilerine sahip olmasını ve o verinin, tüm struct geçerli olduğu sürece geçerli olmasını istiyoruz.
>
> Struct'ların başka bir şeye ait verilere referans depolaması da mümkündür, ancak bunu yapmak, Bölüm 10'da tartışacağımız bir Rust özelliği olan _ömürlerin_ kullanılmasını gerektirir. Ömürler, bir struct tarafından referans verilen verilerin struct var olduğu sürece geçerli olmasını sağlar. Diyelim ki ömürleri belirtmeden bir struct içinde, *src/main.rs* dosyasındaki gibi bir referans saklamaya çalışıyorsunuz; bu çalışmayacaktır:
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
> Derleyici ömür (lifetime) belirteçlerine ihtiyacı olduğundan şikayet edecektir:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     kullanici_adi: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct Kullanici<'a> {
> 2 |     aktif: bool,
> 3 ~     kullanici_adi: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     eposta: &str,
>   |            ^ expected named lifetime parameter
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
> Bölüm 10'da struct'lar içinde referans depolayabilmeniz için bu hataları nasıl düzelteceğinizi tartışacağız, ancak şimdilik `&str` gibi referanslar yerine `String` gibi sahip olunan türleri kullanarak bu gibi hataları düzelteceğiz.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-veri-turleri.html#tuple-türü
[move]: ch04-01-sahiplik-nedir.html#ways-variables-and-data-interact-move
[copy]: ch04-01-sahiplik-nedir.html#sadece-stack-verisi-stack-only-data-copy
