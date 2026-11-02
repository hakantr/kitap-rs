<!-- Old headings. Do not remove or links may break. -->
<a id="generic-data-types"></a>
<a id="jenerik-veri-turleri"></a>

## Jenerik Veri Türleri

Fonksiyon imzaları veya struct'lar gibi öğeler için daha sonra pek çok farklı somut veri türüyle kullanabileceğimiz tanımlar oluşturmak amacıyla jeneriklerden yararlanırız. Önce fonksiyonları, struct'ları, enum'ları ve metotları jenerikler kullanarak nasıl tanımlayacağımıza bakalım. Ardından jeneriklerin kod performansını nasıl etkilediğini ele alacağız.

<!-- Old headings. Do not remove or links may break. -->
<a id="in-function-definitions"></a>
<a id="fonksiyon-tanimlarinda"></a>

### Fonksiyon Tanımlarında

Jenerik kullanan bir fonksiyon tanımlarken, jenerikleri fonksiyon imzasında normalde parametrelerin ve dönüş değerinin veri türlerini belirttiğimiz yerlere yazarız. Bu yaklaşım kod tekrarını önlerken kodumuzu daha esnek hâle getirir ve fonksiyonumuzu çağıranlara daha geniş bir işlevsellik sunar.

`en_buyuk_bul` fonksiyonumuzdan devam edelim: Liste 10-4, her ikisi de bir dilimdeki en büyük değeri bulan iki fonksiyon göstermektedir. Ardından bunları jenerik kullanan tek bir fonksiyonda birleştireceğiz.

<Listing number="10-4" file-name="src/main.rs" caption="Yalnızca adları ve imzalarındaki türler bakımından farklılaşan iki fonksiyon">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

</Listing>

`en_buyuk_i32` fonksiyonu, Liste 10-3'te ayrı bir fonksiyona çıkardığımız ve bir dilimdeki en büyük `i32` değerini bulan fonksiyondur. `en_buyuk_char` fonksiyonu ise bir dilimdeki en büyük `char` değerini bulur. Her iki fonksiyonun gövdesi birebir aynı koda sahip olduğundan, tek bir fonksiyonda jenerik bir tür parametresi tanımlayarak bu tekrarı ortadan kaldıralım.

Yeni ve tek bir fonksiyonda türleri parametreleştirmek için, tıpkı bir fonksiyonun değer parametrelerine ad verdiğimiz gibi tür parametresine de bir ad vermemiz gerekir. Tür parametresi adı olarak geçerli herhangi bir tanımlayıcıyı kullanabilirsiniz. Ancak biz `T` kullanacağız; çünkü Rust'ta geleneksel olarak tür parametresi adları kısadır —çoğu zaman tek bir harften oluşur— ve Rust'ın tür adlandırma kuralı `UpperCamelCase` biçimindedir. İngilizce _type_ (tür) kelimesinin kısaltması olan `T`, çoğu Rust programcısının varsayılan tercihidir.

Fonksiyon gövdesinde bir parametre kullandığımızda, derleyicinin bu adın ne anlama geldiğini bilmesi için parametre adını imzada bildirmemiz gerekir. Benzer şekilde, bir fonksiyon imzasında bir tür parametresi adı kullanacağımızda da bu adı kullanmadan önce bildirmeliyiz. Jenerik `en_buyuk_bul` fonksiyonunu tanımlamak için tür adı bildirimlerini fonksiyon adı ile parametre listesi arasına, açılı ayraçlar (`<>`) içine yerleştiririz:

```rust,ignore
fn en_buyuk_bul<T>(liste: &[T]) -> &T {
```

Bu tanımı, “`en_buyuk_bul` fonksiyonu bir `T` türü üzerinde jeneriktir” şeklinde okuruz. Bu fonksiyon, `T` türündeki değerlerden oluşan bir dilim olan `liste` adında tek bir parametre alır. `en_buyuk_bul` fonksiyonu yine aynı `T` türündeki bir değere referans döndürecektir.

Liste 10-5, imzasında jenerik veri türünü kullanan birleştirilmiş `en_buyuk_bul` fonksiyon tanımını gösterir. Liste ayrıca fonksiyonu ister `i32` değerlerinden ister `char` değerlerinden oluşan bir dilimle nasıl çağırabileceğimizi de gösterir. Bu kodun henüz derlenmeyeceğine dikkat edin.

<Listing number="10-5" file-name="src/main.rs" caption="Jenerik tür parametreleri kullanan `en_buyuk_bul` fonksiyonu; bu kod henüz derlenmez">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

