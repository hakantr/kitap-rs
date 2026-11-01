<!-- Old headings. Do not remove or links may break. -->
<a id="methods"></a>
<a id="metotlar-methods"></a>

## Metotlar

Metotlar fonksiyonlara benzer: Bunları da `fn` anahtar kelimesi ve bir adla bildiririz; parametreleri ve bir dönüş değerleri olabilir ve metot başka bir yerden çağrıldığında çalıştırılacak kodu barındırırlar. Fonksiyonlardan farklı olarak metotlar bir struct'ın (ya da sırasıyla [Bölüm 6][enums]<!-- ignore --> ve [Bölüm 18][trait-objects]<!-- ignore -->'de ele alacağımız bir enum'ın veya *trait nesnesinin*) bağlamında tanımlanır ve ilk parametreleri her zaman metodun üzerinde çağrıldığı struct örneğini temsil eden `self` olur.

<!-- Old headings. Do not remove or links may break. -->
<a id="defining-methods"></a>
<a id="method-syntax"></a>
<a id="metot-sözdizimi-method-syntax"></a>

### Metot Sözdizimi

Parametre olarak bir `Dikdortgen` örneği alan `alan` fonksiyonunu değiştirelim ve bunun yerine Liste 5-13'te gösterildiği gibi `Dikdortgen` struct'ı üzerinde tanımlı bir `alan` metodu oluşturalım.

<Listing number="5-13" file-name="src/main.rs" caption="`Dikdortgen` struct'ı üzerinde bir `alan` metodu tanımlama">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

</Listing>

Fonksiyonu `Dikdortgen` bağlamında tanımlamak için `Dikdortgen` için bir `impl` (*uygulama / implementation*) bloğu başlatırız. Bu `impl` bloğu içindeki her şey `Dikdortgen` türüyle ilişkilendirilir. Ardından `alan` fonksiyonunu `impl` süslü parantezlerinin içine taşır ve ilk (bu örnekte aynı zamanda tek) parametreyi hem imzada hem de gövdenin her yerinde `self` olacak şekilde değiştiririz. Daha önce `alan` fonksiyonunu çağırıp `dikdortgen1`'i argüman olarak geçirdiğimiz `main` fonksiyonunda ise artık `Dikdortgen` örneğimiz üzerindeki `alan` metodunu çağırmak için *metot sözdizimini (method syntax)* kullanabiliriz. Metot sözdizimi bir örnekten hemen sonra gelir: Bir nokta koyar, ardından metodun adını, parantezleri ve varsa argümanları yazarız.

`alan` metodunun imzasında `dikdortgen: &Dikdortgen` yerine `&self` kullanırız. Buradaki `&self` aslında `self: &Self` ifadesinin kısaltmasıdır. Bir `impl` bloğu içinde `Self` türü, o `impl` bloğunun ait olduğu türün takma adıdır. Metotların ilk parametresi `Self` türünde `self` adında bir parametre olmak zorundadır; bu nedenle Rust, ilk parametre konumunda bunu yalnızca `self` adıyla kısaltmanıza olanak tanır. Yine de tıpkı `dikdortgen: &Dikdortgen` kullanımında olduğu gibi, bu metodun `Self` örneğini ödünç aldığını belirtmek için `self` kısaltmasının önüne `&` koymamız gerektiğine dikkat edin. Metotlar, diğer herhangi bir parametrede olduğu gibi `self`'in sahipliğini alabilir, burada yaptığımız gibi `self`'i değiştirilemez biçimde ödünç alabilir veya `self`'i değiştirilebilir biçimde ödünç alabilir.

Fonksiyon sürümünde `&Dikdortgen` kullanmamızla aynı nedenden ötürü burada da `&self` seçtik: Sahipliği devralmak istemiyoruz; struct içindeki verileri yalnızca okumak istiyoruz, onlara yazmak istemiyoruz. Metodun yaptığı işin bir parçası olarak üzerinde çağrıldığı örneği değiştirmesini isteseydik ilk parametre olarak `&mut self` kullanırdık. İlk parametre olarak yalnızca `self` yazıp örneğin sahipliğini alan bir metot tanımlamak nadirdir; bu teknik genellikle metot `self`'i başka bir şeye dönüştürdüğünde ve dönüşümden sonra çağıran kodun orijinal örneği kullanmasını engellemek istediğinizde tercih edilir.

