## Modülleri Farklı Dosyalara Ayırma

Şimdiye kadar bu bölümdeki tüm örnekler birden fazla modülü tek bir dosyada tanımladı. Modüller büyüdüğünde, kod içinde gezinmeyi (navigate) kolaylaştırmak için modül tanımlarını ayrı bir dosyaya taşımak isteyebilirsiniz.

Örneğin, birden çok restoran modülü içeren Liste 7-17'deki koddan başlayalım. Tüm modülleri crate kök dosyasında tanımlamak yerine, modülleri ayrı dosyalara çıkaracağız. Bu durumda crate kök dosyası _src/lib.rs_'dir, ancak bu prosedür, crate kök dosyası _src/main.rs_ olan ikili crate'lerde de çalışır.

İlk olarak `restoran_on_kisim` modülünü kendi dosyasına çıkaracağız. `restoran_on_kisim` modülü için süslü parantezlerin içindeki kodu kaldırın ve _src/lib.rs_'in Liste 7-21'de gösterilen kodu içermesi için sadece `mod restoran_on_kisim;` bildirimini (declaration) bırakın. Liste 7-22'deki _src/restoran_on_kisim.rs_ dosyasını oluşturana kadar bu kodun derlenmeyeceğini unutmayın.

<Listing number="7-21" file-name="src/lib.rs" caption="Gövdesi *src/restoran_on_kisim.rs* dosyasında olacak olan `restoran_on_kisim` modülünü bildirmek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

</Listing>

Daha sonra, Liste 7-22'de gösterildiği gibi, süslü parantezlerin içinde bulunan kodu _src/restoran_on_kisim.rs_ adlı yeni bir dosyaya yerleştirin. Derleyici, bu dosyaya bakması gerektiğini bilir çünkü crate kökünde `restoran_on_kisim` adında bir modül bildirimiyle karşılaşmıştır.

<Listing number="7-22" file-name="src/restoran_on_kisim.rs" caption="*src/restoran_on_kisim.rs* içindeki `restoran_on_kisim` modülünün tanımları">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/restoran_on_kisim.rs}}
```

</Listing>

Modül ağacınızda bir dosyayı yüklemek için `mod` bildirimini yalnızca _bir kez_ kullanmanız gerektiğini unutmayın. Derleyici, dosyanın projenin bir parçası olduğunu bildiğinde (ve `mod` ifadesini nereye koyduğunuzdan dolayı kodun modül ağacının neresinde bulunduğunu anladığında), projenizdeki diğer dosyalar yüklenen dosyanın koduna ["Modül Ağacındaki Bir Öğeye Başvurmak İçin Yollar (Paths)"][paths]<!-- ignore --> bölümünde anlatıldığı gibi o dosyanın nerede tanımlandığını gösteren bir yol kullanarak atıfta bulunmalıdır. Başka bir deyişle, `mod` diğer programlama dillerinde görmüş olabileceğiniz bir "include" (dahil etme) işlemi _değildir_.

Sonraki adımda, `karsilama` modülünü kendi dosyasına çıkaracağız. Bu süreç biraz farklıdır çünkü `karsilama`, kök (root) modülünün değil, `restoran_on_kisim` modülünün bir alt modülüdür. `karsilama` dosyasını modül ağacındaki ata modüllerine göre adlandırılacak yeni bir dizine, bu durumda _src/restoran_on_kisim_ dizinine yerleştireceğiz.

`karsilama` modülünü taşımaya başlamak için _src/restoran_on_kisim.rs_ dosyasını, yalnızca `karsilama` modülünün bildirimini içerecek şekilde değiştiriyoruz:

<Listing file-name="src/restoran_on_kisim.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/restoran_on_kisim.rs}}
```

</Listing>

Daha sonra, `karsilama` modülünde yapılan tanımları içermesi için _src/restoran_on_kisim_ adlı bir dizin ve içinde bir _karsilama.rs_ dosyası oluşturuyoruz:

