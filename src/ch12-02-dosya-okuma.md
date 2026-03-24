## Dosya Okuma

Şimdi, `dosya_yolu` argümanında belirtilen dosyayı okuma işlevselliği ekleyeceğiz. İlk olarak, test etmek için örnek bir dosyaya ihtiyacımız var: Birkaç tekrarlanan kelime içeren, çok satırlı az miktarda metne sahip bir dosya kullanacağız. Liste 12-3'te işe yarayacak kısa bir şiir var! Projenizin kök dizininde _siir.txt_ adında bir dosya oluşturun ve aşağıdaki dizeleri girin.

<Listing number="12-3" file-name="siir.txt" caption="Kısa bir şiir iyi bir test örneği oluşturur.">

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/siir.txt}}
```

</Listing>

Metni yerine yerleştirdikten sonra, _src/main.rs_ dosyasını düzenleyin ve Liste 12-4'te gösterildiği gibi dosyayı okuyacak kodu ekleyin.

<Listing number="12-4" file-name="src/main.rs" caption="İkinci argüman tarafından belirtilen dosyanın içeriğini okumak">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

</Listing>

İlk olarak, `use` ifadesiyle standart kütüphanenin ilgili bir bölümünü dahil ediyoruz: Dosyaları işlemek için `std::fs`'ye ihtiyacımız var.

`main` fonksiyonunda, yeni `fs::read_to_string` ifadesi `dosya_yolu`nu alır, o dosyayı açar ve dosyanın içeriklerini barındıran `std::io::Result<String>` türünde bir değer döndürür.

Bundan sonra, programın şu ana kadar çalıştığını kontrol edebilmemiz için dosya okunduktan sonra `icerik` değerini yazdıran geçici bir `println!` ifadesi daha ekliyoruz.

İlk komut satırı argümanı olarak herhangi bir string (henüz arama kısmını uygulamadığımız için) ve ikinci argüman olarak _siir.txt_ dosyasını kullanarak bu kodu çalıştıralım:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

Harika! Kod dosyanın içeriğini okudu ve sonra yazdırdı. Fakat kodun birkaç kusuru var. Şu anda `main` fonksiyonunun birden çok sorumluluğu var: Genel olarak, her fonksiyon yalnızca bir fikirden sorumluysa fonksiyonlar daha net ve bakımı daha kolaydır. Diğer bir sorun ise hataları (errors) elimizden geldiğince iyi yönetmiyor olmamız. Program henüz küçük olduğu için bu kusurlar büyük bir problem değil, ancak program büyüdükçe bunları temiz bir şekilde düzeltmek daha da zorlaşacaktır. Az miktarda kodu yeniden düzenlemek çok daha kolay olduğundan, program geliştirirken yeniden düzenlemeye erken başlamak iyi bir pratiktir. Bunu bir sonraki adımda yapacağız.