> Not: Bir metot tanımlarken ilk parametreyi seçmek için şu pratik ölçütü kullanabilirsiniz:
> - **`&self` (varsayılan tercih)**: Metot yalnızca örneğin alanlarını okuyup bir sonuç hesaplıyorsa kullanılır. Çağıran kod örneğin sahipliğini korur ve metot çağrısından sonra da örneği kullanmaya devam edebilir.
> - **`&mut self`**: Metot örneğin alanlarını yerinde güncelliyorsa kullanılır. Bu tür bir metodu çağırabilmek için ilgili değişkenin `let mut dikdortgen1 = ...` şeklinde değiştirilebilir tanımlanmış olması gerekir.
> - **`self` (tüketen metot)**: Metot mevcut örneği tüketip içindeki veriyi başka bir türe dönüştürüyorsa (örneğin bir `String`'i ham bayt vektörüne dönüştüren `String::into_bytes`) veya bir kaynağı kapatıp geçersiz kılıyorsa kullanılır; derleyici çağrıdan sonra eski örneğin yanlışlıkla tekrar kullanılmasını derleme zamanında engeller.

Fonksiyonlar yerine metot kullanmanın temel nedeni, metot sözdizimini kullanabilmenin ve her metodun imzasında `self`'in türünü tekrarlamak zorunda kalmamanın yanı sıra, kod düzenidir. Bir türün örneğiyle yapabileceğimiz her şeyi tek bir `impl` bloğunda toplarız; böylece kodumuzu ileride kullanacak kişilerin `Dikdortgen`'in yeteneklerini sunduğumuz kütüphanenin farklı yerlerinde aramak zorunda kalmasını önleriz.

Bir metoda struct'ın alanlarından biriyle aynı adı vermeyi de seçebileceğimize dikkat edin. Örneğin `Dikdortgen` üzerinde yine `genislik` adında bir metot tanımlayabiliriz:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

</Listing>

Burada `genislik` metodunun, örnekteki `genislik` alanının değeri `0`'dan büyükse `true`, `0` ise `false` döndürmesini seçtik: Aynı adlı bir metot içinde o alanı istediğimiz amaçla kullanabiliriz. `main` içinde `dikdortgen1.genislik` ifadesinin ardından parantez kullandığımızda Rust `genislik` metodunu kastettiğimizi anlar. Parantez kullanmadığımızda ise Rust `genislik` alanını kastettiğimizi bilir.

Her zaman olmasa da çoğu durumda, bir metoda bir alanla aynı adı verdiğimizde yalnızca alandaki değeri döndürmesini ve başka hiçbir şey yapmamasını isteriz. Bu tür metotlara *alıcı metotlar (getters)* denir ve Rust, bazı diğer dillerin aksine struct alanları için bunları otomatik olarak uygulamaz. Alıcı metotlar oldukça kullanışlıdır; çünkü alanı gizli (*private*), metodu ise açık (*public*) yaparak bu alana türün açık API'sinin bir parçası olarak salt okunur erişim sağlayabilirsiniz. Açık ve gizli kavramlarının ne olduğunu ve bir alanı ya da metodu nasıl açık veya gizli olarak belirleyeceğimizi [Bölüm 7][public]<!-- ignore -->'de ele alacağız.

<!-- Old headings. Do not remove or links may break. -->
<a id="wheres-the---operator"></a>
<a id="--operatörüne-ne-oldu"></a>

> ### `->` Operatörü Nerede?
>
> C ve C++ dillerinde metot çağırmak için iki farklı operatör kullanılır: Metodu doğrudan nesne üzerinde çağırıyorsanız `.`, nesneyi gösteren bir işaretçi (*pointer*) üzerinden çağırıyorsanız ve önce işaretçinin *başvurusunu çözmeniz (dereference)* gerekiyorsa `->` kullanırsınız. Başka bir deyişle, `nesne` bir işaretçiyse `nesne->bir_seyler_yap()` ifadesi `(*nesne).bir_seyler_yap()` ifadesine benzer.
>
> Rust'ta `->` operatörünün bir karşılığı yoktur; bunun yerine Rust'ta *otomatik referanslama ve başvuru çözme (automatic referencing and dereferencing)* adı verilen bir özellik bulunur. Metot çağırma işlemi, Rust'ta bu davranışın görüldüğü sayılı yerlerden biridir.
>
> Mekanizma şöyle çalışır: `nesne.bir_seyler_yap()` şeklinde bir metot çağırdığınızda Rust, `nesne`'nin metodun imzasıyla eşleşmesi için gereken `&`, `&mut` veya `*` operatörlerini otomatik olarak ekler. Başka bir deyişle, aşağıdaki iki çağrı birbiriyle tamamen aynıdır:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
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
> İlk yazım çok daha temiz görünür. Bu otomatik referanslama davranışı sorunsuz çalışır, çünkü metotların net bir *alıcısı (receiver)* vardır: `self`'in türü. Bir metodun alıcısı ve adı belli olduğunda Rust, metodun okuma (`&self`), değiştirme (`&mut self`) veya tüketme (`self`) işlemlerinden hangisini yaptığını kesin olarak belirleyebilir. Rust'ın metot alıcıları için ödünç almayı örtük (*implicit*) hâle getirmesi, sahiplik kurallarını pratikte ergonomik kılan en önemli etkenlerden biridir.

<!-- Old headings. Do not remove or links may break. -->
<a id="methods-with-more-parameters"></a>

### Daha Fazla Parametre Alan Metotlar

`Dikdortgen` struct'ı üzerinde ikinci bir metot uygulayarak metot kullanımı konusunda pratik yapalım. Bu kez bir `Dikdortgen` örneğinin başka bir `Dikdortgen` örneğini almasını ve ikinci `Dikdortgen` tamamen `self`'in (birinci `Dikdortgen`'in) içine sığabiliyorsa `true`, aksi hâlde `false` döndürmesini istiyoruz. Yani `kapsayabilir_mi` metodunu tanımladıktan sonra Liste 5-14'te gösterilen programı yazabilmek istiyoruz.

<Listing number="5-14" file-name="src/main.rs" caption="Henüz yazılmamış `kapsayabilir_mi` metodunun kullanımı">

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

</Listing>

`dikdortgen2`'nin her iki boyutu da `dikdortgen1`'in boyutlarından küçük olduğu, buna karşılık `dikdortgen3` `dikdortgen1`'den daha geniş olduğu için beklenen çıktı şöyle görünmelidir:

```text
dikdortgen1, dikdortgen2'yi kapsayabilir mi? true
dikdortgen1, dikdortgen3'ü kapsayabilir mi? false
```

Bir metot tanımlamak istediğimizi biliyoruz, dolayısıyla bu tanım `impl Dikdortgen` bloğunun içinde yer alacak. Metodun adı `kapsayabilir_mi` olacak ve parametre olarak başka bir `Dikdortgen` örneğinin değiştirilemez ödünç alımını kabul edecek. Parametrenin türünün ne olacağını metodu çağıran koda bakarak da anlayabiliriz: `dikdortgen1.kapsayabilir_mi(&dikdortgen2)` çağrısı argüman olarak bir `Dikdortgen` örneği olan `dikdortgen2`'nin değiştirilemez ödünç alımını, yani `&dikdortgen2` ifadesini geçirir. Bu oldukça mantıklıdır; çünkü `dikdortgen2`'yi yalnızca okumamız yeterlidir (yazmamız gerekseydi değiştirilebilir bir ödünç alıma ihtiyaç duyardık) ve `kapsayabilir_mi` metodunu çağırdıktan sonra `dikdortgen2`'yi tekrar kullanabilmek için `main` fonksiyonunun sahipliği elinde tutmasını isteriz. `kapsayabilir_mi` metodunun dönüş değeri bir mantıksal değer (`bool`) olacak ve metodun uygulaması `self`'in genişlik ve yüksekliğinin diğer `Dikdortgen` örneğinin sırasıyla genişlik ve yüksekliğinden büyük olup olmadığını kontrol edecektir. Yeni `kapsayabilir_mi` metodunu Liste 5-13'teki `impl` bloğuna ekleyelim; sonuç Liste 5-15'te gösterilmiştir.

<Listing number="5-15" file-name="src/main.rs" caption="`Dikdortgen` üzerinde başka bir `Dikdortgen` örneğini parametre olarak alan `kapsayabilir_mi` metodunu uygulama">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

</Listing>

Bu kodu Liste 5-14'teki `main` fonksiyonuyla birlikte çalıştırdığımızda istediğimiz çıktıyı elde ederiz. Metotlar, imzada `self` parametresinden sonra eklediğimiz birden fazla parametre alabilir ve bu parametreler tıpkı fonksiyonlardaki parametreler gibi çalışır.

<!-- Old headings. Do not remove or links may break. -->
<a id="associated-functions"></a>
<a id="i̇lişkili-fonksiyonlar-associated-functions"></a>
<a id="ilişkili-fonksiyonlar-associated-functions"></a>

### İlişkili Fonksiyonlar

Bir `impl` bloğu içinde tanımlanan tüm fonksiyonlara *ilişkili fonksiyonlar (associated functions)* denir; çünkü bu fonksiyonlar `impl` kelimesinden sonra adı verilen türle ilişkilendirilmiştir. İlk parametresi `self` olmayan (ve dolayısıyla metot olmayan) ilişkili fonksiyonlar da tanımlayabiliriz; çünkü bunların çalışmak için ilgili türün bir örneğine ihtiyacı yoktur. Bu tarz bir fonksiyonu daha önce zaten kullanmıştık: `String` türü üzerinde tanımlı olan `String::from` fonksiyonu.

Metot olmayan ilişkili fonksiyonlar çoğunlukla struct'ın yeni bir örneğini döndüren *yapıcılar (constructors)* olarak kullanılır. Bunlara sıklıkla `new` adı verilir, ancak `new` özel bir ad değildir ve dile yerleşik bir anahtar kelime olarak tanımlanmamıştır. Örneğin tek bir boyut parametresi alıp bunu hem genişlik hem de yükseklik olarak kullanan `kare` adında bir ilişkili fonksiyon sunmayı seçebiliriz; böylece aynı değeri iki kez belirtmek zorunda kalmadan kare bir `Dikdortgen` oluşturmayı kolaylaştırırız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

Dönüş türündeki ve fonksiyon gövdesindeki `Self` anahtar kelimeleri, `impl` anahtar kelimesinden sonra gelen türün — bu örnekte `Dikdortgen` türünün — takma adıdır.

Bu ilişkili fonksiyonu çağırmak için struct adıyla birlikte `::` sözdizimini kullanırız; `let k = Dikdortgen::kare(3);` ifadesi buna bir örnektir. Bu fonksiyon struct'ın *ad alanı (namespace)* altında yer alır: `::` sözdizimi hem ilişkili fonksiyonlar hem de modüllerin oluşturduğu ad alanları için kullanılır. Modülleri [Bölüm 7][modules]<!-- ignore -->'de ele alacağız.

> Not: Rust'ta, diğer pek çok nesne yönelimli dilde bulunan özel bir `new` anahtar kelimesi ya da aynı adla farklı parametreler alan *aşırı yüklenmiş (overloaded)* yapıcılar yoktur. Bunun yerine `Vec::new()`, `Vec::with_capacity(...)`, `String::from(...)` veya `Dikdortgen::kare(...)` gibi açıklayıcı adlara sahip ilişkili fonksiyonlar tanımlamak standart Rust idiyomudur; böylece bir değerin nasıl oluşturulduğu çağrı noktasında ilk bakışta anlaşılır. Ayrıca tanımda `Self` kullanmak, ileride struct'ın adını değiştirdiğinizde `impl` bloğu içindeki dönüş türlerini ve örneklemeleri tek tek güncelleme zahmetini ortadan kaldırır.

<!-- Old headings. Do not remove or links may break. -->
<a id="multiple-impl-blocks"></a>

### Birden Fazla `impl` Bloğu

Her struct'ın birden fazla `impl` bloğuna sahip olmasına izin verilir. Örneğin Liste 5-15, her metodun kendi ayrı `impl` bloğunda yer aldığı Liste 5-16'daki kodla tamamen eşdeğerdir.

<Listing number="5-16" caption="Liste 5-15'i birden fazla `impl` bloğu kullanarak yeniden yazma">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

</Listing>

Burada bu metotları birden fazla `impl` bloğuna ayırmamızı gerektiren bir neden yok; ancak bu sözdizimi geçerlidir. Jenerik türleri ve trait'leri ele alacağımız Bölüm 10'da, birden fazla `impl` bloğunun yararlı olduğu bir durumu göreceğiz.

<!-- Old headings. Do not remove or links may break. -->
<a id="summary"></a>

## Özet

Struct'lar, çalıştığınız problem alanı için anlamlı özel türler oluşturmanızı sağlar. Struct kullanarak birbiriyle ilişkili veri parçalarını bir arada tutabilir ve kodunuzu anlaşılır kılmak için her bir parçaya açıklayıcı bir ad verebilirsiniz. `impl` bloklarında ise türünüzle ilişkili fonksiyonlar tanımlayabilirsiniz; metotlar da struct örneklerinizin sahip olacağı davranışı belirlemenize olanak tanıyan bir tür ilişkili fonksiyondur.

Ancak özel türler oluşturmanın tek yolu struct'lar değildir: Araç çantanıza yeni bir araç daha eklemek için şimdi Rust'ın enum özelliğine geçelim.

[enums]: ch06-00-enumlar-ve-desen-eslestirme.html
[trait-objects]: ch18-02-trait-nesneleri.html
[public]: ch07-03-oge-yollari.html#yolları-pub-anahtar-kelimesiyle-açığa-çıkarmak-exposing
[modules]: ch07-02-kapsam-ve-gizliligi-kontrol-etme.html
