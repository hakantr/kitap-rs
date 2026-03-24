# Tahmin Oyunu Programlama

Birlikte uygulamalı bir proje üzerinde çalışarak Rust'a giriş yapalım! Bu bölüm,
bazı yaygın Rust kavramlarını gerçek bir programda nasıl kullanacağınızı göstererek
size tanıtır. `let`, `match`, metotlar, ilişkili fonksiyonlar (associated functions),
harici crate'ler ve daha fazlası hakkında bilgi edineceksiniz! İlerleyen bölümlerde bu
fikirleri daha ayrıntılı olarak inceleyeceğiz. Bu bölümde sadece temel bilgilerin pratiğini
yapacaksınız.

Klasik bir başlangıç programlama problemini uygulayacağız: bir tahmin oyunu.
Şöyle çalışıyor: Program 1 ile 100 arasında rastgele bir tam sayı üretecek.
Daha sonra oyuncudan bir tahmin girmesini isteyecek. Bir tahmin girildikten sonra,
program tahminin çok düşük veya çok yüksek olduğunu belirtecek. Tahmin doğruysa,
oyun bir tebrik mesajı yazdıracak ve kapanacak.

## Yeni Bir Proje Kurulumu

Yeni bir proje kurmak için, Bölüm 1'de oluşturduğunuz _projects_ dizinine gidin
ve Cargo'yu kullanarak yeni bir proje oluşturun, örneğin:

```console
$ cargo new tahmin_oyunu
$ cd tahmin_oyunu
```

İlk komut olan `cargo new`, projenin adını (`tahmin_oyunu`) ilk argüman olarak alır.
İkinci komut ise dizini yeni projenin dizinine değiştirir.

Oluşturulan _Cargo.toml_ dosyasına bakın:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name tahmin_oyunu
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Bölüm 1'de gördüğünüz gibi, `cargo new` sizin için bir "Merhaba, dünya!" programı üretir.
_src/main.rs_ dosyasına göz atın:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Şimdi bu "Merhaba, dünya!" programını derleyelim ve aynı adımda `cargo run` komutunu
kullanarak çalıştıralım:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

`run` komutu, bu oyunda da yapacağımız gibi, bir projede hızlıca ardışık geliştirmeler
yapmanız ve bir sonrakine geçmeden önce her iterasyonu hızlıca test etmeniz gerektiğinde çok işe yarar.

_src/main.rs_ dosyasını tekrar açın. Tüm kodu bu dosyaya yazacaksınız.

## Bir Tahmini İşlemek

Tahmin oyunu programının ilk bölümü kullanıcıdan girdi isteyecek, bu girdiyi işleyecek
ve girdinin beklenen biçimde olup olmadığını kontrol edecektir. Başlamak için,
oyuncunun bir tahmin girmesine izin vereceğiz. Liste 2-1'deki kodu _src/main.rs_
dosyasına girin.

<Listing number="2-1" file-name="src/main.rs" caption="Kullanıcıdan bir tahmin alan ve bunu yazdıran kod">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

</Listing>

Bu kod pek çok bilgi içeriyor, bu yüzden satır satır üzerinden geçelim.
Kullanıcı girdisini almak ve ardından sonucu çıktı olarak yazdırmak için
`io` (girdi/çıktı) kütüphanesini kapsama dahil etmemiz gerekir.
`io` kütüphanesi, `std` olarak bilinen standart kütüphaneden gelir:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Varsayılan olarak Rust, standart kütüphanede tanımlanmış ve her programın kapsamına
dahil ettiği bir dizi öğeye sahiptir. Bu diziye _prelude_ (başlangıç) denir ve içindeki
her şeyi [standart kütüphane dokümantasyonunda][prelude] görebilirsiniz.

Eğer kullanmak istediğiniz bir tür prelude içinde değilse, o türü bir `use`
ifadesi ile açıkça kapsama dahil etmeniz gerekir. `std::io` kütüphanesini
kullanmak, kullanıcı girdisini kabul etme yeteneği de dahil olmak üzere size
bir dizi yararlı özellik sağlar.

Bölüm 1'de gördüğünüz gibi, `main` fonksiyonu programa giriş noktasıdır:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

`fn` sözdizimi (syntax) yeni bir fonksiyon tanımlar; `()` parantezleri
parametre olmadığını gösterir; ve `{` süslü parantezi fonksiyonun gövdesini başlatır.

Yine Bölüm 1'de öğrendiğiniz gibi, `println!` ekrana bir metin (string) yazdıran
bir makrodur:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Bu kod, oyunun ne olduğunu belirten bir metin yazdırıyor ve kullanıcıdan girdi talep ediyor.

### Değerleri Değişkenlerle Saklamak

Sırada, kullanıcı girdisini saklamak için aşağıdaki gibi bir _değişken_ (variable) oluşturacağız:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Şimdi program ilginçleşmeye başlıyor! Bu küçük satırda pek çok şey oluyor. Değişkeni
oluşturmak için `let` ifadesini kullanıyoruz. İşte başka bir örnek:

```rust,ignore
let elmalar = 5;
```

Bu satır `elmalar` adında yeni bir değişken oluşturur ve onu `5` değerine bağlar (bind).
Rust'ta değişkenler varsayılan olarak değiştirilemezdir, yani değişkene
bir değer verdiğimizde bu değer değişmez. Bu kavramı Bölüm 3'teki
["Değişkenler ve Değiştirilebilirlik"][variables-and-mutability]<!-- ignore --> kısmında
ayrıntılı olarak tartışacağız. Bir değişkeni değiştirilebilir yapmak için
değişken adından önce `mut` ekleriz:

```rust,ignore
let elmalar = 5; // değiştirilemez
let mut muzlar = 5; // değiştirilebilir
```

> Not: `//` sözdizimi, satırın sonuna kadar devam eden bir yorum (comment) başlatır.
> Rust, yorumlardaki her şeyi görmezden gelir. Yorumları [Bölüm 3][comments]<!-- ignore -->'te
> daha ayrıntılı olarak tartışacağız.

