<!-- Old headings. Do not remove or links may break. -->

<a id="defining-modules-to-control-scope-and-privacy"></a>
<a id="control-scope-and-privacy-with-modules"></a>
<a id="modullerle-kapsam-ve-gizliligi-kontrol-etme"></a>

## Modüllerle Kapsam ve Gizliliği Kontrol Etme

Bu alt bölümde modüllerden ve modül sisteminin diğer parçalarından söz edeceğiz: öğeleri adlandırmanızı sağlayan _yollar (paths)_, bir yolu kapsama dahil eden `use` anahtar kelimesi ve öğeleri dışarıya açık hâle getiren `pub` anahtar kelimesi. Ayrıca `as` anahtar kelimesini, harici paketleri ve glob operatörünü de ele alacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="modules-cheat-sheet"></a>
<a id="modüller-kopya-kağıdı-cheat-sheet"></a>
<a id="moduller-kopya-kagidi-cheat-sheet"></a>

### Modüller Hızlı Başvuru Özeti

Ayrıntılara geçmeden önce modüllerin, yolların, `use` ve `pub` anahtar kelimelerinin derleyicide nasıl çalıştığına ve çoğu geliştiricinin kodunu nasıl düzenlediğine dair hızlı bir başvuru özeti sunuyoruz. Bu bölüm boyunca bu kuralların her birini örneklerle inceleyeceğiz; ancak modüllerin çalışma biçimini hatırlamak istediğinizde burası geri dönebileceğiniz harika bir başvuru noktasıdır.

- **Crate kökünden başlama**: Bir crate derlenirken derleyici, derlenecek kodu bulmak için önce crate kök dosyasına (kütüphane crate'leri için genellikle _src/lib.rs_, ikili crate'ler içinse _src/main.rs_) bakar.
- **Modül bildirme**: Crate kök dosyasında yeni modüller bildirebilirsiniz; örneğin `mod bahce;` bildirimiyle bir "bahce" modülü bildirdiğinizi varsayalım. Derleyici, bu modülün kodunu şu konumlarda arar:
  - Satır içinde: `mod bahce` bildiriminden sonra noktalı virgül yerine açılan süslü parantezlerin içinde
  - _src/bahce.rs_ dosyasında
  - _src/bahce/mod.rs_ dosyasında
- **Alt modül bildirme**: Crate kökü dışındaki herhangi bir dosyada alt modüller bildirebilirsiniz. Örneğin _src/bahce.rs_ içinde `mod sebzeler;` bildiriminde bulunabilirsiniz. Derleyici, alt modülün kodunu üst modülün adını taşıyan dizin altında şu konumlarda arar:
  - Satır içinde: doğrudan `mod sebzeler` bildiriminin ardından, noktalı virgül yerine açılan süslü parantezlerin içinde
  - _src/bahce/sebzeler.rs_ dosyasında
  - _src/bahce/sebzeler/mod.rs_ dosyasında
- **Modüllerdeki kodlara giden yollar**: Bir modül crate'inizin parçası hâline geldiğinde, gizlilik kuralları izin verdiği sürece aynı crate'in herhangi bir yerinden ilgili yolu kullanarak o modüldeki koda başvurabilirsiniz. Örneğin `bahce` altındaki `sebzeler` modülünde yer alan bir `Kuskonmaz` türüne `crate::bahce::sebzeler::Kuskonmaz` yoluyla ulaşılır.
- **Gizli ve açık erişim**: Bir modülün içindeki kod, varsayılan olarak üst modüllerine karşı *gizlidir (private)*. Bir modülü *açık (public)* hâle getirmek için `mod` yerine `pub mod` ile bildirin. Açık bir modülün içindeki öğeleri de dışarıya açmak için bildirimlerinin önüne `pub` ekleyin.
- **`use` anahtar kelimesi**: Bir kapsam içinde `use` anahtar kelimesi, uzun yolların tekrarını azaltmak amacıyla öğeler için kısayollar oluşturur. `crate::bahce::sebzeler::Kuskonmaz` yoluna başvurabilen herhangi bir kapsamda `use crate::bahce::sebzeler::Kuskonmaz;` ile bir kısayol tanımlayabilirsiniz; bundan sonra o kapsamda bu türü kullanmak için yalnızca `Kuskonmaz` yazmanız yeterlidir.

