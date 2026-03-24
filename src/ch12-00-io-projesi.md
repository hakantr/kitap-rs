# Bir G/Ç (I/O) Projesi: Bir Komut Satırı Programı Oluşturmak

Bu bölüm, şimdiye kadar öğrendiğiniz pek çok becerinin bir özeti ve standart kütüphanenin birkaç özelliğinin daha keşfidir. Şu an sahip olduğunuz bazı Rust kavramlarını pratik yapmak için, dosya ve komut satırı girdisi/çıktısı (input/output) ile etkileşime giren bir komut satırı aracı oluşturacağız.

Rust'ın hızı, güvenliği, tek ikili dosya çıktısı (single binary output) ve çapraz platform (cross-platform) desteği, onu komut satırı araçları oluşturmak için ideal bir dil haline getirir; bu nedenle projemiz için klasik komut satırı arama aracı `grep`'in (dünya çapında bir düzenli ifade arama ve yazdırma - **g**lobally search a **r**egular **e**xpression and **p**rint) kendi sürümünü yapacağız. En basit kullanım senaryosunda, `grep` belirtilen bir dosyada belirtilen bir string'i (dizgiyi) arar. Bunu yapmak için `grep` bir dosya yolunu ve bir string'i argüman olarak alır. Ardından dosyayı okur, o dosyada string argümanını içeren satırları bulur ve o satırları yazdırır.

Yol boyunca, komut satırı aracımızın diğer birçok komut satırı aracının kullandığı terminal özelliklerini kullanmasını nasıl sağlayacağımızı göstereceğiz. Kullanıcının aracımızın davranışını yapılandırmasına izin vermek için bir çevre değişkeninin değerini okuyacağız. Ayrıca, kullanıcının başarılı çıktıyı bir dosyaya yönlendirirken hata mesajlarını ekranda görebilmesi için hata mesajlarını standart çıktı (`stdout`) yerine standart hata konsolu akışına (`stderr`) yazdıracağız.

Rust topluluğunun bir üyesi olan Andrew Gallant, zaten `ripgrep` adında tam özellikli, çok hızlı bir `grep` sürümü yarattı. Karşılaştırıldığında, bizim versiyonumuz oldukça basit olacak, ancak bu bölüm size `ripgrep` gibi gerçek dünya projelerini anlamanız için gereken bazı arka plan bilgilerini verecektir.

Bizim `grep` projemiz şu ana kadar öğrendiğiniz bazı kavramları birleştirecek:

- Kodu organize etmek ([Bölüm 7][ch7]<!-- ignore -->)
- Vektörleri ve stringleri kullanmak ([Bölüm 8][ch8]<!-- ignore -->)
- Hata yönetimi (Error handling) ([Bölüm 9][ch9]<!-- ignore -->)
- Uygun yerlerde traitleri ve ömürleri kullanmak ([Bölüm 10][ch10]<!-- ignore -->)
- Test yazmak ([Bölüm 11][ch11]<!-- ignore -->)

Ayrıca [Bölüm 13][ch13]<!-- ignore --> ve [Bölüm 18][ch18]<!-- ignore -->'in ayrıntılı olarak ele alacağı kapanışları, yineleyicileri ve trait nesnelerini kısaca tanıtacağız.

[ch7]: ch07-00-paketler-crateler-ve-moduller.html
[ch8]: ch08-00-yaygin-koleksiyonlar.html
[ch9]: ch09-00-hata-yonetimi.html
[ch10]: ch10-00-jenerikler.html
[ch11]: ch11-00-test-yazma.html
[ch13]: ch13-00-fonksiyonel-ozellikler.html
[ch18]: ch18-00-nyp.html