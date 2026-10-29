<!-- Old headings. Do not remove or links may break. -->

<a id="cargo-workspaces"></a>
<a id="cargo-çalışma-alanları"></a>

## Cargo Çalışma Alanları (Workspaces)

Bölüm 12'de hem bir ikili crate hem de bir kütüphane crate'i içeren bir paket
oluşturmuştuk. Projeniz geliştikçe kütüphane crate'inin giderek büyüdüğünü ve
paketinizi birden fazla kütüphane crate'ine bölmek istediğinizi fark
edebilirsiniz. Cargo, eşzamanlı olarak birlikte geliştirilen birbiriyle ilişkili
birden çok paketi yönetmeye yardımcı olan _çalışma alanları (workspaces)_ adlı
bir özellik sunar.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-a-workspace"></a>
<a id="bir-çalışma-alanı-oluşturmak"></a>

### Çalışma Alanı Oluşturma

_Çalışma alanı_, aynı _Cargo.lock_ dosyasını ve aynı çıktı dizinini paylaşan
paketler kümesidir. Çalışma alanının yapısına odaklanabilmek için çok basit kod
kullanan bir proje oluşturalım. Bir çalışma alanını yapılandırmanın birden fazla
yolu vardır; biz burada yaygın kullanılan bir yöntemi göstereceğiz. Bir ikili
crate ve iki kütüphane crate'i içeren bir çalışma alanımız olacak. Ana
işlevselliği sunacak olan ikili crate, bu iki kütüphaneye bağımlı olacak.
Kütüphanelerden biri `bir_ekle` fonksiyonunu, diğeri ise `iki_ekle`
fonksiyonunu sağlayacak. Bu üç crate aynı çalışma alanının parçası olacak. İşe
çalışma alanı için yeni bir dizin oluşturarak başlayalım:

```console
$ mkdir topla
$ cd topla
```

Ardından _topla_ dizininde, tüm çalışma alanını yapılandıracak _Cargo.toml_
dosyasını oluşturuyoruz. Bu dosyada `[package]` bölümü yer almayacak; bunun
yerine çalışma alanına üye paketler eklememizi sağlayan bir `[workspace]`
bölümüyle başlayacak. Ayrıca çalışma alanımızda Cargo'nun bağımlılık çözümleyici
algoritmasının en güncel ve gelişmiş sürümünü kullanmak için `resolver` değerini
özellikle `"3"` olarak ayarlıyoruz:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/topla/Cargo.toml}}
```

> Not: Normalde tekli bir pakette `edition = "2024"` ayarı Cargo'nun 3. sürüm
> bağımlılık çözümleyicisini otomatik olarak etkinleştirir. Ancak başında
> `[package]` bölümü bulunmayan bu tür *sanal çalışma alanı (virtual workspace)*
> _Cargo.toml_ dosyalarında `edition` alanı yer almadığından, `resolver = "3"`
> satırını açıkça belirtmezseniz Cargo geriye dönük uyumluluk amacıyla eski
> çözümleyiciye (`"1"`) döner ve derleme sırasında uyarı üretir.

Şimdi _topla_ dizini içinde `cargo new` çalıştırarak `toplayici` ikili
crate'ini oluşturalım:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/topla
remove `members = ["toplayici"]` from Cargo.toml
rm -rf toplayici
cargo new toplayici
copy output below
-->

```console
$ cargo new toplayici
     Created binary (application) `toplayici` package
      Adding `toplayici` as member of workspace at `file:///projects/topla`
```

Bir çalışma alanı içinde `cargo new` çalıştırmak, yeni oluşturulan paketi
çalışma alanının _Cargo.toml_ dosyasındaki `[workspace]` tanımında yer alan
`members` anahtarına da otomatik olarak ekler:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/topla/Cargo.toml}}
```

Bu noktada `cargo build` çalıştırarak çalışma alanını derleyebiliriz. _topla_
dizininizdeki dosyalar şu şekilde görünmelidir:

```text
├── Cargo.lock
├── Cargo.toml
├── toplayici
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Çalışma alanının en üst düzeyde derlenmiş çıktıların yerleştirileceği tek bir
_target_ dizini vardır; `toplayici` paketinin kendine ait ayrı bir _target_
dizini yoktur. Hatta `cargo build` komutunu _toplayici_ dizininin içinden
çalıştırsak bile derlenmiş çıktılar _topla/toplayici/target_ yerine yine
_topla/target_ dizinine yerleştirilir. Cargo bir çalışma alanındaki _target_
dizinini bu şekilde yapılandırır çünkü çalışma alanındaki crate'lerin birbirine
bağımlı olması amaçlanır. Her crate'in kendi _target_ dizini olsaydı, her crate
derlenmiş çıktıları kendi _target_ dizinine yerleştirmek için çalışma alanındaki
diğer crate'lerin her birini yeniden derlemek zorunda kalırdı. Tek bir _target_
dizinini paylaşarak crate'ler gereksiz yeniden derlemelerin önüne geçer.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-the-second-package-in-the-workspace"></a>
<a id="çalışma-alanına-ikinci-paketi-eklemek"></a>

### Çalışma Alanında İkinci Paketi Oluşturma

Şimdi çalışma alanında bir üye paket daha oluşturalım ve adını `bir_ekle`
koyalım. `bir_ekle` adında yeni bir kütüphane crate'i üretin:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/topla
remove `"bir_ekle"` from `members` list in Cargo.toml
rm -rf bir_ekle
cargo new bir_ekle --lib
copy output below
-->

```console
$ cargo new bir_ekle --lib
     Created library `bir_ekle` package
      Adding `bir_ekle` as member of workspace at `file:///projects/topla`
```

Üst düzey _Cargo.toml_ dosyası artık `members` listesinde _bir_ekle_ yolunu da
içerecektir:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/topla/Cargo.toml}}
```

_topla_ dizininizde artık şu dizin ve dosyalar bulunmalıdır:

```text
├── Cargo.lock
├── Cargo.toml
├── bir_ekle
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── toplayici
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

_bir_ekle/src/lib.rs_ dosyasına bir `bir_ekle` fonksiyonu ekleyelim:

<span class="filename">Dosya Adı: bir_ekle/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/topla/bir_ekle/src/lib.rs}}
```

Artık ikili crate'imizi barındıran `toplayici` paketinin, kütüphanemizi içeren
`bir_ekle` paketine bağımlı olmasını sağlayabiliriz. Öncelikle
_toplayici/Cargo.toml_ dosyasına `bir_ekle` için bir yol bağımlılığı (*path
dependency*) eklememiz gerekecek.

<span class="filename">Dosya Adı: toplayici/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/topla/toplayici/Cargo.toml:6:7}}
```

Cargo bir çalışma alanındaki crate'lerin birbirine bağımlı olacağını
kendiliğinden varsaymaz; bu nedenle bağımlılık ilişkilerini açıkça belirtmemiz
gerekir.

Şimdi (`bir_ekle` crate'inden gelen) `bir_ekle` fonksiyonunu `toplayici`
crate'inde kullanalım. _toplayici/src/main.rs_ dosyasını açın ve `main`
fonksiyonunu Liste 14-7'deki gibi `bir_ekle` fonksiyonunu çağıracak şekilde
değiştirin.

<Listing number="14-7" file-name="toplayici/src/main.rs" caption="`bir_ekle` kütüphane crate'ini `toplayici` crate'i içinden kullanmak">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/topla/toplayici/src/main.rs}}
```

</Listing>