Burada, bu kuralları somutlaştırmak için `arka_bahce` adlı bir ikili crate oluşturuyoruz. Yine *arka_bahce* adını taşıyan crate dizini şu dosya ve dizinleri içerir:

```text
arka_bahce
├── Cargo.lock
├── Cargo.toml
└── src
    ├── bahce
    │   └── sebzeler.rs
    ├── bahce.rs
    └── main.rs
```

Bu örnekte crate kök dosyası _src/main.rs_ dosyasıdır ve içeriği şöyledir:

<Listing file-name="src/main.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

</Listing>

`pub mod bahce;` satırı, derleyiciye _src/bahce.rs_ dosyasında bulduğu kodu derlemeye dahil etmesini söyler. Bu dosyanın içeriği şöyledir:

<Listing file-name="src/bahce.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/bahce.rs}}
```

</Listing>

Buradaki `pub mod sebzeler;` bildirimi de _src/bahce/sebzeler.rs_ dosyasındaki kodun dahil edileceği anlamına gelir. O kod ise şöyledir:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/bahce/sebzeler.rs}}
```

Şimdi bu kuralların ayrıntılarına inelim ve pratikte nasıl çalıştıklarını görelim!

<!-- Old headings. Do not remove or links may break. -->

<a id="grouping-related-code-in-modules"></a>
<a id="ilgili-kodu-modullerde-gruplama"></a>

### İlgili Kodu Modüllerde Gruplama

_Modüller_, okunabilirlik ve kolay yeniden kullanım amacıyla bir crate içindeki kodu düzenlememize olanak tanır. Bir modülün içindeki kod varsayılan olarak gizli olduğundan, modüller öğelerin _gizliliğini (privacy)_ kontrol etmemizi de sağlar. Gizli öğeler, dışarıdan kullanıma kapalı olan iç uygulama ayrıntılarıdır. Modülleri ve içlerindeki öğeleri açık hâle getirmeyi seçebiliriz; böylece dışarıdaki kodların bunları kullanmasına ve bunlara bağımlı olmasına izin vermiş oluruz.

Örnek olarak bir restoranın işlevselliğini sunan bir kütüphane crate'i yazalım. Bir restoranın gerçek işleyişini kodlamaktan ziyade kodun organizasyonuna odaklanmak için fonksiyonların yalnızca imzalarını tanımlayacak, gövdelerini ise boş bırakacağız.

Restoran sektöründe bir işletmenin alanları genellikle *ön kısım* (müşteri salonu — *front of house*) ve *arka kısım* (mutfak ve geri plan — *back of house*) olarak ikiye ayrılır. _Ön kısım_, müşterilerin ağırlandığı alandır; karşılama görevlilerinin müşterileri masalara yerleştirdiği, garsonların sipariş ve ödeme aldığı, barmenlerin ise içecekleri hazırladığı bölümleri kapsar. _Arka kısım_ ise şef ve aşçıların mutfakta yemek pişirdiği, bulaşıkçıların temizlik yaptığı ve yöneticilerin idari işleri yürüttüğü bölümdür.

Crate'imizi bu düzene göre yapılandırmak için fonksiyonlarını iç içe geçmiş modüller hâlinde organize edebiliriz. `cargo new restoran --lib` komutunu çalıştırarak `restoran` adında yeni bir kütüphane oluşturun. Ardından bazı modülleri ve fonksiyon imzalarını tanımlamak için Liste 7-1'deki kodu _src/lib.rs_ dosyasına yazın; bu kod restoranın ön kısmını oluşturur.

<Listing number="7-1" file-name="src/lib.rs" caption="Fonksiyonları barındıran alt modüller içeren `restoran_on_kisim` modülü">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

</Listing>

Bir modülü, `mod` anahtar kelimesinin ardından modülün adını (bu örnekte `restoran_on_kisim`) yazarak tanımlarız. Modülün gövdesi ise süslü parantezler içine yazılır. Bir modülün içine, buradaki `karsilama` ve `servis` modüllerinde olduğu gibi başka modüller yerleştirebiliriz. Modüller ayrıca struct'lar, enum'lar, sabitler, trait'ler ve Liste 7-1'deki gibi fonksiyonlar da dahil olmak üzere diğer öğelerin tanımlarını barındırabilir.

