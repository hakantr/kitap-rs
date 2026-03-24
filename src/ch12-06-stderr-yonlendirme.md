<!-- Old headings. Do not remove or links may break. -->

<a id="writing-error-messages-to-standard-error-instead-of-standard-output"></a>

## Hataları Standart Hataya (Standard Error) Yönlendirmek

Şu anda `println!` makrosunu kullanarak tüm çıktılarımızı terminale yazıyoruz. Çoğu terminalde iki tür çıktı vardır: Genel bilgiler için _standart çıktı_ (`stdout`) ve hata mesajları için _standart hata_ (`stderr`). Bu ayrım, kullanıcıların bir programın başarılı çıktısını bir dosyaya yönlendirmeyi, ancak hata mesajlarını yine de ekrana yazdırmayı seçmesini sağlar.

`println!` makrosu yalnızca standart çıktıya yazdırma yeteneğine sahiptir, bu yüzden standart hataya yazdırmak için başka bir şey kullanmalıyız.

### Hataların Nereye Yazıldığını Kontrol Etmek

İlk olarak, standart hataya yazdırmak istediğimiz hata mesajları da dahil olmak üzere, `minigrep` tarafından yazdırılan içeriğin şu anda standart çıktıya nasıl yazıldığını gözlemleyelim. Bunu standart çıktı akışını bilerek bir hataya neden olurken bir dosyaya yönlendirerek yapacağız. Standart hata akışını yönlendirmeyeceğiz, bu nedenle standart hataya gönderilen herhangi bir içerik ekranda görüntülenmeye devam edecektir.

Komut satırı programlarının hata mesajlarını standart hata akışına göndermesi beklenir, böylece standart çıktı akışını bir dosyaya yönlendirsek bile hata mesajlarını ekranda görebiliriz. Programımız şu anda iyi davranmıyor: Bunun yerine hata mesajı çıktısını bir dosyaya kaydettiğini görmek üzereyiz!

Bu davranışı göstermek için, programı `>` ve standart çıktı akışını yönlendirmek istediğimiz _ciktilar.txt_ (output.txt) dosya yolu ile çalıştıracağız. Bir hataya neden olması gereken herhangi bir argümanı iletmeyeceğiz:

```console
$ cargo run > ciktilar.txt
```

`>` sözdizimi, kabuğa standart çıktının içeriğini ekran yerine _ciktilar.txt_ dosyasına yazmasını söyler. Ekrana yazdırılmasını beklediğimiz hata mesajını görmedik, bu da mesajın dosyaya gittiği anlamına gelir. _ciktilar.txt_ dosyasının içerdiği şey budur:

```text
Argümanları ayrıştırırken problem oluştu: yeterli argüman yok
```

Evet, hata mesajımız standart çıktıya yazdırılıyor. Bu gibi hata mesajlarının standart hataya yazdırılması çok daha yararlıdır, böylece dosyaya yalnızca başarılı bir çalıştırmadan elde edilen veriler gider. Bunu değiştireceğiz.

### Hataları Standart Hataya (Standard Error) Yazdırmak

Hata mesajlarının yazdırılma şeklini değiştirmek için Liste 12-24'teki kodu kullanacağız. Bu bölümün başlarında yaptığımız yeniden düzenleme nedeniyle, hata mesajlarını yazdıran tüm kodlar tek bir fonksiyonda, yani `main` içindedir. Standart kütüphane, standart hata akışına yazdıran `eprintln!` makrosunu sağlar, bu nedenle `println!` çağırdığımız iki yeri, hata yazdırmak için bunun yerine `eprintln!` kullanacak şekilde değiştirelim.

<Listing number="12-24" file-name="src/main.rs" caption="`eprintln!` kullanarak hata mesajlarını standart çıktı yerine standart hataya yazmak">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

</Listing>

Şimdi programı hiçbir argüman olmadan ve standart çıktıyı `>` ile yönlendirerek aynı şekilde tekrar çalıştıralım:

```console
$ cargo run > ciktilar.txt
Argümanları ayrıştırırken problem oluştu: yeterli argüman yok
```

Artık hatayı ekranda görüyoruz ve _ciktilar.txt_ hiçbir şey içermiyor ki bu da komut satırı programlarından beklediğimiz davranıştır.

Hata vermeyen ancak standart çıktıyı yine de aşağıdaki gibi bir dosyaya yönlendiren argümanlarla programı tekrar çalıştıralım:

```console
$ cargo run -- ne siir.txt > ciktilar.txt
```

Terminalde herhangi bir çıktı görmeyeceğiz ve _ciktilar.txt_ sonuçlarımızı barındıracak:

<span class="filename">Dosya adı: ciktilar.txt</span>

```text
Gelme, artık neye yarar?
```

Bu, uygun olduğu şekilde başarılı çıktılar için standart çıktıyı ve hata çıktıları için standart hatayı kullandığımızı gösterir.

## Özet

Bu bölüm, şimdiye kadar öğrendiğiniz bazı önemli kavramları özetledi ve Rust'ta yaygın G/Ç (I/O) işlemlerinin nasıl gerçekleştirileceğini ele aldı. Komut satırı argümanlarını (command line arguments), dosyaları, çevre değişkenlerini (environment variables) ve hataları yazdırmak için `eprintln!` makrosunu kullanarak, artık komut satırı uygulamaları yazmaya hazırsınız. Önceki bölümlerdeki kavramlarla birleştirildiğinde, kodunuz iyi organize edilmiş olacak, verileri uygun veri yapılarında etkili bir şekilde depolayacak, hataları güzel bir şekilde yönetecek ve iyi test edilmiş olacaktır.

Sırada fonksiyonel dillerden etkilenen bazı Rust özelliklerini inceleyeceğiz: kapanışlar ve yineleyiciler.
