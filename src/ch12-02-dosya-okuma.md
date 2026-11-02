<!-- Old headings. Do not remove or links may break. -->
<a id="reading-a-file"></a>
<a id="dosya-okuma"></a>
<a id="bir-dosyayi-okuma"></a>

## Bir Dosyayı Okuma

Şimdi, `dosya_yolu` argümanında belirtilen dosyayı okuma işlevselliğini ekleyeceğiz. Öncelikle bu özelliği test etmek için örnek bir dosyaya ihtiyacımız var: Birkaç satıra yayılan ve tekrar eden bazı kelimeler barındıran kısa bir metin dosyası kullanacağız. Liste 12-3'te yer alan Necip Fazıl Kısakürek şiiri bu iş için gayet uygundur (orijinal kitapta Emily Dickinson'ın “I’m Nobody! Who are you?” şiirinin bulunduğu _poem.txt_ dosyası kullanılır; Türkçe çeviride ise ilerleyen bölümlerdeki arama ve büyük/küçük harf duyarlılığı örnekleriyle uyumlu olması için bu şiir tercih edilmiştir). Projenizin kök dizininde _siir.txt_ adında bir dosya oluşturun ve “Beklenen” şiirinin dizelerini bu dosyaya girin.

<Listing number="12-3" file-name="siir.txt" caption="Necip Fazıl Kısakürek'in bir şiiri iyi bir test örneği oluşturur.">

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/siir.txt}}
```

</Listing>

Metin dosyasını hazırladıktan sonra _src/main.rs_ dosyasını düzenleyin ve Liste 12-4'te gösterildiği gibi dosyayı okuyacak kodu ekleyin.

<Listing number="12-4" file-name="src/main.rs" caption="İkinci argümanla belirtilen dosyanın içeriğini okuma">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

</Listing>

İlk olarak bir `use` deyimiyle standart kütüphanenin ilgili bölümünü kapsama dahil ediyoruz: Dosyalarla çalışmak için `std::fs` modülüne ihtiyacımız var.

`main` fonksiyonunda eklediğimiz yeni `fs::read_to_string` çağrısı `dosya_yolu` değerini alır, ilgili dosyayı açar ve dosya içeriğini barındıran `std::io::Result<String>` türünde bir değer döndürür. Hemen ardından gelen `.expect("Dosya okunamadı")` çağrısı ise okuma başarılı olduğunda bu `Result` içindeki `String` değerini `icerik` değişkenine çıkarır; dosya okunamazsa programı belirtilen mesajla panikletir.

Sonrasında, programın bu aşamaya kadar doğru çalıştığını kontrol edebilmek için dosya okunduktan sonra `icerik` değerini ekrana yazdıran geçici bir `println!` çağrısı daha ekliyoruz.

Henüz arama kısmını uygulamadığımız için ilk komut satırı argümanı olarak herhangi bir string, ikinci argüman olarak da _siir.txt_ dosyasını vererek bu kodu çalıştıralım:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

> Not: `fs::read_to_string`, arka planda dosyayı açan, geçerli UTF-8 metninin tamamını tek seferde öbekte tahsis edilen bir `String` içine okuyan ve işlem bittiğinde dosya tanıtıcısını otomatik olarak kapatan pratik bir yardımcı fonksiyondur. Küçük ve orta boyutlu metin dosyaları için bu yöntem son derece kullanışlıdır; ancak gigabaytlarca büyüklükteki metin veya günlük dosyalarında tüm içeriği tek seferde belleğe yüklememek için dosyayı `std::fs::File::open` ile açıp `std::io::BufReader` ve `BufRead::lines` aracılığıyla satır satır akış hâlinde okumak çok daha verimlidir. Ayrıca komutta verdiğimiz `siir.txt` gibi göreli dosya yolları, derlenen ikili dosyanın bulunduğu `target/debug/` dizinine göre değil, programın çalıştırıldığı mevcut çalışma dizinine göre çözümlenir; `cargo run` komutunu proje kök dizininde çalıştırdığımız için program kök dizindeki _siir.txt_ dosyasını doğrudan bulur.

Harika! Kod dosyanın içeriğini okudu ve ardından ekrana yazdırdı. Ancak kodumuzun birkaç tasarım kusuru var. Şu anda `main` fonksiyonu birden fazla sorumluluk üstleniyor (hem argümanları ayrıştırıyor hem de dosyayı okuyor) ve `sorgu` ile `dosya_yolu` gibi yapılandırma değişkenleri ortak bir yapıda gruplanmadan dağınık biçimde tutuluyor; oysa genel olarak her fonksiyon yalnızca tek bir görevden sorumlu olduğunda fonksiyonlar çok daha anlaşılır ve bakımı kolay olur. Diğer önemli sorun ise hataları olabileceği kadar iyi yönetmiyor olmamız: Olası hata noktaları ve `expect` çağrıları kodun içine dağılmış durumda, üstelik üretilen hata mesajları hatanın gerçek nedenini kullanıcıya açıklamakta yetersiz kalıyor. Programımız henüz küçük olduğu için bu kusurlar şimdilik büyük bir sorun oluşturmuyor; ancak program büyüdükçe bunları temiz bir şekilde düzeltmek zorlaşacaktır. Daha az miktardaki kodu yeniden düzenlemek çok daha kolay olduğundan, program geliştirirken yeniden düzenlemeye erken aşamada başlamak iyi bir alışkanlıktır. Bir sonraki adımda bunu yapacağız.
