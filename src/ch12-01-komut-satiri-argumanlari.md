<!-- Old headings. Do not remove or links may break. -->
<a id="accepting-command-line-arguments"></a>
<a id="komut-satırı-argümanlarını-kabul-etmek"></a>
<a id="komut-satiri-argumanlarini-kabul-etmek"></a>

## Komut Satırı Argümanlarını Kabul Etme

Her zaman olduğu gibi `cargo new` komutuyla yeni bir proje oluşturalım. Sisteminizde halihazırda kurulu olabilecek `grep` aracından ayırt edebilmek için projemize `minigrep` adını vereceğiz:

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

İlk görevimiz, `minigrep` aracının iki komut satırı argümanını kabul etmesini sağlamaktır: dosya yolu ve aranacak string. Başka bir deyişle programımızı `cargo run` komutu, ardından gelen argümanların `cargo` yerine programımıza ait olduğunu belirten iki tire (`--`), aranacak string ve içinde arama yapılacak dosyanın yoluyla şu şekilde çalıştırabilmek istiyoruz:

```console
$ cargo run -- aranacakstring ornek-dosyaadi.txt
```

Şu anda `cargo new` tarafından oluşturulan program, kendisine verdiğimiz argümanları işleyemez. [crates.io](https://crates.io/) üzerinde komut satırı argümanlarını kabul eden bir program yazmayı kolaylaştıran hazır kütüphaneler bulunsa da bu kavramı yeni öğrendiğiniz için bu özelliği kendimiz uygulayalım.

<!-- Old headings. Do not remove or links may break. -->
<a id="reading-the-argument-values"></a>
<a id="argüman-değerlerini-okumak"></a>
<a id="arguman-degerlerini-okumak"></a>

### Argüman Değerlerini Okuma

`minigrep`'in kendisine ilettiğimiz komut satırı argümanlarının değerlerini okuyabilmesi için Rust standart kütüphanesinde yer alan `std::env::args` fonksiyonuna ihtiyacımız vardır. Bu fonksiyon, `minigrep`'e aktarılan komut satırı argümanları üzerinde bir iteratör (*iterator*) döndürür. İteratörleri [Bölüm 13][ch13]<!-- ignore -->'te ayrıntılı olarak ele alacağız. Şimdilik iteratörler hakkında yalnızca iki ayrıntıyı bilmeniz yeterlidir: İteratörler bir dizi değer üretir ve bir iteratör üzerinde `collect` metodunu çağırarak onu, ürettiği tüm öğeleri barındıran vektör gibi bir koleksiyona dönüştürebiliriz.

Liste 12-1'deki kod, `minigrep` programınızın kendisine aktarılan tüm komut satırı argümanlarını okumasını ve ardından bu değerleri bir vektörde toplamasını sağlar.

<Listing number="12-1" file-name="src/main.rs" caption="Komut satırı argümanlarını bir vektörde toplama ve yazdırma">

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

</Listing>

İlk olarak, `args` fonksiyonunu kullanabilmek için bir `use` deyimiyle `std::env` modülünü kapsama dahil ediyoruz. `std::env::args` fonksiyonunun iç içe iki modül katmanında yer aldığına dikkat edin. [Bölüm 7][ch7-idiomatic-use]<!-- ignore -->'de ele aldığımız gibi, kullanmak istediğimiz fonksiyon birden fazla modül katmanının içinde yer aldığında doğrudan fonksiyonun kendisini değil, üst modülünü kapsama dahil etmeyi tercih ederiz. Böylece `std::env` içindeki diğer fonksiyonları da kolayca kullanabiliriz. Ayrıca bu yaklaşım, `use std::env::args` ekleyip fonksiyonu yalnızca `args` adıyla çağırmaktan çok daha nettir; çünkü tek başına `args` çağrısı kolaylıkla mevcut modülde tanımlanmış yerel bir fonksiyon sanılabilir.

> ### `args` Fonksiyonu ve Geçersiz Unicode
>
> Argümanlardan herhangi biri geçersiz Unicode içeriyorsa `std::env::args` fonksiyonunun panikleyeceğini unutmayın. Programınızın geçersiz Unicode barındıran argümanları da kabul etmesi gerekiyorsa bunun yerine `std::env::args_os` fonksiyonunu kullanın. Bu fonksiyon, `String` değerleri yerine `OsString` değerleri üreten bir iteratör döndürür. `OsString` değerleri platformdan platforma farklılık gösterdiği ve `String` değerlerine kıyasla kullanımı daha karmaşık olduğu için burada sadelik adına `std::env::args` kullanmayı tercih ettik. (Türkçe karakterler geçerli Unicode kodlamasına sahip olduğundan `std::env::args` Türkçe metin ve dosya adlarıyla sorunsuz çalışır; "geçersiz Unicode" durumu yalnızca işletim sisteminden UTF-8/UTF-16 kurallarına uymayan bozuk diziler geldiğinde ortaya çıkar.)

`main` fonksiyonunun ilk satırında `env::args` fonksiyonunu çağırıyor ve döndürdüğü iteratörü, ürettiği tüm değerleri içeren bir vektöre dönüştürmek için hemen ardından `collect` metodunu kullanıyoruz. `collect` metoduyla pek çok farklı koleksiyon türü oluşturulabildiğinden, string'lerden oluşan bir vektör (`Vec<String>`) istediğimizi belirtmek için `argumanlar` değişkeninin türünü açıkça bildiriyoruz. Rust'ta değişkenlerin türünü açıkça belirtmeniz çok nadiren gerekse de derleyici hangi koleksiyon türünü istediğinizi tek başına çıkaramadığı için `collect`, tür bildirimi eklemeniz gereken başlıca metotlardan biridir.

> Not: `collect` metodu dönüş türü üzerinde jeneriktir ve standart kütüphanedeki `FromIterator` trait'ini uygulayan `Vec<String>`, `String` veya `HashSet<String>` gibi pek çok farklı tür üretebilir; bu yüzden hiçbir tür bilgisi vermezseniz derleyici `error[E0283]: type annotations needed` (*tür bildirimleri gerekli*) hatası üretir. Liste 12-1'deki `let argumanlar: Vec<String> = ...` yazımına alternatif olarak, öğe türünü derleyicinin çıkarmasına bırakan `let argumanlar: Vec<_> = env::args().collect();` biçimini ya da türü doğrudan metot çağrısında belirten *turbofish* sözdizimini (`let argumanlar = env::args().collect::<Vec<String>>();`) de sıkça görebilirsiniz.

Son olarak hata ayıklama makrosu `dbg!` ile vektörü yazdırıyoruz. Kodu önce hiçbir argüman vermeden, ardından iki argümanla çalıştırmayı deneyelim:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

Vektördeki ilk değerin, ikili dosyamızın adı olan `"target/debug/minigrep"` olduğuna dikkat edin. Bu davranış, C dilindeki argüman listesi kuralıyla örtüşür ve programların çalıştırıldıkları adı çalışma sırasında öğrenmesine olanak tanır. Mesajlarda programın adını yazdırmak ya da programı çağırmak için kullanılan komut satırı takma adına göre programın davranışını değiştirmek istediğinizde program adına erişebilmek genellikle kullanışlıdır. Ancak bu bölümün amaçları doğrultusunda bu ilk değeri yok sayacak ve yalnızca ihtiyacımız olan iki argümanı kaydedeceğiz.

> Not: İkinci komuttaki `--` ayırıcısı, Cargo'ya kendisinden sonra gelen argümanları Cargo seçeneği olarak yorumlamayıp doğrudan derlenen ikili dosyaya iletmesini söyler; örneğin programınıza `--help` gibi tireyle başlayan bir argüman göndermek istediğinizde `--` kullanmazsanız Cargo kendi yardım ekranını gösterir. Vektörün `argumanlar[0]` konumunda çalıştırılabilir dosya yolunun yer alması ise C ve POSIX standartlarındaki `argv[0]` geleneğinden gelir: Örneğin Linux'taki `busybox` veya Rust araç zincirini yöneten `rustup` gibi çok amaçlı ikili dosyalar, hangi adla (`cargo`, `rustc`, `rustdoc` vb.) çağrıldıklarını bu ilk elemandan okuyarak tek bir ikili dosya üzerinden farklı araçlar gibi davranabilir.

<!-- Old headings. Do not remove or links may break. -->
<a id="saving-the-argument-values-in-variables"></a>
<a id="argüman-değerlerini-değişkenlere-kaydetmek-saving"></a>
<a id="arguman-degerlerini-degiskenlere-kaydetmek-saving"></a>
<a id="argüman-değerlerini-değişkenlere-kaydetmek"></a>
<a id="arguman-degerlerini-degiskenlere-kaydetmek"></a>

### Argüman Değerlerini Değişkenlere Kaydetme

Programımız şu anda komut satırı argümanları olarak belirtilen değerlere erişebiliyor. Şimdi bu iki argümanın değerini programın geri kalanı boyunca kullanabilmek üzere değişkenlere kaydetmemiz gerekiyor. Bunu Liste 12-2'de yapıyoruz.

<Listing number="12-2" file-name="src/main.rs" caption="Sorgu argümanını ve dosya yolu argümanını tutacak değişkenler oluşturma">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

</Listing>

Vektörü yazdırdığımızda gördüğümüz gibi programın adı vektördeki ilk değeri, yani `argumanlar[0]` konumunu kaplar; bu yüzden kendi argümanlarımızı `1` numaralı indeksten itibaren okumaya başlıyoruz. `minigrep`'in aldığı ilk argüman aradığımız string olduğundan, ilk argümana ait bir referansı `sorgu` değişkenine atıyoruz. İkinci argüman ise dosya yolu olacağından, ikinci argümana ait bir referansı `dosya_yolu` değişkenine yerleştiriyoruz.

Kodun amaçladığımız gibi çalıştığını kanıtlamak için bu değişkenlerin değerlerini geçici olarak ekrana yazdırıyoruz. Programı bu kez `test` ve `ornek.txt` argümanlarıyla tekrar çalıştıralım:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

Harika, program çalışıyor! İhtiyacımız olan argümanların değerleri doğru değişkenlere kaydediliyor. Daha sonra, kullanıcının hiç argüman girmemesi gibi olası hatalı durumlarla başa çıkmak için hata yönetimi ekleyeceğiz (örneğin şu hâliyle programı argümansız çalıştırırsanız `&argumanlar[1]` erişimi sınır aşımı nedeniyle panikleyecektir); şimdilik bu durumu göz ardı edip programa dosya okuma yeteneği kazandırmaya odaklanacağız.

[ch13]: ch13-00-fonksiyonel-ozellikler.html
[ch7-idiomatic-use]: ch07-04-use-anahtar-kelimesi.html#deyimsel-use-yolları-oluşturma
