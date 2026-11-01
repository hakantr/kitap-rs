<!-- Old headings. Do not remove or links may break. -->

<a id="separating-modules-into-different-files"></a>
<a id="modulleri-farkli-dosyalara-ayirma"></a>

## Modülleri Farklı Dosyalara Ayırma

Bu bölümdeki tüm örneklerde şimdiye kadar birden fazla modülü tek bir dosyada tanımladık. Modüller büyüdüğünde, kod içinde gezinmeyi kolaylaştırmak amacıyla modül tanımlarını ayrı dosyalara taşımak isteyebilirsiniz.

Örneğin, birden fazla restoran modülü barındıran Liste 7-17'deki koddan başlayalım. Tüm modülleri crate kök dosyasında tanımlamak yerine her birini ayrı dosyalara ayıracağız. Bu örnekte crate kök dosyası _src/lib.rs_ dosyasıdır; ancak aynı yöntem, crate kök dosyası _src/main.rs_ olan ikili crate'lerde de işe yarar.

İlk olarak `restoran_on_kisim` modülünü kendi dosyasına çıkaracağız. `restoran_on_kisim` modülünün süslü parantezleri arasındaki kodu kaldırıp geriye yalnızca `mod restoran_on_kisim;` bildirimini bırakın; böylece _src/lib.rs_ dosyası Liste 7-21'deki hâlini alacaktır. Liste 7-22'deki _src/restoran_on_kisim.rs_ dosyasını oluşturana kadar bu kodun derlenmeyeceğini unutmayın.

<Listing number="7-21" file-name="src/lib.rs" caption="Gövdesi *src/restoran_on_kisim.rs* dosyasında yer alacak `restoran_on_kisim` modülünü bildirme">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

</Listing>

Ardından, daha önce süslü parantezlerin içinde yer alan kodu Liste 7-22'de gösterildiği gibi _src/restoran_on_kisim.rs_ adlı yeni bir dosyaya yerleştirin. Derleyici, crate kökünde `restoran_on_kisim` adında gövdesiz bir modül bildirimiyle (`mod restoran_on_kisim;`) karşılaştığı için modülün kodunu bu dosyada araması gerektiğini bilir.

<Listing number="7-22" file-name="src/restoran_on_kisim.rs" caption="*src/restoran_on_kisim.rs* dosyasında `restoran_on_kisim` modülü içindeki tanımlar">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/restoran_on_kisim.rs}}
```

</Listing>

Modül ağacınızda bir dosyayı yüklemek için `mod` bildirimini yalnızca _bir kez_ kullanmanız gerektiğine dikkat edin. Derleyici, dosyanın projenin bir parçası olduğunu ve `mod` deyimini koyduğunuz yere göre kodun modül ağacının neresinde konumlandığını bir kez öğrendikten sonra, projenizdeki diğer dosyalar yüklenen bu dosyanın koduna ["Modül Ağacındaki Bir Öğeye Başvurmak İçin Yollar"][paths]<!-- ignore --> kısmında ele aldığımız gibi modülün bildirildiği konuma giden bir yol üzerinden başvurmalıdır. Başka bir deyişle `mod`, diğer programlama dillerinde görmüş olabileceğiniz C/C++ `#include` veya Python/JavaScript `import` benzeri bir dosya içe aktarma işlemi _değildir_.

> Not: Rust'a yeni başlayanların en sık yaptığı hatalardan biri `mod` ile `use` anahtar kelimelerini birbirine karıştırmaktır:
> - **`mod restoran_on_kisim;`** derleyiciye *"Bu noktada yeni bir alt modül oluştur ve gövdesini ilgili `.rs` dosyasından derle"* der. Örneğin _src/lib.rs_ içinde zaten bildirdiğiniz `restoran_on_kisim` modülünü _src/musteri.rs_ adlı başka bir dosyada kullanmak isterken yanlışlıkla tekrar `mod restoran_on_kisim;` yazarsanız derleyici _src/musteri/restoran_on_kisim.rs_ adında yeni bir alt modül dosyası arar ve `error[E0583]: file not found for module 'restoran_on_kisim'` hatası verir (hatta hata mesajının altında modülü `use crate::...` ile içe aktarmanız gerektiğini hatırlatır).
> - **`use crate::restoran_on_kisim;`** ise yeni bir dosya derlemez; modül ağacına daha önce `mod` ile eklenmiş mevcut bir modüle geçerli kapsamda kısa yoldan erişmenizi sağlar. Benzer şekilde hem _src/lib.rs_ hem _src/main.rs_ içeren bir pakette aynı dosyayı her iki kökte de `mod` ile bildirmek yerine modülü yalnızca _src/lib.rs_ içinde bildirmeli, _src/main.rs_ içinden ise kütüphane crate'inin adıyla erişmelisiniz.

Şimdi de `karsilama` modülünü kendi dosyasına çıkaracağız. Bu süreç biraz farklıdır; çünkü `karsilama`, kök modülün değil `restoran_on_kisim` modülünün bir alt modülüdür. `karsilama` modülünün dosyasını, modül ağacındaki ata modüllerinin adını taşıyacak yeni bir dizine — bu örnekte _src/restoran_on_kisim_ dizinine — yerleştireceğiz.

`karsilama` modülünü taşımaya başlamak için _src/restoran_on_kisim.rs_ dosyasını yalnızca `karsilama` modülünün bildirimini içerecek şekilde değiştiriyoruz:

<Listing file-name="src/restoran_on_kisim.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/restoran_on_kisim.rs}}
```

</Listing>

Ardından _src/restoran_on_kisim_ adında bir dizin ve bu dizinin içinde `karsilama` modülündeki tanımları barındıracak _karsilama.rs_ dosyasını oluşturuyoruz:

<Listing file-name="src/restoran_on_kisim/karsilama.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/restoran_on_kisim/karsilama.rs}}
```

</Listing>

Eğer bunun yerine _karsilama.rs_ dosyasını doğrudan _src_ dizinine koysaydık derleyici, _karsilama.rs_ içindeki kodun `restoran_on_kisim` modülünün bir alt modülü olarak değil, doğrudan crate kökünde bildirilen bir `karsilama` modülünde yer almasını beklerdi. Derleyicinin hangi modülün kodu için hangi dosyaya bakacağını belirleyen kuralları sayesinde, dizin ve dosya yapısı modül ağacıyla birebir örtüşür.

<!-- Old headings. Do not remove or links may break. -->

<a id="alternate-file-paths"></a>
<a id="alternatif-dosya-yolları"></a>
<a id="alternatif-dosya-yollari"></a>

> ### Alternatif Dosya Yolları
>
> Şimdiye kadar Rust derleyicisinin kullandığı en deyimsel dosya yollarını ele aldık; ancak Rust daha eski bir dosya yolu stilini de destekler. Crate kökünde bildirilen `restoran_on_kisim` adlı bir modül için derleyici, modülün kodunu şu konumlardan birinde arayacaktır:
>
> - _src/restoran_on_kisim.rs_ (ele aldığımız modern düzen)
> - _src/restoran_on_kisim/mod.rs_ (daha eski stil, hâlâ desteklenen yol)
>
> `restoran_on_kisim` modülünün alt modülü olan `karsilama` adlı bir modül içinse derleyici, modülün kodunu şu konumlardan birinde arayacaktır:
>
> - _src/restoran_on_kisim/karsilama.rs_ (ele aldığımız modern düzen)
> - _src/restoran_on_kisim/karsilama/mod.rs_ (daha eski stil, hâlâ desteklenen yol)
>
> Aynı modül için her iki stili birden kullanırsanız derleyici belirsizlik nedeniyle hata verir (`E0761`). Aynı projedeki farklı modüller için iki stili karışık kullanmaya izin verilir; ancak bu durum projenizde gezinen kişiler için kafa karıştırıcı olabilir.
>
> _mod.rs_ adlı dosyaları kullanan stilin en büyük dezavantajı, projenizde _mod.rs_ adında pek çok dosyanın birikmesi ve bu dosyalar kod editörünüzde aynı anda açık olduğunda sekmeleri birbirinden ayırt etmenin zorlaşmasıdır.

Her modülün kodunu ayrı bir dosyaya taşıdık ve modül ağacı aynı kaldı. `restoranda_yemek_ye` içindeki fonksiyon çağrıları, tanımlar artık farklı dosyalarda bulunsa bile hiçbir değişiklik gerektirmeden çalışmaya devam edecektir. Bu teknik, modüller büyüdükçe onları yeni dosyalara taşımanıza olanak tanır.

_src/lib.rs_ içindeki `pub use crate::restoran_on_kisim::karsilama` deyiminin de değişmediğine ve `use` anahtar kelimesinin crate'in bir parçası olarak hangi dosyaların derlendiği üzerinde hiçbir etkisi olmadığına dikkat edin. `mod` anahtar kelimesi modülleri bildirir; Rust da o modülün içeriğini oluşturacak kod için modülle aynı adı taşıyan dosyaya bakar.

> Not: Modern dosya düzeninin (_src/restoran_on_kisim.rs_ + _src/restoran_on_kisim/karsilama.rs_) editör sekmelerinde _mod.rs_ karmaşasını önlemesinin yanı sıra pratik bir geliştirme avantajı daha vardır: Başlangıçta alt modülü olmayan _src/restoran_on_kisim.rs_ dosyasına ileride bir alt modül eklemek istediğinizde mevcut dosyayı taşımanıza veya yeniden adlandırmanıza gerek kalmaz; yanına _src/restoran_on_kisim/_ dizinini açmanız yeterlidir. Eski _mod.rs_ düzeni ise günümüzde özellikle Bölüm 11'de göreceğimiz _tests/_ dizinindeki ortak entegrasyon testi yardımcı modüllerinde (_tests/ortak/mod.rs_) özel bir amaca hizmet ettiği için hâlâ bilmeye değerdir.

<!-- Old headings. Do not remove or links may break. -->

<a id="summary"></a>
<a id="ozet"></a>

## Özet

Rust, bir modülde tanımlanan öğelere başka bir modülden başvurabilmeniz için bir paketi birden fazla crate'e ve bir crate'i birden fazla modüle bölmenize olanak tanır. Bunu mutlak veya göreli yollar belirterek yapabilirsiniz. Bu yollar, aynı kapsamda bir öğeyi birden fazla kez kullanırken daha kısa bir isim yazabilmeniz için `use` deyimiyle kapsama dahil edilebilir. Modül kodu varsayılan olarak gizlidir; ancak `pub` anahtar kelimesini ekleyerek tanımları dışarıya açık hâle getirebilirsiniz.

Bir sonraki bölümde, standart kütüphanede yer alan ve düzenli biçimde yapılandırdığınız kodunuzda kullanabileceğiniz bazı koleksiyon veri yapılarını inceleyeceğiz.

[paths]: ch07-03-oge-yollari.html