Modülleri kullanarak birbiriyle ilişkili tanımları bir araya toplayabilir ve bu tanımların neden ilişkili olduğunu verdiğimiz modül adıyla ifade edebiliriz. Bu kodu kullanan geliştiriciler, tüm tanımları baştan sona okumak zorunda kalmadan gruplar üzerinden kodda gezinebilir ve aradıkları tanımları çok daha kolay bulabilirler. Koda yeni bir işlevsellik ekleyecek geliştiriciler de programın düzenini bozmamak için yeni kodu nereye yerleştirmeleri gerektiğini bilirler.

Daha önce _src/main.rs_ ve _src/lib.rs_ dosyalarının _crate kökü_ olarak adlandırıldığından bahsetmiştik. Bu adlandırmanın nedeni, bu iki dosyadan herhangi birinin içeriğinin, crate'in _modül ağacı (module tree)_ olarak bilinen modül yapısının en tepesinde `crate` adında bir modül oluşturmasıdır.

Liste 7-2, Liste 7-1'deki yapının modül ağacını göstermektedir.

<Listing number="7-2" caption="Liste 7-1'deki kod için modül ağacı">

```text
crate
 └── restoran_on_kisim
     ├── karsilama
     │   ├── bekleme_listesine_ekle
     │   └── masaya_oturt
     └── servis
         ├── siparis_al
         ├── siparis_servis_et
         └── odeme_al
```

</Listing>

Bu ağaç, bazı modüllerin diğer modüllerin içinde nasıl iç içe yer aldığını gösterir; örneğin `karsilama`, `restoran_on_kisim` modülünün içindedir. Ağaç ayrıca bazı modüllerin birbirinin _kardeşi (sibling)_ olduğunu, yani aynı modül içinde tanımlandığını da gösterir; `karsilama` ve `servis`, `restoran_on_kisim` içinde tanımlanmış kardeş modüllerdir. Bir A modülü B modülünün içindeyse; A'nın B modülünün _alt modülü_ (_çocuğu / child_), B'nin ise A modülünün _üst modülü_ (_ebeveyni / parent_) olduğunu söyleriz. Tüm modül ağacının, derleyici tarafından örtük olarak oluşturulan `crate` adlı kök modülün altında yer aldığına dikkat edin.

Modül ağacı size bilgisayarınızdaki dosya sisteminin dizin ağacını anımsatmış olabilir; bu çok yerinde bir benzetmedir! Tıpkı dosya sistemindeki dizinler gibi, kodunuzu düzenlemek için modülleri kullanırsınız. Yine bir dizindeki dosyalarda olduğu gibi, modüllerimizi bulmanın da bir yoluna ihtiyacımız vardır.

> Not: Modül ağacı dosya sistemindeki dizin ağacına benzese de ikisi arasında önemli bir fark vardır: Dizin ağacı dosyaların diskteki fiziksel yerleşimini gösterirken, modül ağacı tamamen `mod` bildirimleriyle kurulan mantıksal bir hiyerarşidir. Nitekim Liste 7-1'de tek bir _src/lib.rs_ dosyası içinde üç ayrı modül (`restoran_on_kisim`, `karsilama` ve `servis`) tanımladık; yani her modül için yeni bir dosya açmak zorunda değilsiniz ve tersine _src/_ dizinine eklediğiniz bir `.rs` dosyası da `mod` ile açıkça bildirilmedikçe modül ağacına dahil olmaz.
>
> Ayrıca Rust'ta modüllerin ve öğelerin *varsayılan olarak gizli (private by default)* olması bilinçli bir API tasarımı güvencesidir: Hiçbir iç uygulama ayrıntısı yanlışlıkla dış dünyaya sızmaz; böylece `pub` ile açıkça dışarıya açmadığınız her kodu, kullanıcılarınızın kodunu bozma endişesi taşımadan güvenle yeniden düzenleyebilirsiniz. Projeleriniz büyüdüğünde crate'inizin modül ağacını ve öğelerin gizlilik durumunu terminalde Liste 7-2'dekine benzer bir ağaç görünümünde incelemek isterseniz topluluk aracı olan `cargo-modules` eklentisini (`cargo install cargo-modules` kurulumunun ardından `cargo modules structure` komutuyla) kullanabilirsiniz.
