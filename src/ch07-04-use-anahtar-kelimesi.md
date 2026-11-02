<!-- Old headings. Do not remove or links may break. -->

<a id="bringing-paths-into-scope-with-the-use-keyword"></a>
<a id="use-anahtar-kelimesi-ile-yolları-kapsama-scope-getirmek"></a>
<a id="use-anahtar-kelimesi-ile-yollari-kapsama-scope-getirmek"></a>

## `use` Anahtar Kelimesi ile Yolları Kapsama Dahil Etme

Fonksiyonları çağırmak için yolları her seferinde uzun uzun yazmak zahmetli ve tekrarlayıcı gelebilir. Liste 7-7'de `bekleme_listesine_ekle` fonksiyonuna ulaşmak için ister mutlak ister göreli yolu seçelim, `bekleme_listesine_ekle` fonksiyonunu her çağırmak istediğimizde `restoran_on_kisim` ve `karsilama` modüllerini de belirtmek zorundaydık. Neyse ki bu süreci basitleştirmenin bir yolu vardır: `use` anahtar kelimesiyle bir yol için tek seferlik bir kısayol oluşturabilir ve ardından aynı kapsamın her yerinde bu daha kısa adı kullanabiliriz.

Liste 7-11'de, `restoranda_yemek_ye` içinde `bekleme_listesine_ekle` fonksiyonunu çağırmak için yalnızca `karsilama::bekleme_listesine_ekle` yazmamız yeterli olacak şekilde `crate::restoran_on_kisim::karsilama` modülünü `restoranda_yemek_ye` fonksiyonunun bulunduğu kapsama dahil ediyoruz.

<Listing number="7-11" file-name="src/lib.rs" caption="Bir modülü `use` ile kapsama dahil etme">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

</Listing>

Bir kapsama `use` ile bir yol eklemek, dosya sisteminde sembolik bağlantı (*symbolic link*) oluşturmaya benzer. Crate köküne `use crate::restoran_on_kisim::karsilama` eklediğimizde, tıpkı `karsilama` modülü doğrudan crate kökünde tanımlanmış gibi `karsilama` adı o kapsamda geçerli bir isim hâline gelir. `use` ile kapsama dahil edilen yollar da diğer tüm yollar gibi gizlilik kurallarına tabidir.

`use` deyiminin kısayolu yalnızca yazıldığı kapsam için oluşturduğuna dikkat edin. Liste 7-12, `restoranda_yemek_ye` fonksiyonunu `musteri` adlı yeni bir alt modüle taşır; bu alt modül `use` deyiminden farklı bir kapsam oluşturduğu için fonksiyon gövdesi derlenmeyecektir.

<Listing number="7-12" file-name="src/lib.rs" caption="Bir `use` deyimi yalnızca bulunduğu kapsamda geçerlidir.">

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

</Listing>

Derleyici hatası, kısayolun `musteri` modülü içinde artık geçerli olmadığını gösterir:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

Ayrıca `use` deyiminin kendi kapsamında artık kullanılmadığına dair bir uyarı (`unused import`) üretildiğine de dikkat edin! Bu sorunu çözmek için ya `use` deyimini de `musteri` modülünün içine taşıyabilir ya da `musteri` alt modülünün içinden üst modüldeki kısayola `super::karsilama` ile başvurabilirsiniz.

