<!-- Old headings. Do not remove or links may break. -->
<a id="hello-cargo"></a>

## Merhaba, Cargo!

Cargo, Rust'ın derleme sistemi ve paket yöneticisidir. Rust geliştiricilerinin (*Rustacean*'ların) büyük çoğunluğu projelerini yönetmek için bu aracı kullanır; çünkü Cargo kodunuzu derlemek, kodunuzun ihtiyaç duyduğu kütüphaneleri indirmek ve bu kütüphaneleri derlemek gibi pek çok işi sizin yerinize üstlenir. (Kodunuzun ihtiyaç duyduğu bu kütüphanelere *bağımlılıklar (dependencies)* adını veriyoruz.)

Şimdiye kadar yazdığımız gibi en basit Rust programlarının herhangi bir bağımlılığı yoktur. "Merhaba, dünya!" projesini Cargo ile oluşturmuş olsaydık, Cargo'nun yalnızca kod derleme özelliğinden yararlanırdık. Daha karmaşık Rust programları yazdıkça projelerinize bağımlılıklar ekleyeceksiniz; projeye baştan Cargo ile başlarsanız sonradan bağımlılık eklemek çok daha kolay olacaktır.

Rust projelerinin büyük çoğunluğu Cargo kullandığından, kitabın geri kalanında sizin de Cargo kullandığınız varsayılmaktadır. ["Kurulum"][installation]<!-- ignore --> bölümünde anlatılan resmî yükleyicileri kullandıysanız Cargo, Rust ile birlikte kurulu gelir. Rust'ı başka bir yöntemle kurduysanız terminalinize aşağıdaki komutu girerek Cargo'nun kurulu olup olmadığını kontrol edin:

```console
$ cargo --version
```

Bir sürüm numarası görüyorsanız Cargo kuruludur! `command not found` (komut bulunamadı) gibi bir hata alırsanız Cargo'yu ayrıca nasıl kuracağınızı öğrenmek için kullandığınız kurulum yönteminin belgelerine bakın.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-project-with-cargo"></a>

### Cargo ile Proje Oluşturmak

Şimdi Cargo kullanarak yeni bir proje oluşturalım ve orijinal "Merhaba, dünya!" projemizden ne gibi farkları olduğuna bakalım. _projects_ dizininize (veya kodlarınızı saklamaya karar verdiğiniz dizine) geri dönün. Ardından, hangi işletim sistemini kullanırsanız kullanın şu komutları çalıştırın:

```console
$ cargo new merhaba_cargo
$ cd merhaba_cargo
```

İlk komut, _merhaba_cargo_ adında yeni bir dizin ve proje oluşturur. Projemize _merhaba_cargo_ adını verdiğimiz için Cargo, proje dosyalarını aynı adlı bir dizinin içinde oluşturur.

_merhaba_cargo_ dizinine girip dosyaları listeleyin. Cargo'nun bizim için iki dosya ve bir dizin oluşturduğunu göreceksiniz: bir _Cargo.toml_ dosyası ve içinde _main.rs_ dosyası bulunan bir _src_ dizini.

Cargo ayrıca bir _.gitignore_ dosyasıyla birlikte yeni bir Git deposu da başlatmıştır. `cargo new` komutunu hâlihazırda var olan bir Git deposunun içinde çalıştırırsanız Git dosyaları oluşturulmaz; bu davranışı `cargo new --vcs=git` komutuyla geçersiz kılabilirsiniz.

> Not: Git yaygın bir *sürüm kontrol sistemidir (version control system)*. `--vcs` seçeneğini kullanarak `cargo new` komutunun farklı bir sürüm kontrol sistemi kullanmasını veya hiçbir sürüm kontrol sistemi başlatmamasını sağlayabilirsiniz. Kullanılabilir seçenekleri görmek için `cargo new --help` komutunu çalıştırın.

_Cargo.toml_ dosyasını tercih ettiğiniz metin düzenleyicide açın. İçeriği Liste 1-2'deki koda benzer görünmelidir.

<Listing number="1-2" file-name="Cargo.toml" caption="`cargo new` tarafından oluşturulan *Cargo.toml* dosyasının içeriği">

