## Merhaba, Cargo!

Cargo, Rust'ın derleme (build) sistemi ve paket yöneticisidir. Çoğu Rustacean (Rust programcısı) projelerini yönetmek için bu aracı kullanır; çünkü Cargo, kodunuzu derlemek, kodunuzun bağımlı olduğu kütüphaneleri indirmek ve o kütüphaneleri derlemek gibi sizin için pek çok işi halleder. (Kodunuzun ihtiyaç duyduğu bu kütüphanelere _bağımlılıklar_ (dependencies) diyoruz.)

Şu ana kadar yazdığımız gibi en basit Rust programlarının herhangi bir bağımlılığı yoktur. "Merhaba, dünya!" projesini Cargo ile oluştursaydık, Cargo'nun yalnızca kodunuzu derlemeyle ilgilenen kısmını kullanırdı. Daha karmaşık Rust programları yazdıkça bağımlılıklar ekleyeceksiniz ve eğer bir projeye Cargo kullanarak başlarsanız, bağımlılık eklemek çok daha kolay olacaktır.

Rust projelerinin büyük çoğunluğu Cargo kullandığı için, bu kitabın geri kalanı da sizin Cargo kullandığınızı varsayar. ["Kurulum"][installation]<!-- ignore --> bölümünde bahsedilen resmi yükleyicileri kullandıysanız Cargo, Rust ile birlikte kurulu gelir. Eğer Rust'ı başka bir yolla kurduysanız, terminalinize şunu yazarak Cargo'nun kurulu olup olmadığını kontrol edin:

```console
$ cargo --version
```

Bir sürüm numarası görüyorsanız, Cargo kuruludur! Eğer `command not found` (komut bulunamadı) gibi bir hata görürseniz, Cargo'yu nasıl ayrı olarak kuracağınızı belirlemek için kullandığınız kurulum yönteminin belgelerine bakın.

### Cargo ile Proje Oluşturmak

Cargo'yu kullanarak yeni bir proje oluşturalım ve orijinal "Merhaba, dünya!" projemizden nasıl farklı olduğuna bakalım. _projects_ dizininize (veya kodunuzu nerede saklamaya karar verdiyseniz oraya) geri dönün. Ardından, herhangi bir işletim sisteminde aşağıdakini çalıştırın:

```console
$ cargo new merhaba_cargo
$ cd merhaba_cargo
```

İlk komut _merhaba_cargo_ adında yeni bir dizin ve proje oluşturur. Projemize _merhaba_cargo_ adını verdik ve Cargo, dosyalarını aynı isimde bir dizin içinde oluşturur.

_merhaba_cargo_ dizinine gidin ve dosyaları listeleyin. Cargo'nun bizim için iki dosya ve bir dizin oluşturduğunu göreceksiniz: bir _Cargo.toml_ dosyası ve içinde _main.rs_ dosyası bulunan bir _src_ dizini.

Ayrıca bir _.gitignore_ dosyası ile birlikte yeni bir Git deposu (repository) da başlatmıştır (initialize). Eğer mevcut bir Git deposunun içinde `cargo new` komutunu çalıştırırsanız Git dosyaları oluşturulmaz; `cargo new --vcs=git` kullanarak bu davranışı geçersiz kılabilirsiniz.

> Not: Git yaygın bir sürüm kontrol sistemidir (version control system). `--vcs` bayrağını (flag) kullanarak `cargo new` komutunun farklı bir sürüm kontrol sistemi kullanmasını sağlayabilir veya hiçbir sürüm kontrol sistemi kullanmamasını belirtebilirsiniz. Mevcut seçenekleri görmek için `cargo new --help` komutunu çalıştırın.

_Cargo.toml_ dosyasını tercih ettiğiniz metin düzenleyicide açın. Liste 1-2'deki koda benzer görünmelidir.

<Listing number="1-2" file-name="Cargo.toml" caption="`cargo new` tarafından oluşturulan *Cargo.toml* dosyasının içeriği">

```toml
[package]
name = "merhaba_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

</Listing>

Bu dosya, Cargo'nun yapılandırma formatı olan [_TOML_][toml]<!-- ignore --> (_Tom's Obvious, Minimal Language_) formatındadır.

İlk satır olan `[package]`, aşağıdaki ifadelerin bir paketi yapılandırdığını belirten bir bölüm başlığıdır. Bu dosyaya daha fazla bilgi ekledikçe, başka bölümler de ekleyeceğiz.

Sonraki üç satır, Cargo'nun programınızı derlemek için ihtiyaç duyduğu yapılandırma bilgilerini ayarlar: isim, sürüm (version) ve kullanılacak Rust sürümü (edition). `edition` anahtarından [Ek E][ekler-e]<!-- ignore -->'de bahsedeceğiz.

Son satır olan `[dependencies]`, projenizin bağımlılıklarından herhangi birini listelemeniz için bir bölümün başlangıcıdır. Rust'ta kod paketlerine _crate_ (kasa) adı verilir. Bu proje için başka bir crate'e ihtiyacımız olmayacak, ancak Bölüm 2'deki ilk projede olacak, bu yüzden o zaman bu bağımlılıklar (dependencies) bölümünü kullanacağız.

Şimdi _src/main.rs_ dosyasını açın ve bir göz atın:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
fn main() {
    println!("Merhaba, dünya!");
}
```