Tahmin oyunu programına dönecek olursak, artık `let mut tahmin`'in `tahmin` adında
değiştirilebilir bir değişken tanıtacağını biliyorsunuz. Eşittir işareti (`=`),
Rust'a değişkeni şu an bir şeye bağlamak istediğimizi söyler. Eşittir işaretinin
sağında, `tahmin` değişkeninin bağlandığı değer bulunur; bu değer, yeni bir `String`
örneği döndüren bir fonksiyon olan `String::new`'i çağırmanın sonucudur.
[`String`][string]<!-- ignore -->, standart kütüphane tarafından sağlanan; genişleyebilir,
UTF-8 kodlu bir metin türüdür.

`::new` satırındaki `::` sözdizimi, `new`'in `String` türünün ilişkili (associated) bir
fonksiyonu olduğunu gösterir. Bir _ilişkili fonksiyon_, bir tür (burada `String`)
üzerinde uygulanan (implemented) fonksiyondur. Bu `new` fonksiyonu yeni ve boş bir
metin (string) oluşturur. Birçok türde bir `new` fonksiyonu bulacaksınız, çünkü bu,
yeni bir değer üreten bir fonksiyon için yaygın olarak kullanılan bir isimdir.

Özetle, `let mut tahmin = String::new();` satırı, şu anda yeni ve boş bir `String`
örneğine bağlı olan, değiştirilebilir bir değişken oluşturmuştur. Vay canına!

### Kullanıcı Girdisini Almak

Programın ilk satırında standart kütüphaneden girdi/çıktı işlevselliğini `use std::io;` ile
dahil ettiğimizi hatırlayın. Şimdi `io` modülünden `stdin` fonksiyonunu çağıracağız,
bu da kullanıcı girdisini ele almamızı sağlayacak:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Eğer `io` modülünü programın başında `use std::io;` ile içe aktarmasaydık (import),
yine de bu fonksiyon çağrısını `std::io::stdin` olarak yazarak kullanabilirdik.
`stdin` fonksiyonu, terminaliniz için standart girdiye erişimi temsil eden
bir tür olan [`std::io::Stdin`][iostdin]<!-- ignore --> örneğini döndürür.

Ardından gelen `.read_line(&mut tahmin)` satırı, kullanıcıdan girdi almak için
standart girdi arayüzünde [`read_line`][read_line]<!-- ignore --> metotunu çağırır.
Ayrıca, kullanıcı girdisinin hangi metne (string) kaydedileceğini söylemek için `&mut tahmin`
ifadesini `read_line`'a argüman olarak veriyoruz. `read_line`'ın tüm işi, kullanıcının
standart girdiye yazdıklarını almak ve bunu bir metne eklemektir (içeriğinin üzerine yazmadan).
Bu nedenle o metni bir argüman olarak aktarıyoruz. Metin argümanı değiştirilebilir olmalıdır,
böylece metot metnin içeriğini değiştirebilir.

`&` işareti, bu argümanın bir _referans_ olduğunu gösterir. Bu, veriyi belleğe birden fazla
kez kopyalamanıza gerek kalmadan kodunuzun birden fazla parçasının aynı veriye
erişmesine izin veren bir yoldur. Referanslar karmaşık bir özelliktir ve Rust'ın en büyük
avantajlarından biri referansları kullanmanın ne kadar güvenli ve kolay olmasıdır. Bu programı
bitirmek için bu detayların çoğunu bilmenize gerek yoktur. Şimdilik bilmeniz gereken tek şey,
tıpkı değişkenler gibi, referansların da varsayılan olarak değiştirilemez olduğudur.
Bu nedenle onu değiştirilebilir yapmak için `&tahmin` yerine `&mut tahmin`
yazmanız gerekir. (Bölüm 4, referansları daha kapsamlı bir şekilde açıklayacaktır.)

<!-- Old headings. Do not remove or links may break. -->

<a id="handling-potential-failure-with-the-result-type"></a>

### Potansiyel Bir Hatayı `Result` ile Ele Almak

Hala aynı kod satırı üzerinde çalışıyoruz. Şimdi kodun üçüncü satırını tartışıyoruz,
ancak bunun hala tek bir mantıksal kod satırının bir parçası olduğunu unutmayın.
Sonraki kısım bu metottur:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Bu kodu şu şekilde de yazabilirdik:

```rust,ignore
io::stdin().read_line(&mut tahmin).expect("Satır okunamadı");
```

Ancak tek bir uzun satırı okumak zordur, bu yüzden onu bölmek en iyisidir.
`.method_adi()` sözdizimi ile bir metot çağırdığınızda uzun satırları bölmeye yardımcı
olması için yeni bir satır (newline) ve diğer boşlukları (whitespace) kullanmak
genellikle akıllıcadır. Şimdi bu satırın ne işe yaradığını tartışalım.

Daha önce bahsedildiği gibi, `read_line` kullanıcının girdiği her şeyi
ona aktardığımız metne (string) koyar, ancak aynı zamanda bir `Result` değeri döndürür.
[`Result`][result]<!-- ignore -->, çoğunlukla bir _enum_ olarak adlandırılan
bir [_enumeration_][enums]<!-- ignore -->'dır. Bu, birden fazla olası durumdan
birinde olabilen bir türdür. Her olası duruma _varyant_ (variant) diyoruz.

[Bölüm 6][enums]<!-- ignore --> enum'ları daha ayrıntılı olarak ele alacaktır.
Bu `Result` türlerinin amacı, hata ayıklama bilgilerini kodlamaktır.

`Result`'ın varyantları `Ok` ve `Err`'dir. `Ok` varyantı işlemin başarılı olduğunu
gösterir ve başarıyla üretilen değeri içerir. `Err` varyantı işlemin başarısız olduğu
anlamına gelir ve işlemin nasıl veya neden başarısız olduğuna dair bilgiler içerir.