```toml
[package]
name = "merhaba_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

</Listing>

Bu dosya, Cargo'nun yapılandırma biçimi olan [_TOML_][toml]<!-- ignore --> (*Tom's Obvious, Minimal Language*) biçimindedir.

İlk satırdaki `[package]`, izleyen deyimlerin bir paketi yapılandırdığını belirten bir bölüm başlığıdır. Bu dosyaya daha fazla bilgi ekledikçe başka bölümler de ekleyeceğiz.

Sonraki üç satır, Cargo'nun programınızı derlemek için ihtiyaç duyduğu yapılandırma bilgilerini belirler: paket adı, paket sürümü ve kullanılacak Rust sürümü (*edition*). `edition` anahtarını [Ek E][appendix-e]<!-- ignore --> bölümünde ayrıntılı olarak ele alacağız.

Son satırdaki `[dependencies]`, projenizin bağımlılıklarını listeleyeceğiniz bölümün başlangıcıdır. Rust'ta kod paketlerine *crate* adı verilir. Bu projede başka bir crate'e ihtiyacımız olmayacak; ancak Bölüm 2'deki ilk projemizde harici bir crate kullanacağımız için bu bağımlılıklar bölümünden yararlanacağız.

Şimdi _src/main.rs_ dosyasını açın ve bir göz atın:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo, tıpkı Liste 1-1'de yazdığımız programa karşılık gelen bir "Hello, world!" başlangıç programı oluşturdu! Şu ana kadar önceki projemiz ile Cargo'nun oluşturduğu proje arasındaki farklar; Cargo'nun kaynak kodu _src_ dizinine yerleştirmesi ve en üst dizinde bir _Cargo.toml_ yapılandırma dosyasının bulunmasıdır.

> Not: Cargo'nun yerleşik proje şablonu İngilizce olduğundan `cargo new` komutu _src/main.rs_ içinde varsayılan olarak `"Hello, world!"` metnini üretir. Dilerseniz bu metni Liste 1-1'deki gibi `"Merhaba, dünya!"` olarak değiştirebilir, dilerseniz aşağıdaki adımlarda olduğu gibi bırakabilirsiniz.

Cargo, kaynak dosyalarınızın _src_ dizini içinde bulunmasını bekler. En üst düzey proje dizini yalnızca README dosyaları, lisans bilgileri, yapılandırma dosyaları ve doğrudan kaynak kodunuzla ilgili olmayan diğer dosyalar içindir. Cargo kullanmak projelerinizi düzenli tutmanıza yardımcı olur: Her şeyin bir yeri vardır ve her şey yerli yerindedir.

"Merhaba, dünya!" projesinde yaptığımız gibi Cargo kullanmadan başladığınız bir projeyi sonradan Cargo projesine dönüştürebilirsiniz. Bunun için proje kodunu _src_ dizinine taşıyıp uygun bir _Cargo.toml_ dosyası oluşturmanız yeterlidir. _Cargo.toml_ dosyasını oluşturmanın kolay bir yolu da mevcut dizinde `cargo init` komutunu çalıştırmaktır; bu komut dosyayı sizin için otomatik olarak üretir.

<!-- Old headings. Do not remove or links may break. -->
<a id="building-and-running-a-cargo-project"></a>

### Bir Cargo Projesini Derlemek ve Çalıştırmak

Şimdi "Hello, world!" programını Cargo ile derleyip çalıştırdığımızda nelerin değiştiğine bakalım! _merhaba_cargo_ dizininizdeyken aşağıdaki komutu girerek projenizi derleyin:

```console
$ cargo build
   Compiling merhaba_cargo v0.1.0 (file:///projects/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Bu komut, çalıştırılabilir dosyayı bulunduğunuz dizin yerine _target/debug/merhaba_cargo_ (veya Windows'ta _target\debug\merhaba_cargo.exe_) konumunda oluşturur. Varsayılan derleme bir hata ayıklama (*debug*) derlemesi olduğundan Cargo, ikili dosyayı _debug_ adlı bir dizinin içine koyar. Çalıştırılabilir dosyayı şu komutla çalıştırabilirsiniz:

```console
$ ./target/debug/merhaba_cargo # veya Windows'ta .\target\debug\merhaba_cargo.exe
Hello, world!
```

Her şey yolunda giderse terminale `Hello, world!` yazdırılmalıdır. `cargo build` komutunu ilk kez çalıştırmak, Cargo'nun en üst dizinde yeni bir dosya daha oluşturmasını sağlar: _Cargo.lock_. Bu dosya, projenizdeki bağımlılıkların tam sürümlerini kayıt altında tutar. Bu projenin henüz bağımlılığı olmadığından dosyanın içeriği oldukça kısadır. Bu dosyayı hiçbir zaman elle değiştirmenize gerek kalmaz; Cargo içeriğini sizin yerinize otomatik olarak yönetir.

> Not: _Cargo.toml_ ile _Cargo.lock_ arasındaki farkı bilmek faydalıdır: _Cargo.toml_ sizin düzenlediğiniz ve istediğiniz bağımlılık sürümlerini genel olarak belirttiğiniz yapılandırma dosyasıdır; _Cargo.lock_ ise Cargo'nun derleme sırasında çözdüğü **kesin sürüm numaralarını** kilitlediği dosyadır. Bu sayede projeniz başka bir bilgisayarda veya aylar sonra derlendiğinde bile birebir aynı paket sürümleri kullanılır. Çalıştırılabilir projelerde _Cargo.lock_ dosyasını Git deposuna dahil etmek standart pratiktir; derleme çıktılarının bulunduğu _target_ dizini ise Cargo'nun oluşturduğu _.gitignore_ dosyası sayesinde Git dışında tutulur.

Az önce projemizi `cargo build` ile derledik ve `./target/debug/merhaba_cargo` ile çalıştırdık; ancak kodu derleyip üretilen çalıştırılabilir dosyayı tek bir komutla çalıştırmak için `cargo run` komutunu da kullanabiliriz:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/merhaba_cargo`
Hello, world!
```

`cargo run` kullanmak, önce `cargo build` komutunu çalıştırmayı ve ardından ikili dosyanın tam yolunu yazmayı hatırlamaktan çok daha pratiktir; bu nedenle geliştiricilerin çoğu `cargo run` komutunu tercih eder.

Dikkat ederseniz bu kez Cargo'nun `merhaba_cargo` paketini derlediğini belirten bir çıktı görmedik. Cargo kaynak dosyaların değişmediğini fark etti ve projeyi yeniden derlemek yerine doğrudan mevcut ikili dosyayı çalıştırdı. Kaynak kodunuzda değişiklik yapmış olsaydınız Cargo çalıştırmadan önce projeyi yeniden derlerdi ve şu çıktıyı görürdünüz:

```console
$ cargo run
   Compiling merhaba_cargo v0.1.0 (file:///projects/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/merhaba_cargo`
Hello, world!
```

Cargo ayrıca `cargo check` adında bir komut da sunar. Bu komut, kodunuzun sorunsuz derlenip derlenmediğini hızla denetler ancak çalıştırılabilir bir dosya üretmez:

```console
$ cargo check
   Checking merhaba_cargo v0.1.0 (file:///projects/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Neden çalıştırılabilir bir dosya üretilmemesini isteyesiniz? Çoğu zaman `cargo check`, çalıştırılabilir ikili dosya üretme adımını atladığı için `cargo build` komutundan çok daha hızlıdır. Kod yazarken çalışmanızı sık sık kontrol ediyorsanız `cargo check` kullanmak, projenizin hâlâ derlenip derlenmediğini çok daha kısa sürede öğrenmenizi sağlar! Bu nedenle pek çok Rust geliştiricisi program yazarken kodun derlendiğinden emin olmak için düzenli aralıklarla `cargo check` çalıştırır; çalıştırılabilir dosyayı gerçekten kullanmaya hazır olduklarında ise `cargo build` (veya `cargo run`) komutuna başvurur.

Cargo hakkında şimdiye kadar öğrendiklerimizi özetleyelim:

- `cargo new` komutuyla yeni bir proje oluşturabiliriz.
- `cargo build` komutuyla bir projeyi derleyebiliriz.
- `cargo run` komutuyla bir projeyi tek adımda derleyip çalıştırabiliriz.
- `cargo check` komutuyla ikili dosya üretmeden projedeki hataları denetleyebiliriz.
- Cargo, derleme çıktısını kaynak kodumuzla aynı dizine kaydetmek yerine _target/debug_ dizininde saklar.

Cargo kullanmanın bir diğer avantajı da hangi işletim sisteminde çalışırsanız çalışın komutların tamamen aynı olmasıdır. Dolayısıyla bu noktadan itibaren Linux ve macOS ile Windows için ayrı komut talimatları vermeyeceğiz.

<!-- Old headings. Do not remove or links may break. -->
<a id="building-for-release"></a>
<a id="yayın-release-için-derlemek"></a>
<a id="yayin-release-icin-derlemek"></a>

### Yayın İçin Derlemek

Projeniz nihayet yayınlanmaya hazır olduğunda, optimizasyonlarla derlemek için `cargo build --release` komutunu kullanabilirsiniz. Bu komut, çalıştırılabilir dosyayı _target/debug_ yerine _target/release_ dizininde oluşturur. Optimizasyonlar Rust kodunuzun çok daha hızlı çalışmasını sağlar; ancak bu optimizasyonları uygulamak programınızın derlenme süresini uzatır. İki farklı derleme profilinin bulunmasının nedeni tam olarak budur: Biri hızlı ve sık yeniden derleme yapmak istediğiniz geliştirme süreci içindir; diğeri ise kullanıcıya teslim edeceğiniz, tekrar tekrar derlenmeyecek ve mümkün olduğunca hızlı çalışması gereken nihai programı üretmek içindir. Kodunuzun çalışma süresini ölçüyorsanız (*benchmarking*), derlemeyi mutlaka `cargo build --release` komutuyla yaptığınızdan ve ölçümleri _target/release_ dizinindeki çalıştırılabilir dosyayla gerçekleştirdiğinizden emin olun.

> Not: Cargo'nun varsayılan **`dev` profili**, terminal çıktısındaki `[unoptimized + debuginfo]` ifadesinden de görebileceğiniz üzere kod optimizasyonlarını kapalı tutar ve hata ayıklama (*debug*) sembollerini ikili dosyaya ekler; bu sayede derleme çok hızlı tamamlanır. **`release` profili** (`cargo build --release` veya doğrudan çalıştırmak için `cargo run --release`) ise kapsamlı derleyici optimizasyonlarını devreye sokar. Derleme profillerini _Cargo.toml_ üzerinden nasıl özelleştirebileceğinizi Bölüm 14'te inceleyeceğiz.

<!-- Old headings. Do not remove or links may break. -->
<a id="cargo-as-convention"></a>
<a id="leveraging-cargos-conventions"></a>

### Cargo Geleneklerinden Yararlanmak

Çok basit projelerde Cargo, doğrudan `rustc` kullanmaya kıyasla büyük bir fark yaratmıyor gibi görünebilir; ancak programlarınız karmaşıklaştıkça değerini kanıtlayacaktır. Programlar birden fazla dosyaya yayıldığında veya bir bağımlılığa ihtiyaç duyduğunda, derleme sürecini Cargo'nun koordine etmesine izin vermek işinizi çok kolaylaştırır.

`merhaba_cargo` projesi her ne kadar basit olsa da Rust kariyerinizin geri kalanında başvuracağınız gerçek araç takımının büyük bir kısmını şimdiden kullanıyor. Nitekim mevcut herhangi bir Rust projesi üzerinde çalışmak için Git ile kodu indirip proje dizinine geçmek ve derlemek üzere şu komutları kullanmanız yeterlidir:

```console
$ git clone example.org/bir_proje
$ cd bir_proje
$ cargo build
```

Cargo hakkında daha fazla bilgi edinmek için [Cargo belgelerine][cargo] göz atabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="summary"></a>

## Özet

Rust yolculuğunuza şimdiden harika bir başlangıç yaptınız! Bu bölümde şunları öğrendiniz:

- `rustup` kullanarak Rust'ın en son kararlı sürümünü kurmayı,
- Daha yeni bir Rust sürümüne güncellemeyi,
- Yerel olarak kurulmuş belgeleri açmayı,
- Doğrudan `rustc` kullanarak bir "Merhaba, dünya!" programı yazıp çalıştırmayı,
- Cargo'nun standart düzenini kullanarak yeni bir proje oluşturup çalıştırmayı.

Artık Rust kodu okumaya ve yazmaya alışmak için daha kapsamlı bir program geliştirmenin tam zamanı! Bu doğrultuda Bölüm 2'de bir sayı tahmin oyunu geliştireceğiz. Öncelikle yaygın programlama kavramlarının Rust'ta nasıl çalıştığını öğrenerek başlamayı tercih ederseniz doğrudan Bölüm 3'e geçebilir ve ardından Bölüm 2'ye dönebilirsiniz.

[installation]: ch01-01-kurulum.html#kurulum
[toml]: https://toml.io
[appendix-e]: ekler-05-surumler.html
[cargo]: https://doc.rust-lang.org/cargo/
