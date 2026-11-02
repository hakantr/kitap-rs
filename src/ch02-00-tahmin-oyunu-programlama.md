<!-- Old headings. Do not remove or links may break. -->

<a id="programming-a-guessing-game"></a>
<a id="tahmin-oyunu-programlama"></a>

# Bir Tahmin Oyunu Programlama

Birlikte uygulamalı bir proje üzerinde çalışarak Rust'a hızlı bir giriş yapalım! Bu bölüm, yaygın kullanılan bazı Rust kavramlarını gerçek bir program içinde nasıl kullanacağınızı göstererek tanıtır. `let`, `match`, metotlar, *ilişkili fonksiyonlar (associated functions)*, harici crate'ler ve daha pek çok kavramla tanışacaksınız! İlerleyen bölümlerde bu konuları tüm ayrıntılarıyla inceleyeceğiz; bu bölümde ise yalnızca temellerin pratiğini yapacağız.

Klasik bir başlangıç programlama problemini uygulayacağız: bir sayı tahmin oyunu. Oyun şöyle çalışacak: Program 1 ile 100 arasında rastgele bir tam sayı üretecek ve ardından oyuncudan bir tahmin girmesini isteyecek. Oyuncu tahminini girdikten sonra program, bu tahminin çok düşük mü yoksa çok yüksek mi olduğunu belirtecek. Tahmin doğruysa oyun bir tebrik mesajı yazdırıp sonlanacak.

<a id="setting-up-a-new-project"></a>

## Yeni Bir Proje Kurulumu

Yeni bir proje kurmak için Bölüm 1'de oluşturduğunuz _projects_ dizinine gidin ve Cargo kullanarak aşağıdaki gibi yeni bir proje oluşturun:

```console
$ cargo new tahmin_oyunu
$ cd tahmin_oyunu
```

İlk komut olan `cargo new`, projenin adını (`tahmin_oyunu`) ilk argüman olarak alır. İkinci komut ise çalışma dizinini yeni oluşturulan projenin dizinine geçirir.

Oluşturulan _Cargo.toml_ dosyasına göz atalım:

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

Bölüm 1'de gördüğünüz gibi `cargo new` sizin için bir "Merhaba, dünya!" başlangıç programı üretir (varsayılan şablon `"Hello, world!"` yazdırır; burada Türkçe karşılığıyla gösterilmiştir). _src/main.rs_ dosyasına göz atın:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Şimdi bu "Merhaba, dünya!" programını `cargo run` komutuyla tek adımda derleyip çalıştıralım:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

`run` komutu, bu oyunda da yapacağımız gibi bir proje üzerinde hızlı adımlarla geliştirme yapmanız ve bir sonraki aşamaya geçmeden önce her adımı çabucak test etmeniz gerektiğinde çok işinize yarar.

_src/main.rs_ dosyasını yeniden açın. Tüm kodu bu dosyaya yazacaksınız.

<a id="processing-a-guess"></a>

## Bir Tahmini İşlemek

Tahmin oyunu programının ilk kısmı kullanıcıdan girdi isteyecek, bu girdiyi işleyecek ve girdinin beklenen biçimde olup olmadığını kontrol edecektir. Başlangıç olarak oyuncunun bir tahmin girmesini sağlayalım. Liste 2-1'deki kodu _src/main.rs_ dosyasına yazın.

<Listing number="2-1" file-name="src/main.rs" caption="Kullanıcıdan bir tahmin alıp ekrana yazdıran kod">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

</Listing>

Bu kod pek çok bilgi içeriyor; gelin satır satır üzerinden geçelim. Kullanıcıdan girdi almak ve ardından sonucu çıktı olarak yazdırmak için girdi/çıktı kütüphanesi olan `io` modülünü kapsama dahil etmemiz gerekir. `io` kütüphanesi, `std` olarak bilinen standart kütüphaneden gelir:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Rust, varsayılan olarak standart kütüphanede tanımlanmış belirli bir öğe kümesini her programın kapsamına otomatik olarak dahil eder. Bu kümeye _prelude_ (ön tanımlı kapsam) adı verilir ve içindeki her şeyi [standart kütüphane belgelerinde][prelude] görebilirsiniz.

Kullanmak istediğiniz bir tür prelude içinde yer almıyorsa o türü bir `use` deyimiyle açıkça kapsama dahil etmeniz gerekir. `std::io` kütüphanesini kullanmak, kullanıcı girdisini kabul etme olanağı da dahil olmak üzere pek çok yararlı özellik sunar.

Bölüm 1'de gördüğünüz gibi `main` fonksiyonu programın giriş noktasıdır:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

`fn` sözdizimi yeni bir fonksiyon bildirir; `()` parantezleri parametre bulunmadığını gösterir; `{` süslü parantezi ise fonksiyonun gövdesini başlatır.

Yine Bölüm 1'de öğrendiğiniz gibi `println!`, ekrana bir metin yazdıran bir makrodur:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Bu kod, oyunun ne olduğunu belirten bir karşılama metni yazdırır ve kullanıcıdan girdi ister.

<a id="storing-values-with-variables"></a>

### Değerleri Değişkenlerle Saklamak

Sırada, kullanıcı girdisini saklamak üzere aşağıdaki gibi bir _değişken_ oluşturmak var:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Şimdi program ilginçleşmeye başlıyor! Bu kısacık satırda pek çok şey gerçekleşiyor. Bir değişken oluşturmak için `let` deyimini kullanırız. İşte başka bir örnek:

```rust,ignore
let elmalar = 5;
```

Bu satır `elmalar` adında yeni bir değişken oluşturur ve onu `5` değerine *bağlar (bind)*. Rust'ta değişkenler varsayılan olarak değiştirilemezdir (*immutable*); yani değişkene bir kez değer verdiğimizde bu değer bir daha değişmez. Bu kavramı Bölüm 3'teki ["Değişkenler ve Değiştirilebilirlik"][variables-and-mutability]<!-- ignore --> kısmında ayrıntılı olarak inceleyeceğiz. Bir değişkeni değiştirilebilir (*mutable*) yapmak için değişken adından önce `mut` anahtar kelimesini ekleriz:

```rust,ignore
let elmalar = 5; // değiştirilemez
let mut muzlar = 5; // değiştirilebilir
```