`Result` türündeki değerler, tıpkı herhangi bir türdeki değerler gibi, üzerlerinde
tanımlanmış metotlara sahiptir. Bir `Result` örneğinin çağırabileceğiniz bir
[`expect` metodu][expect]<!-- ignore --> vardır. Eğer bu `Result` örneği bir
`Err` değeriyse, `expect` programın çökmesine (crash) neden olur ve
`expect`'e argüman olarak verdiğiniz mesajı görüntüler. Eğer `read_line`
metodu bir `Err` döndürdüyse, bu muhtemelen altta yatan işletim sisteminden kaynaklanan
bir hatanın sonucu olacaktır. Eğer bu `Result` örneği bir `Ok` değeriyse,
`expect`, `Ok`'un barındırdığı dönüş değerini alacak ve kullanabilmeniz
için sadece bu değeri size döndürecektir. Bu durumda, bu değer kullanıcının
girdisindeki bayt (byte) sayısıdır.

Eğer `expect`'i çağırmazsanız, program derlenecektir ancak bir uyarı (warning)
alacaksınız:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust, `read_line`'dan dönen `Result` değerini kullanmadığınız konusunda sizi
uyararak programın olası bir hatayı ele almadığını belirtir.

Uyarıyı bastırmanın doğru yolu, aslında hata yönetimi (error-handling)
kodunu yazmaktır; ancak bizim durumumuzda bir sorun oluştuğunda programın
çökmesini istiyoruz, bu nedenle `expect` kullanabiliriz. Hatalardan kurtulmayı
(recovering from errors) [Bölüm 9][recover]<!-- ignore -->'da öğreneceksiniz.

### `println!` Yer Tutucuları (Placeholders) ile Değerleri Yazdırmak

Kapanış süslü parantezini saymazsak, şu ana kadarki kodda tartışılacak
sadece bir satır daha kaldı:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Bu satır, artık kullanıcının girdisini içeren metni yazdırır. `{}` şeklindeki
süslü parantez takımı bir yer tutucudur (placeholder): `{}`'yi, bir değeri
yerinde tutan küçük yengeç kıskaçları gibi düşünün. Bir değişkenin değerini
yazdırırken, değişken adı süslü parantezlerin içine gidebilir. Bir ifadenin
değerlendirilmesinin sonucunu yazdırırken, format dizisine (format string) boş süslü
parantezler yerleştirin, ardından format dizisinden sonra virgülle ayrılmış bir
ifadeler listesi (list of expressions) ekleyin; bu ifadeler her bir boş süslü
parantez yer tutucusunda aynı sırayla yazdırılacaktır. Bir değişkeni ve bir ifadenin
sonucunu tek bir `println!` çağrısıyla yazdırmak şu şekilde görünecektir:

```rust
let x = 5;
let y = 10;

println!("x = {x} ve y + 2 = {}", y + 2);
```

Bu kod `x = 5 ve y + 2 = 12` yazdıracaktır.

### İlk Kısmı Test Etmek

Tahmin oyununun ilk bölümünü test edelim. `cargo run` komutunu kullanarak çalıştırın:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Lütfen tahmininizi girin.
6
Tahmininiz: 6
```

Bu noktada, oyunun ilk kısmı tamamlandı: Klavyeden girdi alıyoruz
ve ardından onu yazdırıyoruz.

## Gizli Bir Sayı Üretmek

Sırada, kullanıcının tahmin etmeye çalışacağı gizli bir sayı üretmemiz gerekiyor.
Oyunun birden fazla kez oynanmasının eğlenceli olması için gizli sayı her seferinde
farklı olmalıdır. Oyunun çok zor olmaması için 1 ile 100 arasında rastgele bir
sayı kullanacağız. Rust henüz standart kütüphanesinde rastgele sayı işlevselliği
içermiyor. Ancak Rust ekibi, söz konusu işlevselliğe sahip bir [`rand` crate'i][randcrate]
sağlıyor.

<!-- Old headings. Do not remove or links may break. -->
<a id="using-a-crate-to-get-more-functionality"></a>

### Bir Crate ile İşlevselliği Artırmak

Bir crate'in, Rust kaynak kodu dosyalarından oluşan bir koleksiyon olduğunu hatırlayın.
Geliştirdiğimiz proje çalıştırılabilir bir crate'tir. `rand` crate'i ise,
başka programlarda kullanılması amaçlanan ve kendi başına çalıştırılamayan kodlar içeren bir
kütüphane (library) crate'idir.

Harici crate'lerin koordinasyonu, Cargo'nun gerçekten parladığı yerdir. `rand` kullanan
bir kod yazabilmemiz için önce _Cargo.toml_ dosyasını değiştirip `rand` crate'ini
bir bağımlılık (dependency) olarak eklememiz gerekir. Şimdi o dosyayı açın ve Cargo'nun
sizin için oluşturduğu `[dependencies]` bölüm başlığının hemen altına şu satırı ekleyin.
`rand`'i bu sürüm numarasıyla birlikte aynen burada verdiğimiz gibi belirttiğinizden emin
olun, aksi takdirde bu öğreticideki kod örnekleri çalışmayabilir:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

_Cargo.toml_ dosyasında, bir başlıktan sonra gelen her şey, başka bir bölüm
başlayana kadar devam eden o bölümün parçasıdır. `[dependencies]` bölümünde
Cargo'ya, projenizin hangi harici crate'lere bağımlı olduğunu ve bu crate'lerin
hangi sürümlerini gerektirdiğinizi söylersiniz. Bu örnekte, `rand` crate'ini semantik (anlamsal)
sürüm belirteci `0.8.5` ile belirtiyoruz. Cargo, sürüm numaralarını yazmak için bir
standart olan [Semantik Sürümlemeyi][semver]<!-- ignore --> (bazen _SemVer_ olarak
adlandırılır) anlar. `0.8.5` belirteci aslında `^0.8.5` için bir kısaltmadır ve
bu da en az 0.8.5 olan ancak 0.9.0'ın altında kalan tüm sürümleri kapsadığı anlamına gelir.

Cargo bu sürümlerin 0.8.5 sürümüyle uyumlu genel API'lere sahip olduğunu
kabul eder ve bu spesifikasyon, bu bölümdeki kodla hâlâ derlenebilecek en son
yama sürümünü (patch release) almanızı garanti eder. Sürümü 0.9.0 veya
daha büyük olanların ise aşağıdaki örneklerin kullandığıyla aynı API'ye sahip olması
garanti edilmez.

Şimdi koddaki hiçbir şeyi değiştirmeden Liste 2-2'de gösterildiği gibi projeyi derleyelim.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

<Listing number="2-2" caption="`rand` crate'ini bağımlılık olarak ekledikten sonra `cargo build` çalıştırmanın çıktısı">

```console
$ cargo build
  Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
