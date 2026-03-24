<!-- Old headings. Do not remove or links may break. -->

<a id="defining-modules-to-control-scope-and-privacy"></a>

## Modüllerle Kapsam ve Gizliliği Kontrol Etme

Bu bölümde modüllerden ve modül sisteminin diğer parçalarından, yani öğeleri isimlendirmenizi sağlayan _yollardan_ (paths), bir yolu kapsama getiren `use` anahtar kelimesinden ve öğeleri herkese açık hale getiren `pub` anahtar kelimesinden bahsedeceğiz. Ayrıca `as` anahtar kelimesini, harici paketleri ve glob operatörünü de ele alacağız.

### Modüller Kopya Kağıdı (Cheat Sheet)

Modüllerin ve yolların detaylarına girmeden önce, burada modüllerin, yolların, `use` anahtar kelimesinin ve `pub` anahtar kelimesinin derleyicide nasıl çalıştığına ve çoğu geliştiricinin kodlarını nasıl düzenlediğine dair hızlı bir başvuru kılavuzu (quick reference) sunuyoruz. Bu bölüm boyunca bu kuralların her birini örneklerle inceleyeceğiz, ancak modüllerin nasıl çalıştığını hatırlamak için burası harika bir başvuru noktasıdır.

- **Crate kökünden başlayın**: Bir crate'i derlerken, derleyici önce derlenecek kodu bulmak için crate kök dosyasına (genellikle bir kütüphane crate'i için _src/lib.rs_ ve bir ikili crate için _src/main.rs_) bakar.
- **Modülleri bildirmek (declaring modules)**: Crate kök dosyasında yeni modüller bildirebilirsiniz; diyelim ki `mod bahce;` ile bir "bahce" modülü bildirdiniz. Derleyici, bu modülün kodunu şu konumlarda arayacaktır:
  - Satır içi, `mod bahce`'den sonraki noktalı virgül yerine konulan süslü parantezlerin içinde
  - _src/bahce.rs_ dosyasında
  - _src/bahce/mod.rs_ dosyasında
- **Alt modülleri bildirmek (declaring submodules)**: Crate kökü dışındaki herhangi bir dosyada alt modüller bildirebilirsiniz. Örneğin, _src/bahce.rs_ içinde `mod sebzeler;` bildirebilirsiniz. Derleyici, alt modülün kodunu ana modülün adını taşıyan dizinde şu konumlarda arayacaktır:
  - Satır içi, doğrudan `mod sebzeler`'den sonra, noktalı virgül yerine süslü parantezlerin içinde
  - _src/bahce/sebzeler.rs_ dosyasında
  - _src/bahce/sebzeler/mod.rs_ dosyasında
- **Modüllerdeki kodların yolları**: Bir modül crate'inizin bir parçası olduğunda, gizlilik kuralları izin verdiği sürece, kodun yolunu kullanarak o crate'in başka bir yerinden o modüldeki koda başvurabilirsiniz. Örneğin, bahce sebzeler modülündeki bir `Kuskonmaz` (Asparagus) türü `crate::bahce::sebzeler::Kuskonmaz` adresinde (yolunda) bulunacaktır.
- **Gizli ve açık**: Bir modül içindeki kod varsayılan olarak üst modüllerinden gizlidir. Bir modülü açık hale getirmek için `mod` yerine `pub mod` ile bildirin. Açık bir modül içindeki öğeleri de açık hale getirmek için bildirimlerinden (declaration) önce `pub` kullanın.
- **`use` anahtar kelimesi**: Bir kapsam içinde, `use` anahtar kelimesi uzun yolların tekrarını azaltmak için öğelere kısayollar oluşturur. `crate::bahce::sebzeler::Kuskonmaz`'a başvurabilen herhangi bir kapsamda, `use crate::bahce::sebzeler::Kuskonmaz;` ile bir kısayol oluşturabilirsiniz; ve o andan itibaren bu türü o kapsamda kullanmak için yalnızca `Kuskonmaz` yazmanız yeterlidir.

Burada, bu kuralları gösteren `arka_bahce` (backyard) adlı bir ikili crate oluşturuyoruz. Crate'in yine _arka_bahce_ adını taşıyan dizini, şu dosyaları ve dizinleri içerir:

```text
arka_bahce
├── Cargo.lock
├── Cargo.toml
└── src
    ├── bahce
    │   └── sebzeler.rs
    ├── bahce.rs
    └── main.rs
```

Bu durumda crate kök dosyası _src/main.rs_'dir ve şunları içerir:

<Listing file-name="src/main.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

</Listing>

`pub mod bahce;` satırı, derleyiciye _src/bahce.rs_ dosyasında bulduğu kodu dahil etmesini söyler:

<Listing file-name="src/bahce.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/bahce.rs}}
```

</Listing>

Burada, `pub mod sebzeler;`, _src/bahce/sebzeler.rs_ dosyasındaki kodun da dahil edildiği anlamına gelir. O kod da şudur:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/bahce/sebzeler.rs}}
```

