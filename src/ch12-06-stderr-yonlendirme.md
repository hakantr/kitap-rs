<!-- Old headings. Do not remove or links may break. -->
<a id="writing-error-messages-to-standard-error-instead-of-standard-output"></a>
<a id="redirecting-errors-to-standard-error"></a>
<a id="hataları-standart-hataya-standard-error-yönlendirmek"></a>
<a id="hatalari-standart-hataya-standard-error-yonlendirmek"></a>

## Hataları Standart Hata Çıktısına Yönlendirme

Şu anda `println!` makrosunu kullanarak tüm çıktılarımızı terminale yazıyoruz. Çoğu terminalde iki tür çıktı akışı bulunur: Genel bilgiler için _standart çıktı_ (*standard output*, `stdout`) ve hata mesajları için _standart hata_ (*standard error*, `stderr`). Bu ayrım sayesinde kullanıcılar, bir programın başarılı çıktısını bir dosyaya yönlendirirken hata mesajlarının yine de ekrana yazdırılmasını sağlayabilir.

`println!` makrosu yalnızca standart çıktıya yazdırabilir; dolayısıyla standart hataya yazdırmak için başka bir araç kullanmamız gerekir.

<!-- Old headings. Do not remove or links may break. -->
<a id="checking-where-errors-are-written"></a>
<a id="hataların-nereye-yazıldığını-kontrol-etmek"></a>
<a id="hatalarin-nereye-yazildigini-kontrol-etmek"></a>

### Hataların Nereye Yazıldığını Kontrol Etme

İlk olarak, standart hataya yazdırmak istediğimiz hata mesajları da dahil olmak üzere `minigrep` tarafından üretilen tüm çıktının şu anda standart çıktıya nasıl yazıldığını gözlemleyelim. Bunu görmek için bilerek bir hataya yol açarken standart çıktı akışını bir dosyaya yönlendireceğiz. Standart hata akışını yönlendirmeyeceğimiz için standart hataya gönderilen her türlü içerik ekranda görünmeye devam edecektir.

Komut satırı programlarının hata mesajlarını standart hata akışına göndermesi beklenir; böylece standart çıktı akışını bir dosyaya yönlendirsek bile hata mesajlarını ekranda görmeye devam edebiliriz. Programımız şu anda bu kurala uygun davranmıyor: Birazdan hata mesajı çıktısını ekrana basmak yerine dosyaya kaydettiğini göreceğiz!

Bu davranışı göstermek için programı `>` operatörü ve standart çıktı akışını yönlendirmek istediğimiz _cikti.txt_ dosya yoluyla çalıştıracağız. Hataya yol açması için hiçbir argüman iletmeyeceğiz:

```console
$ cargo run > cikti.txt
```

`>` sözdizimi, kabuğa standart çıktının içeriğini ekran yerine _cikti.txt_ dosyasına yazmasını söyler. Ekranda görmeyi beklediğimiz hata mesajıyla karşılaşmadık; demek ki mesaj dosyaya yazılmış olmalı. _cikti.txt_ dosyasının içeriği şöyledir:

```text
Argümanları ayrıştırırken problem oluştu: yeterli argüman yok
```

Gerçekten de hata mesajımız standart çıktıya yazdırılıyor. Bu tür hata mesajlarının standart hataya yazdırılması çok daha kullanışlıdır; böylece dosyaya yalnızca başarılı bir çalıştırmadan elde edilen veriler kaydedilir. Şimdi bunu değiştireceğiz.

<!-- Old headings. Do not remove or links may break. -->
<a id="printing-errors-to-standard-error"></a>
<a id="hataları-standart-hataya-standard-error-yazdırmak"></a>
<a id="hatalari-standart-hataya-standard-error-yazdirmak"></a>
<a id="hataları-standart-hataya-yazdırma"></a>

### Hataları Standart Hata Çıktısına Yazdırma

Hata mesajlarının yazdırılma biçimini değiştirmek için Liste 12-24'teki kodu kullanacağız. Bu bölümün başlarında yaptığımız yeniden düzenleme sayesinde, hata mesajlarını yazdıran tüm kodlar tek bir fonksiyonda, yani `main` içinde toplanmış durumda. Standart kütüphane, standart hata akışına yazdıran `eprintln!` makrosunu sunar; dolayısıyla hata yazdırmak için `println!` çağırdığımız iki yeri `eprintln!` kullanacak şekilde değiştirelim.