> Not: Bir fonksiyonun içindeki `{ ... }` bloklarının aksine `mod musteri { ... }` bildirimi yeni ve bağımsız bir modül ad alanı başlatır; bu nedenle üst modülde `use` ile tanımlanan kısayollar alt modüllerde doğrudan (önek olmadan) görünmez. Derleyici çıktısındaki `` cannot find module or crate `karsilama` in this scope `` (*bu kapsamda `karsilama` modülü veya crate'i bulunamıyor*) hatasının nedeni budur. Bununla birlikte alt modüller üst modüllerin gizli öğelerine erişebildiği için `super::karsilama` veya derleyicinin önerdiği `use crate::karsilama;` yoluyla üst modüldeki kısayolu kullanabilirsiniz. Aynı kapsam kuralı diğer yönde de işler: Bir `use` deyimi yalnızca dosyanın başına değil, bir fonksiyon gövdesinin içine de yazılabilir; bu durumda oluşturulan kısayol yalnızca o fonksiyon bloğunda geçerli olur.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-idiomatic-use-paths"></a>
<a id="idiomatik-use-yolları-oluşturmak"></a>
<a id="i̇diomatik-use-yolları-oluşturmak"></a>
<a id="idiomatik-use-yollari-olusturmak"></a>

### Deyimsel `use` Yolları Oluşturma

Liste 7-11'de, Liste 7-13'teki gibi aynı sonucu elde etmek üzere `use` yolunu doğrudan `bekleme_listesine_ekle` fonksiyonuna kadar uzatmak yerine neden `use crate::restoran_on_kisim::karsilama` yazıp ardından `restoranda_yemek_ye` içinde `karsilama::bekleme_listesine_ekle` çağrısı yaptığımızı merak etmiş olabilirsiniz.

<Listing number="7-13" file-name="src/lib.rs" caption="`bekleme_listesine_ekle` fonksiyonunu deyimsel olmayan bir biçimde `use` ile kapsama dahil etme">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

</Listing>

Hem Liste 7-11 hem de Liste 7-13 aynı işi görse de bir fonksiyonu `use` ile kapsama dahil etmenin *deyimsel (idiomatic)* yolu Liste 7-11'deki kullanımdır. Fonksiyonun üst modülünü `use` ile kapsama dahil etmek, fonksiyonu çağırırken bu üst modülü belirtmemizi gerektirir. Çağrı noktasında üst modülü yazmak, tam yolun tekrarını en aza indirirken fonksiyonun yerel olarak tanımlanmadığını da açıkça gösterir. Liste 7-13'teki kodda ise `bekleme_listesine_ekle` fonksiyonunun nerede tanımlandığı belirsizdir.

Öte yandan struct'ları, enum'ları ve fonksiyon dışındaki diğer öğeleri `use` ile kapsama dahil ederken öğenin tam yolunu belirtmek deyimseldir. Liste 7-14, standart kütüphanedeki `HashMap` struct'ını bir ikili crate'in kapsamına dahil etmenin deyimsel yolunu gösterir.

<Listing number="7-14" file-name="src/main.rs" caption="`HashMap` struct'ını deyimsel biçimde kapsama dahil etme">

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

</Listing>

Bu kullanım geleneğinin arkasında teknik bir zorunluluk yoktur; yalnızca zaman içinde yerleşmiş bir uzlaşıdır ve geliştiriciler Rust kodunu bu biçimde okuyup yazmaya alışmıştır.

Bu geleneğin istisnası, `use` deyimleriyle aynı ada sahip iki öğeyi aynı kapsama dahil etmemiz gereken durumlardır; çünkü Rust aynı kapsamda isim çakışmasına izin vermez. Liste 7-15, aynı adı taşıyan ancak farklı üst modüllerde bulunan iki `Result` türünün kapsama nasıl dahil edileceğini ve bunlara nasıl başvurulacağını gösterir.

<Listing number="7-15" file-name="src/lib.rs" caption="Aynı ada sahip iki türü aynı kapsama dahil etmek, üst modüllerini kullanmayı gerektirir.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

</Listing>

Gördüğünüz gibi, üst modülleri kullanmak iki `Result` türünü birbirinden ayırt eder. Bunun yerine `use std::fmt::Result` ve `use std::io::Result` yazsaydık aynı kapsamda iki ayrı `Result` türümüz olurdu ve `Result` yazdığımızda Rust hangisini kastettiğimizi bilemezdi.

<!-- Old headings. Do not remove or links may break. -->

<a id="providing-new-names-with-the-as-keyword"></a>
<a id="as-anahtar-kelimesiyle-yeni-i̇simler-sağlamak"></a>
<a id="as-anahtar-kelimesiyle-yeni-isimler-sağlamak"></a>
<a id="as-anahtar-kelimesiyle-yeni-isimler-saglamak"></a>

### `as` Anahtar Kelimesiyle Yeni İsimler Verme

Aynı ada sahip iki türü `use` ile aynı kapsama dahil etme sorununun bir çözümü daha vardır: Yoldan sonra `as` anahtar kelimesini kullanarak türe yeni bir yerel ad, yani *takma ad (alias)* verebiliriz. Liste 7-16, iki `Result` türünden birini `as` ile yeniden adlandırarak Liste 7-15'teki kodu yazmanın alternatif yolunu gösterir.

<Listing number="7-16" file-name="src/lib.rs" caption="Kapsama dahil edilen bir türü `as` anahtar kelimesiyle yeniden adlandırma">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

</Listing>

İkinci `use` deyiminde `std::io::Result` türü için `IoResult` yeni adını seçtik; bu ad, aynı kapsama dahil ettiğimiz `std::fmt` içindeki `Result` ile çakışmayacaktır. Hem Liste 7-15 hem de Liste 7-16 deyimsel kabul edilir; dolayısıyla hangisini tercih edeceğiniz size kalmıştır!

<!-- Old headings. Do not remove or links may break. -->

<a id="re-exporting-names-with-pub-use"></a>
<a id="pub-use-ile-i̇simleri-yeniden-dışa-aktarmak-re-exporting"></a>
<a id="pub-use-ile-isimleri-yeniden-dışa-aktarmak-re-exporting"></a>
<a id="pub-use-ile-isimleri-yeniden-disa-aktarmak-re-exporting"></a>

### `pub use` ile İsimleri Yeniden Dışa Aktarma

Bir ismi `use` anahtar kelimesiyle kapsama dahil ettiğimizde bu isim, içe aktarıldığı kapsam açısından gizli kalır. O kapsamın dışındaki kodların da bu isme sanki doğrudan o kapsamda tanımlanmış gibi başvurabilmesini sağlamak için `pub` ve `use` anahtar kelimelerini birleştirebiliriz. Bu tekniğe *yeniden dışa aktarma (re-exporting)* denir; çünkü bir öğeyi kendi kapsamımıza dahil ederken aynı zamanda başkalarının da kendi kapsamlarına dahil edebileceği şekilde dışarıya açmış oluruz.

Liste 7-17, Liste 7-11'deki kodda kök modülde yer alan `use` deyiminin `pub use` olarak değiştirilmiş hâlini gösterir.

<Listing number="7-17" file-name="src/lib.rs" caption="`pub use` ile bir ismi yeni kapsam üzerinden dışarıdaki tüm kodların kullanımına açma">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

</Listing>

Bu değişiklikten önce harici kodların `bekleme_listesine_ekle` fonksiyonunu `restoran::restoran_on_kisim::karsilama::bekleme_listesine_ekle()` yolunu kullanarak çağırması gerekirdi; üstelik bunun çalışabilmesi için `restoran_on_kisim` modülünün de `pub` olarak işaretlenmesi şarttı. Kök modüldeki `pub use` deyimi `karsilama` modülünü yeniden dışa aktardığından, harici kodlar artık doğrudan `restoran::karsilama::bekleme_listesine_ekle()` yolunu kullanabilir.

Yeniden dışa aktarma, kodunuzun iç yapısı ile kodunuzu çağıran geliştiricilerin problem alanını (*domain*) düşünme biçimi birbirinden farklı olduğunda son derece yararlıdır. Örneğin bu restoran benzetmesinde işletmeciler işleri "ön kısım" ve "arka kısım" ayrımıyla düşünür; oysa müşteriler restoranın bölümlerini büyük olasılıkla bu kavramlarla düşünmez. `pub use` sayesinde kodumuzu içeride bir yapıyla geliştirebilir, dışarıya ise daha yalın ve farklı bir yapıyla sunabiliriz. Böylece kütüphanemiz hem üzerinde çalışan geliştiriciler hem de onu kullanan programcılar için iyi düzenlenmiş olur. Bölüm 14'teki ["Kullanışlı Bir Açık API Dışa Aktarmak"][ch14-pub-use]<!-- ignore --> başlığında bir başka `pub use` örneğini ve bunun crate dokümantasyonunu nasıl etkilediğini inceleyeceğiz.

> Not: Gerçek dünya Rust kütüphanelerinde yaygın bir mimari pratik, iç modülleri gizli (`mod restoran_on_kisim;` gibi `pub` olmadan) tutarak dosyaları mantıksal alt dizinlere ayırmak, dış kullanıcıların ihtiyaç duyacağı ana tür ve modülleri ise _src/lib.rs_ içinde `pub use` ile düz ve ergonomik bir hiyerarşiyle dışa açmaktır. Bu yaklaşım geriye dönük uyumluluğu da kolaylaştırır: İleride iç modülleri yeniden adlandırdığınızda veya farklı dosyalara taşıdığınızda, _src/lib.rs_ içindeki `pub use` satırlarını koruduğunuz sürece kütüphanenizi kullanan dış kodlar bu iç yeniden düzenlemeden hiç etkilenmez.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-external-packages"></a>
<a id="harici-external-paketleri-kullanmak"></a>

### Harici Paketleri Kullanma

Bölüm 2'de rastgele sayılar üretmek için `rand` adlı harici bir paketi kullanan bir tahmin oyunu projesi geliştirdik. `rand` paketini projemizde kullanmak için _Cargo.toml_ dosyamıza şu satırı eklemiştik:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:

* ch01-01-installation.md
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<Listing file-name="Cargo.toml">

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

</Listing>

_Cargo.toml_ dosyasına `rand` paketini bir bağımlılık olarak eklemek, Cargo'ya `rand` paketini ve onun ihtiyaç duyduğu bağımlılıkları [crates.io](https://crates.io/) adresinden indirip projemiz için kullanılabilir hâle getirmesini söyler.

Ardından `rand` tanımlarını paketimizin kapsamına dahil etmek için crate'in adı olan `rand` ile başlayan bir `use` satırı ekledik ve kapsama dahil etmek istediğimiz öğeleri belirttik. Bölüm 2'deki ["Rastgele Bir Sayı Üretmek"][rand]<!-- ignore --> kısmında `rand::prelude` modülündeki öğeleri kapsama dahil ettiğimizi ve `rand::rng` fonksiyonunu çağırdığımızı hatırlayın:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Rust topluluğunun üyeleri [crates.io](https://crates.io/) adresinde pek çok paketi kullanıma sunmuştur ve bunlardan herhangi birini kendi paketinize dahil etmek aynı adımları gerektirir: Paketi _Cargo.toml_ dosyanızda listelemek ve crate'indeki öğeleri kapsamınıza dahil etmek için `use` kullanmak.

Standart kütüphane olan `std`'nin de paketimiz açısından harici bir crate olduğuna dikkat edin. Standart kütüphane Rust diliyle birlikte geldiğinden `std`'yi dahil etmek için _Cargo.toml_ dosyamızı değiştirmemize gerek yoktur. Ancak oradaki öğeleri paketimizin kapsamına dahil etmek için yine de `use` ile başvurmamız gerekir. Örneğin `HashMap` için şu satırı kullanırız:

```rust
use std::collections::HashMap;
```

Bu, standart kütüphane crate'inin adı olan `std` ile başlayan mutlak bir yoldur.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-nested-paths-to-clean-up-large-use-lists"></a>
<a id="using-nested-paths-to-clean-up-use-lists"></a>
<a id="uzun-use-listelerini-temizlemek-i̇çin-i̇ç-i̇çe-nested-yollar-kullanmak"></a>
<a id="uzun-use-listelerini-temizlemek-için-iç-içe-nested-yollar-kullanmak"></a>
<a id="uzun-use-listelerini-temizlemek-icin-ic-ice-nested-yollar-kullanmak"></a>

### `use` Listelerini Sadeleştirmek İçin İç İçe Yollar Kullanma

Aynı crate'te ya da aynı modülde tanımlanmış birden çok öğeyi kullanıyorsak her bir öğeyi ayrı satırda listelemek dosyalarımızda dikey olarak çok yer kaplayabilir. Örneğin Liste 2-4'teki tahmin oyununda yer alan şu iki `use` deyimi `std` içinden öğeleri kapsama dahil eder:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

</Listing>

Bunun yerine aynı öğeleri tek satırda kapsama dahil etmek için *iç içe yollar (nested paths)* kullanabiliriz. Bunu Liste 7-18'de gösterildiği gibi yolun ortak kısmını, ardından çift iki nokta üst üste işaretini (`::`) ve yolların farklılaşan kısımlarını süslü parantezler içinde listeleyerek yaparız.

<Listing number="7-18" file-name="src/main.rs" caption="Aynı öneke sahip birden çok öğeyi kapsama dahil etmek için iç içe yol belirtme">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

</Listing>

Daha büyük programlarda aynı crate'ten ya da modülden pek çok öğeyi iç içe yollarla kapsama dahil etmek, gereken ayrı `use` deyimlerinin sayısını önemli ölçüde azaltabilir!

İç içe yolu bir yolun herhangi bir seviyesinde kullanabiliriz; bu, alt yollardan birini paylaşan iki `use` deyimini birleştirirken işe yarar. Örneğin Liste 7-19 iki `use` deyimi gösterir: Biri `std::io` modülünü, diğeri ise `std::io::Write` öğesini kapsama dahil eder.

<Listing number="7-19" file-name="src/lib.rs" caption="Biri diğerinin alt yolu olan iki `use` deyimi">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

</Listing>

Bu iki yolun ortak kısmı `std::io`'dur ve bu aynı zamanda birinci yolun tamamıdır. Bu iki yolu tek bir `use` deyiminde birleştirmek için Liste 7-20'de gösterildiği gibi iç içe yol içinde `self` anahtar kelimesini kullanabiliriz.

<Listing number="7-20" file-name="src/lib.rs" caption="Liste 7-19'daki yolları tek bir `use` deyiminde birleştirme">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

</Listing>

Bu satır hem `std::io` modülünü hem de `std::io::Write` öğesini kapsama dahil eder.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-glob-operator"></a>
<a id="importing-items-with-the-glob-operator"></a>
<a id="öğeleri-glob-operatörü-ile-i̇çe-aktarmak"></a>
<a id="öğeleri-glob-operatörü-ile-içe-aktarmak"></a>
<a id="ogeleri-glob-operatoru-ile-ice-aktarmak"></a>

### Glob Operatörü ile Öğeleri İçe Aktarma

Bir yolda tanımlı _tüm_ açık öğeleri kapsama dahil etmek istiyorsak o yolun ardından `*` glob operatörünü yazabiliriz:

```rust
use std::collections::*;
```

Bu `use` deyimi, `std::collections` içinde tanımlı tüm açık öğeleri geçerli kapsama dahil eder. Glob operatörünü kullanırken dikkatli olun! Glob kullanımı, hangi isimlerin kapsamda bulunduğunu ve programınızda geçen bir ismin nerede tanımlandığını anlamanızı zorlaştırabilir. Ayrıca kullandığınız bağımlılık kendi tanımlarını değiştirirse içe aktardığınız öğeler de değişir; örneğin bağımlılık, aynı kapsamda sizin tanımladığınız bir öğeyle aynı adı taşıyan yeni bir tanım eklerse bağımlılığı güncellediğinizde derleyici hatalarıyla karşılaşabilirsiniz.

Glob operatörü, test yazarken test edilecek her şeyi `tests` modülünün içine almak için sıkça kullanılır; bunu Bölüm 11'deki ["Nasıl Test Yazılır"][writing-tests]<!-- ignore --> kısmında ele alacağız. Glob operatörü bazen _prelude_ (ön tanımlı kapsam) deseninin bir parçası olarak da kullanılır: Bu desen hakkında daha fazla bilgi için [standart kütüphane dokümantasyonuna](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)<!-- ignore --> bakabilirsiniz.

> Not: Sıradan uygulama kodunda glob (`*`) operatöründen kaçınılmasının temel nedeni ad alanı kirliliği ve sürüm yükseltmelerinde yaşanabilecek sürpriz isim çakışmalarıdır; buna karşılık `prelude` ve `tests` modüllerinde durum farklıdır:
> - **`prelude` deseni**: Bir kütüphanenin `prelude` modülü (örneğin yukarıda kullandığımız `rand::prelude`), kütüphanedeki öğelerin tamamını değil, yazarların glob ile güvenle içe aktarılmak üzere özenle seçip yeniden dışa aktardığı (`pub use`) az sayıdaki temel trait ve türü barındırır. Özellikle metot çağrılarının çalışabilmesi için kapsama alınması gereken trait'leri (`RngExt` gibi) kullanıcıya tek satırda (`use rand::prelude::*;`) sunmak amacıyla tasarlanmıştır.
> - **`tests` modülü**: Birim testleri genellikle test ettikleri kodla aynı dosyanın içindeki `mod tests` alt modülünde yazılır ve `use super::*;` ile üst modüldeki tüm öğeleri (gizli olanlar dahil) tek seferde kapsama alır. Üst modül zaten sizin yazdığınız ve kontrol ettiğiniz kod olduğundan burada harici bağımlılık kaynaklı beklenmedik çakışma riski yoktur.

[ch14-pub-use]: ch14-02-crates-io-da-yayinlama.html#kullanışlı-bir-açık-api-dışa-aktarmak
[rand]: ch02-00-tahmin-oyunu-programlama.html#rastgele-bir-sayı-üretmek
[writing-tests]: ch11-01-nasil-test-yazilir.html#testler-nasıl-yazılır
