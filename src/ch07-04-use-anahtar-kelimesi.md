## `use` Anahtar Kelimesi ile Yolları Kapsama (Scope) Getirmek

Fonksiyonları çağırmak için yolları (paths) uzun uzun yazmak zorunda kalmak kullanışsız ve tekrarlayan bir iş gibi hissettirebilir. Liste 7-7'de, `bekleme_listesine_ekle` fonksiyonu için mutlak ya da göreceli yolu seçmiş olsak da, `bekleme_listesine_ekle`'yi her çağırmak istediğimizde `restoran_on_kisim` ve `karsilama`'yı da belirtmek zorundaydık. Neyse ki bu süreci basitleştirmenin bir yolu var: `use` anahtar kelimesiyle bir yolun kısayolunu bir kez oluşturabilir ve ardından daha kısa olan bu adı kapsamın diğer her yerinde kullanabiliriz.

Liste 7-11'de, `bekleme_listesine_ekle` fonksiyonunu `restoranda_yemek_ye` içinde çağırmak için yalnızca `karsilama::bekleme_listesine_ekle` belirtmemiz gerekecek şekilde, `crate::restoran_on_kisim::karsilama` modülünü `restoranda_yemek_ye` fonksiyonunun kapsamına dahil ediyoruz.

<Listing number="7-11" file-name="src/lib.rs" caption="Bir modülü `use` ile kapsama getirmek">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

</Listing>

Bir kapsama `use` ve bir yol eklemek, dosya sisteminde sembolik bir bağlantı (symbolic link) oluşturmaya benzer. Crate köküne `use crate::restoran_on_kisim::karsilama` ekleyerek, tıpkı `karsilama` modülü crate kökünde tanımlanmış gibi, `karsilama` artık o kapsamda geçerli bir isim haline gelir. `use` ile kapsama getirilen yollar da tıpkı diğer yollar gibi gizliliği kontrol eder.

`use`'un kısayolu yalnızca kullanıldığı belirli kapsam için oluşturduğuna dikkat edin. Liste 7-12, `restoranda_yemek_ye` fonksiyonunu `musteri` adlı yeni bir alt modüle taşır ve bu modül `use` ifadesinden farklı bir kapsam olduğu için fonksiyon gövdesi derlenmez.

<Listing number="7-12" file-name="src/lib.rs" caption="Bir `use` ifadesi yalnızca bulunduğu kapsamda geçerlidir.">

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

</Listing>

Derleyici hatası, kısayolun artık `musteri` modülü içinde geçerli olmadığını gösterir:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

Ayrıca `use` ifadesinin kendi kapsamında artık kullanılmadığına dair bir uyarı olduğuna da dikkat edin! Bu sorunu çözmek için `use` ifadesini de `musteri` modülünün içine taşıyın veya alt `musteri` modülünden, üst modüldeki kısayola `super::karsilama` ile başvurun.

### İdiomatik `use` Yolları Oluşturmak

Liste 7-11'de, neden `use crate::restoran_on_kisim::karsilama`'yı belirtip ardından `restoranda_yemek_ye` içinde `karsilama::bekleme_listesine_ekle`'yi çağırdığımızı merak etmiş olabilirsiniz; Liste 7-13'teki gibi aynı sonucu elde etmek için `use` yolunu neden ta `bekleme_listesine_ekle` fonksiyonuna kadar belirtmedik?

<Listing number="7-13" file-name="src/lib.rs" caption="`bekleme_listesine_ekle` fonksiyonunu idiomatik olmayan bir şekilde `use` ile kapsama getirmek">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

</Listing>

Liste 7-11 ve Liste 7-13 aynı işi başarsa da, bir fonksiyonu `use` ile kapsama getirmenin idiomatik (dile özgü doğru kullanım) yolu Liste 7-11'dir. Fonksiyonun ebeveyn modülünü `use` ile kapsama getirmek, fonksiyonu çağırırken o ebeveyn modülü belirtmemiz gerektiği anlamına gelir. Fonksiyonu çağırırken ebeveyn modülü belirtmek, bir yandan tam yolun tekrarlanmasını en aza indirirken diğer yandan fonksiyonun yerel olarak tanımlanmadığını da açıkça belli eder. Liste 7-13'teki kodda ise `bekleme_listesine_ekle` fonksiyonunun nerede tanımlandığı belirsizdir.