<Listing number="12-24" file-name="src/main.rs" caption="`eprintln!` kullanarak hata mesajlarını standart çıktı yerine standart hataya yazdırma">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

</Listing>

Şimdi programı hiçbir argüman vermeden ve standart çıktıyı `>` ile yönlendirerek aynı şekilde tekrar çalıştıralım:

```console
$ cargo run > cikti.txt
Argümanları ayrıştırırken problem oluştu: yeterli argüman yok
```

Artık hatayı ekranda görüyoruz ve _cikti.txt_ dosyası hiçbir şey içermiyor; komut satırı programlarından beklenen davranış da tam olarak budur.

Şimdi programı hataya yol açmayan argümanlarla tekrar çalıştıralım, ancak standart çıktıyı yine aşağıdaki gibi bir dosyaya yönlendirelim:

```console
$ cargo run -- ne siir.txt > cikti.txt
```

Terminalde herhangi bir çıktı görmeyeceğiz ve _cikti.txt_ dosyası arama sonuçlarımızı barındıracak:

<span class="filename">Dosya adı: cikti.txt</span>

```text
Gelme, artık neye yarar?
```

Bu sonuç, artık başarılı çıktılar için standart çıktıyı, hata mesajları içinse standart hatayı olması gerektiği gibi kullandığımızı gösterir.

> Not: Unix türevi sistemlerde (Linux, macOS) ve modern Windows kabuklarında başlatılan her sürece otomatik olarak numaralandırılmış üç standart akış (dosya tanıtıcısı, *file descriptor*) atanır:
> - **`0` — `stdin` (standart girdi):** Klavyeden veya başka bir süreçten gelen veriyi okur (`std::io::stdin()`).
> - **`1` — `stdout` (standart çıktı):** Programın ürettiği normal sonuçları taşır (`print!`, `println!`).
> - **`2` — `stderr` (standart hata):** Hata, uyarı ve tanılama mesajlarını taşır (`eprint!`, `eprintln!`, `dbg!`).
>
> Kabukta yazdığınız `>` operatörü aslında `1>` ifadesinin kısaltmasıdır ve yalnızca `stdout` akışını yönlendirir. Yalnızca hata mesajlarını bir dosyaya kaydetmek isterseniz `cargo run 2> hatalar.txt`, hem normal çıktıyı hem de hata mesajlarını aynı dosyada birleştirmek isterseniz `cargo run > tum_cikti.txt 2>&1` sözdizimini kullanabilirsiniz.
>
> `eprintln!` kullanımı özellikle borulama (*piping*, `|`) işlemlerinde hayati önem taşır. Örneğin eşleşen satır sayısını bulmak için `minigrep ne siir.txt | wc -l` gibi bir komut zinciri kurduğunuzda kabuk, birinci programın yalnızca `stdout` (`1`) akışını ikinci programın `stdin` (`0`) akışına bağlar. Hataları `println!` ile `stdout`'a yazarsanız sonraki program hata metnini gerçek veri sanıp yanlış sonuç üretir (örneğin hata mesajının satırını da sayar!) ya da ayrıştırma hatasıyla çöker; `eprintln!` kullandığınızda ise hata mesajları boruya karışmadan doğrudan ekranda görünür ve sonraki araca yalnızca temiz veri aktarılır.

<!-- Old headings. Do not remove or links may break. -->
<a id="summary"></a>
<a id="ozet"></a>

## Özet

Bu bölümde, şimdiye kadar öğrendiğiniz pek çok temel kavramı yeniden hatırladık ve Rust'ta yaygın G/Ç işlemlerinin nasıl gerçekleştirileceğini ele aldık. Komut satırı argümanlarını, dosyaları, çevre değişkenlerini ve hata yazdırmaya yarayan `eprintln!` makrosunu kullanarak artık komut satırı uygulamaları geliştirmeye hazırsınız. Bunları önceki bölümlerdeki kavramlarla bir araya getirdiğinizde kodunuz iyi organize edilmiş ve iyi test edilmiş olacak; verileri uygun veri yapılarında verimli biçimde saklayıp hataları düzgünce yönetecektir.

Bir sonraki bölümde fonksiyonel dillerden esinlenen bazı Rust özelliklerini inceleyeceğiz: kapanışlar ve iteratörler.
