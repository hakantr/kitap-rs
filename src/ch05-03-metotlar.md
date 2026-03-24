## Metotlar (Methods)

Metotlar fonksiyonlara oldukça benzer: Onları da `fn` anahtar kelimesi ve bir isimle tanımlarız, parametre alabilirler, bir değer döndürebilirler ve başka bir yerden çağrıldıklarında çalıştırılacak bazı kodlar barındırırlar. Ancak fonksiyonlardan farklı olarak metotlar, bir yapının (struct) (veya [Bölüm 6][enums]<!-- ignore --> ve [Bölüm 18][trait-objects]<!-- ignore -->'de göreceğimiz üzere bir `enum` veya `trait` nesnesinin) bağlamı içinde tanımlanırlar. Metotların ilk parametresi her zaman `self` olmalıdır; bu parametre, metodun üzerinde çağrıldığı yapı (struct) örneğini temsil eder.

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-methods"></a>

### Metot Sözdizimi (Method Syntax)

Hadi parametre olarak bir `Dikdortgen` örneği alan eski `alan` fonksiyonumuzu değiştirelim ve bunun yerine Liste 5-13'te gösterildiği gibi `Dikdortgen` yapısı (struct) üzerinde tanımlı bir `alan` metodu oluşturalım.

<Listing number="5-13" file-name="src/main.rs" caption="`Dikdortgen` struct'ı üzerinde bir `alan` metodu tanımlanması">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

</Listing>

Fonksiyonu `Dikdortgen` bağlamında tanımlamak için, `Dikdortgen` adına bir `impl` (uygulama/implementation) bloğu başlatıyoruz. Bu `impl` bloğu içindeki her şey `Dikdortgen` tipiyle ilişkilendirilecektir. Daha sonra `alan` fonksiyonunu `impl` süslü parantezlerinin içine taşıyoruz ve imzadaki ilk (ve bu durumda tek) parametreyi hem tanımda hem de gövdede `self` olacak şekilde değiştiriyoruz. Önceden `alan` fonksiyonunu çağırıp `dikdortgen1`'i argüman olarak ilettiğimiz `main` fonksiyonunun içinde, artık `Dikdortgen` örneğimiz üzerinde `alan` metodunu çağırmak için *metot sözdizimini* (method syntax) kullanabiliriz. Metot sözdizimi doğrudan örneğin adından sonra gelir: Bir nokta ekleriz, ardından metodun adını, parantezleri ve varsa argümanları yazarız.

`alan` metodunun imzasında `dikdortgen: &Dikdortgen` yerine `&self` kullanıyoruz. Buradaki `&self`, aslında `self: &Self` ifadesinin kısaltmasıdır. Bir `impl` bloğu içerisinde `Self` tipi, o `impl` bloğunun ait olduğu tipin kısa adıdır. Metotların ilk parametresinin adı `self` ve tipi `Self` olmak zorundadır; bu nedenle Rust, ilk parametre alanında bunu sadece `self` ismiyle kısaltmanıza olanak tanır. Ancak tıpkı `dikdortgen: &Dikdortgen` kullanımında olduğu gibi, bu metodun `Self` örneğini ödünç aldığını belirtmek için `self` kısaltmasının önüne `&` işareti koymamız gerektiğini unutmayın. Metotlar; aynen diğer parametrelerde olduğu gibi `self`'in sahipliğini alabilir, burada yaptığımız gibi `self`'i değiştirilemez şekilde ödünç alabilir veya `self`'i değiştirilebilir şekilde ödünç alabilirler.

Fonksiyon versiyonunda neden `&Dikdortgen` kullandıysak, burada da aynı nedenden ötürü `&self` kullandık: Sahipliği almak istemiyoruz; yalnızca yapıdaki verileri okumak istiyoruz, onlara yazmak istemiyoruz. Eğer metodun, çağrıldığı örneği değiştirmesini isteseydik, ilk parametre olarak `&mut self` kullanırdık. İlk parametre olarak sadece `self` kullanarak örneğin sahipliğini tamamen alan bir metot yazmak nadir görülen bir durumdur; bu teknik genellikle metot `self`'i başka bir şeye dönüştürdüğünde ve dönüşümden sonra çağıran kişinin orijinal örneği kullanmasını engellemek istediğinizde kullanılır.

Metot sözdiziminin sağladığı kolaylıklar ve her metodun imzasında `self` tipini tekrar etme zorunluluğunun ortadan kalkmasının yanı sıra, fonksiyonlar yerine metotları kullanmanın asıl ana nedeni kod organizasyonudur. Bir tipin örneğiyle yapabileceğimiz her şeyi tek bir `impl` bloğunda topladık. Böylece kodumuzu kullanacak kişilerin, `Dikdortgen`'in yeteneklerini kütüphanemizin çeşitli yerlerinde aramak zorunda kalmalarını engellemiş olduk.

Ayrıca, bir metoda yapının (struct) alanlarından (field) biriyle aynı ismi vermeyi de seçebiliriz. Örneğin, `Dikdortgen` üzerinde aynı zamanda `genislik` adında bir metot tanımlayabiliriz:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

</Listing>

Burada `genislik` metodunun, örnekteki `genislik` alanının değeri `0`'dan büyükse `true`, `0` ise `false` döndürmesini istedik: Kısacası, bir alanı aynı isimli bir metot içinde herhangi bir amaçla kullanabiliriz. `main` fonksiyonunda, `dikdortgen1.genislik` ifadesinden sonra parantez kullandığımızda, Rust bizim `genislik` *metodunu* kastettiğimizi anlar. Parantez kullanmadığımızda ise Rust bizim `genislik` *alanını* (field) kastettiğimizi bilir.

Sıklıkla (ama her zaman değil), bir metoda yapıdaki alanla aynı ismi verdiğimizde, bu metodun yalnızca o alandaki değeri döndürmesini ve başka hiçbir şey yapmamasını isteriz. Bu tür metotlara *alıcılar* (getters) denir. Bazı diğer dillerin aksine Rust, yapı alanları için alıcıları (getter) otomatik olarak uygulamaz. Alıcılar faydalıdır, çünkü alanı gizli (private), ancak metodu açık yapabilirsiniz. Böylece bu alana sadece okunabilir (read-only) bir erişim sağlayarak onu tipin genel API'sinin bir parçası haline getirebilirsiniz. "Açık" ve "gizli" (private) kavramlarının ne olduğunu ve bir alanın veya metodun nasıl bu şekilde tasarlanacağını Bölüm 7'de tartışacağız.

> ### `->` Operatörüne Ne Oldu?
>
> C ve C++ dillerinde metot çağırmak için iki farklı operatör kullanılır: Eğer metodu doğrudan obje üzerinde çağırıyorsanız `.`, eğer metodu objeye işaret eden bir işaretçi (pointer) üzerinden çağırıyorsanız ve önce işaretçinin referansını kaldırmanız (dereference) gerekiyorsa `->` kullanırsınız. Diğer bir deyişle, eğer `obje` bir işaretçiyse, `obje->bir_seyler_yap()` ifadesi aslında `(*obje).bir_seyler_yap()` ifadesine benzer.
>
> Rust'ta `->` operatörünün doğrudan bir karşılığı yoktur; bunun yerine Rust'ta *otomatik referans alma ve kaldırma (automatic referencing and dereferencing)* adı verilen bir özellik vardır. Metot çağırma işlemi, Rust'ta bu davranışın görüldüğü nadir yerlerden biridir.
>
> Sistem şu şekilde işler: Siz `obje.bir_seyler_yap()` şeklinde bir metot çağırdığınızda, Rust otomatik olarak `&`, `&mut` veya `*` işaretlerini ekleyerek `obje`'nin metodun imzasına uymasını sağlar. Yani, aşağıdaki iki ifade aslında tamamen aynıdır:
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Nokta {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Nokta {
> #    fn mesafe(&self, diger: &Nokta) -> f64 {
> #        let x_karesi = f64::powi(diger.x - self.x, 2);
> #        let y_karesi = f64::powi(diger.y - self.y, 2);
> #
> #        f64::sqrt(x_karesi + y_karesi)
> #    }
> # }
> # let n1 = Nokta { x: 0.0, y: 0.0 };
> # let n2 = Nokta { x: 5.0, y: 6.5 };
> n1.mesafe(&n2);
> (&n1).mesafe(&n2);
> ```
>
> İlk ifade göze çok daha temiz görünür. Bu otomatik referans alma davranışı sorunsuz çalışır çünkü metotların net bir alıcısı (receiver) vardır: `self`'in tipi. Metodun ismine ve alıcısına bakan Rust, bu metodun okuma (`&self`), değiştirme (`&mut self`) veya tüketme (`self`) işlemi yapıp yapmadığını kesin olarak anlayabilir. Rust'ın, metot alıcıları için ödünç almayı örtülü (implicit) hale getirmesi, sahiplik kurallarının pratikte çok daha ergonomik kullanılabilmesinin büyük bir parçasıdır.

### Daha Fazla Parametre Alan Metotlar

Şimdi `Dikdortgen` yapımız üzerinde ikinci bir metot yazarak pratik yapalım. Bu kez, bir `Dikdortgen` örneğinin başka bir `Dikdortgen` örneğini parametre olarak almasını istiyoruz. Eğer ikinci `Dikdortgen`, tamamen `self`'in (yani birinci `Dikdortgen`'in) içine sığabiliyorsa `true`, sığamıyorsa `false` döndürmeli. Yani, `kapsayabilir_mi` metodunu tanımladığımızda, Liste 5-14'te gösterilen programı yazabilmek istiyoruz.

<Listing number="5-14" file-name="src/main.rs" caption="Henüz yazılmamış olan `kapsayabilir_mi` metodunun kullanımı">

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

</Listing>

Beklenen çıktı aşağıdaki gibi olmalıdır; çünkü `dikdortgen2`'nin her iki boyutu da `dikdortgen1`'den küçüktür, ancak `dikdortgen3` `dikdortgen1`'den daha geniştir:

```text
dikdortgen1, dikdortgen2'yi kapsayabilir mi? true
dikdortgen1, dikdortgen3'ü kapsayabilir mi? false
```

Bir metot tanımlamak istediğimizi biliyoruz, bu yüzden bunu `impl Dikdortgen` bloğunun içine yazacağız. Metodun adı `kapsayabilir_mi` olacak ve parametre olarak başka bir `Dikdortgen`'in değiştirilemez referansını alacak. Parametrenin tipinin ne olması gerektiğini, metodu çağıran koda bakarak da anlayabiliriz: `dikdortgen1.kapsayabilir_mi(&dikdortgen2)` satırı argüman olarak `&dikdortgen2`'yi iletiyor, ki bu da bir `Dikdortgen` örneği olan `dikdortgen2`'nin değiştirilemez bir ödünç alımıdır. Bu oldukça mantıklı; çünkü sadece `dikdortgen2`'yi okumaya ihtiyacımız var (eğer yazmaya/değiştirmeye ihtiyacımız olsaydı değiştirilebilir -mutable- bir referans alırdık) ve `kapsayabilir_mi` metodunu çağırdıktan sonra `dikdortgen2`'yi tekrar kullanabilmek için `main` fonksiyonunun onun sahipliğini elinde tutmasını istiyoruz.

`kapsayabilir_mi` metodunun dönüş değeri bir Boolean (mantıksal değer) olacak ve metodun gövdesinde `self`'in genişlik ve yüksekliğinin, diğer `Dikdortgen`'in genişlik ve yüksekliğinden büyük olup olmadığını kontrol edeceğiz. Hadi Liste 5-13'teki `impl` bloğumuza yeni `kapsayabilir_mi` metodunu ekleyelim (Liste 5-15).

<Listing number="5-15" file-name="src/main.rs" caption="`Dikdortgen` üzerinde başka bir `Dikdortgen` örneğini parametre olarak alan `kapsayabilir_mi` metodunun uygulanması">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

</Listing>

Bu kodu Liste 5-14'teki `main` fonksiyonu ile birlikte çalıştırdığımızda istediğimiz çıktıyı alacağız. Metotlar, `self` parametresinden sonra eklediğimiz birden fazla parametreyi alabilirler ve bu parametreler tıpkı standart fonksiyonlardaki parametreler gibi çalışır.

### İlişkili Fonksiyonlar (Associated Functions)

Bir `impl` bloğu içinde tanımlanan tüm fonksiyonlara *ilişkili fonksiyonlar* (associated functions) denir; çünkü bu fonksiyonlar `impl` kelimesinden sonra adı yazılan tiple ilişkilendirilmişlerdir. İlk parametresi `self` olmayan (ve dolayısıyla metot olmayan) ilişkili fonksiyonlar da tanımlayabiliriz. Bunlar metot değildir çünkü çalışmak için ilgili tipin bir örneğine ihtiyaç duymazlar. Biz zaten bu tarz bir fonksiyonu daha önce kullandık: `String` tipi üzerinde tanımlı olan `String::from` fonksiyonu.

Metot olmayan ilişkili fonksiyonlar genellikle, yapının yeni bir örneğini döndürecek "kurucular" (constructors) olarak kullanılırlar. Bunlara çoğunlukla `new` adı verilir, ancak `new` dilde yerleşik olarak bulunan veya özel bir anlam taşıyan bir kelime değildir. Örneğin, genişlik ve yükseklik için aynı değeri kullanan ve böylece bir kare oluşturmayı kolaylaştıran `kare` adında bir ilişkili fonksiyon tanımlayabiliriz. Bu sayede aynı değeri iki kez girmek zorunda kalmayız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

Dönüş tipindeki ve fonksiyon gövdesindeki `Self` anahtar kelimesi, `impl` anahtar kelimesinden sonra görünen tip için bir kısaltmadır; yani bu durumda `Dikdortgen`'i temsil eder.

Bu ilişkili fonksiyonu çağırmak için yapının (struct) adıyla birlikte `::` sözdizimini kullanırız; `let kare_obje = Dikdortgen::kare(3);` buna güzel bir örnektir. Bu fonksiyon, yapı tarafından isim alanına (namespace) dahil edilmiştir: `::` sözdizimi hem ilişkili fonksiyonlar hem de modüllerin oluşturduğu isim alanları (namespaces) için kullanılır. Modülleri [Bölüm 7][modules]<!-- ignore -->'de detaylıca tartışacağız.

### Birden Fazla `impl` Bloğu

Her yapının (struct) birden fazla `impl` bloğuna sahip olmasına izin verilir. Örneğin, Liste 5-15, her metodun kendi ayrı `impl` bloğunda yer aldığı Liste 5-16'daki kodla tamamen eşdeğerdir.

<Listing number="5-16" caption="Liste 5-15'in birden fazla `impl` bloğu kullanılarak yeniden yazılması">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

</Listing>

Burada metotları birden fazla `impl` bloğuna ayırmak için geçerli bir nedenimiz olmasa da, bu tamamen geçerli bir sözdizimidir. Jenerik tipleri ve özellikleri (traits) tartışacağımız Bölüm 10'da, birden fazla `impl` bloğunun oldukça faydalı olduğu durumları göreceğiz.

## Özet

Yapılar (Structs), çalıştığınız problem alanına (domain) uygun ve anlamlı özel tipler oluşturmanızı sağlar. Yapıları kullanarak, birbiriyle ilişkili veri parçalarını birbirine bağlı tutabilir ve kodunuzu netleştirmek için her bir parçaya anlamlı bir isim verebilirsiniz. `impl` bloklarında ise tipinizle ilişkili fonksiyonları tanımlayabilirsiniz. Metotlar da bir tür ilişkili fonksiyondur ve yapı örneklerinizin sergileyeceği davranışları (behavior) belirlemenize olanak tanır.

Ancak özel tipler (custom types) oluşturmanın tek yolu yapılar (structs) değildir: Şimdi araç çantanıza yeni bir araç daha eklemek için Rust'ın `enum` özelliğine geçelim.

[enums]: ch06-00-enumlar-ve-desen-eslestirme.html
[trait-objects]: ch18-02-trait-nesneleri.md
[public]: ch07-03-oge-yollari.html#yolları-pub-anahtar-kelimesiyle-açığa-çıkarmak-exposing
[modules]: ch07-02-kapsam-ve-gizliligi-kontrol-etme.html