```

</Listing>

Farklı sürüm numaraları görebilirsiniz (ancak SemVer sayesinde hepsi kodla uyumlu
olacaktır!) ve farklı satırlar (işletim sistemine bağlı olarak) olabilir, ve
satırlar farklı bir sırada olabilir.

Harici bir bağımlılığı dahil ettiğimizde, Cargo o bağımlılığın ihtiyaç duyduğu
her şeyin en son sürümlerini, [Crates.io][cratesio]'daki verilerin bir kopyası
olan _kayıt defterinden_ (registry) getirir (fetch). Crates.io, Rust ekosistemindeki
insanların başkalarının kullanması için açık kaynaklı Rust projelerini
yayınladıkları yerdir.

Cargo, kayıt defterini güncelledikten sonra `[dependencies]` bölümünü kontrol
eder ve listelenen fakat henüz indirilmemiş crate'leri indirir. Bu örnekte
sadece `rand`'i bir bağımlılık olarak listelemiş olsak da, Cargo ayrıca `rand`'in
çalışması için bağımlı olduğu diğer crate'leri de alır. Crate'leri indirdikten sonra,
Rust bunları derler ve ardından bağımlılıklar mevcut olacak şekilde projeyi
derler.

Eğer hiçbir değişiklik yapmadan `cargo build` komutunu hemen tekrar çalıştırırsanız,
`Finished` satırı haricinde herhangi bir çıktı almazsınız. Cargo, bağımlılıkları daha
önceden indirip derlediğini ve sizin _Cargo.toml_ dosyanızda bunlar hakkında hiçbir şeyi
değiştirmediğinizi bilir. Ayrıca kodunuz hakkında da hiçbir şeyi değiştirmediğinizi bilir,
bu yüzden onu da yeniden derlemez. Yapacak bir şeyi olmadığı için basitçe çıkış yapar.

Eğer _src/main.rs_ dosyasını açıp önemsiz bir değişiklik yaparsanız ve ardından kaydedip
tekrar derlerseniz, yalnızca iki satır çıktı görürsünüz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

Bu satırlar, Cargo'nun derlemeyi yalnızca _src/main.rs_ dosyanızdaki ufak
değişikliğinizle güncellediğini gösterir. Bağımlılıklarınız değişmedi, bu yüzden Cargo,
bunlar için zaten indirip derlediği şeyleri yeniden kullanabileceğini biliyor.

<!-- Old headings. Do not remove or links may break. -->
<a id="ensuring-reproducible-builds-with-the-cargo-lock-file"></a>

#### Tekrarlanabilir Derlemeleri Sağlamak (Reproducible Builds)

Cargo, siz veya başka herhangi bir kişi kodunuzu her derlediğinde aynı artefaktı
(artifact) yeniden derleyebilmenizi sağlayan bir mekanizmaya sahiptir: Cargo, siz
aksi bir şey belirtene kadar yalnızca belirttiğiniz bağımlılık sürümlerini
kullanacaktır. Örneğin, gelecek hafta `rand` crate'inin 0.8.6 sürümünün çıktığını,
ve bu sürümün önemli bir hata düzeltmesi içerdiğini, ancak kodunuzu bozacak bir
gerileme (regression) de barındırdığını varsayalım. Bunun üstesinden gelmek için,
Rust ilk `cargo build` komutunu çalıştırdığınızda _Cargo.lock_ dosyasını oluşturur,
ve biz şu an _tahmin_oyunu_ dizinimizde buna sahibiz.

Bir projeyi ilk kez derlediğinizde, Cargo kriterlere uyan bağımlılıkların
tüm sürümlerini belirler (figures out) ve daha sonra bunları _Cargo.lock_ dosyasına yazar.
Projenizi gelecekte derlediğinizde Cargo, _Cargo.lock_ dosyasının mevcut olduğunu görecek
ve sürümleri yeniden belirleme işini yapmaktansa orada belirtilen sürümleri kullanacaktır.
Bu, tekrarlanabilir bir derlemeye otomatik olarak sahip olmanızı sağlar. Başka bir
deyişle, siz açıkça yükseltme (upgrade) yapana kadar, _Cargo.lock_ dosyası
sayesinde projeniz 0.8.5 sürümünde kalacaktır. _Cargo.lock_ dosyası, tekrarlanabilir
derlemeler için önemli olduğundan, genellikle projenizdeki geri kalan kodlarla birlikte
sürüm kontrolüne (source control) dahil edilir.

#### Yeni Bir Sürüm Almak İçin Bir Crate'i Güncellemek

Bir crate'i güncellemek _istediğinizde_, Cargo `update` komutunu sağlar;
bu komut _Cargo.lock_ dosyasını yok sayacak ve _Cargo.toml_ dosyasındaki
spesifikasyonlarınıza uyan en son sürümleri belirleyecektir. Ardından Cargo,
bu sürümleri _Cargo.lock_ dosyasına yazacaktır. Aksi takdirde, varsayılan olarak Cargo
yalnızca 0.8.5'ten büyük ve 0.9.0'dan küçük sürümleri arayacaktır. Eğer `rand` crate'i
iki yeni sürüm olan 0.8.6 ve 0.999.0'ı yayınlamışsa, `cargo update` çalıştırdığınızda
şunu görürdünüz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.999.0)
```