Cargo, tıpkı Liste 1-1'de yazdığımız gibi, sizin için bir "Merhaba, dünya!" programı oluşturdu! Şu ana kadar projemiz ile Cargo'nun oluşturduğu proje arasındaki farklar; Cargo'nun kodu _src_ dizinine koyması ve en üst (top-level) dizinde bir _Cargo.toml_ yapılandırma dosyamızın olmasıdır.

Cargo, kaynak dosyalarınızın _src_ dizini içinde bulunmasını bekler. En üst düzey proje dizini yalnızca README dosyaları, lisans bilgileri, yapılandırma dosyaları ve kodunuzla ilgisi olmayan diğer şeyler içindir. Cargo kullanmak projelerinizi düzenlemenize yardımcı olur. Her şeyin bir yeri vardır ve her şey yerli yerindedir.

"Merhaba, dünya!" projesinde yaptığımız gibi Cargo kullanmayan bir projeye başladıysanız, onu Cargo kullanan bir projeye dönüştürebilirsiniz. Proje kodunu _src_ dizinine taşıyın ve uygun bir _Cargo.toml_ dosyası oluşturun. Bu _Cargo.toml_ dosyasını elde etmenin kolay bir yolu, onu sizin için otomatik olarak oluşturacak olan `cargo init` komutunu çalıştırmaktır.

### Bir Cargo Projesini Derlemek ve Çalıştırmak

Şimdi "Merhaba, dünya!" programını Cargo ile derleyip çalıştırdığımızda nelerin farklı olduğuna bakalım! _merhaba_cargo_ dizininizden şu komutu girerek projenizi derleyin:

```console
$ cargo build
   Compiling merhaba_cargo v0.1.0 (file:///projects/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Bu komut, çalıştırılabilir dosyayı geçerli dizininizde oluşturmak yerine _target/debug/merhaba_cargo_ (veya Windows'ta _target\debug\merhaba_cargo.exe_) konumunda oluşturur. Varsayılan (default) derleme bir hata ayıklama derlemesi olduğundan, Cargo ikili dosyayı _debug_ adlı bir dizine koyar. Çalıştırılabilir dosyayı şu komutla çalıştırabilirsiniz:

```console
$ ./target/debug/merhaba_cargo # veya Windows'ta .\target\debug\merhaba_cargo.exe
Merhaba, dünya!
```

Eğer her şey yolunda giderse, terminale `Merhaba, dünya!` yazdırılmalıdır. `cargo build` komutunu ilk kez çalıştırmak, Cargo'nun en üst düzeyde (top-level) yeni bir dosya oluşturmasına da neden olur: _Cargo.lock_. Bu dosya projenizdeki bağımlılıkların kesin (exact) sürümlerini takip eder. Bu projenin bağımlılığı yoktur, bu nedenle dosya biraz boştur. Bu dosyayı hiçbir zaman manuel olarak değiştirmenize gerek kalmayacaktır; Cargo içeriğini sizin yerinize yönetir.

Az önce `cargo build` ile bir proje derledik ve `./target/debug/merhaba_cargo` ile çalıştırdık, ancak kodu derlemek ve ardından ortaya çıkan çalıştırılabilir dosyayı tek bir komutla çalıştırmak için `cargo run` komutunu da kullanabiliriz:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/merhaba_cargo`
Merhaba, dünya!
```

`cargo run` kullanmak, `cargo build` komutunu çalıştırmayı ve ardından ikili dosyanın tüm yolunu kullanmayı hatırlamak zorunda kalmaktan daha uygundur, bu yüzden çoğu geliştirici `cargo run` kullanır.

Dikkat ederseniz bu kez Cargo'nun `merhaba_cargo` projesini derlediğini (compiling) belirten bir çıktı görmedik. Cargo dosyaların değişmediğini anladı, bu yüzden yeniden derlemek yerine sadece ikili dosyayı çalıştırdı. Eğer kaynak kodunuzu değiştirmiş olsaydınız, Cargo çalıştırmadan önce projeyi yeniden derlerdi ve şu çıktıyı görürdünüz:

```console
$ cargo run
   Compiling merhaba_cargo v0.1.0 (file:///projects/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/merhaba_cargo`
Merhaba, dünya!
```

Cargo ayrıca `cargo check` adında bir komut da sağlar. Bu komut, kodunuzun derlendiğinden emin olmak için kodunuzu hızlıca kontrol eder ancak çalıştırılabilir bir dosya (executable) üretmez:

```console
$ cargo check
   Checking merhaba_cargo v0.1.0 (file:///projects/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Neden çalıştırılabilir bir dosya istemeyesiniz? Çoğu zaman `cargo check`, çalıştırılabilir bir dosya üretme adımını atladığı için `cargo build`'den çok daha hızlıdır. Kodu yazarken çalışmanızı sürekli olarak kontrol ediyorsanız, `cargo check` kullanmak projenizin hala derlenip derlenmediğini bilme sürecinizi hızlandıracaktır! Bu nedenle pek çok Rustacean programlarını yazarken derlendiğinden emin olmak için periyodik olarak `cargo check` çalıştırır. Daha sonra, çalıştırılabilir dosyayı kullanmaya hazır olduklarında `cargo build` komutunu çalıştırırlar.

Cargo hakkında şu ana kadar öğrendiklerimizi özetleyelim:

- `cargo new` kullanarak bir proje oluşturabiliriz.
- `cargo build` kullanarak bir projeyi derleyebiliriz.
- `cargo run` kullanarak bir projeyi tek adımda derleyip çalıştırabiliriz.
- Hataları kontrol etmek için bir ikili dosya üretmeden `cargo check` kullanarak bir projeyi derleyebiliriz.
- Cargo derleme sonucunu kodumuzla aynı dizine kaydetmek yerine, onu _target/debug_ dizininde saklar.

Cargo kullanmanın bir diğer avantajı da, hangi işletim sisteminde çalışırsanız çalışın komutların aynı olmasıdır. Dolayısıyla, bu noktada artık Linux ve macOS ile Windows için özel (farklı) talimatlar sunmayacağız.

### Yayın (Release) İçin Derlemek

Projeniz sonunda yayınlanmaya (release) hazır olduğunda, optimizasyonlarla derlemek için `cargo build --release` komutunu kullanabilirsiniz. Bu komut, çalıştırılabilir dosyayı _target/debug_ yerine _target/release_ dizininde oluşturacaktır. Optimizasyonlar Rust kodunuzun daha hızlı çalışmasını sağlar, ancak bunları açmak programınızın derlenmesi için geçen süreyi uzatır. İşte bu yüzden iki farklı profil vardır: biri geliştirme için, yani hızlı ve sık bir şekilde yeniden derlemek istediğiniz zaman; diğeri ise bir kullanıcıya vereceğiniz, tekrar tekrar derlenmeyecek ve mümkün olduğunca hızlı çalışacak son programı derlemek içindir. Eğer kodunuzun çalışma süresini test ediyorsanız (benchmarking), `cargo build --release` komutunu çalıştırdığınızdan ve _target/release_ dizinindeki çalıştırılabilir dosya ile test ettiğinizden emin olun.

<!-- Old headings. Do not remove or links may break. -->
<a id="cargo-as-convention"></a>

### Cargo Geleneklerinden Yararlanmak

Basit projelerde Cargo sadece `rustc` kullanmaya kıyasla çok fazla bir değer (fayda) sağlamaz, ancak programlarınız daha karmaşık hale geldikçe değerini kanıtlayacaktır. Programlar birden fazla dosyaya yayıldığında veya bir bağımlılığa (dependency) ihtiyaç duyduğunda, derlemeyi Cargo'nun koordine etmesine izin vermek çok daha kolaydır.

`merhaba_cargo` projesi basit olsa da, artık Rust kariyerinizin geri kalanında kullanacağınız gerçek araçların çoğunu kullanıyor. Aslında, mevcut herhangi bir proje üzerinde çalışmak için Git kullanarak kodu almak, o projenin dizinine gitmek ve derlemek için aşağıdaki komutları kullanabilirsiniz:

```console
$ git clone example.org/bazi_projeler
$ cd bazi_projeler
$ cargo build
```

Cargo hakkında daha fazla bilgi için [kendi dokümantasyonuna][cargo] göz atın.

## Özet

Rust yolculuğunuza şimdiden harika bir başlangıç yaptınız! Bu bölümde şunları nasıl yapacağınızı öğrendiniz:

- `rustup` kullanarak en son kararlı Rust sürümünü kurmak.
- Daha yeni bir Rust sürümüne güncellemek.
- Yerel olarak yüklenmiş dokümantasyonu açmak.
- Doğrudan `rustc` kullanarak bir "Merhaba, dünya!" programı yazmak ve çalıştırmak.
- Cargo'nun geleneklerini kullanarak yeni bir proje oluşturmak ve çalıştırmak.

Rust kodu okumaya ve yazmaya alışmak adına daha kapsamlı bir program geliştirmek için şimdi harika bir zaman. Bu yüzden, 2. Bölüm'de bir tahmin oyunu programı geliştireceğiz. Eğer öncelikle genel programlama kavramlarının Rust'ta nasıl çalıştığını öğrenerek başlamayı tercih ederseniz, Bölüm 3'e bakabilir ve ardından Bölüm 2'ye geri dönebilirsiniz.

[installation]: ch01-01-kurulum.html#kurulum
[toml]: https://toml.io
[ekler-e]: ekler-05-surumler.html
[cargo]: https://doc.rust-lang.org/cargo/