> Not: `//` sözdizimi, satır sonuna kadar devam eden bir yorum başlatır. Rust yorumların içindeki her şeyi yok sayar. Yorumları [Bölüm 3][comments]<!-- ignore -->'te daha ayrıntılı olarak ele alacağız.

Tahmin oyunu programına dönecek olursak artık `let mut tahmin` kısmının `tahmin` adında değiştirilebilir bir değişken tanımlayacağını biliyorsunuz. Eşittir işareti (`=`), Rust'a değişkene şimdi bir değer bağlamak istediğimizi söyler. Eşittir işaretinin sağında `tahmin` değişkeninin bağlandığı değer yer alır; bu değer, yeni bir `String` örneği döndüren `String::new` fonksiyonunun çağrılmasıyla elde edilir. [`String`][string]<!-- ignore -->, standart kütüphane tarafından sağlanan, büyüyebilen ve UTF-8 kodlamalı bir metin türüdür.

`::new` yazımındaki `::` sözdizimi, `new` fonksiyonunun `String` türüne ait bir ilişkili fonksiyon olduğunu gösterir. Bir _ilişkili fonksiyon_, belirli bir tür (bu örnekte `String`) üzerinde tanımlanmış fonksiyondur. Bu `new` fonksiyonu yeni ve boş bir string oluşturur. Pek çok tür üzerinde bir `new` fonksiyonuyla karşılaşırsınız; çünkü `new`, herhangi bir türden yeni bir değer üreten fonksiyonlar için yaygın bir isimdir.

Özetle `let mut tahmin = String::new();` satırı, şu an yeni ve boş bir `String` örneğine bağlı olan değiştirilebilir bir değişken oluşturmuştur. Vay canına!

<a id="receiving-user-input"></a>

### Kullanıcı Girdisini Almak

Programın ilk satırında standart kütüphanedeki girdi/çıktı işlevselliğini `use std::io;` ile dahil ettiğimizi hatırlayın. Şimdi kullanıcı girdisini işleyebilmek için `io` modülündeki `stdin` fonksiyonunu çağıracağız:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Programın başında `use std::io;` ile `io` modülünü kapsama dahil etmemiş olsaydık bile, bu fonksiyon çağrısını `std::io::stdin` şeklinde açıkça yazarak yine kullanabilirdik. `stdin` fonksiyonu, terminalinizin standart girdisine ait bir tutamacı (*handle*) temsil eden [`std::io::Stdin`][iostdin]<!-- ignore --> türünden bir örnek döndürür.

Ardından gelen `.read_line(&mut tahmin)` satırı, kullanıcıdan girdi almak üzere standart girdi tutamacı üzerinde [`read_line`][read_line]<!-- ignore --> metodunu çağırır. Kullanıcı girdisini hangi string içine kaydedeceğini belirtmek için `read_line` metoduna argüman olarak `&mut tahmin` değerini geçiriyoruz. `read_line` metodunun görevi, kullanıcının standart girdiye yazdığı metni alıp mevcut içeriğin üzerine yazmadan verilen string'in sonuna eklemektir; bu nedenle hedef string'i argüman olarak iletiriz. Metodun string içeriğini değiştirebilmesi için bu argümanın değiştirilebilir olması gerekir.

`&` işareti bu argümanın bir *referans (reference)* olduğunu belirtir. Referanslar, veriyi bellekte defalarca kopyalamaya gerek kalmadan kodunuzun farklı bölümlerinin aynı veriye erişmesine olanak tanır. Referanslar karmaşık bir özelliktir ve Rust'ın en büyük güçlü yönlerinden biri referansları hem son derece güvenli hem de kullanımı kolay biçimde sunmasıdır. Bu programı tamamlamak için tüm bu ayrıntıları şimdiden bilmenize gerek yok. Şimdilik bilmeniz gereken tek şey, tıpkı değişkenler gibi referansların da varsayılan olarak değiştirilemez olduğudur. Dolayısıyla referansı değiştirilebilir kılmak için `&tahmin` yerine `&mut tahmin` yazmanız gerekir. (Bölüm 4 referansları çok daha kapsamlı biçimde açıklayacaktır.)

<!-- Old headings. Do not remove or links may break. -->

<a id="handling-potential-failure-with-the-result-type"></a>
<a id="handling-potential-failure-with-result"></a>

### Potansiyel Bir Hatayı `Result` ile Ele Almak

Hâlâ aynı kod satırı üzerinde çalışıyoruz. Metin olarak üçüncü satırı inceliyor olsak da bunun hâlâ tek bir mantıksal kod satırının parçası olduğuna dikkat edin. Sıradaki kısım şu metot çağrısıdır:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Bu kodu tek satırda şu şekilde de yazabilirdik:

```rust,ignore
io::stdin().read_line(&mut tahmin).expect("Satır okunamadı");
```

Ancak tek bir uzun satırı okumak zordur; bu yüzden onu bölmek en doğrusudur. `.metot_adi()` sözdizimiyle zincirleme metot çağrıları yaparken uzun satırları yeni satır ve girintilerle ayırmak okunabilirliği büyük ölçüde artırır. Şimdi bu satırın ne işe yaradığına bakalım.

Daha önce belirttiğimiz gibi `read_line`, kullanıcının yazdığı metni kendisine geçirdiğimiz string içine ekler; ancak bunun yanı sıra bir `Result` değeri de döndürür. [`Result`][result]<!-- ignore -->, genellikle kısaca _enum_ olarak adlandırılan bir [_numaralandırma (enumeration)_][enums]<!-- ignore --> türüdür; yani birden fazla olası durumdan birinde bulunabilen bir türdür. Bu olası durumların her birine *varyant (variant)* adını veririz.

[Bölüm 6][enums]<!-- ignore --> enum'ları çok daha ayrıntılı biçimde ele alacaktır. `Result` türlerinin amacı, hata yönetimine ilişkin bilgiyi taşımaktır.

`Result` türünün varyantları `Ok` ve `Err`'dir. `Ok` varyantı işlemin başarılı olduğunu belirtir ve başarıyla üretilen değeri içinde taşır. `Err` varyantı ise işlemin başarısız olduğu anlamına gelir ve işlemin nasıl ya da neden başarısız olduğuna dair bilgiyi barındırır.