Üst düzey _topla_ dizininde `cargo build` çalıştırarak çalışma alanını
derleyelim!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/topla
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling bir_ekle v0.1.0 (file:///projects/topla/bir_ekle)
   Compiling toplayici v0.1.0 (file:///projects/topla/toplayici)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

İkili crate'i _topla_ dizininden çalıştırmak için `cargo run` komutuyla birlikte
`-p` argümanını ve paket adını kullanarak çalışma alanındaki hangi paketi
çalıştırmak istediğimizi belirtebiliriz:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/topla
cargo run -p toplayici
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p toplayici
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/toplayici`
Merhaba, dünya! 10 artı bir 11 eder!
```

Bu komut, _toplayici/src/main.rs_ içindeki ve `bir_ekle` crate'ine bağımlı olan
kodu çalıştırır.

<!-- Old headings. Do not remove or links may break. -->

<a id="depending-on-an-external-package-in-a-workspace"></a>
<a id="depending-on-an-external-package"></a>
<a id="harici-bir-pakete-bağımlı-olmak"></a>

### Harici Bir Pakete Bağımlı Olma

Çalışma alanında her crate'in kendi dizininde ayrı bir _Cargo.lock_ bulunması
yerine, en üst düzeyde yalnızca tek bir _Cargo.lock_ dosyası bulunduğuna dikkat
edin. Bu yapı, tüm crate'lerin bütün bağımlılıkların aynı sürümünü kullanmasını
güvence altına alır. `rand` paketini hem _toplayici/Cargo.toml_ hem de
_bir_ekle/Cargo.toml_ dosyalarına eklersek, Cargo her ikisini de tek bir `rand`
sürümüne çözümler ve bunu tek _Cargo.lock_ dosyasına kaydeder. Çalışma
alanındaki tüm crate'lerin aynı bağımlılıkları kullanmasını sağlamak, bu
crate'lerin birbirleriyle her zaman uyumlu olacağı anlamına gelir. Şimdi `rand`
crate'ini `bir_ekle` crate'inde kullanabilmek için _bir_ekle/Cargo.toml_
dosyasındaki `[dependencies]` bölümüne ekleyelim:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:

* ch01-01-installation.md
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Dosya Adı: bir_ekle/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/topla/bir_ekle/Cargo.toml:6:7}}
```

Artık _bir_ekle/src/lib.rs_ dosyasına `use rand;` ekleyebiliriz; _topla_
dizininde `cargo build` çalıştırarak tüm çalışma alanını derlemek `rand`
crate'ini indirip derleyecektir. Kapsama dahil ettiğimiz `rand` crate'ine kodda
başvurmadığımız için bir uyarı alırız:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/topla
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.10.1
   --snip--
   Compiling rand v0.10.1
   Compiling bir_ekle v0.1.0 (file:///projects/topla/bir_ekle)
warning: unused import: `rand`
 --> bir_ekle/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `bir_ekle` (lib) generated 1 warning (run `cargo fix --lib -p bir_ekle` to apply 1 suggestion)
   Compiling toplayici v0.1.0 (file:///projects/topla/toplayici)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```