Cargo, 0.999.0 sürümünü yok sayar. Bu noktada _Cargo.lock_ dosyanızda artık kullandığınız
`rand` crate sürümünün 0.8.6 olduğunu belirten bir değişiklik fark edersiniz. `rand` 0.999.0 sürümünü
veya 0.999._x_ serisindeki herhangi bir sürümü kullanmak için, bunun yerine _Cargo.toml_ dosyanızı
şu şekilde güncellemeniz gerekir (bu değişikliği yapmayın çünkü aşağıdaki örnekler
`rand` 0.8 kullandığınızı varsayıyor):

```toml
[dependencies]
rand = "0.999.0"
```

`cargo build`'i bir sonraki çalıştırışınızda, Cargo, mevcut crate'lerin kayıt
defterini güncelleyecek ve belirttiğiniz yeni sürüme göre `rand` gereksinimlerinizi
yeniden değerlendirecektir.

[Cargo][doccargo]<!-- ignore --> ve [ekosistemi][doccratesio]<!-- ignore -->
hakkında söylenecek çok daha fazla şey var ve bunları Bölüm 14'te tartışacağız;
ancak şimdilik bilmeniz gerekenler bunlar. Cargo, kütüphaneleri yeniden kullanmayı
çok kolaylaştırdığı için, Rust geliştiricileri (Rustaceans) bir dizi paketten
birleştirilmiş daha küçük projeler yazabilirler.

### Rastgele Bir Sayı Üretmek

Tahmin edilecek sayıyı üretmek için `rand` kullanmaya başlayalım. Bir sonraki
adım, Liste 2-3'te gösterildiği gibi _src/main.rs_ dosyasını güncellemektir.

<Listing number="2-3" file-name="src/main.rs" caption="Rastgele bir sayı üretmek için kod ekleme">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

</Listing>

Öncelikle, `use rand::Rng;` satırını ekliyoruz. `Rng` trait'i (özelliği),
rastgele sayı üreteçlerinin (generators) uyguladığı metotları
tanımlar ve bu metotları kullanabilmemiz için bu trait kapsamda olmalıdır.
Bölüm 10, trait'leri detaylı bir şekilde ele alacaktır.

Sonrasında ortaya iki satır ekliyoruz. İlk satırda, kullanacağımız belirli
rastgele sayı üretecini bize veren `rand::thread_rng` fonksiyonunu çağırıyoruz:
Bu üreteç, yürütmenin mevcut iş parçacığına özeldir (local)
ve işletim sistemi tarafından tohumlanır (seeded). Sonra da rastgele sayı
üreteci üzerinde `gen_range` metodunu çağırıyoruz. Bu metot, `use rand::Rng;`
ifadesi ile kapsama dahil ettiğimiz `Rng` trait'i tarafından tanımlanmıştır.
`gen_range` metodu argüman olarak bir aralık ifadesi (range expression) alır
ve o aralıkta rastgele bir sayı üretir. Burada kullandığımız aralık ifadesi türü
`baslangic..=bitis` (start..=end) formundadır ve alt ile üst sınırları dahil
eder; bu nedenle 1 ile 100 arasında bir sayı talep etmek için `1..=100` belirtmemiz gerekir.

> Not: Bir crate'ten hangi trait'leri kullanacağınızı ve hangi metotları ile fonksiyonları
> çağıracağınızı hemen bilemezsiniz; bu yüzden her crate'in kullanımı ile ilgili
> talimatları içeren dokümantasyonları vardır. Cargo'nun başka bir şık özelliği de,
> `cargo doc --open` komutunu çalıştırmanın tüm bağımlılıklarınız tarafından sağlanan
> belgeleri yerel olarak oluşturması (build) ve bunları tarayıcınızda açmasıdır.
> Örneğin, `rand` crate'indeki diğer işlevlerle ilgileniyorsanız, `cargo doc --open`
> çalıştırın ve soldaki kenar çubuğundan (sidebar) `rand`'a tıklayın.

İkinci yeni satır gizli sayıyı yazdırır. Bu, programı geliştirirken onu test
edebilmek için kullanışlıdır, ancak nihai sürümden bunu sileceğiz. Program,
başlar başlamaz cevabı yazdırıyorsa pek de bir oyun sayılmaz!