<Listing file-name="src/restoran_on_kisim/karsilama.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/restoran_on_kisim/karsilama.rs}}
```

</Listing>

Eğer _karsilama.rs_ dosyasını _src_ dizinine koysaydık derleyici _karsilama.rs_ kodunun, `restoran_on_kisim` modülünün bir alt modülü olarak değil de doğrudan crate kökünde bildirilen bir `karsilama` modülünde olmasını beklerdi. Derleyicinin hangi modüllerin kodu için hangi dosyaları kontrol edeceğine dair kuralları, dizinlerin ve dosyaların modül ağacıyla daha yakından eşleştiği anlamına gelir.

> ### Alternatif Dosya Yolları
>
> Şimdiye kadar Rust derleyicisinin kullandığı en idiomatik dosya yollarını ele aldık, ancak Rust daha eski bir dosya yolu stilini de destekler. Crate kökünde tanımlanan `restoran_on_kisim` adlı bir modül için derleyici modülün kodunu şurada arayacaktır:
>
> - _src/restoran_on_kisim.rs_ (bizim işlediğimiz)
> - _src/restoran_on_kisim/mod.rs_ (daha eski stil, hala desteklenen yol)
>
> `restoran_on_kisim` modülünün alt modülü olan `karsilama` adlı bir modül için ise, derleyici modülün kodunu şurada arayacaktır:
>
> - _src/restoran_on_kisim/karsilama.rs_ (bizim işlediğimiz)
> - _src/restoran_on_kisim/karsilama/mod.rs_ (daha eski stil, hala desteklenen yol)
>
> Aynı modül için her iki stili de kullanırsanız, derleyici hatası alırsınız. Aynı projedeki farklı modüller için her iki stili bir arada kullanmaya izin verilir ancak projenizde gezinen kişiler için kafa karıştırıcı olabilir.
>
> _mod.rs_ adlı dosyaları kullanan stilin ana dezavantajı, projenizde _mod.rs_ adlı birçok dosyanın bulunması ve bu dosyaların editörünüzde aynı anda açık olduğunda kafa karışıklığı yaratabilmesidir.

Her modülün kodunu ayrı bir dosyaya taşıdık ve modül ağacı aynı kaldı. `restoranda_yemek_ye` içindeki fonksiyon çağrıları, tanımlar farklı dosyalarda yaşasa bile herhangi bir değişiklik olmadan çalışacaktır. Bu teknik, modüllerin boyutları büyüdükçe onları yeni dosyalara taşımanıza olanak tanır.

_src/lib.rs_ içindeki `pub use crate::restoran_on_kisim::karsilama` ifadesinin de değişmediğine ve `use` anahtar kelimesinin crate'in bir parçası olarak hangi dosyaların derlendiği üzerinde hiçbir etkisinin olmadığına dikkat edin. `mod` anahtar kelimesi modülleri tanımlar (declare) ve Rust o modülün içine giren kod için modülle aynı ada sahip bir dosyaya bakar.

## Özet

Rust, bir modülde tanımlanan öğelere başka bir modülden atıfta bulunabilmeniz (refer to) için bir paketi birden çok crate'e ve bir crate'i birden çok modüle bölmenizi sağlar. Bunu mutlak veya göreceli yollar (absolute or relative paths) belirterek yapabilirsiniz. Bu yollar, o kapsamda öğenin birden çok kullanımı için daha kısa bir yol kullanabilmeniz adına `use` ifadesiyle kapsama dahil edilebilir. Modül kodu varsayılan olarak gizlidir, ancak `pub` anahtar kelimesini ekleyerek tanımları açık hale getirebilirsiniz.

Bir sonraki bölümde, standart kütüphanede (standard library) bulunan ve kendi düzenli organize edilmiş kodunuzda kullanabileceğiniz bazı koleksiyon veri yapılarına (collection data structures) bakacağız.

[paths]: ch07-03-oge-yollari.html
