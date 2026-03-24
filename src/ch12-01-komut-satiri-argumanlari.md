## Komut Satırı Argümanlarını Kabul Etmek

Her zaman olduğu gibi `cargo new` ile yeni bir proje oluşturalım. Sisteminizde zaten var olabilecek `grep` aracından ayırt etmek için projemize `minigrep` adını vereceğiz:

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

İlk görev `minigrep`'in iki komut satırı argümanını kabul etmesini sağlamaktır: dosya yolu ve aranacak string (dizgi). Yani, programımızı `cargo run` ile, sonraki argümanların `cargo` için değil programımız için olduğunu belirtmek için iki tire (`--`), aranacak string ve içinde aranacak dosyanın yolu ile şu şekilde çalıştırabilmek istiyoruz:

```console
$ cargo run -- aranacakstring ornek-dosyaadi.txt
```

Şu anda `cargo new` tarafından üretilen program verdiğimiz argümanları işleyemez. [crates.io](https://crates.io/)'daki mevcut bazı kütüphaneler komut satırı argümanlarını kabul eden bir program yazmaya yardımcı olabilir, ancak bu kavramı yeni öğrendiğiniz için bu yeteneği kendimiz uygulayalım.

### Argüman Değerlerini Okumak

`minigrep`'in ona ilettiğimiz komut satırı argümanlarının değerlerini okumasını sağlamak için, Rust'ın standart kütüphanesinde sağlanan `std::env::args` fonksiyonuna ihtiyacımız olacak. Bu fonksiyon, `minigrep`'e iletilen komut satırı argümanlarının bir yineleyicisini döndürür. Yineleyicileri (iteratörleri) [Bölüm 13][ch13]<!-- ignore -->'te tam olarak ele alacağız. Şimdilik yineleyiciler hakkında yalnızca iki ayrıntı bilmeniz gerekir: Yineleyiciler bir dizi değer üretir ve yineleyicinin ürettiği tüm öğeleri barındıran vektör gibi bir koleksiyona dönüştürmek için yineleyici üzerinde `collect` (topla) metodunu çağırabiliriz.

Liste 12-1'deki kod, `minigrep` programınızın kendisine aktarılan tüm komut satırı argümanlarını okumasını ve ardından değerleri bir vektörde toplamasını sağlar.

<Listing number="12-1" file-name="src/main.rs" caption="Komut satırı argümanlarını bir vektöre toplamak ve yazdırmak">

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

</Listing>

İlk olarak, `args` (argümanlar) fonksiyonunu kullanabilmek için `use` ifadesiyle `std::env` modülünü kapsama dahil ediyoruz. `std::env::args` fonksiyonunun iki seviyeli modüllerin içine yerleştirildiğine dikkat edin. [Bölüm 7][ch7-idiomatic-use]<!-- ignore -->'de tartıştığımız gibi, istenen fonksiyonun birden fazla modül içine yerleştirildiği durumlarda fonksiyon yerine ebeveyn modülü kapsama almayı seçtik. Bunu yaparak, `std::env`'deki diğer fonksiyonları kolayca kullanabiliriz. Ayrıca, `use std::env::args` eklemekten ve ardından fonksiyonu yalnızca `args` ile çağırmaktan daha az belirsizdir, çünkü `args` kolayca mevcut modülde tanımlanmış bir fonksiyon sanılabilir.

> ### `args` Fonksiyonu ve Geçersiz Unicode
>
> Herhangi bir argüman geçersiz Unicode içeriyorsa `std::env::args`'ın panikleyeceğini unutmayın. Programınızın geçersiz Unicode barındıran argümanları kabul etmesi gerekiyorsa, bunun yerine `std::env::args_os` kullanın. Bu fonksiyon, `String` değerleri yerine `OsString` değerleri üreten bir yineleyici döndürür. Basitlik adına burada `std::env::args` kullanmayı seçtik, çünkü `OsString` değerleri platforma göre farklılık gösterir ve üzerinde çalışılması `String` değerlerinden daha karmaşıktır.

`main` fonksiyonunun ilk satırında `env::args` çağırıyoruz ve yineleyiciyi, yineleyicinin ürettiği tüm değerleri barındıran bir vektöre dönüştürmek için hemen `collect` kullanıyoruz. Birçok türde koleksiyon oluşturmak için `collect` fonksiyonunu kullanabiliriz, bu nedenle stringlerden oluşan bir vektör istediğimizi belirtmek için `argumanlar` değişkeninin türünü açıkça açıklıyoruz. Rust'ta türleri açıklamanız çok nadiren gerekse de, Rust istediğiniz koleksiyon türünü çıkaramadığı için `collect` genellikle açıklamanız gereken bir fonksiyondur.

Son olarak, `dbg!` (hata ayıklama) makrosunu kullanarak vektörü yazdırıyoruz. Önce argüman olmadan sonra iki argümanla kodu çalıştırmayı deneyelim:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

Vektördeki ilk değerin `"target/debug/minigrep"` olduğuna dikkat edin; bu, bizim ikili dosyamızın adıdır. Bu, C'deki argüman listesinin davranışıyla eşleşerek, programların çalıştırılmalarında çağrıldıkları adı kullanmalarına izin verir. Mesajlarda program adını yazdırmak isterseniz veya programı çağırmak için hangi komut satırı takma adının kullanıldığına bağlı olarak programın davranışını değiştirmek isterseniz program ismine erişebilmek genellikle kullanışlıdır. Ancak bu bölümün amaçları doğrultusunda bunu görmezden geleceğiz ve yalnızca ihtiyacımız olan iki argümanı kaydedeceğiz.

### Argüman Değerlerini Değişkenlere Kaydetmek (Saving)

Program şu anda komut satırı argümanları olarak belirtilen değerlere erişebiliyor. Şimdi iki argümanın değerini, değerleri programın geri kalanı boyunca kullanabilmemiz için değişkenlere kaydetmemiz gerekiyor. Bunu Liste 12-2'de yapıyoruz.

<Listing number="12-2" file-name="src/main.rs" caption="Sorgu argümanını ve dosya yolu argümanını tutacak değişkenler oluşturmak">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

</Listing>

Vektörü yazdırdığımızda gördüğümüz gibi programın adı vektördeki ilk değeri `argumanlar[0]`'da alıyor, bu yüzden argümanlara 1. indeksten başlıyoruz. `minigrep`'in aldığı ilk argüman aradığımız string'dir (dizgidir), bu nedenle ilk argümana bir referansı `sorgu` değişkenine koyuyoruz. İkinci argüman dosya yolu olacaktır, bu yüzden ikinci argümana bir referansı `dosya_yolu` değişkenine yerleştiriyoruz.

Kodun amaçladığımız gibi çalıştığını kanıtlamak için bu değişkenlerin değerlerini geçici olarak yazdırıyoruz. Hadi bu programı tekrar `test` ve `ornek.txt` (sample.txt) argümanlarıyla çalıştıralım:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

Harika, program çalışıyor! İhtiyacımız olan argümanların değerleri doğru değişkenlere kaydediliyor. Daha sonra kullanıcının argüman sağlamaması gibi bazı olası hatalı durumlarla başa çıkmak için bazı hata yönetimi ekleyeceğiz; şimdilik bu durumu göz ardı edeceğiz ve bunun yerine dosya okuma özellikleri eklemeye çalışacağız.

[ch13]: ch13-00-fonksiyonel-ozellikler.html
[ch7-idiomatic-use]: ch07-04-use-anahtar-kelimesi.html#idiomatik-use-yolları-oluşturmak