Programı birkaç kez çalıştırmayı deneyin:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Gizli sayı: 7
Lütfen tahmininizi girin.
4
Tahmininiz: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Gizli sayı: 83
Lütfen tahmininizi girin.
5
Tahmininiz: 5
```

Farklı rastgele sayılar almalısınız ve hepsi de 1 ile 100 arasında sayılar olmalıdır. Harika iş çıkardınız!

## Tahmini Gizli Sayıyla Karşılaştırmak

Artık kullanıcı girdisine ve rastgele bir sayıya sahip olduğumuza göre,
onları karşılaştırabiliriz. Bu adım Liste 2-4'te gösterilmektedir. Not: Bu kod
daha sonra açıklayacağımız nedenlerden ötürü henüz derlenmeyecektir.

<Listing number="2-4" file-name="src/main.rs" caption="İki sayıyı karşılaştırmanın olası dönüş (return) değerlerini ele alma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

</Listing>

Öncelikle, standart kütüphaneden `std::cmp::Ordering` adlı bir türü kapsama
dahil eden başka bir `use` ifadesi ekliyoruz. `Ordering` türü başka bir enum'dur
ve `Less` (Daha Küçük), `Greater` (Daha Büyük) ve `Equal` (Eşit) varyantlarına
sahiptir. Bunlar, iki değeri karşılaştırdığınızda mümkün olan üç sonuçtur.

Sonra en alta `Ordering` türünü kullanan beş yeni satır ekliyoruz. `cmp`
metodu iki değeri karşılaştırır ve karşılaştırılabilen herhangi bir şey üzerinde
çağrılabilir. Ne ile karşılaştırma yapmak istiyorsanız onun bir referansını
alır: Burada `tahmin`'i `gizli_sayi` ile karşılaştırıyor. Sonrasında, `use`
ifadesiyle kapsama dahil ettiğimiz `Ordering` enum'ının bir varyantını döndürür.
`cmp` çağrısından, `tahmin` ve `gizli_sayi` değerleriyle hangi `Ordering`
varyantının döndüğüne bağlı olarak bundan sonra ne yapacağımıza karar
vermek için bir [`match`][match]<!-- ignore --> ifadesi kullanıyoruz.

Bir `match` ifadesi _kollardan_ (arms) oluşur. Bir kol (arm), eşleştirilecek
bir _desenden_ (pattern) ve `match`'e verilen değer o kolun desenine uyduğunda
çalıştırılması gereken koddan oluşur. Rust, `match`'e verilen değeri alır
ve sırayla her bir kolun deseni boyunca bakar. Desenler (patterns) ve
`match` yapısı çok güçlü Rust özellikleridir: Kodunuzun karşılaşabileceği
çeşitli durumları ifade etmenizi ve hepsini ele aldığınızdan emin olmanızı sağlarlar.
Bu özellikler sırasıyla Bölüm 6 ve Bölüm 19'da ayrıntılı olarak ele alınacaktır.

Burada kullandığımız `match` ifadesi ile bir örnek üzerinden gidelim.
Diyelim ki kullanıcı 50 tahmininde bulundu ve bu kez rastgele üretilen gizli sayı 38.

Kod 50'yi 38 ile karşılaştırdığında, `cmp` metodu `Ordering::Greater` (Daha Büyük)
döndürecektir, çünkü 50, 38'den büyüktür. `match` ifadesi `Ordering::Greater`
değerini alır ve her bir kolun desenini (pattern) kontrol etmeye başlar.
İlk kolun deseni olan `Ordering::Less`'e bakar ve `Ordering::Greater`
değerinin `Ordering::Less` (Daha Küçük) ile eşleşmediğini görür; bu yüzden
o koldaki kodu yok sayar ve bir sonraki kola geçer. Sonraki kolun
deseni `Ordering::Greater`'dır, ki bu da `Ordering::Greater` ile _eşleşir_!
O koldaki ilişkili kod çalıştırılacak ve ekrana `Çok büyük!` yazdıracaktır.
`match` ifadesi ilk başarılı eşleşmeden sonra sona erer, bu senaryoda
artık son kola bakmayacaktır.

Fakat Liste 2-4'teki kod henüz derlenmeyecektir. Deneyelim:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Hatanın özü _uyumsuz türler_ (mismatched types) olduğunu belirtiyor.
Rust güçlü, statik bir tür sistemine (static type system) sahiptir. Ancak,
aynı zamanda tür çıkarımı (type inference) da vardır. `let mut tahmin = String::new()`
yazdığımızda, Rust `tahmin`'in bir `String` olması gerektiği çıkarımını
yapabildi ve bizim türü yazmamıza gerek bırakmadı. Öte yandan, `gizli_sayi`
bir sayı türüdür. Rust'ın 1 ile 100 arasında değer alabilen birkaç
sayı türü vardır: 32 bitlik bir sayı olan `i32`; işaretsiz (unsigned) 32 bitlik
bir sayı olan `u32`; 64 bitlik bir sayı olan `i64`; ve diğerleri. Aksi
belirtilmediği sürece Rust varsayılan olarak `i32` kullanır. Eğer Rust'ın
başka bir sayısal tür çıkarımı yapmasına neden olacak tür bilgisini
başka bir yere eklemediyseniz, `gizli_sayi`'nın türü budur. Hatanın nedeni,
Rust'ın bir metin türü (string) ile bir sayı türünü karşılaştıramamasıdır.

Nihayetinde, programın girdi olarak okuduğu `String`'i bir sayı türüne
dönüştürmek istiyoruz, böylece onu gizli sayıyla sayısal olarak
karşılaştırabileceğiz. Bunu `main` fonksiyonunun gövdesine
şu satırı ekleyerek yapıyoruz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

O satır şudur:

```rust,ignore
let tahmin: u32 = tahmin.trim().parse().expect("Lütfen bir sayı yazın!");
```

`tahmin` adında bir değişken oluşturuyoruz. Ama bir dakika, programda zaten
`tahmin` adında bir değişken yok muydu? Var, ancak Rust yararlı bir şekilde
`tahmin`'in önceki değerini yenisiyle gölgelendirmemize (shadow) olanak tanır.
_Gölgelendirme_, bizi örneğin `tahmin_str` ve `tahmin` gibi
iki farklı değişken oluşturmaya zorlamak yerine `tahmin` değişken adını
yeniden kullanmamıza olanak tanır. Bunu [Bölüm 3][shadowing]<!-- ignore -->'te
daha ayrıntılı ele alacağız, ancak şimdilik bu özelliğin genellikle
bir değeri bir türden diğerine dönüştürmek istediğinizde kullanıldığını bilin.

Bu yeni değişkeni `tahmin.trim().parse()` ifadesine bağlıyoruz.
İfadedeki `tahmin`, girdiyi metin (string) olarak barındıran
orijinal `tahmin` değişkenine işaret eder. Bir `String` örneği üzerindeki
`trim` metodu, metnin başındaki ve sonundaki tüm boşlukları ortadan kaldıracaktır.
Metni (string) yalnızca sayısal veri içerebilen bir `u32`'ye dönüştürmeden önce
bunu yapmamız gerekir. Kullanıcının `read_line`'ı tamamlamak ve tahminini
girmek için <kbd>enter</kbd> (giriş) tuşuna basması gerekir; bu da metne
bir yeni satır (newline) karakteri ekler. Örneğin, kullanıcı <kbd>5</kbd> yazıp
<kbd>enter</kbd>'a basarsa, `tahmin` şöyle görünür: `5\n`. `\n` "yeni satırı"
temsil eder. (Windows'ta <kbd>enter</kbd>'a basmak satır başı ve yeni satır,
`\r\n` ile sonuçlanır.) `trim` metodu `\n` veya `\r\n`'i ortadan kaldırarak
sadece `5` olmasını sağlar.

Metinler (strings) üzerindeki [`parse` metodu][parse]<!-- ignore -->,
bir metni başka bir türe dönüştürür. Burada, onu bir metinden bir sayıya
dönüştürmek için kullanıyoruz. `let tahmin: u32` diyerek Rust'a tam
olarak istediğimiz sayı türünü söylemeliyiz. `tahmin`'den sonraki
iki nokta üst üste (`:`), Rust'a değişkenin türünü belirteceğimizi söyler.
Rust'ın birkaç yerleşik sayı türü vardır; burada gördüğünüz `u32` işaretsiz,
32 bitlik bir tam sayıdır. Küçük pozitif bir sayı için iyi bir
varsayılan seçimdir. Diğer sayı türlerini [Bölüm 3][integers]<!-- ignore -->'te
öğreneceksiniz.

Ek olarak, bu örnek programdaki `u32` bildirmi (annotation) ve `gizli_sayi` ile
yapılan karşılaştırma, Rust'ın `gizli_sayi`'nın da bir `u32` olması gerektiği
çıkarımını yapacağı anlamına gelir. Yani, artık karşılaştırma aynı
türden iki değer arasında olacaktır!

`parse` metodu yalnızca mantıksal olarak sayılara dönüştürülebilen
karakterler üzerinde çalışacaktır ve bu nedenle kolayca hatalara neden olabilir.
Örneğin, metin `A👍%` içerseydi, bunu bir sayıya dönüştürmenin hiçbir yolu olmazdı.
Başarısız olabileceği için `parse` metodu da `read_line` metodunun yaptığı gibi
(daha önce ["Potansiyel Bir Hatayı `Result` ile Ele Almak"](#potansiyel-bir-hatayı-result-ile-ele-almak)<!-- ignore --> kısmında tartışılmıştı) bir `Result`
türü döndürür. Bu `Result`'ı, `expect` metodunu tekrar kullanarak aynı
şekilde ele alacağız. Eğer `parse` metodu, metinden bir sayı oluşturamadığı
için bir `Err` `Result` varyantı döndürürse, `expect` çağrısı oyunu çökertecek
(crash) ve ona verdiğimiz mesajı yazdıracaktır. Eğer `parse`, metni
başarıyla bir sayıya dönüştürebilirse, `Result`'ın `Ok` varyantını döndürür;
ve `expect`, `Ok` değerinden istediğimiz numarayı döndürecektir.

Programı şimdi çalıştıralım:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
touch src/main.rs
cargo run
  76
-->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Gizli sayı: 58
Lütfen tahmininizi girin.
  76
Tahmininiz: 76
Çok büyük!
```