Diğer türlerdeki değerler gibi `Result` türündeki değerlerin de üzerlerinde tanımlanmış metotları vardır. Bir `Result` örneği, çağırabileceğiniz bir [`expect` metoduna][expect]<!-- ignore --> sahiptir. Eğer bu `Result` örneği bir `Err` değeriyse `expect` programın çökmesine (*panic*) neden olur ve `expect`'e argüman olarak verdiğiniz mesajı ekrana yazdırır. `read_line` metodunun bir `Err` döndürmesi büyük olasılıkla altta yatan işletim sisteminden kaynaklanan bir girdi/çıktı hatasının sonucu olacaktır. Öte yandan bu `Result` örneği bir `Ok` değeriyse `expect` metodu `Ok` içinde tutulan dönüş değerini çıkarır ve doğrudan kullanabilmeniz için size döndürür. Bu örnekte söz konusu değer, kullanıcının girdisindeki bayt sayısıdır (UTF-8 kodlamasında her bir ASCII karakteri 1 bayt, `'ç'`, `'ğ'`, `'ı'`, `'ö'`, `'ş'`, `'ü'` gibi Türkçe harfler ise ikişer bayt yer kaplar).

Eğer `expect` metodunu çağırmazsanız program yine derlenir, ancak derleyiciden bir uyarı alırsınız:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust derleyicisi, `read_line` çağrısından dönen `Result` değerini kullanmadığınız konusunda (`unused Result that must be used`) sizi uyarır; bu uyarı programın olası bir hatayı henüz ele almadığına işaret eder.

Bu uyarıyı gidermenin doğru yolu gerçek bir hata yönetimi kodu yazmaktır; ancak şu anki aşamada bir sorun oluştuğunda programın doğrudan durmasını istediğimizden `expect` kullanabiliriz. Hatalardan kurtularak çalışmaya devam etmeyi [Bölüm 9][recover]<!-- ignore -->'da öğreneceksiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="printing-values-with-println-placeholders"></a>
<a id="println-yer-tutucuları-placeholders-ile-değerleri-yazdırmak"></a>

### `println!` Yer Tutucuları ile Değerleri Yazdırmak

Kapanış süslü parantezini saymazsak şu ana kadarki kodda inceleyeceğimiz yalnızca bir satır kaldı:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Bu satır, artık kullanıcının girdisini barındıran string'i ekrana yazdırır. `{}` süslü parantez çifti bir *yer tutucudur (placeholder)*: `{}` işaretini, bir değeri yerinde tutan küçük yengeç kıskaçları gibi düşünebilirsiniz. Bir değişkenin değerini yazdırırken değişken adını doğrudan süslü parantezlerin içine yazabilirsiniz. Bir ifadenin (*expression*) hesaplanan sonucunu yazdırırken ise biçimlendirme metnine boş `{}` süslü parantezler yerleştirir, ardından biçimlendirme metninden sonra virgülle ayrılmış ifadeleri sırasıyla verirsiniz; Rust bu ifadelerin sonuçlarını boş yer tutuculara aynı sırayla yerleştirir. Tek bir `println!` çağrısında hem bir değişkeni hem de bir ifadenin sonucunu yazdırmak şöyle görünür:

```rust
let x = 5;
let y = 10;

println!("x = {x} ve y + 2 = {}", y + 2);
```

Bu kod ekrana `x = 5 ve y + 2 = 12` yazdıracaktır.

<a id="testing-the-first-part"></a>

### İlk Kısmı Test Etmek

Tahmin oyununun ilk bölümünü test edelim. `cargo run` komutuyla programı çalıştırın:

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

Bu noktada oyunun ilk kısmı tamamlandı: Klavyeden girdi alıyor ve bu girdiyi ekrana yazdırabiliyoruz.

<a id="generating-a-secret-number"></a>

## Gizli Bir Sayı Üretmek