Üst düzey _Cargo.lock_ dosyası artık `bir_ekle` crate'inin `rand` bağımlılığına
ilişkin bilgileri içerir. Bununla birlikte, `rand` çalışma alanının bir yerinde
kullanılıyor olsa bile, diğer crate'lerin _Cargo.toml_ dosyalarına da `rand`
eklemediğimiz sürece onu çalışma alanındaki diğer crate'lerde kullanamayız.
Örneğin `toplayici` paketine ait _toplayici/src/main.rs_ dosyasına `use rand;`
eklersek bir hata alırız:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/topla
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
  --snip--
   Compiling toplayici v0.1.0 (file:///projects/topla/toplayici)
error[E0432]: unresolved import `rand`
 --> toplayici/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

Bunu düzeltmek için `toplayici` paketinin _Cargo.toml_ dosyasını düzenleyip
`rand` paketinin onun için de bir bağımlılık olduğunu belirtin. `toplayici`
paketini derlemek, _Cargo.lock_ dosyasında `toplayici` paketinin bağımlılık
listesine `rand` paketini ekleyecektir; ancak `rand` paketinin ek bir kopyası
indirilmeyecektir. Cargo, uyumlu `rand` sürümleri belirttikleri sürece çalışma
alanındaki her pakette yer alan ve `rand` kullanan tüm crate'lerin aynı sürümü
kullanmasını sağlayarak bize hem disk alanı kazandırır hem de çalışma
alanındaki crate'lerin birbirleriyle uyumlu kalmasını güvence altına alır.

Çalışma alanındaki crate'ler aynı bağımlılığın birbiriyle uyumsuz sürümlerini
belirtirse Cargo bunların her birini ayrı ayrı çözümler, ancak yine de mümkün
olan en az sayıda sürümü çözümlemeye çalışır.

> Not: Çok sayıda paketten oluşan projelerde aynı bağımlılık sürümünü veya ortak
> paket üst verilerini (`version`, `edition`, `license` vb.) her üye paketin
> _Cargo.toml_ dosyasında tekrar tekrar yazmak ve güncel tutmak sürüm
> uyuşmazlıklarına yol açabilir. Cargo, bunu tek merkezden yönetmek için
> **çalışma alanı mirası (workspace inheritance)** özelliğini sunar. Üst düzey
> _Cargo.toml_ dosyasında ortak paket alanlarını `[workspace.package]` altında,
> ortak bağımlılıkları ise `[workspace.dependencies]` altında bir kez
> tanımlayabilirsiniz:
>
> ```toml
> [workspace]
> resolver = "3"
> members = ["toplayici", "bir_ekle"]
>
> [workspace.package]
> version = "0.1.0"
> edition = "2024"
>
> [workspace.dependencies]
> rand = "0.10.1"
> bir_ekle = { path = "bir_ekle" }
> ```
>
> Ardından üye paketlerin kendi _Cargo.toml_ dosyalarında (örneğin
> _toplayici/Cargo.toml_) sürüm numarası veya dosya yolu tekrarlamak yerine
> `workspace = true` anahtarını kullanması yeterlidir:
>
> ```toml
> [package]
> name = "toplayici"
> version.workspace = true
> edition.workspace = true
>
> [dependencies]
> rand.workspace = true
> bir_ekle.workspace = true
> ```
>
> Böylece bir bağımlılığı güncellemeniz gerektiğinde yalnızca kök _Cargo.toml_
> dosyasındaki tek bir satırı değiştirirsiniz ve çalışma alanındaki tüm
> crate'lerin aynı sürüm tanımını paylaşması garanti altına alınır.

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-a-test-to-a-workspace"></a>
<a id="çalışma-alanına-test-eklemek"></a>

### Çalışma Alanına Test Ekleme

Bir iyileştirme daha yapalım ve `bir_ekle` crate'i içinde `bir_ekle::bir_ekle`
fonksiyonunun bir testini ekleyelim:

<span class="filename">Dosya Adı: bir_ekle/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/topla/bir_ekle/src/lib.rs}}
```

Şimdi üst düzey _topla_ dizininde `cargo test` çalıştırın. Bu şekilde
yapılandırılmış bir çalışma alanında `cargo test` çalıştırmak, çalışma
alanındaki tüm crate'lerin testlerini çalıştıracaktır:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/topla
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling bir_ekle v0.1.0 (file:///projects/topla/bir_ekle)
   Compiling toplayici v0.1.0 (file:///projects/topla/toplayici)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/bir_ekle-93c49ee75dc46543)

running 1 test
test tests::calisir ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/toplayici-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests bir_ekle

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Çıktının ilk bölümü, `bir_ekle` crate'indeki `calisir` testinin geçtiğini
gösterir. Bir sonraki bölüm `toplayici` crate'inde sıfır test bulunduğunu, son
bölüm ise `bir_ekle` crate'inde sıfır belgelendirme testi bulunduğunu gösterir.

Üst düzey dizinden `-p` bayrağını kullanıp test etmek istediğimiz crate'in adını
belirterek çalışma alanındaki belirli bir crate'in testlerini de
çalıştırabiliriz:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/topla
cargo test -p bir_ekle
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p bir_ekle
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/bir_ekle-93c49ee75dc46543)

running 1 test
test tests::calisir ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests bir_ekle

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Bu çıktı, `cargo test` komutunun yalnızca `bir_ekle` crate'i için testleri
çalıştırdığını ve `toplayici` crate'inin testlerini çalıştırmadığını gösterir.

Çalışma alanındaki crate'leri [crates.io](https://crates.io/)<!-- ignore -->
üzerinde yayınlarsanız, çalışma alanındaki her crate'in ayrı ayrı yayınlanması
gerekecektir. Tıpkı `cargo test` komutunda olduğu gibi, `-p` bayrağını kullanıp
yayınlamak istediğimiz crate'in adını belirterek çalışma alanımızdaki belirli
bir crate'i yayınlayabiliriz.

Ek pratik yapmak için bu çalışma alanına `bir_ekle` crate'ine benzer bir
şekilde `iki_ekle` crate'ini ekleyin!

Projeniz büyüdükçe bir çalışma alanı kullanmayı değerlendirin: Tek parça büyük
bir kod yığını yerine daha küçük ve anlaşılması daha kolay bileşenlerle
çalışmanıza olanak tanır. Üstelik crate'ler sıklıkla aynı anda değiştiriliyorsa,
onları bir çalışma alanında tutmak crate'ler arasındaki eşgüdümü de
kolaylaştırabilir.