Öte yandan, struct'ları, enum'ları ve diğer öğeleri `use` ile içe aktarırken, tam yolu belirtmek idiomatiktir. Liste 7-14, standart kütüphanenin `HashMap` struct'ını bir ikili crate'in kapsamına almanın idiomatik yolunu gösterir.

<Listing number="7-14" file-name="src/main.rs" caption="`HashMap`'i idiomatik bir şekilde kapsama getirmek">

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

</Listing>

Bu idiomatik kullanımın arkasında çok güçlü bir neden yoktur: Yalnızca ortaya çıkmış bir gelenektir ve insanlar Rust kodunu bu şekilde okuyup yazmaya alışmışlardır.

Bu idiomun (geleneğin) istisnası, `use` ifadeleriyle aynı ada sahip iki öğeyi kapsama getiriyorsak geçerlidir; çünkü Rust buna izin vermez. Liste 7-15, aynı isme fakat farklı üst modüllere sahip iki `Result` türünün kapsama nasıl getirileceğini ve bunlara nasıl atıfta bulunulacağını gösterir.

<Listing number="7-15" file-name="src/lib.rs" caption="Aynı ada sahip iki türü aynı kapsama getirmek, üst modüllerinin kullanılmasını gerektirir.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

</Listing>

Gördüğünüz gibi, ebeveyn modülleri kullanmak iki `Result` türünü birbirinden ayırır. Bunun yerine `use std::fmt::Result` ve `use std::io::Result` ifadelerini belirtseydik, aynı kapsamda iki `Result` türümüz olurdu ve biz `Result` kelimesini kullandığımızda Rust hangisini kastettiğimizi bilemezdi.

### `as` Anahtar Kelimesiyle Yeni İsimler Sağlamak

Aynı ada sahip iki türü `use` ile aynı kapsama getirme probleminin başka bir çözümü daha vardır: Yolun ardından, tür için `as` ve yeni bir yerel ad veya _takma ad_ belirtebiliriz. Liste 7-16, Liste 7-15'teki kodu `as` kullanarak iki `Result` türünden birini yeniden adlandırmak suretiyle yazmanın başka bir yolunu gösterir.

<Listing number="7-16" file-name="src/lib.rs" caption="`as` anahtar kelimesi ile kapsama getirilen bir türü yeniden adlandırma">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

</Listing>

İkinci `use` ifadesinde `std::io::Result` türü için yeni `IoResult` adını seçtik; bu ad kapsama dahil ettiğimiz `std::fmt` içindeki `Result` ile çakışmayacaktır (conflict). Hem Liste 7-15 hem de Liste 7-16 idiomatik kabul edilir, dolayısıyla seçim size kalmış!

### `pub use` ile İsimleri Yeniden Dışa Aktarmak (Re-exporting)

Bir adı `use` anahtar kelimesi ile kapsama aldığımızda, bu ad onu içe aktardığımız (import) kapsam için gizli olur. O kapsamın dışındaki kodların bu isme, sanki o kapsamda tanımlanmış gibi atıfta bulunmasını (refer to) sağlamak için `pub` ve `use` kelimelerini birleştirebiliriz. Bu tekniğe _yeniden dışa aktarma_ denir çünkü biz bir öğeyi kendi kapsamımıza getirirken aynı zamanda diğerlerinin de o öğeyi kendi kapsamlarına getirebilmesi için onu kullanılabilir kılıyoruz.

Liste 7-17, Liste 7-11'deki kodda kök modüldeki (root module) `use` ifadesinin `pub use` olarak değiştirilmiş halini göstermektedir.

<Listing number="7-17" file-name="src/lib.rs" caption="`pub use` ile yeni bir kapsamdan herhangi bir kodun kullanabileceği bir ad yaratmak">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

</Listing>

