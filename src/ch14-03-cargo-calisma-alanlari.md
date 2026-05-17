## Cargo Çalışma Alanları

12. bölümde hem ikili crate hem de kütüphane crate içeren bir paket
oluşturmuştuk. Projeniz büyüdükçe kütüphane crate'in de büyümeye devam ettiğini
ve paketi birden fazla kütüphane crate'e bölmek istediğinizi fark edebilirsiniz.
Cargo'nun _çalışma alanı (workspace)_ adı verilen özelliği, birlikte
geliştirilen ilişkili birden çok paketi yönetmeyi kolaylaştırır.

### Bir Çalışma Alanı Oluşturmak

_Çalışma alanı_, aynı _Cargo.lock_ dosyasını ve aynı çıktı dizinini paylaşan
paketler kümesidir. Yapıya odaklanabilmek için basit kod kullanan bir çalışma
alanı örneği oluşturalım. Çalışma alanını düzenlemenin farklı yolları vardır;
biz yalnızca yaygın bir yaklaşımı göstereceğiz. Bir ikili crate ve iki kütüphane
crate içeren bir çalışma alanımız olacak. Ana işlevi sunacak ikili crate,
iki kütüphaneye bağımlı olacak. Kütüphanelerden biri `bir_ekle` fonksiyonunu,
diğeri ise `iki_ekle` fonksiyonunu sağlayacak. Bu üç crate aynı çalışma alanının
parçası olacak. Önce çalışma alanı için yeni bir dizin oluşturalım:

```console
$ mkdir add
$ cd add
```

Ardından _add_ dizininde, tüm çalışma alanını yapılandıracak _Cargo.toml_
dosyasını oluşturalım. Bu dosyada `[package]` bölümü olmayacak. Onun yerine
üyeleri eklememizi sağlayacak bir `[workspace]` bölümüyle başlayacak. Ayrıca
çözümleyicinin en güncel sürümünü kullanmak için `resolver` değerini `"3"`
yapacağız:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/topla/Cargo.toml}}
```

Şimdi _topla_ dizini içinde `cargo new` çalıştırarak `toplayici` ikili crate'ini
oluşturalım:

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

Bir çalışma alanı içinde `cargo new` çalıştırıldığında, Cargo yeni oluşturulan
paketi çalışma alanı _Cargo.toml_ dosyasındaki `[workspace]` tanımının `members`
anahtarına da otomatik olarak ekler:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/topla/Cargo.toml}}
```

Bu noktada `cargo build` çalıştırarak çalışma alanını derleyebiliriz. _topla_
dizininizin dosya yapısı şu hale gelir:

```text
├── Cargo.lock
├── Cargo.toml
├── toplayici
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Çalışma alanının en üst düzeyde tek bir _target_ dizini vardır ve derlenmiş
yapıtlar bu dizine yazılır; `toplayici` paketinin kendine ait bir _target_ dizini
yoktur. Hatta `cargo build`i _toplayici_ dizininden çalıştırsak bile çıktılar yine
_topla/target_ altına gider; _topla/toplayici/target_ altına değil. Cargo çalışma
alanlarında _target_ dizinini böyle düzenler çünkü bu crate'lerin birbirine
bağımlı olması beklenir. Her crate'in kendi _target_ dizini olsaydı, her crate
diğer crate'leri de kendi dizinine çıktı koymak için yeniden derlemek zorunda
kalırdı. Ortak bir _target_ dizini paylaşmak gereksiz yeniden derlemeleri önler.

### Çalışma Alanına İkinci Paketi Eklemek

Şimdi çalışma alanına bir üye paket daha ekleyelim ve buna `bir_ekle` diyelim.
`bir_ekle` adlı yeni bir kütüphane crate'i üretin:

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

Üst düzey _Cargo.toml_ artık `members` listesinde _bir_ekle_ yolunu da içerir:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/topla/Cargo.toml}}
```

_topla_ dizininizin dosya yapısı artık şöyle olur:

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

Şimdi _bir_ekle/src/lib.rs_ dosyasına `bir_ekle` fonksiyonunu ekleyelim:

<span class="filename">Dosya Adı: bir_ekle/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/topla/bir_ekle/src/lib.rs}}
```

Artık ikili crate'imizi barındıran `toplayici` paketinin, bu kütüphaneyi içeren
pakete bağımlı olmasını sağlayabiliriz. Önce _toplayici/Cargo.toml_ dosyasına bu
crate için yol bağımlılığı eklememiz gerekir.

<span class="filename">Dosya Adı: toplayici/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/topla/toplayici/Cargo.toml:6:7}}
```

Cargo, çalışma alanındaki crate'lerin birbirine bağımlı olacağını kendiliğinden
varsaymaz; bu ilişkileri açıkça yazmamız gerekir.

Şimdi `toplayici` crate'inde `bir_ekle` fonksiyonunu kullanalım. _toplayici/src/main.rs_
dosyasını açıp `main` fonksiyonunu, Liste 14-7'deki gibi `bir_ekle`
fonksiyonunu çağıracak şekilde değiştirin.