Şimdi bu kuralların detaylarına girelim ve pratikte nasıl uygulandıklarını gösterelim!

### İlgili Kodu Modüllerde Gruplama

_Modüller_, okunabilirlik ve kolay yeniden kullanım için bir crate içindeki kodu düzenlememize olanak tanır. Modüller aynı zamanda öğelerin _gizliliğini_ kontrol etmemizi sağlar çünkü bir modül içindeki kod varsayılan olarak gizlidir. Gizli öğeler, dışarıdan kullanıma açık olmayan dahili (internal) uygulama ayrıntılarıdır. Modülleri ve içlerindeki öğeleri açık hale getirmeyi seçebiliriz, bu da onları açığa çıkararak harici kodların onları kullanmasına ve onlara bağımlı olmasına olanak tanır.

Örnek olarak, bir restoranın işlevselliğini sağlayan bir kütüphane (library) crate'i yazalım. Fonksiyonların imzalarını tanımlayacağız, ancak bir restoranın uygulanmasından ziyade kodun organizasyonuna odaklanmak için gövdelerini boş bırakacağız.

Restoran sektöründe, bir restoranın bazı bölümleri ön kısım (front of house), diğerleri ise arka kısım (back of house) olarak adlandırılır. _Ön kısım_ müşterilerin bulunduğu yerdir; burası karşılamanın (hosts) müşterileri oturttuğu, servis elemanlarının (servers) siparişleri ve ödemeleri aldığı ve barmenlerin içecekleri hazırladığı yeri kapsar. _Arka kısım_ ise şeflerin ve aşçıların mutfakta çalıştığı, bulaşıkçıların temizlik yaptığı ve yöneticilerin idari işleri yürüttüğü yerdir.

Crate'imizi bu şekilde yapılandırmak için, fonksiyonlarını iç içe geçmiş modüller halinde düzenleyebiliriz. `cargo new restoran --lib` komutunu çalıştırarak `restoran` adında yeni bir kütüphane oluşturun. Ardından, bazı modülleri ve fonksiyon imzalarını tanımlamak için Liste 7-1'deki kodu _src/lib.rs_ dosyasına girin; bu kod restoranın ön kısmını temsil eder.

<Listing number="7-1" file-name="src/lib.rs" caption="İçinde fonksiyonlar barındıran başka modüller içeren bir `restoran_on_kisim` modülü">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

</Listing>

`mod` anahtar kelimesini ve ardından modülün adını (bu durumda `restoran_on_kisim`) kullanarak bir modül tanımlarız. Modülün gövdesi daha sonra süslü parantezlerin içine girer. Modüllerin içine, bu durumda `karsilama` ve `servis` modüllerinde olduğu gibi başka modüller yerleştirebiliriz. Modüller ayrıca struct'lar, enum'lar, sabitler (constants), trait'ler ve Liste 7-1'de olduğu gibi fonksiyonlar gibi diğer öğeler için tanımlar da barındırabilir.

Modülleri kullanarak, birbiriyle ilişkili tanımları bir arada gruplayabilir ve neden ilişkili olduklarını isimlendirebiliriz. Bu kodu kullanan programcılar, tüm tanımları okumak zorunda kalmadan, gruplara göre kod içinde gezinebilirler ve bu da onlarla alakalı tanımları bulmalarını kolaylaştırır. Bu koda yeni işlevsellik ekleyen programcılar, programın düzenli kalması için kodu nereye yerleştireceklerini bileceklerdir.

Daha önce, _src/main.rs_ ve _src/lib.rs_ dosyalarının _crate kökleri_ (crate roots) olarak adlandırıldığından bahsetmiştik. Bu şekilde adlandırılmalarının nedeni, bu iki dosyadan herhangi birinin içeriğinin, crate'in modül yapısının kökünde (bu yapı _modül ağacı_ - _module tree_ olarak bilinir) `crate` adında bir modül oluşturmasıdır.

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

Bu ağaç, bazı modüllerin diğer modüllerin içine nasıl yuvalandığını (iç içe geçtiğini) gösterir; örneğin, `karsilama`, `restoran_on_kisim`'in içine yuvalanmıştır. Ağaç ayrıca bazı modüllerin _kardeş_ (siblings) olduğunu, yani aynı modül içinde tanımlandıklarını gösterir; `karsilama` ve `servis`, `restoran_on_kisim` içinde tanımlanmış kardeş modüllerdir. Eğer A modülü B modülünün içindeyse, A modülünün B modülünün _çocuğu_ ve B modülünün A modülünün _ebeveyni_ olduğunu söyleriz. Tüm modül ağacının, örtük (implicit) `crate` adlı modülün altında köklendiğine (rooted) dikkat edin.

Modül ağacı, bilgisayarınızdaki dosya sisteminin dizin ağacını hatırlatabilir; bu çok yerinde bir karşılaştırmadır! Tıpkı dosya sistemindeki dizinler gibi, kodunuzu düzenlemek için de modülleri kullanırsınız. Ve bir dizindeki dosyalar gibi, modüllerimizi bulmanın da bir yoluna ihtiyacımız var.