Bu değişiklikten önce, harici kodların `bekleme_listesine_ekle` fonksiyonunu `restoran::restoran_on_kisim::karsilama::bekleme_listesine_ekle()` yolunu kullanarak çağırması gerekiyordu; ki bu da `restoran_on_kisim` modülünün `pub` olarak işaretlenmesini gerektirirdi. Şimdi bu `pub use`, kök modülden (root module) `karsilama` modülünü yeniden dışa aktardığı için, harici kodlar bunun yerine `restoran::karsilama::bekleme_listesine_ekle()` yolunu kullanabilir.

Yeniden dışa aktarma işlemi, kodunuzun iç yapısı (internal structure), kodunuzu çağıran programcıların alanı (domain) nasıl düşüneceğinden farklı olduğunda yararlıdır. Örneğin, bu restoran metaforunda, restoranı işleten kişiler "ön kısım" ve "arka kısım" hakkında düşünürler. Ancak bir restoranı ziyaret eden müşteriler büyük olasılıkla restoranın bölümleri hakkında bu terimlerle düşünmeyeceklerdir. `pub use` ile kodumuzu tek bir yapıyla yazabilir ancak farklı bir yapıyla açığa çıkarabiliriz. Bunu yapmak, kütüphanemizi hem kütüphane üzerinde çalışan programcılar hem de kütüphaneyi çağıran programcılar için iyi organize edilmiş hale getirir. Bölüm 14'teki ["Kullanışlı Bir Genel API'yi Dışa Aktarmak" (Exporting a Convenient Public API)][ch14-pub-use]<!-- ignore --> başlığında, `pub use` için başka bir örneğe ve bunun crate'inizin dokümantasyonunu nasıl etkilediğine bakacağız.

### Harici (External) Paketleri Kullanmak

Bölüm 2'de, rastgele sayılar elde etmek için `rand` adlı harici bir paketi kullanan bir tahmin oyunu projesi programladık. `rand` paketini projemizde kullanmak için, _Cargo.toml_ dosyamıza şu satırı ekledik:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<Listing file-name="Cargo.toml">

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

</Listing>

_Cargo.toml_ dosyasına `rand` paketini bir bağımlılık (dependency) olarak eklemek, Cargo'ya `rand` paketini ve tüm bağımlılıklarını [crates.io](https://crates.io/) adresinden indirip projemiz için erişilebilir kılmasını söyler.

Ardından, `rand` tanımlarını paketimizin kapsamına dahil etmek için, crate'in adı olan `rand` ile başlayan bir `use` satırı ekledik ve kapsama dahil etmek istediğimiz öğeleri listeledik. Hatırlarsanız Bölüm 2'deki ["Rastgele Bir Sayı Üretmek"][rand]<!-- ignore --> kısmında `Rng` trait'ini kapsama almış ve `rand::thread_rng` fonksiyonunu çağırmıştık:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Rust topluluğunun üyeleri [crates.io](https://crates.io/) adresinde pek çok paketi kullanıma sunmuştur ve bunlardan herhangi birini kendi paketinize çekmek de aynı adımları gerektirir: Onları paketinizin _Cargo.toml_ dosyasında listelemek ve içerdikleri öğeleri kendi kapsamınıza almak için `use` kullanmak.

Standart `std` kütüphanesinin de paketimiz için harici bir crate olduğuna dikkat edin. Standart kütüphane Rust diliyle birlikte geldiğinden (shipped), _Cargo.toml_ dosyamızı `std`'yi içerecek şekilde değiştirmemize gerek yoktur. Ancak yine de oradaki öğeleri paketimizin kapsamına dahil etmek için ona `use` ile başvurmamız gerekir. Örneğin, `HashMap` ile şu satırı kullanırdık:

```rust
use std::collections::HashMap;
```

Bu, standart kütüphane crate'inin adı olan `std` ile başlayan mutlak (absolute) bir yoldur.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-nested-paths-to-clean-up-large-use-lists"></a>

### Uzun `use` Listelerini Temizlemek İçin İç İçe (Nested) Yollar Kullanmak