Güzel! Tahminden önce boşluklar eklenmiş olmasına rağmen, program yine de kullanıcının
76 tahmininde bulunduğunu anladı (figured out). Farklı türdeki girdilerle farklı
davranışları doğrulamak için programı birkaç kez çalıştırın: Sayıyı doğru tahmin edin,
çok büyük bir sayı tahmin edin ve çok küçük bir sayı tahmin edin.

Şu anda oyunun büyük bir kısmı çalışıyor, ancak kullanıcı yalnızca bir tahmin yapabiliyor.
Bir döngü (loop) ekleyerek bunu değiştirelim!

## Döngüyle Birden Fazla Tahmine İzin Vermek

`loop` anahtar kelimesi sonsuz bir döngü oluşturur. Kullanıcılara sayıyı tahmin
etmeleri için daha fazla şans vermek adına bir döngü ekleyeceğiz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Gördüğünüz gibi, tahmini giriş isteminden itibaren her şeyi bir döngü
içine taşıdık. Döngünün içindeki satırların her birini fazladan dört
boşluk (space) girintilediğinizden (indent) emin olun ve programı
tekrar çalıştırın. Program artık sonsuza dek başka bir tahmin isteyecektir,
ki bu aslında yeni bir sorun ortaya çıkarır. Kullanıcı çıkamayacak gibi
görünüyor!