Sırada, kullanıcının tahmin etmeye çalışacağı gizli bir sayı üretmek var. Oyunun tekrar tekrar oynanmasının keyifli olması için gizli sayı her seferinde farklı olmalıdır. Oyunun çok zor olmaması adına 1 ile 100 arasında rastgele bir sayı kullanacağız. Rust, standart kütüphanesinde henüz rastgele sayı üretme işlevselliği barındırmaz. Bununla birlikte Rust ekibi, bu işlevselliği sunan [`rand` crate'ini][randcrate] sağlamaktadır.

<!-- Old headings. Do not remove or links may break. -->
<a id="using-a-crate-to-get-more-functionality"></a>
<a id="increasing-functionality-with-a-crate"></a>

### Bir Crate ile İşlevselliği Artırmak

Bir crate'in, Rust kaynak kodu dosyalarından oluşan bir bütün olduğunu hatırlayın. Şimdiye kadar geliştirdiğimiz proje, çalıştırılabilir bir dosya üreten bir *ikili crate*'tir (*binary crate*). `rand` crate'i ise başka programlarda kullanılmak üzere tasarlanmış kodlar içeren ve tek başına çalıştırılamayan bir *kütüphane crate'idir* (*library crate*).

Harici crate'lerin yönetimi, Cargo'nun gerçekten parladığı alandır. `rand` kullanan kodu yazmadan önce _Cargo.toml_ dosyasını düzenleyip `rand` crate'ini bir *bağımlılık (dependency)* olarak eklememiz gerekir. Şimdi _Cargo.toml_ dosyasını açın ve Cargo'nun sizin için oluşturduğu `[dependencies]` bölüm başlığının altına aşağıdaki satırı ekleyin. `rand` paketini burada verdiğimiz sürüm numarasıyla birebir aynı şekilde belirttiğinizden emin olun; aksi takdirde bu bölümdeki kod örnekleri çalışmayabilir:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:

* ch01-01-installation.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

_Cargo.toml_ dosyasında bir başlıktan sonra gelen her satır, yeni bir bölüm başlayana kadar o bölümün parçası kabul edilir. `[dependencies]` bölümünde Cargo'ya, projenizin hangi harici crate'lere bağımlı olduğunu ve bu crate'lerin hangi sürümlerine ihtiyaç duyduğunuzu bildirirsiniz. Bu örnekte `rand` crate'ini semantik sürüm belirteci `0.10.1` ile belirtiyoruz. Cargo, sürüm numaralarını yazmaya yönelik bir standart olan [Semantik Sürümlemeyi][semver]<!-- ignore --> (kısaca _SemVer_) anlar. `0.10.1` belirteci aslında `^0.10.1` ifadesinin kısaltmasıdır; bu da en az `0.10.1` olan ancak `0.11.0`'ın altında kalan her sürümün kullanılabileceği anlamına gelir.

Cargo bu aralıktaki sürümlerin `0.10.1` sürümüyle uyumlu genel API'lere sahip olduğunu kabul eder; bu tanım sayesinde bu bölümdeki kodla sorunsuz derlenmeye devam edecek en güncel yama sürümünü (*patch release*) almanız sağlanır. `0.11.0` veya üzerindeki sürümlerin ise aşağıdaki örneklerin kullandığı API ile aynı kalacağı garanti edilmez.

> Not: SemVer'de sürüm numarası `ANA.ALT.YAMA` (*major.minor.patch*)
> biçimindedir. Ana sürümü `1` veya daha büyük olan kararlı API'lerde, geriye
> dönük uyumsuz değişiklikler ana sürüm numarası artırılarak yayımlanır. Ana
> sürümü `0` olan crate'lerde ise API henüz kararlı kabul edilmez. Cargo,
> `0.10.1` gibi bir belirteçte uyumluluk sınırı olarak ilk sıfır olmayan
> bileşeni (`10`) kullanır ve `0.11.0`'ı uyumsuz kabul eder. Ana sürümü `1` veya
> daha büyük olan bir crate için `"1.2.3"` yazmak, `1.2.3` dahil bu sürümden
> başlayıp `2.0.0` hariç bu sınıra kadar olan uyumlu sürümlere izin verir;
> `1.2.2` gibi daha eski bir sürüme izin vermez.

Şimdi kodda hiçbir değişiklik yapmadan Liste 2-2'de gösterildiği gibi projeyi derleyelim.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

<Listing number="2-2" caption="`rand` crate'ini bağımlılık olarak ekledikten sonra `cargo build` komutunun çıktısı">

```console
$ cargo build
    Updating crates.io index
     Locking 8 packages to latest Rust 1.96.0 compatible versions
  Downloaded rand_core v0.10.1
  Downloaded chacha20 v0.10.1
  Downloaded rand v0.10.1
  Downloaded 3 crates (162.9KiB) in 0.59s
   Compiling libc v0.2.186
   Compiling rand_core v0.10.1
   Compiling getrandom v0.4.3
   Compiling cfg-if v1.0.4
   Compiling chacha20 v0.10.1
   Compiling rand v0.10.1
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.03s
```

</Listing>

Kendi ekranınızda farklı sürüm numaraları (ancak SemVer sayesinde hepsi bu kodla uyumlu olacaktır!) ve işletim sisteminize bağlı olarak farklı satırlar görebilirsiniz; ayrıca bu satırların sırası da değişebilir.

Harici bir bağımlılık eklediğimizde Cargo, o bağımlılığın ihtiyaç duyduğu tüm paketlerin uyumlu en son sürümlerini [Crates.io][cratesio] verilerinin bir kopyası olan *kayıt dizininden (registry)* çeker. Crates.io, Rust ekosistemindeki geliştiricilerin açık kaynaklı Rust projelerini başkalarının da kullanabilmesi için yayınladıkları merkezdir.

Cargo, kayıt dizinini güncelledikten sonra `[dependencies]` bölümünü kontrol eder ve listelenen ancak henüz bilgisayarınızda önbelleğe alınmamış tüm crate'leri indirir. Bu örnekte bağımlılık olarak yalnızca `rand`'i yazmış olsak da Cargo, `rand`'in çalışmak için ihtiyaç duyduğu alt crate'leri de (`rand_core`, `chacha20`, `getrandom` vb.) otomatik olarak temin eder. Crate'leri indirdikten sonra Rust önce bu bağımlılıkları, ardından da bağımlılıkları hazır hâle gelen projemizi derler.

Hiçbir değişiklik yapmadan hemen ardından `cargo build` komutunu tekrar çalıştırırsanız `Finished` satırı dışında hiçbir çıktı almazsınız. Cargo tüm bağımlılıkları zaten indirip derlediğini ve _Cargo.toml_ dosyanızda bunlarla ilgili bir değişiklik yapmadığınızı bilir. Kendi kodunuzda da bir değişiklik olmadığını bildiğinden onu da yeniden derlemez; yapılacak iş kalmadığı için doğrudan sonlanır.

_src/main.rs_ dosyasını açıp küçük bir değişiklik yapar, kaydeder ve tekrar derlerseniz yalnızca iki satırlık bir çıktı görürsünüz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

Bu satırlar, Cargo'nun derlemeyi yalnızca _src/main.rs_ dosyasındaki ufak değişikliğiniz için güncellediğini gösterir. Bağımlılıklarınız değişmediğinden Cargo daha önce indirip derlediği çıktıları olduğu gibi yeniden kullanır.

<!-- Old headings. Do not remove or links may break. -->
<a id="ensuring-reproducible-builds-with-the-cargo-lock-file"></a>
<a id="ensuring-reproducible-builds"></a>
<a id="tekrarlanabilir-derlemeleri-sağlamak-reproducible-builds"></a>

#### Tekrarlanabilir Derlemeleri Sağlamak

Cargo, siz veya bir başkası kodunuzu her derlediğinde birebir aynı derleme çıktısını (*artifact*) yeniden üretebilmenizi sağlayan bir mekanizmaya sahiptir: Siz aksi yönde talimat verene kadar Cargo yalnızca kilitlediği bağımlılık sürümlerini kullanır. Örneğin gelecek hafta `rand` crate'inin `0.10.2` sürümünün çıktığını, bu sürümün önemli bir hata düzeltmesi içerdiğini ancak aynı zamanda kodunuzu bozacak bir gerileme (*regression*) barındırdığını varsayalım. Bu sorunu önlemek için Rust, `cargo build` komutunu ilk çalıştırdığınızda _Cargo.lock_ dosyasını oluşturur; şu an _tahmin_oyunu_ dizinimizde bu dosya mevcuttur.

Bir projeyi ilk kez derlediğinizde Cargo, ölçütlerinize uyan tüm bağımlılık sürümlerini hesaplar ve kesin sürüm numaralarını _Cargo.lock_ dosyasına yazar. Projenizi ileride yeniden derlediğinizde Cargo _Cargo.lock_ dosyasının var olduğunu görür ve sürümleri baştan çözmek yerine doğrudan bu dosyada kayıtlı sürümleri kullanır. Böylece otomatik olarak tekrarlanabilir bir derleme elde edersiniz. Başka bir deyişle, _Cargo.lock_ dosyası sayesinde siz açıkça yükseltme yapana kadar projeniz `0.10.1` sürümünde kalacaktır. _Cargo.lock_ dosyası tekrarlanabilir derlemeler açısından kritik öneme sahip olduğundan genellikle projenin geri kalan kaynak kodlarıyla birlikte sürüm kontrol sistemine (örneğin Git'e) dahil edilir.

<a id="updating-a-crate-to-get-a-new-version"></a>

#### Yeni Bir Sürüm Almak İçin Bir Crate'i Güncellemek

Bir crate'i gerçekten güncellemek _istediğinizde_ Cargo bunun için `update` komutunu sunar. Bu komut mevcut _Cargo.lock_ dosyasını yok sayar, _Cargo.toml_ dosyasındaki sürüm kurallarınıza uyan en güncel sürümleri bulur ve bu yeni sürümleri _Cargo.lock_ dosyasına yazar. Varsayılan olarak Cargo yalnızca `0.10.1`'den büyük ve `0.11.0`'dan küçük sürümleri arayacaktır. Örneğin `rand` crate'i `0.10.2` ve `0.999.0` olmak üzere iki yeni sürüm yayınlamış olsaydı, `cargo update` komutunu çalıştırdığınızda şu çıktıyı görürdünüz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.96.0 compatible version
    Updating rand v0.10.1 -> v0.10.2 (available: v0.999.0)
```

Cargo `0.999.0` sürümünü dikkate almaz. Bu noktada _Cargo.lock_ dosyanızda da artık kullandığınız `rand` sürümünün `0.10.2` olarak güncellendiğini görürsünüz. `rand`'in `0.999.0` sürümünü veya `0.999.x` serisindeki herhangi bir sürümü kullanmak isteseydiniz _Cargo.toml_ dosyasını şu şekilde güncellemeniz gerekirdi (ancak bu değişikliği gerçekte yapmayın; çünkü aşağıdaki örnekler `rand` `0.10` serisini kullandığınızı varsaymaktadır):

```toml
[dependencies]
rand = "0.999.0"
```

Bir sonraki `cargo build` çalıştırışınızda Cargo, mevcut crate kayıt dizinini güncelleyecek ve `rand` gereksinimlerinizi belirttiğiniz yeni sürüme göre yeniden değerlendirecektir.

[Cargo][doccargo]<!-- ignore --> ve [ekosistemi][doccratesio]<!-- ignore --> hakkında söylenecek çok daha fazla şey var; bunları Bölüm 14'te ele alacağız, ancak şimdilik bilmeniz gerekenler bu kadar. Cargo kütüphaneleri yeniden kullanmayı son derece kolaylaştırdığı için Rust geliştiricileri (*Rustaceans*), birden çok paketin bir araya getirilmesiyle oluşan daha küçük ve odaklanmış projeler yazabilirler.

<a id="generating-a-random-number"></a>

### Rastgele Bir Sayı Üretmek

Şimdi tahmin edilecek sayıyı üretmek için `rand` crate'ini kullanmaya başlayalım. Bir sonraki adım, _src/main.rs_ dosyasını Liste 2-3'te gösterildiği gibi güncellemektir.

<Listing number="2-3" file-name="src/main.rs" caption="Rastgele bir sayı üretmek için kod ekleme">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

</Listing>

İlk olarak `use rand::prelude::*;` satırını ekliyoruz. `prelude` modülü, `rand` crate'inin en sık kullanılan öğelerini içerir; sondaki `*` işaretiyle birlikte bu `use` deyimi, modüldeki tüm öğeleri programımızın kapsamında doğrudan kullanılabilir hâle getirir.

Ardından ortaya iki yeni satır ekliyoruz. İlk satırda, kullanacağımız rastgele sayı üretecini bize veren `rand::rng` fonksiyonunu çağırıyoruz: Bu üreteç, çalışmakta olan mevcut thread'e (*iş parçacığına*) özeldir ve işletim sistemi tarafından tohumlanır (*seeded*). Sonrasında bu rastgele sayı üreteci üzerinde `random_range` metodunu çağırıyoruz. Bu metot, `use rand::prelude::*;` deyimiyle kapsama dahil ettiğimiz `RngExt` trait'i tarafından tanımlanmıştır. `RngExt` trait'i rastgele sayı üreteçlerinin uyguladığı metotları tanımlar ve bu metotları çağırabilmemiz için söz konusu trait'in kapsamda bulunması gerekir. Trait kavramını Bölüm 10'da ayrıntılı olarak inceleyeceğiz. `random_range` metodu argüman olarak bir aralık ifadesi (*range expression*) alır ve bu aralıkta rastgele bir sayı üretir. Burada kullandığımız aralık ifadesi `baslangic..=bitis` biçimindedir ve hem alt hem de üst sınırı aralığa dahil eder; dolayısıyla 1 ile 100 arasında bir sayı istemek için `1..=100` yazmamız gerekir.

> Not: Bir crate'ten neleri kapsama dahil edeceğinizi ve hangi metot ya da fonksiyonları çağıracağınızı ezbere bilemezsiniz; bu nedenle her crate nasıl kullanılacağını açıklayan kendi belgelerine sahiptir. Cargo'nun çok kullanışlı bir diğer özelliği de `cargo doc --open` komutudur: Bu komut, projenizdeki tüm bağımlılıkların belgelerini yerel olarak derler ve tarayıcınızda açar. Örneğin `rand` crate'indeki diğer işlevleri merak ediyorsanız `cargo doc --open` komutunu çalıştırıp sol kenar çubuğundan `rand`'e tıklayabilirsiniz.

Eklediğimiz ikinci satır gizli sayıyı ekrana yazdırır. Programı geliştirirken test edebilmek açısından bu oldukça faydalıdır, ancak son sürümde bu satırı sileceğiz. Program başlar başlamaz doğru cevabı söylerse pek de bir tahmin oyunu sayılmaz!

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

Her çalıştırmada farklı rastgele sayılar görmelisiniz ve bunların tamamı 1 ile 100 arasında olmalıdır. Derleme sırasında uyarılar alırsanız bunları güvenle göz ardı edebilirsiniz. Hata alırsanız lütfen _Cargo.toml_ dosyanızda `rand = "0.10.1"` yazdığından emin olun; `rand`'in gelecekteki ana sürümleri farklı bir API sunabilir, ancak `0.10` serisindeki herhangi bir sürüm bu bölümdeki kodla çalışacaktır.

> Not: İnternetteki daha eski Rust kaynaklarında veya kod örneklerinde `rand` crate'inin `0.8` sürümüne ait `use rand::Rng;` ve `rand::thread_rng().gen_range(1..=100)` kullanımlarıyla karşılaşabilirsiniz. `rand` `0.9` sürümüyle `rand::thread_rng()` yerine `rand::rng()`, `gen_range` yerine `random_range` kullanılmaya başlanmış; `0.10` sürümünde ise `random_range` gibi metotları sağlayan trait `Rng` yerine `RngExt` adını almıştır; bu kitaptaki `use rand::prelude::*;` satırı bu trait'i kapsama dahil eder.

<a id="comparing-the-guess-to-the-secret-number"></a>

## Tahmini Gizli Sayıyla Karşılaştırmak

Artık elimizde hem kullanıcı girdisi hem de rastgele bir sayı olduğuna göre bu ikisini karşılaştırabiliriz. Bu adım Liste 2-4'te gösterilmiştir. Açıklayacağımız nedenlerden ötürü bu kodun şu an için henüz derlenmeyeceğine dikkat edin.

<Listing number="2-4" file-name="src/main.rs" caption="İki sayıyı karşılaştırmanın olası dönüş değerlerini ele alma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

</Listing>

İlk olarak standart kütüphaneden `std::cmp::Ordering` adlı türü kapsama dahil eden bir `use` deyimi daha ekliyoruz. `Ordering` türü de bir enum'dır ve `Less` (daha küçük), `Greater` (daha büyük) ve `Equal` (eşit) varyantlarına sahiptir. İki değeri karşılaştırdığınızda ortaya çıkabilecek üç olası sonuç bunlardır.

Ardından dosyanın sonuna `Ordering` türünü kullanan beş yeni satır ekliyoruz. `cmp` metodu iki değeri karşılaştırır ve karşılaştırılabilen her değer üzerinde çağrılabilir. Karşılaştırmak istediğiniz değere bir referans alır: Burada `tahmin` değişkenini `gizli_sayi` ile karşılaştırıyoruz. Ardından `use` deyimiyle kapsama aldığımız `Ordering` enum'ının bir varyantını döndürür. `tahmin` ve `gizli_sayi` değerleriyle yapılan `cmp` çağrısından hangi `Ordering` varyantının döndüğüne göre ne yapacağımıza karar vermek için bir [`match`][match]<!-- ignore --> ifadesi kullanıyoruz.

Bir `match` ifadesi *kollardan (`match` arms)* oluşur. Her kol, eşleştirilecek bir *desenden (pattern)* ve `match`'e verilen değer o kolun desenine uyduğunda çalıştırılacak koddan meydana gelir. Rust, `match` ifadesine verilen değeri alır ve sırayla her bir kolun deseniyle karşılaştırır. Desenler ve `match` yapısı Rust'ın çok güçlü özellikleridir: Kodunuzun karşılaşabileceği çeşitli durumları açıkça ifade etmenizi ve tüm olasılıkları eksiksiz ele aldığınızdan emin olmanızı sağlarlar. Bu özellikler sırasıyla Bölüm 6 ve Bölüm 19'da ayrıntılı olarak incelenecektir.

Burada kullandığımız `match` ifadesinin nasıl çalıştığını bir örnek üzerinden adım adım izleyelim. Diyelim ki kullanıcı `50` tahmininde bulundu ve bu turda rastgele üretilen gizli sayı `38` oldu.

Kod `50` ile `38`'i karşılaştırdığında, `50` sayısı `38`'den büyük olduğu için `cmp` metodu `Ordering::Greater` döndürür. `match` ifadesi `Ordering::Greater` değerini alır ve her kolun desenini sırayla kontrol etmeye başlar. İlk kolun deseni olan `Ordering::Less`'e bakar ve `Ordering::Greater` değerinin `Ordering::Less` ile eşleşmediğini görür; dolayısıyla bu koldaki kodu atlayıp bir sonraki kola geçer. İkinci kolun deseni `Ordering::Greater`'dır ve elimizdeki `Ordering::Greater` değeriyle _eşleşir_! Bu koldaki kod çalıştırılır ve ekrana `Çok büyük!` yazdırılır. İlk başarılı eşleşmenin ardından `match` ifadesi sona erdiğinden bu senaryoda son kola hiç bakılmaz.

Ancak Liste 2-4'teki kod henüz derlenmeyecektir. Gelin deneyelim:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Hata mesajının özü *uyumsuz türler (`mismatched types`)* bulunduğunu bildiriyor. Rust güçlü ve statik bir tür sistemine sahiptir. Bununla birlikte *tür çıkarımı (type inference)* yeteneği de vardır. `let mut tahmin = String::new()` yazdığımızda Rust, `tahmin` değişkeninin bir `String` olması gerektiğini kendiliğinden çıkardı ve türü açıkça yazmamızı istemedi. Öte yandan `gizli_sayi` bir sayı türüdür. Rust'ta 1 ile 100 arasında değer tutabilen birden fazla sayı türü bulunur: 32 bitlik işaretli tam sayı `i32`, 32 bitlik işaretsiz tam sayı `u32`, 64 bitlik tam sayı `i64` ve diğerleri. Aksi belirtilmedikçe Rust tam sayılar için varsayılan olarak `i32` kullanır; başka bir yerde derleyicinin farklı bir sayısal tür çıkarmasına yol açacak bir tür bilgisi vermediğiniz sürece `gizli_sayi` değişkeninin türü `i32` olur. Hatanın nedeni, Rust'ın bir metin (`String`) ile bir sayıyı doğrudan karşılaştıramamasıdır.

> Not: Derleyici çıktısındaki `expected &String, found &{integer}` (*`&String` referansı beklenirken `&{integer}` tam sayı referansı bulundu*) ifadesine dikkat edin. `cmp` metodunu `String` türündeki `tahmin` değişkeni üzerinde çağırdığımız için (`tahmin.cmp(&gizli_sayi)`), derleyici argümanın da aynı türden (`&String`) olmasını bekler ve `&gizli_sayi` tam sayı referansı geldiğinde tür uyuşmazlığı hatası verir.

Sonuç olarak programın girdi olarak okuduğu `String` değerini bir sayı türüne dönüştürmek istiyoruz ki gizli sayıyla sayısal olarak karşılaştırabilelim. Bunu `main` fonksiyonunun gövdesine şu satırı ekleyerek yapıyoruz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

Eklediğimiz satır şudur:

```rust,ignore
let tahmin: u32 = tahmin.trim().parse().expect("Lütfen bir sayı yazın!");
```

Burada yine `tahmin` adında bir değişken oluşturuyoruz. İyi ama programda zaten `tahmin` adında bir değişken yok muydu? Evet, var; ancak Rust önceki `tahmin` değerini yenisiyle *gölgelememize (shadowing)* olanak tanır. _Gölgeleme_, bizi `tahmin_str` ve `tahmin` gibi iki ayrı değişken oluşturmaya zorlamak yerine aynı `tahmin` adını yeniden kullanmamızı sağlar. Bu özelliği [Bölüm 3][shadowing]<!-- ignore -->'te daha ayrıntılı ele alacağız; şimdilik gölgelemenin en sık bir değeri bir türden başka bir türe dönüştürürken kullanıldığını bilmeniz yeterlidir.

Bu yeni değişkeni `tahmin.trim().parse()` ifadesine bağlıyoruz. Eşittirin sağındaki ifadede geçen `tahmin`, kullanıcı girdisini metin olarak tutan ilk `tahmin` değişkenini temsil eder. Bir `String` örneği üzerindeki `trim` metodu, metnin başındaki ve sonundaki tüm boşluk karakterlerini temizler. Yalnızca sayısal veri tutabilen `u32` türüne dönüştürme yapmadan önce bu adımı mutlaka uygulamamız gerekir; çünkü kullanıcının `read_line` çağrısını tamamlayıp tahminini göndermek için <kbd>enter</kbd> tuşuna basması gerekir ve bu da string'in sonuna bir satır sonu karakteri ekler. Örneğin kullanıcı <kbd>5</kbd> yazıp <kbd>enter</kbd>'a bastığında `tahmin` gerçekte şöyle görünür: `5\n`. Buradaki `\n` "yeni satır" (*newline*) karakterini temsil eder. (Windows'ta <kbd>enter</kbd> tuşuna basmak satır başı ve yeni satır anlamına gelen `\r\n` dizisini ekler.) `trim` metodu `\n` veya `\r\n` karakterlerini (ve varsa fazladan boşlukları) temizleyerek geriye yalnızca `5` kalmasını sağlar.

String'ler üzerindeki [`parse` metodu][parse]<!-- ignore -->, bir metni başka bir türe dönüştürür. Burada onu bir metni sayıya çevirmek için kullanıyoruz. `parse` pek çok farklı sayı türü üretebildiğinden, `let tahmin: u32` yazarak Rust'a tam olarak hangi sayı türünü istediğimizi bildirmemiz gerekir. `tahmin` adından sonraki iki nokta üst üste (`:`), değişkenin türünü açıkça belirteceğimizi (*type annotation*) söyler. Rust'ta çeşitli yerleşik tam sayı türleri vardır; burada gördüğünüz `u32`, işaretsiz ve 32 bitlik bir tam sayıdır. Küçük pozitif sayılar için iyi bir varsayılan tercihtir. Diğer sayı türlerini [Bölüm 3][integers]<!-- ignore -->'te öğreneceksiniz.

Üstelik bu programda `tahmin` için yaptığımız `u32` tür bildirimi ve aşağıda `gizli_sayi` ile yapılan karşılaştırma sayesinde Rust, `gizli_sayi` değişkeninin de bir `u32` olması gerektiği çıkarımını otomatik olarak yapar. Böylece karşılaştırma artık aynı türden iki değer arasında gerçekleşir!

`parse` metodu yalnızca mantıksal olarak sayıya dönüştürülebilecek karakterler üzerinde çalışabilir ve bu yüzden kolayca hatayla karşılaşabilir. Örneğin string içinde `A👍%` yazıyor olsaydı bunu bir sayıya çevirmenin hiçbir yolu olmazdı. Başarısız olma ihtimali bulunduğundan `parse` metodu da tıpkı `read_line` metodu gibi (daha önce ["Potansiyel Bir Hatayı `Result` ile Ele Almak"](#potansiyel-bir-hatayı-result-ile-ele-almak)<!-- ignore --> kısmında incelediğimiz üzere) bir `Result` türü döndürür. Bu `Result` değerini de yine `expect` metodunu kullanarak aynı şekilde ele alacağız. Eğer `parse` metodu verilen metinden bir sayı üretemediği için `Err` varyantı döndürürse `expect` çağrısı oyunu durdurup verdiğimiz hata mesajını ekrana yazdıracaktır. Eğer `parse` metni başarıyla sayıya dönüştürürse `Result`'ın `Ok` varyantını döndürür ve `expect` de `Ok` içindeki sayıyı çıkarıp bize verir.

Şimdi programı çalıştıralım:

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

Güzel! Tahminden önce boşluk bırakılmış olmasına rağmen program kullanıcının 76 tahmininde bulunduğunu sorunsuzca anladı. Farklı girdilerle programın davranışını doğrulamak için birkaç kez çalıştırın: Sayıyı doğru tahmin edin, çok büyük bir sayı girin ve çok küçük bir sayı girin.

Oyunun büyük kısmı artık çalışıyor, ancak kullanıcı yalnızca tek bir tahmin yapabiliyor. Bir döngü ekleyerek bunu değiştirelim!

<a id="allowing-multiple-guesses-with-looping"></a>

## Döngüyle Birden Fazla Tahmine İzin Vermek

`loop` anahtar kelimesi sonsuz bir döngü oluşturur. Kullanıcılara sayıyı bulmaları için birden fazla şans tanımak üzere bir döngü ekleyelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Gördüğünüz gibi, tahmin isteme satırından itibaren her şeyi bir `loop` bloğunun içine taşıdık. Döngü içindeki satırların her birini dörder boşluk daha girintilediğinizden emin olun ve programı yeniden çalıştırın. Program artık sonsuza dek yeni bir tahmin isteyecektir; tabii bu da beraberinde yeni bir sorun getirir: Kullanıcı oyundan çıkamıyor gibi görünüyor!

Kullanıcı <kbd>ctrl</kbd>-<kbd>C</kbd> klavye kısayolunu kullanarak programı her zaman yarıda kesebilir. Ancak ["Tahmini Gizli Sayıyla Karşılaştırmak"](#tahmini-gizli-sayıyla-karşılaştırmak)<!-- ignore --> kısmındaki `parse` açıklamasında değindiğimiz üzere, bu doymak bilmez canavardan kaçmanın bir yolu daha vardır: Kullanıcı sayı olmayan bir girdi yazarsa program çöker. Aşağıda gösterildiği gibi oyundan çıkmak için şimdilik bu davranıştan yararlanabiliriz:

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

thread 'main' (6694925) panicked at src/main.rs:22:49:
Lütfen bir sayı yazın!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`cikis` yazmak gerçekten de oyundan çıkmanızı sağlar; ancak fark edeceğiniz üzere sayı olmayan herhangi bir başka girdi de aynı çökme sonucunu doğuracaktır. Bu en hafif tabirle pek ideal bir çözüm değildir; üstelik doğru sayı tahmin edildiğinde de oyunun kendiliğinden durmasını istiyoruz.

<a id="quitting-after-a-correct-guess"></a>

### Doğru Tahminden Sonra Çıkmak

Kullanıcı kazandığında oyunun sonlanması için bir `break` deyimi ekleyelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

`Kazandınız!` satırından hemen sonra `break` eklemek, kullanıcı gizli sayıyı doğru tahmin ettiğinde programın döngüden çıkmasını sağlar. Döngü `main` fonksiyonunun son kısmı olduğundan döngüden çıkmak aynı zamanda programın sonlanması anlamına gelir.

<a id="handling-invalid-input"></a>

### Geçersiz Girdiyi Ele Almak

Oyunun davranışını biraz daha iyileştirmek için, kullanıcı sayı olmayan bir girdi yazdığında programı çökertmek yerine bu geçersiz girdiyi yok sayıp kullanıcının tahmin etmeye devam edebilmesini sağlayalım. Bunu Liste 2-5'te gösterildiği gibi `tahmin` değişkeninin `String`'den `u32`'ye dönüştürüldüğü satırı değiştirerek yapabiliriz.

<Listing number="2-5" file-name="src/main.rs" caption="Sayısal olmayan bir tahmini yok sayıp programı çökertmek yerine yeni bir tahmin isteme">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

</Listing>

Hata anında programı çökertmekten hatayı gerçekten ele almaya geçmek için `expect` çağrısını bir `match` ifadesiyle değiştiriyoruz. `parse` metodunun bir `Result` türü döndürdüğünü ve `Result`'ın `Ok` ile `Err` varyantlarına sahip bir enum olduğunu hatırlayın. Burada, tıpkı `cmp` metodunun döndürdüğü `Ordering` sonucunda yaptığımız gibi bir `match` ifadesi kullanıyoruz.

Eğer `parse` metni başarıyla bir sayıya dönüştürebilirse elde edilen sayıyı içeren bir `Ok` değeri döndürür. Bu `Ok` değeri ilk kolun `Ok(sayi)` deseniyle eşleşir ve `match` ifadesi `parse`'ın üretip `Ok` içine koyduğu `sayi` değerini dışarı döndürür. Bu sayı da oluşturduğumuz yeni `tahmin` değişkenine tam istediğimiz şekilde atanır.

Eğer `parse` metni bir sayıya _dönüştüremezse_ hata hakkında ek bilgi taşıyan bir `Err` değeri döndürür. `Err` değeri ilk `match` kolundaki `Ok(sayi)` deseniyle eşleşmez, ancak ikinci koldaki `Err(_)` deseniyle eşleşir. Alt çizgi (`_`) her değeri yakalayan (*catch-all*) bir joker desendir; bu örnekte içinde hangi hata bilgisi bulunursa bulunsun tüm `Err` değerleriyle eşleşmek istediğimizi belirtiyoruz. Böylece program ikinci kolun kodu olan `continue` ifadesini çalıştırır; bu da programa `loop` döngüsünün bir sonraki adımına geçip kullanıcıdan yeniden tahmin istemesini söyler. Kısacası program, `parse` sırasında oluşabilecek tüm hataları zarifçe yok saymış olur!

Artık programdaki her şey beklendiği gibi çalışmalıdır. Hemen deneyelim:

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

Harika! Son bir küçük dokunuşla tahmin oyununu tamamlayacağız. Programın hâlâ gizli sayıyı ekrana yazdırdığını hatırlayın. Test aşamasında bu çok işimize yaradı ama gerçek oyunu bozuyor. Gizli sayıyı yazdıran `println!` satırını silelim. Liste 2-6 oyunun tamamlanmış son kodunu göstermektedir.

<Listing number="2-6" file-name="src/main.rs" caption="Tamamlanmış tahmin oyunu kodu">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

</Listing>

Bu noktada tahmin oyununu başarıyla geliştirdiniz. Tebrikler!

<a id="summary"></a>

## Özet

Bu proje sizi pek çok yeni Rust kavramıyla uygulamalı olarak tanıştırmanın keyifli bir yoluydu: `let`, `match`, fonksiyonlar, harici crate'lerin kullanımı ve daha fazlası. İzleyen bölümlerde bu kavramları çok daha ayrıntılı olarak öğreneceksiniz. Bölüm 3; değişkenler, veri türleri ve fonksiyonlar gibi çoğu programlama dilinde bulunan kavramları ele alır ve bunların Rust'ta nasıl kullanıldığını gösterir. Bölüm 4, Rust'ı diğer dillerden ayıran temel özellik olan *sahipliği (ownership)* inceler. Bölüm 5 struct'ları ve metot sözdizimini ele alırken Bölüm 6 enum'ların nasıl çalıştığını açıklar.

[prelude]: https://doc.rust-lang.org/std/prelude/index.html
[variables-and-mutability]: ch03-01-degiskenler-ve-degistirilebilirlik.html#değişkenler-ve-değiştirilebilirlik-mutability
[comments]: ch03-04-yorumlar.html
[string]: https://doc.rust-lang.org/std/string/struct.String.html
[iostdin]: https://doc.rust-lang.org/std/io/struct.Stdin.html
[read_line]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[enums]: ch06-00-enumlar-ve-desen-eslestirme.html
[expect]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
[recover]: ch09-02-result-ile-kurtarilabilir-hatalar.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match-kontrol-akisi.html
[shadowing]: ch03-01-degiskenler-ve-degistirilebilirlik.html#gölgelendirme-shadowing
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[integers]: ch03-02-veri-turleri.html#tam-sayı-türleri-integer-types