Eğer aynı crate'te ya da aynı modülde tanımlanmış birden çok öğeyi kullanıyorsak, her bir öğeyi kendi satırında listelemek dosyalarımızda dikey olarak çok yer kaplayabilir. Örneğin, Liste 2-4'teki tahmin oyununda yer alan bu iki `use` ifadesi, öğeleri `std`'den kapsama getirmektedir:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

</Listing>

Bunun yerine, aynı öğeleri tek bir satırda kapsama getirmek için iç içe yollar (nested paths) kullanabiliriz. Bunu, Liste 7-18'de gösterildiği gibi yolun ortak kısmını, ardından iki nokta üst üste işaretini (::) ve yolların farklılaşan kısımlarının bir listesini süslü parantezler içine alarak yaparız.

<Listing number="7-18" file-name="src/main.rs" caption="Aynı öneke (prefix) sahip birden çok öğeyi kapsama almak için iç içe bir yol belirtmek">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

</Listing>

Büyük programlarda, aynı crate'ten ya da modülden birçok öğeyi iç içe yollar (nested paths) kullanarak kapsama getirmek, ihtiyaç duyulan ayrı `use` ifadelerinin sayısını büyük ölçüde azaltabilir!

Bir yolda herhangi bir seviyede iç içe yol kullanabiliriz; bu, ortak bir alt yolu (subpath) paylaşan iki `use` ifadesini birleştirirken yararlıdır. Örneğin, Liste 7-19 iki adet `use` ifadesi göstermektedir: biri `std::io`'yu kapsama getirirken, diğeri `std::io::Write`'ı kapsama getirir.

<Listing number="7-19" file-name="src/lib.rs" caption="Birinin diğerinin alt yolu (subpath) olduğu iki `use` ifadesi">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

</Listing>

Bu iki yolun ortak noktası `std::io`'dur ve bu tamamen ilk yola karşılık gelir. Bu iki yolu tek bir `use` ifadesinde birleştirmek için, Liste 7-20'de gösterildiği gibi iç içe yolda `self` kullanabiliriz.

<Listing number="7-20" file-name="src/lib.rs" caption="Liste 7-19'daki yolları tek bir `use` ifadesinde birleştirmek">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

</Listing>

Bu satır `std::io` ve `std::io::Write`'ı kapsama alır.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-glob-operator"></a>

### Öğeleri Glob Operatörü ile İçe Aktarmak

Eğer bir yolda tanımlı _tüm_ açık öğeleri kapsama almak istiyorsak, o yolu belirtebilir ve ardından `*` glob operatörünü koyabiliriz:

```rust
use std::collections::*;
```

Bu `use` ifadesi, `std::collections` içinde tanımlı tüm açık öğeleri geçerli kapsama getirir. Glob operatörünü kullanırken dikkatli olun! Glob, hangi isimlerin kapsamda olduğunu ve programınızda kullanılan bir ismin nerede tanımlandığını anlamanızı zorlaştırabilir. Ayrıca, bağımlılık (dependency) kendi tanımlarını değiştirirse, içe aktardığınız (imported) şeyler de değişir; bu da örneğin, bağımlılık, aynı kapsamdaki sizin bir tanımınızla aynı isme sahip bir tanım eklerse, bağımlılığı güncellediğinizde derleyici hatalarına yol açabilir.

Glob operatörü, test yazarken test edilecek her şeyi `tests` modülü içine almak için sıkça kullanılır; bu konuyu Bölüm 11'deki ["Nasıl Test Yazılır?"][writing-tests]<!-- ignore --> kısmında ele alacağız. Glob operatörü bazen _prelude_ (ön hazırlık/başlangıç) kalıbının (pattern) bir parçası olarak da kullanılır: Bu kalıp hakkında daha fazla bilgi edinmek için [standart kütüphane dokümantasyonuna](../std/prelude/index.html#other-preludes)<!-- ignore --> bakabilirsiniz.

[ch14-pub-use]: ch14-02-crates-io-da-yayinlama.html#kullanışlı-bir-açık-api-dışa-aktarmak
[rand]: ch02-00-tahmin-oyunu-programlama.html#rastgele-bir-sayı-üretmek
[writing-tests]: ch11-01-nasil-test-yazilir.html#testler-nasıl-yazılır