<Listing number="14-7" file-name="toplayici/src/main.rs" caption="`toplayici` crate'inden `bir_ekle` kütüphane fonksiyonunu kullanmak">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/topla/toplayici/src/main.rs}}
```

</Listing>

Şimdi çalışma alanının en üst düzeyindeki _topla_ dizininde `cargo build`
çalıştırarak her şeyi derleyelim!

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

İkili crate'i _topla_ dizininden çalıştırmak için, çalışma alanındaki hangi paketi
çalıştırmak istediğimizi `-p` bayrağı ve paket adıyla belirtiriz:

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

Bu komut, _toplayici/src/main.rs_ içindeki ve `bir_ekle` crate'ine bağımlı olan kodu
çalıştırır.

<!-- Old headings. Do not remove or links may break. -->

<a id="depending-on-an-external-package-in-a-workspace"></a>

### Harici Bir Pakete Bağımlı Olmak

Çalışma alanında her crate dizininde ayrı bir _Cargo.lock_ yerine, üst düzeyde
tek bir _Cargo.lock_ dosyası bulunduğuna dikkat edin. Bu, tüm crate'lerin tüm
bağımlılıkların aynı sürümünü kullanmasını sağlar. _toplayici/Cargo.toml_ ve
_bir_ekle/Cargo.toml_ dosyalarına `rand` paketini eklersek, Cargo bunları tek
bir `rand` sürümüne çözer ve bunu tek _Cargo.lock_ dosyasına yazar. Böylece
çalışma alanındaki tüm crate'ler birbirleriyle uyumlu kalır. Şimdi `rand`
crate'ini _bir_ekle/Cargo.toml_ içindeki `[dependencies]` bölümüne ekleyelim ki
onu `bir_ekle` crate'inde kullanabilelim:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Dosya Adı: bir_ekle/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/topla/bir_ekle/Cargo.toml:6:7}}
```

Şimdi _bir_ekle/src/lib.rs_ dosyasına `use rand;` ekleyebiliriz. Ardından _topla_
dizininde `cargo build` çalıştırınca, tüm çalışma alanıyla birlikte `rand`
crate'i de indirilip derlenir. `rand`ı kapsama aldığımız halde kullanmadığımız
için bir uyarı görürüz:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/topla
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling bir_ekle v0.1.0 (file:///projects/topla/bir_ekle)
warning: unused import: `rand`
 --> bir_ekle/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `bir_ekle` (lib) generated 1 warning (run `cargo fix --lib -p bir_ekle` to apply 1 suggestion)
   Compiling toplayici v0.1.0 (file:///projects/topla/toplayici)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```

Artık üst düzey _Cargo.lock_ dosyası, `bir_ekle` crate'inin `rand` bağımlılığına
ilişkin bilgiyi içerir. Ama `rand` çalışma alanında bir yerde kullanılıyor diye,
onu diğer crate'lerde de otomatik kullanamayız; bunun için onların _Cargo.toml_
dosyalarına da `rand` eklememiz gerekir. Örneğin `toplayici` paketine ait
_toplayici/src/main.rs_ dosyasına `use rand;` yazarsak hata alırız:

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

Bunu düzeltmek için `toplayici` paketinin _Cargo.toml_ dosyasını düzenleyip `rand`ın
ona da bağımlılık olduğunu belirtmemiz gerekir. `toplayici` paketini derlemek,
_Cargo.lock_ içinde `toplayici` için bağımlılık listesine `rand`ı ekler; ama `rand`ın
ek bir kopyası indirilmez. Cargo, çalışma alanındaki her pakette `rand`
kullanan crate'lerin, uyumlu sürüm belirttikleri sürece aynı `rand` sürümünü
kullanmasını sağlar. Bu da hem yer kazandırır hem de crate'lerin birbiriyle
uyumlu kalmasını sağlar.

Çalışma alanındaki crate'ler aynı bağımlılığın birbiriyle uyumsuz sürümlerini
isterse, Cargo hepsini ayrı ayrı çözer; ama yine de olabildiğince az sayıda
sürüm kullanmaya çalışır.

### Çalışma Alanına Test Eklemek

Bir geliştirme daha yapalım ve `bir_ekle` crate'i içinde `bir_ekle::bir_ekle`
fonksiyonu için bir test ekleyelim:

<span class="filename">Dosya Adı: bir_ekle/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/topla/bir_ekle/src/lib.rs}}
```

Şimdi üst düzey _topla_ dizininde `cargo test` çalıştırın. Böyle yapılandırılmış
bir çalışma alanında `cargo test`, çalışma alanındaki tüm crate'lerin testlerini
çalıştırır:

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

Çıktının ilk bölümü, `bir_ekle` crate'i içindeki `calisir` testinin geçtiğini
gösterir. Sonraki bölümde `toplayici` crate'inde hiç test bulunmadığını, son bölümde
de `bir_ekle` crate'inde hiç belge testi olmadığını görürüz.

Üst düzey dizinden, çalışma alanındaki tek bir crate için de test
çalıştırabiliriz. Bunun için `-p` bayrağını kullanıp test etmek istediğimiz
crate adını belirtiriz:

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

Bu çıktı, `cargo test`in yalnızca `bir_ekle` crate'inin testlerini
çalıştırdığını ve `toplayici` crate'inin testlerini çalıştırmadığını gösterir.

Çalışma alanındaki crate'leri [crates.io](https://crates.io/)<!-- ignore -->
üzerinde yayımlarsanız, her crate'i ayrı ayrı yayımlamanız gerekir. `cargo test`
örneğinde olduğu gibi, `-p` bayrağını kullanarak çalışma alanındaki belirli bir
crate'i yayımlayabilirsiniz.

Ek pratik olarak, bu çalışma alanına `iki_ekle` adında bir crate'i de `bir_ekle`
crate'iyle benzer biçimde ekleyin!

Projeniz büyüdükçe çalışma alanı kullanmayı düşünün: Tek parça büyük bir kod
yığını yerine daha küçük ve daha kolay anlaşılır bileşenlerle çalışmanıza olanak
tanır. Üstelik çalışma alanındaki crate'ler aynı anda sık değişiyorsa, aralarındaki
eşgüdümü de kolaylaştırır.