Kullanıcı her zaman <kbd>ctrl</kbd>-<kbd>C</kbd> klavye kısayolunu
kullanarak programı durdurabilir. Ancak, ["Tahmini Gizli Sayıyla Karşılaştırmak"](#tahmini-gizli-sayıyla-karşılaştırmak)<!-- ignore --> kısmındaki `parse`
tartışmasında bahsedildiği gibi, bu doyumsuz canavardan kaçmanın başka bir yolu daha var:
Eğer kullanıcı sayı olmayan bir girdi girerse, program çökecektir (crash).
Aşağıda gösterildiği gibi kullanıcının çıkabilmesini (quit) sağlamak için
bundan faydalanabiliriz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
touch src/main.rs
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Gizli sayı: 59
Lütfen tahmininizi girin.
45
Tahmininiz: 45
Çok küçük!
Lütfen tahmininizi girin.
60
Tahmininiz: 60
Çok büyük!
Lütfen tahmininizi girin.
59
Tahmininiz: 59
Kazandınız!
Lütfen tahmininizi girin.
cikis

thread 'main' panicked at src/main.rs:28:47:
Lütfen bir sayı yazın!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`cikis` (quit) yazmak oyundan çıkmanızı sağlayacaktır, ancak
fark edeceğiniz üzere başka herhangi bir sayısal olmayan girdi de aynısını yapacaktır.
Bu, en hafif tabirle ideal bir durum değildir (suboptimal); oyunun, doğru
sayı tahmin edildiğinde de durmasını istiyoruz.

### Doğru Tahminden Sonra Çıkmak

Kullanıcı kazandığında oyundan çıkmasını sağlamak için bir `break`
(kır) ifadesi ekleyerek oyunu programlayalım:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

`Kazandınız!` ifadesinden sonra `break` satırını eklemek, kullanıcı gizli
sayıyı doğru tahmin ettiğinde programın döngüden çıkmasını sağlar.
Döngüden çıkmak aynı zamanda programdan çıkmak anlamına da gelir,
çünkü döngü `main`'in son kısmıdır.

### Geçersiz Girdiyi Ele Almak

Oyunun davranışını daha da iyileştirmek için, kullanıcı sayısal olmayan
bir girdi girdiğinde programı çökertmek (crash) yerine, kullanıcının tahmin etmeye
devam edebilmesi için oyunun sayı olmayan veriyi görmezden gelmesini sağlayalım.
Bunu Liste 2-5'te gösterildiği gibi, `tahmin`'in `String`'den `u32`'ye
dönüştürüldüğü satırı değiştirerek yapabiliriz.

<Listing number="2-5" file-name="src/main.rs" caption="Sayısal olmayan bir tahmini yok saymak ve programı çökertmek yerine başka bir tahmin istemek">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

</Listing>

Bir hata durumunda programı çökertmekten ziyade hatayı yönetmeye (handle the error)
geçmek için, bir `expect` çağrısından çıkıp `match` ifadesine geçiş yapıyoruz.
`parse`'ın bir `Result` türü döndürdüğünü ve `Result`'ın `Ok` ile `Err` varyantlarına
sahip bir enum olduğunu hatırlayın. Burada, tıpkı `cmp` metodunun `Ordering` sonucunda
yaptığımız gibi bir `match` ifadesi kullanıyoruz.

Eğer `parse` metni başarıyla bir sayıya dönüştürebilirse, elde edilen
sayıyı içeren bir `Ok` değeri döndürür. O `Ok` değeri, ilk kolun deseniyle
eşleşecek ve `match` ifadesi, sadece `parse`'ın ürettiği ve `Ok` değerinin
içine koyduğu `sayi` (num) değerini döndürecektir. Bu sayı, oluşturduğumuz yeni
`tahmin` değişkeninde tam da istediğimiz yere gelecektir.

Eğer `parse` metni bir sayıya dönüştüremezse (_not_ able), hata hakkında daha
fazla bilgi barındıran bir `Err` değeri döndürür. `Err` değeri, ilk `match` kolundaki
`Ok(sayi)` deseni ile eşleşmez, ancak ikinci koldaki `Err(_)` deseni ile eşleşir.
Alt çizgi (`_`), her şeyi yakalayan (catch-all) bir değerdir; bu örnekte, içlerinde
ne bilgi olursa olsun tüm `Err` değerleriyle eşleşmek istediğimizi söylüyoruz. Böylece,
program ikinci kolun kodu olan `continue`'yu (devam et) çalıştırır; bu da
programa `loop`'un bir sonraki döngüsüne (iteration) geçmesini ve
başka bir tahmin istemesini söyler. Yani etkin olarak, program
`parse`'ın karşılaşabileceği tüm hataları yok sayar!

Artık programdaki her şey beklendiği gibi çalışmalıdır. Deneyelim:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Gizli sayı: 61
Lütfen tahmininizi girin.
10
Tahmininiz: 10
Çok küçük!
Lütfen tahmininizi girin.
99
Tahmininiz: 99
Çok büyük!
Lütfen tahmininizi girin.
foo
Lütfen tahmininizi girin.
61
Tahmininiz: 61
Kazandınız!
```

Harika! Son bir küçük ayarlama (tweak) ile tahmin oyununu bitireceğiz. Programın hâlâ
gizli sayıyı yazdırdığını hatırlayın. Bu, test için iyi çalıştı, ancak oyunu
mahvediyor. Gizli sayıyı çıktı olarak veren `println!` ifadesini silelim.
Liste 2-6 son kodu göstermektedir.

<Listing number="2-6" file-name="src/main.rs" caption="Tamamlanmış tahmin oyunu kodu">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

</Listing>

Bu noktada tahmin oyununu başarıyla inşa ettiniz. Tebrikler!

## Özet

Bu proje sizi birçok yeni Rust kavramıyla uygulamalı olarak
tanıştırmanın bir yoluydu: `let`, `match`, fonksiyonlar, harici crate'lerin
kullanımı ve daha fazlası. Sonraki birkaç bölümde bu kavramlar
hakkında daha fazla ayrıntı öğreneceksiniz. Bölüm 3, çoğu
programlama dilinin sahip olduğu değişkenler, veri türleri ve
fonksiyonlar gibi kavramları ele alır ve bunların Rust'ta
nasıl kullanılacağını gösterir. Bölüm 4, Rust'ı diğer dillerden
farklı kılan bir özellik olan sahipliği araştırır.
Bölüm 5 struct'ları (yapıları) ve metot sözdizimini (method syntax) tartışırken,
Bölüm 6 enum'ların nasıl çalıştığını açıklar.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-degiskenler-ve-degistirilebilirlik.html#değişkenler-ve-değiştirilebilirlik-mutability
[comments]: ch03-04-yorumlar.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enumlar-ve-desen-eslestirme.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-result-ile-kurtarilabilir-hatalar.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match-kontrol-akisi.html
[shadowing]: ch03-01-degiskenler-ve-degistirilebilirlik.html#gölgelendirme-shadowing
[parse]: ../std/primitive.str.html#method.parse
[integers]: ch03-02-veri-turleri.html#tam-sayı-türleri-integer-types