</Listing>

Bu kodu mevcut hâliyle derlersek şu hatayı alırız:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

Derleyici, `binary operation '>' cannot be applied to type '&T'` (`>` ikili işlemi `&T` türüne uygulanamaz) hatasını verir ve yardım (`help`) satırında bir trait olan `std::cmp::PartialOrd`'dan söz eder; bir sonraki bölümde trait'leri ele alacağız. Şimdilik bu hatanın, `en_buyuk_bul` gövdesinin `T`'nin alabileceği olası tüm türler için çalışmayacağını belirttiğini bilmeniz yeterlidir. Gövde içinde `T` türündeki değerleri karşılaştırmak istediğimizden, yalnızca değerleri sıralanabilen türleri kullanabiliriz. Karşılaştırma işlemlerini mümkün kılmak üzere standart kütüphanede, türler üzerinde uygulayabileceğiniz `std::cmp::PartialOrd` trait'i bulunur (bu trait hakkında daha fazla bilgi için Ek C'ye bakabilirsiniz). Liste 10-5'i düzeltmek için yardım metninin önerisini izleyerek `T` için geçerli olan türleri yalnızca `PartialOrd` uygulayanlarla sınırlandırabiliriz. Standart kütüphane hem `i32` hem de `char` için `PartialOrd` trait'ini uyguladığından kod bu değişiklikle sorunsuz derlenecektir.

> Not: Jenerik bir fonksiyonu çağırırken `T` yerine hangi somut türün geçeceğini genellikle açıkça yazmazsınız; derleyici bunu argümanlardan çıkarır. Liste 10-5'teki `en_buyuk_bul(&sayi_listesi)` çağrısında `sayi_listesi` bir `Vec<i32>` olduğu için `T` kendiliğinden `i32` olur; `en_buyuk_bul(&karakter_listesi)` çağrısında ise `char` olur. Derleyicinin türü argümanlardan çıkaramadığı ya da birden fazla türün mümkün olduğu durumlarda türü, *turbofish* adı verilen `::<>` sözdizimiyle doğrudan çağrıda belirtebilirsiniz: `en_buyuk_bul::<i32>(&sayi_listesi)`. Her iki durumda da derleyici, bu bölümün sonunda göreceğiniz gibi kullanılan her somut tür için fonksiyonun ayrı bir kopyasını üretir.

<!-- Old headings. Do not remove or links may break. -->
<a id="in-struct-definitions"></a>
<a id="struct-yapı-tanımlarında"></a>
<a id="struct-yapi-tanimlarinda"></a>
<a id="struct-tanimlarinda"></a>

### Struct Tanımlarında

`<>` sözdizimini kullanarak bir veya daha fazla alanında jenerik tür parametresi barındıran struct'lar da tanımlayabiliriz. Liste 10-6, herhangi bir türden `x` ve `y` koordinat değerlerini tutacak bir `Nokta<T>` struct'ı tanımlar.

<Listing number="10-6" file-name="src/main.rs" caption="`T` türünde `x` ve `y` değerlerini tutan bir `Nokta<T>` struct'ı">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

</Listing>

Struct tanımlarında jenerik kullanma sözdizimi, fonksiyon tanımlarındakine benzer. Önce struct adının hemen ardından açılı ayraçlar içinde tür parametresinin adını bildiririz. Ardından struct tanımında normalde somut veri türlerini yazacağımız yerlerde bu jenerik türü kullanırız.

`Nokta<T>`'yi tanımlarken yalnızca tek bir jenerik tür kullandığımız için bu tanım, `Nokta<T>` struct'ının bir `T` türü üzerinde jenerik olduğunu ve bu tür hangisi olursa olsun `x` ile `y` alanlarının _her ikisinin de_ aynı türde olduğunu söyler. Liste 10-7'deki gibi farklı türlerde değerlere sahip bir `Nokta<T>` örneği oluşturmaya çalışırsak kodumuz derlenmez.

<Listing number="10-7" file-name="src/main.rs" caption="İkisi de aynı `T` jenerik veri türüne sahip olduğu için `x` ve `y` alanları aynı türde olmalıdır.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

</Listing>

Bu örnekte `x` alanına tam sayı olan `5` değerini atadığımızda, derleyiciye bu `Nokta<T>` örneği için `T` jenerik türünün bir tam sayı olacağını bildirmiş oluruz. Ardından `x` ile aynı türde olmasını şart koştuğumuz `y` alanı için `4.0` değerini verdiğimizde, `expected integer, found floating-point number` (tam sayı bekleniyordu, kayan noktalı sayı bulundu) açıklamasıyla şu tür uyuşmazlığı hatasını alırız:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

Hem `x` hem de `y` alanının jenerik olduğu fakat birbirinden farklı türler alabildiği bir `Nokta` struct'ı tanımlamak için birden fazla jenerik tür parametresi kullanabiliriz. Örneğin Liste 10-8'de `Nokta` tanımını `T` ve `U` türleri üzerinde jenerik olacak şekilde değiştiriyoruz; böylece `x` alanı `T` türünde, `y` alanı ise `U` türünde olabiliyor.

<Listing number="10-8" file-name="src/main.rs" caption="İki tür üzerinde jenerik olan ve böylece `x` ile `y`'nin farklı türlerde değerler alabilmesini sağlayan bir `Nokta<T, U>`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

</Listing>

Artık gösterilen tüm `Nokta` örneklerine izin verilir! Bir tanımda istediğiniz kadar jenerik tür parametresi kullanabilirsiniz; ancak birkaç taneden fazlasını kullanmak kodunuzun okunmasını güçleştirir. Kodunuzda çok sayıda jenerik türe ihtiyaç duyduğunuzu fark ederseniz, bu durum kodunuzun daha küçük parçalara ayrılarak yeniden yapılandırılması gerektiğinin bir işareti olabilir.

<!-- Old headings. Do not remove or links may break. -->
<a id="in-enum-definitions"></a>
<a id="enum-tanimlarinda"></a>

### Enum Tanımlarında

Struct'larda yaptığımız gibi enum'ları da varyantlarında jenerik veri türleri tutacak şekilde tanımlayabiliriz. Standart kütüphanenin sunduğu ve Bölüm 6'da kullandığımız `Option<T>` enum'ına yeniden göz atalım:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Bu tanım artık size çok daha anlamlı gelmelidir. Gördüğünüz gibi `Option<T>` enum'ı `T` türü üzerinde jeneriktir ve iki varyanta sahiptir: `T` türünden bir değer tutan `Some` ile hiçbir değer tutmayan `None` varyantı. `Option<T>` enum'ını kullanarak isteğe bağlı bir değer kavramını soyut biçimde ifade edebiliriz; üstelik `Option<T>` jenerik olduğu için söz konusu değerin türü ne olursa olsun bu soyutlamayı kullanabiliriz.

Enum'lar birden fazla jenerik tür de kullanabilir. Bölüm 9'da kullandığımız `Result` enum'ının tanımı buna bir örnektir:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` enum'ı, `T` ve `E` olmak üzere iki tür üzerinde jeneriktir ve iki varyantı vardır: `T` türünden bir değer tutan `Ok` ve `E` türünden bir değer tutan `Err`. Bu tanım, başarılı olabilecek (`T` türünde bir değer döndüren) veya başarısız olabilecek (`E` türünde bir hata döndüren) bir işlemimizin bulunduğu her yerde `Result` enum'ını kullanmayı son derece elverişli kılar. Nitekim Liste 9-3'te bir dosyayı açarken kullandığımız yapı tam olarak buydu: Dosya başarıyla açıldığında `T` yerine `std::fs::File` türü, dosya açılırken sorun yaşandığında ise `E` yerine `std::io::Error` türü geçmişti.

Kodunuzda yalnızca tuttukları değerlerin türleri bakımından farklılaşan birden fazla struct veya enum tanımı bulunduğunu fark ettiğinizde, jenerik türler kullanarak bu tekrarı ortadan kaldırabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="in-method-definitions"></a>
<a id="metot-tanimlarinda"></a>

### Metot Tanımlarında

Bölüm 5'te yaptığımız gibi struct ve enum'lar üzerinde metotlar uygulayabilir, bu metotların tanımlarında da jenerik türler kullanabiliriz. Liste 10-9, Liste 10-6'da tanımladığımız `Nokta<T>` struct'ını ve üzerinde tanımlanmış `x` adlı metodu gösterir.

<Listing number="10-9" file-name="src/main.rs" caption="`Nokta<T>` struct'ı üzerinde, `T` türündeki `x` alanına bir referans döndüren `x` adlı metodun uygulanması">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

</Listing>

Burada `Nokta<T>` üzerinde, `x` alanındaki veriye bir referans döndüren `x` adında bir metot tanımladık.

Metotları `Nokta<T>` türü üzerinde uyguladığımızı belirtebilmek için `T`'yi `impl` anahtar kelimesinin hemen ardından bildirmemiz gerektiğine dikkat edin. `T`'yi `impl` sonrasında jenerik bir tür olarak bildirdiğimizde Rust, `Nokta` içindeki açılı ayraçlarda yer alan türün somut bir tür değil, jenerik bir tür olduğunu anlar. Bu jenerik parametre için struct tanımında bildirilenden farklı bir ad da seçebilirdik; ancak aynı adı kullanmak yerleşik kuraldır. Jenerik bir tür bildiren bir `impl` bloğu içinde yazdığınız metot, jenerik türün yerine hangi somut tür geçerse geçsin bu türün her örneği üzerinde tanımlı olur.

Bir tür üzerinde metot tanımlarken jenerik türlere kısıtlamalar da getirebiliriz. Örneğin metotları herhangi bir jenerik türdeki `Nokta<T>` örnekleri yerine yalnızca `Nokta<f32>` örnekleri üzerinde uygulayabiliriz. Liste 10-10'da somut `f32` türünü kullanıyoruz; yani `impl` sonrasında herhangi bir tür parametresi bildirmiyoruz.

<Listing number="10-10" file-name="src/main.rs" caption="`T` jenerik tür parametresi için belirli bir somut türe sahip bir struct'a uygulanan `impl` bloğu">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

</Listing>

Bu kod, `Nokta<f32>` türünün `orijinden_uzaklik` adlı bir metoda sahip olacağı, `T`'nin `f32` olmadığı diğer `Nokta<T>` örneklerinde ise bu metodun tanımlı olmayacağı anlamına gelir. Metot, noktamızın `(0.0, 0.0)` koordinatlarındaki başlangıç noktasına olan uzaklığını hesaplar ve yalnızca kayan noktalı sayı türlerinde bulunan matematiksel işlemleri (`powi` ve `sqrt`) kullanır.

> Not: `impl<T> Nokta<T>` ile `impl Nokta<f32>` sözdizimi arasındaki fark, açılı
> ayraçların iki farklı görev üstlenmesinden kaynaklanır: `impl` anahtar
> kelimesinin hemen ardından gelen `<T>`, yeni bir jenerik tür parametresi
> **bildirerek onu kapsama dahil eder**; `Nokta<T>` içindeki `<T>` ise bu
> bildirilen parametreyi **kullanır**. İlk `<T>`'yi unutup `impl Nokta<T>`
> yazarsanız derleyici `T`'yi tıpkı `f32` gibi somut bir tür sanır ve kapsamda
> `T` adında bir tür bulamadığı için (`E0412`) hata verir. Buna karşılık `impl
> Nokta<f32>` yazdığımızda `f32` zaten bilinen somut bir tür olduğundan
> `impl`'den sonra yeni bir tür parametresi bildirmeyiz.

Bir struct tanımındaki jenerik tür parametreleri, aynı struct'ın metot imzalarında kullandığınız jenerik tür parametreleriyle her zaman aynı olmak zorunda değildir. Liste 10-11, örneği daha anlaşılır kılmak amacıyla `Nokta` struct'ı için `X1` ve `Y1`, `karistir` metodunun imzası içinse `X2` ve `Y2` jenerik türlerini kullanır. Bu metot, `self` olan `Nokta` değerinden (`X1` türündeki) `x` alanını ve parametre olarak geçirilen `Nokta` değerinden (`Y2` türündeki) `y` alanını alarak yeni bir `Nokta` örneği oluşturur.

<Listing number="10-11" file-name="src/main.rs" caption="Kendi struct tanımındakilerden farklı jenerik türler kullanan bir metot">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

</Listing>

`main` fonksiyonunda, `x` için bir `i32` (`5` değeriyle) ve `y` için bir `f64` (`10.4` değeriyle) tutan `p1` adında bir `Nokta` tanımladık. `p2` değişkeni ise `x` için bir string dilimine (`"Merhaba"` değeriyle) ve `y` için bir `char` değerine (`'c'` değeriyle) sahip bir `Nokta` struct'ıdır. `p1` üzerinde `p2` argümanıyla `karistir` metodunu çağırmak bize `p3` değerini verir: `x` alanı `p1`'den geldiği için `p3`'ün `x` alanı `i32` türünde, `y` alanı ise `p2`'den geldiği için `char` türünde olur. `println!` makrosu çağrısı ekrana `p3.x = 5, p3.y = c` yazdırır.

Bu örneğin amacı, bazı jenerik parametrelerin `impl` ile, bazılarının ise metot tanımıyla bildirildiği bir durumu göstermektir. Burada `X1` ve `Y1` jenerik parametreleri struct tanımına ait oldukları için `impl` sonrasında bildirilir. `X2` ve `Y2` jenerik parametreleri ise yalnızca bu metotla ilgili olduklarından `fn karistir` sonrasında bildirilir.

<!-- Old headings. Do not remove or links may break. -->
<a id="performance-of-code-using-generics"></a>
<a id="jenerikleri-kullanan-kodların-performansı"></a>
<a id="jenerikleri-kullanan-kodlarin-performansi"></a>
<a id="jenerik-kullanan-kodlarin-performansi"></a>

### Jenerik Kullanan Kodların Performansı

Jenerik tür parametreleri kullanmanın çalışma zamanında bir maliyeti olup olmadığını merak ediyor olabilirsiniz. İyi haber şu ki jenerik türler kullanmak, programınızın somut türler kullanan eşdeğer bir koda göre daha yavaş çalışmasına yol açmaz.

Rust bunu, derleme zamanında jenerik kullanan kod üzerinde *tek biçimlileştirme (monomorphization)* —diğer adıyla *monomorfizasyon*— uygulayarak başarır. _Tek biçimlileştirme_, derleme sırasında kullanılan somut türleri yerlerine koyarak jenerik kodu türe özgü koda dönüştürme sürecidir. Bu süreçte derleyici, Liste 10-5'te jenerik fonksiyonu oluştururken izlediğimiz adımların tam tersini yapar: jenerik kodun çağrıldığı tüm yerlere bakar ve jenerik kodun birlikte çağrıldığı somut türler için özelleştirilmiş kod üretir.

Standart kütüphanenin jenerik `Option<T>` enum'ını kullanarak bunun nasıl çalıştığına bakalım:

```rust
let tamsayi = Some(5);
let ondalikli = Some(5.0);
```

Rust bu kodu derlediğinde tek biçimlileştirme uygular. Bu işlem sırasında derleyici, `Option<T>` örneklerinde kullanılan değerleri okur ve iki farklı `Option<T>` kullanımı tespit eder: biri `i32`, diğeri ise `f64` türündedir. Bunun sonucunda `Option<T>`'nin jenerik tanımını `i32` ve `f64` için özelleştirilmiş iki ayrı tanıma açar ve jenerik tanımı bu türe özgü tanımlarla değiştirir.

Kodun tek biçimlileştirilmiş hâli aşağıdakine benzer (derleyici burada açıklama amacıyla kullandıklarımızdan farklı iç adlar üretir):

<Listing file-name="src/main.rs">

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let tamsayi = Option_i32::Some(5);
    let ondalikli = Option_f64::Some(5.0);
}
```

</Listing>

Jenerik `Option<T>`, derleyicinin oluşturduğu türe özgü tanımlarla değiştirilir. Rust jenerik kodu her bir kullanımda somut türü belirten koda derlediği için jenerikleri kullanmanın çalışma zamanında hiçbir ek maliyeti yoktur. Kod çalıştığında, her bir tanımı elle tek tek çoğaltmışız gibi performans gösterir. Tek biçimlileştirme süreci, Rust'ın jeneriklerini çalışma zamanında son derece verimli kılar.

> Not: Tek biçimlileştirme sayesinde derleyici hangi somut fonksiyon veya
> metodun çağrılacağını derleme zamanında kesin olarak bilir; buna *statik
> gönderim (static dispatch)* denir. Bu bilgi, doğrudan çağrılar ve satır içi
> açma (*inlining*) gibi optimizasyonlara olanak tanır; ancak belirli bir
> optimizasyonun uygulanması veya kodun her durumda daha hızlı olması garanti
> edilmez. Başlıca ödünleşimlerden biri, bir jenerik tanım çok sayıda farklı
> somut türle kullanıldığında her tür için ayrı makine kodu üretilmesi nedeniyle
> derleme süresinin ve üretilen ikili dosya (*binary*) boyutunun bir miktar
> artabilmesidir. Homojen olmayan koleksiyonlara veya daha küçük ikili dosya
> boyutuna ihtiyaç duyduğunuz durumlarda Rust, Bölüm 18'de inceleyeceğimiz
> *trait nesneleri (`dyn Trait`)* aracılığıyla çalışma zamanında çözümlenen
> *dinamik gönderim (dynamic dispatch)* seçeneğini de sunar.
