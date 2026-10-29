<!-- Old headings. Do not remove or links may break. -->
<a id="an-io-project-building-a-command-line-program"></a>
<a id="bir-gç-io-projesi-bir-komut-satırı-programı-oluşturmak"></a>
<a id="bir-gc-io-projesi-bir-komut-satiri-programi-olusturmak"></a>

# Bir G/Ç (I/O) Projesi: Komut Satırı Programı Geliştirme

Bu bölümde hem şimdiye kadar edindiğiniz pek çok beceriyi gözden geçirip pekiştirecek hem de standart kütüphanenin birkaç yeni özelliğini keşfedeceğiz. Artık dağarcığınızda bulunan Rust kavramlarını pratiğe dökmek için dosya ve komut satırı girdi/çıktı işlemleriyle etkileşime giren bir komut satırı aracı geliştireceğiz.

Rust'ın hızı, güvenliği, tek ikili dosya çıktısı üretmesi ve çapraz platform desteği, onu komut satırı araçları oluşturmak için ideal bir dil haline getirir. Bu nedenle projemizde, klasik komut satırı arama aracı `grep`'in (**g**lobally search a **r**egular **e**xpression and **p**rint — bir düzenli ifadeyi metnin genelinde ara ve yazdır) kendi sürümünü geliştireceğiz. En basit kullanım senaryosunda `grep`, belirtilen bir dosyada belirtilen bir string'i arar. Bunun için `grep`, argüman olarak bir dosya yolu ve bir string alır; ardından dosyayı okur, bu string argümanını içeren satırları bulur ve ekrana yazdırır.

Bu süreçte, geliştirdiğimiz aracın diğer pek çok komut satırı uygulamasında standart olan terminal özelliklerinden nasıl yararlanacağını da göstereceğiz. Kullanıcının aracımızın davranışını yapılandırabilmesi için bir çevre değişkeninin değerini okuyacağız. Ayrıca hata mesajlarını standart çıktı (`stdout`) yerine standart hata konsol akışına (`stderr`) yazdıracağız; böylece örneğin kullanıcı başarılı çıktıları bir dosyaya yönlendirirken hata mesajlarını ekranda görmeye devam edebilecek.

Rust topluluğu üyelerinden Andrew Gallant, `ripgrep` adında tam donanımlı ve son derece hızlı bir `grep` uygulaması geliştirdi. Onunla kıyaslandığında bizim sürümümüz oldukça sade kalacak; ancak bu bölüm size `ripgrep` gibi gerçek dünya projelerini anlamak için ihtiyaç duyacağınız temel arka plan bilgisini kazandıracak.

Geliştireceğimiz `grep` projesi, şimdiye kadar öğrendiğiniz pek çok kavramı bir araya getirecek:

- Kodu düzenleme ([Bölüm 7][ch7]<!-- ignore -->)
- Vektörleri ve string'leri kullanma ([Bölüm 8][ch8]<!-- ignore -->)
- Hataları yönetme ([Bölüm 9][ch9]<!-- ignore -->)
- Uygun yerlerde trait'leri ve ömürleri kullanma ([Bölüm 10][ch10]<!-- ignore -->)
- Test yazma ([Bölüm 11][ch11]<!-- ignore -->)

Ayrıca [Bölüm 13][ch13]<!-- ignore --> ve [Bölüm 18][ch18]<!-- ignore -->'de ayrıntılı olarak ele alınacak olan *kapanışları (closures)*, *iteratörleri (iterators)* ve *trait nesnelerini (trait objects)* de kısaca tanıtacağız.

> Not: Rust; ek bir çalışma zamanı ortamı gerektirmeyen tek parça ikili çıktısı ve yüksek performansı sayesinde modern komut satırı ekosisteminin en güçlü dillerinden biridir — örneğin terminalde `rg` komutuyla çalıştırılan `ripgrep`, pek çok kod düzenleyicinin dahili arama altyapısına da güç verir. Gerçek dünya projelerinde alt komutlar, bayraklar (`--help`, `--version`) ve karmaşık argüman doğrulamaları için genellikle [`clap`](https://crates.io/crates/clap) gibi kütüphanelerden yararlanılır; bu ekosistemi derinlemesine incelemek isterseniz Rust CLI Çalışma Grubu'nun hazırladığı [*Command Line Applications in Rust*](https://rust-cli.github.io/book/) rehberine göz atabilirsiniz. Bu bölümde geliştireceğimiz `minigrep` projesinde ise temel ilkeleri doğrudan kavramak amacıyla hiçbir harici bağımlılık kullanmayacak; argüman okuma, dosya işlemleri, hata yönetimi, `src/main.rs` ile `src/lib.rs` ayrımı ve test süreçlerini yalnızca standart kütüphaneyle sıfırdan inşa edeceğiz.

[ch7]: ch07-00-paketler-crateler-ve-moduller.html
[ch8]: ch08-00-yaygin-koleksiyonlar.html
[ch9]: ch09-00-hata-yonetimi.html
[ch10]: ch10-00-jenerikler.html
[ch11]: ch11-00-test-yazma.html
[ch13]: ch13-00-fonksiyonel-ozellikler.html
[ch18]: ch18-00-nyp.html