## Jenerik Veri Türleri

Fonksiyon imzaları veya struct'lar (yapılar) gibi öğeler için tanımlamalar oluşturmak amacıyla jenerikleri kullanırız, bunları daha sonra birçok farklı somut veri türüyle kullanabiliriz. İlk olarak jenerikleri kullanarak fonksiyonları, struct'ları, enum'ları ve metotları nasıl tanımlayacağımıza bakalım. Sonra jeneriklerin kod performansını nasıl etkilediğini tartışacağız.

### Fonksiyon Tanımlarında

Jenerik kullanan bir fonksiyon tanımlarken, jenerikleri genellikle parametrelerin ve dönüş değerinin veri türlerini belirteceğimiz yere, yani fonksiyonun imzasına yerleştiririz. Bunu yapmak kodumuzu daha esnek hale getirir ve kod tekrarını engellerken fonksiyonumuzu çağıranlara daha fazla işlevsellik sağlar.

`en_buyuk` fonksiyonumuza devam edersek, Liste 10-4 her ikisi de bir dilimdeki en büyük değeri bulan iki fonksiyon göstermektedir. Daha sonra bunları jenerik kullanan tek bir fonksiyonda birleştireceğiz.

<Listing number="10-4" file-name="src/main.rs" caption="Yalnızca adlarında ve imzalarındaki türlerde farklılık gösteren iki fonksiyon">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

</Listing>

`en_buyuk_i32` fonksiyonu Liste 10-3'te çıkardığımız ve bir dilimdeki en büyük `i32`'yi bulan fonksiyondur. `en_buyuk_char` fonksiyonu ise bir dilimdeki en büyük `char` değerini bulur. Fonksiyon gövdeleri aynı koda sahiptir, bu yüzden tek bir fonksiyona jenerik bir tür parametresi dahil ederek bu tekrarı ortadan kaldıralım.

Yeni, tek bir fonksiyondaki türleri parametre haline getirmek için tıpkı bir fonksiyona geçirilen değer parametrelerinde yaptığımız gibi tür parametresini isimlendirmemiz gerekir. Bir tür parametresi adı olarak herhangi bir tanımlayıcı kullanabilirsiniz. Ancak biz `T` kullanacağız çünkü kural olarak, Rust'taki tür parametresi adları kısadır (genellikle tek bir harf) ve Rust'ın tür adlandırma kuralı UpperCamelCase (BüyükDeveHali) şeklindedir. _Türün_ (Type) kısaltması olan `T`, çoğu Rust programcısının varsayılan seçimidir.

Fonksiyonun gövdesinde bir parametre kullandığımızda, derleyicinin bu adın ne anlama geldiğini bilmesi için imzaya parametre adını bildirmemiz gerekir. Benzer şekilde, bir fonksiyon imzasında bir tür parametresi adı kullandığımızda, kullanmadan önce tür parametresi adını bildirmemiz gerekir. Jenerik `en_buyuk` fonksiyonunu tanımlamak için tür adı tanımlarını fonksiyon adı ve parametre listesi arasına açılı ayraçlar (angle brackets, `<>`) içine yerleştiririz, tıpkı bunun gibi:

```rust,ignore
fn en_buyuk_bul<T>(liste: &[T]) -> &T {
```

Bu tanımı “`en_buyuk_bul` fonksiyonu `T` türü üzerinde jeneriktir” olarak okuruz. Bu fonksiyonun, `T` türündeki değerlerden oluşan bir dilim olan `liste` adında bir parametresi vardır. `en_buyuk_bul` fonksiyonu aynı `T` türünde bir değere referans döndürecektir.

Liste 10-5, imzasında jenerik veri türünü kullanan birleştirilmiş `en_buyuk_bul` fonksiyon tanımını gösterir. Liste ayrıca fonksiyonu bir `i32` değer dilimiyle veya `char` değer dilimiyle nasıl çağırabileceğimizi de gösterir. Bu kodun henüz derlenmeyeceğini unutmayın.

<Listing number="10-5" file-name="src/main.rs" caption="Jenerik tür parametreleri kullanan `en_buyuk_bul` fonksiyonu; bu henüz derlenmiyor">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

</Listing>

Bu kodu şu anda derlersek şu hatayı alırız:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

Yardım metni bir trait olan `std::cmp::PartialOrd`'dan bahsediyor ve bir sonraki bölümde traitler hakkında konuşacağız. Şimdilik bu hatanın, `en_buyuk_bul` gövdesinin `T`'nin olabileceği olası tüm türler için çalışmayacağını ifade ettiğini bilin. Gövdede `T` türündeki değerleri karşılaştırmak istediğimizden, yalnızca değerleri sıralanabilen türleri kullanabiliriz. Karşılaştırmaları mümkün kılmak için standart kütüphane, türler üzerine uygulayabileceğiniz `std::cmp::PartialOrd` trait'ine sahiptir (bu trait hakkında daha fazla bilgi için Ek C'ye bakın). Liste 10-5'i düzeltmek için yardım metninin önerisini izleyebilir ve `T` için geçerli olan türleri yalnızca `PartialOrd` uygulayanlarla kısıtlayabiliriz. Standart kütüphane hem `i32` hem de `char` üzerinde `PartialOrd` uyguladığı için liste o zaman derlenecektir.

### Struct (Yapı) Tanımlarında

Ayrıca bir veya daha fazla alanda `<>` sözdizimini kullanarak jenerik tür parametresi kullanmak için struct'lar tanımlayabiliriz. Liste 10-6 herhangi bir türden `x` ve `y` koordinat değerlerini tutmak için bir `Nokta<T>` struct'ı tanımlar.

<Listing number="10-6" file-name="src/main.rs" caption="`T` türünde `x` ve `y` değerlerini tutan bir `Nokta<T>` yapısı">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

</Listing>

Struct tanımlarında jenerik kullanma sözdizimi, fonksiyon tanımlarında kullanılana benzer. İlk olarak struct isminin hemen arkasında açılı ayraçlar içinde tür parametresinin ismini tanımlıyoruz. Daha sonra struct tanımında normalde somut veri türlerini belirteceğimiz yerlerde jenerik türü kullanıyoruz.

`Nokta<T>`'yi tanımlamak için sadece bir jenerik tür kullandığımızdan, bu tanımın `Nokta<T>` yapısının bir `T` türü üzerinde jenerik olduğunu ve `x` ve `y` alanlarının _ikisinin_ de aynı türden olduğunu söylediğine dikkat edin (bu tür ne olursa olsun). Liste 10-7'deki gibi farklı türlerde değerleri olan bir `Nokta<T>` örneği oluşturursak kodumuz derlenmeyecektir.

<Listing number="10-7" file-name="src/main.rs" caption="`x` ve `y` alanları aynı türden olmalıdır çünkü ikisi de aynı jenerik veri türü `T`'ye sahiptir.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

</Listing>

Bu örnekte `x`'e tamsayı olan `5` değerini atadığımızda derleyiciye `Nokta<T>`'nin bu örneği için `T` jenerik türünün bir tamsayı olacağını bildiririz. Daha sonra, `x` ile aynı türde olmasını tanımladığımız `y` için `4.0` belirttiğimizde, bunun gibi bir tür uyuşmazlığı hatası alırız:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

`x` ve `y`'nin her ikisinin de jenerik olduğu ancak farklı türlere sahip olabildiği bir `Nokta` struct'ı tanımlamak için birden fazla jenerik tür parametresi kullanabiliriz. Örneğin, Liste 10-8'de `x`'in `T` türünde ve `y`'nin `U` türünde olduğu, `T` ve `U` türleri üzerinde jenerik olması için `Nokta`'nın tanımını değiştiriyoruz.

<Listing number="10-8" file-name="src/main.rs" caption="İki tür üzerinde jenerik olan, böylece `x` ve `y`'nin farklı türlerde değerlere sahip olabileceği bir `Nokta<T, U>`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

</Listing>

Artık gösterilen tüm `Nokta` örneklerine izin veriliyor! Bir tanımda istediğiniz kadar çok jenerik tür parametresi kullanabilirsiniz ancak birkaça taneden fazlasını kullanmak kodunuzun okunmasını zorlaştırır. Kodunuzda çok sayıda jenerik türe ihtiyaç duyduğunuzu fark ederseniz, bu durum kodunuzun daha küçük parçalara bölünerek yeniden yapılandırılması gerektiğine işaret edebilir.

### Enum Tanımlarında

Struct'larda yaptığımız gibi enum'ları da varyantlarında (seçeneklerinde) jenerik veri türlerini barındıracak şekilde tanımlayabiliriz. Standart kütüphanenin sağladığı ve Bölüm 6'da kullandığımız `Option<T>` enum'ına bir kez daha bakalım:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Bu tanım artık size daha mantıklı gelmelidir. Gördüğünüz gibi `Option<T>` enum'ı `T` türü üzerinde jeneriktir ve iki varyantı vardır: `T` türünden bir değer tutan `Some` ve hiçbir değer tutmayan bir `None` varyantı. `Option<T>` enum'ını kullanarak isteğe bağlı bir değerin soyut kavramını ifade edebiliriz ve `Option<T>` jenerik olduğu için bu isteğe bağlı değerin türü ne olursa olsun bu soyutlamayı kullanabiliriz.

Enum'lar birden fazla jenerik türü de kullanabilir. Bölüm 9'da kullandığımız `Result` enum'ının tanımı buna bir örnektir:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` enum'ı `T` ve `E` olmak üzere iki tür üzerinde jeneriktir ve iki varyantı vardır: `T` türünden bir değer tutan `Ok` ve `E` türünden bir değer tutan `Err`. Bu tanım, başarılı olabilecek (bir tür `T` değeri döndüren) veya başarısız olabilecek (bir tür `E` hatası döndüren) bir operasyonumuzun olduğu her yerde `Result` enum'ını kullanmayı elverişli hale getirir. Aslında, Liste 9-3'te dosya başarıyla açıldığında `T`'nin `std::fs::File` türüyle doldurulduğu ve dosya açılırken sorunlar olduğunda `E`'nin `std::io::Error` türüyle doldurulduğu bir dosyayı açmak için kullandığımız şey budur.

Kodunuzda sadece tuttukları değerlerin türleri açısından farklılık gösteren çoklu struct veya enum tanımlarına sahip olduğunuz durumları fark ettiğinizde, bunun yerine jenerik türler kullanarak tekrarlardan kaçınabilirsiniz.

### Metot Tanımlarında

Struct ve enum'lar üzerinde metotlar uygulayabilir (Bölüm 5'te yaptığımız gibi) ve jenerik türleri bunların tanımlarında da kullanabiliriz. Liste 10-9, Liste 10-6'da tanımladığımız `Nokta<T>` struct'ı üzerinde uygulanan `x` adlı metodu gösterir.

<Listing number="10-9" file-name="src/main.rs" caption="`Nokta<T>` struct'ı üzerinde, `T` türündeki `x` alanına referans döndürecek olan `x` isimli bir metodu uygulamak">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

</Listing>

Burada `Nokta<T>` üzerinde `x` alanındaki veriye bir referans döndüren `x` adında bir metot tanımladık.

Metotları `Nokta<T>` türü üzerinde uyguladığımızı belirtmek için `T`'yi kullanabilmek adına `impl`'den hemen sonra `T` bildirmemiz gerektiğine dikkat edin. `T`'yi `impl`'den sonra jenerik bir tür olarak bildirerek Rust, `Nokta` içindeki açılı parantezlerdeki türün somut bir türden ziyade jenerik bir tür olduğunu belirleyebilir. Bu jenerik parametre için struct tanımında bildirilen jenerik parametreden farklı bir ad seçebilirdik ancak aynı adı kullanmak kuraldır. Jenerik bir tür bildiren bir `impl` içerisinde bir metot yazarsanız, jenerik türün yerine hangi somut tür (concrete type) gelirse gelsin, o metot türün herhangi bir örneğinde tanımlanacaktır.

Bir tür üzerinde metotları tanımlarken jenerik türler üzerindeki kısıtlamaları da belirtebiliriz. Örneğin herhangi bir jenerik türdeki `Nokta<T>` örnekleri yerine yalnızca `Nokta<f32>` örnekleri üzerinde metotlar uygulayabilirdik. Liste 10-10'da somut tür `f32`'yi kullanıyoruz, bu da `impl`'den sonra herhangi bir tür bildirmediğimiz anlamına geliyor.

<Listing number="10-10" file-name="src/main.rs" caption="Yalnızca `T` jenerik tür parametresi için belirli bir somut türe sahip bir struct'a (yapıya) uygulanan bir `impl` bloğu">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

</Listing>

Bu kod `Nokta<f32>` türünün `orijinden_uzaklik` adlı bir metoda sahip olacağı anlamına gelir; `T`'nin `f32` türünde olmadığı diğer `Nokta<T>` örneklerinde bu metot tanımlı olmayacaktır. Bu metot, noktamızın (0.0, 0.0) koordinatlarındaki noktadan ne kadar uzakta olduğunu ölçer ve yalnızca ondalıklı türler için mevcut olan matematiksel işlemleri kullanır.

Bir struct tanımındaki jenerik tür parametreleri, aynı struct'ın metot imzalarında kullandıklarınızla her zaman aynı değildir. Liste 10-11, örneği daha açık hale getirmek için `Nokta` struct'ı için `X1` ve `Y1` jenerik türlerini ve `karistir` metodunun imzası için `X2` ve `Y2` jenerik türlerini kullanır. Bu metot, ( `X1` türünde olan) `self`'teki (kendisindeki) `Nokta` değerinin `x` değerinden ve ( `Y2` türünde olan) parametre olarak geçilen `Nokta` değerinin `y` değerinden yeni bir `Nokta` örneği yaratır.

<Listing number="10-11" file-name="src/main.rs" caption="Kendi struct'ının tanımından farklı jenerik türler kullanan bir metot">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

</Listing>

`main` fonksiyonunda, `x` için `i32` (değeri `5`) ve `y` için `f64` (değeri `10.4`) olan bir `Nokta` tanımladık. `p2` değişkeni `x` için string dilimine (değeri `"Merhaba"`) ve `y` için bir `char` türüne (değeri `'c'`) sahip bir `Nokta` yapısıdır. `p2` argümanıyla `p1` üzerinde `karistir` çağrısı yapmak bize `p3`'ü verir, ki bu da `x` için bir `i32`'ye sahip olacaktır çünkü `x` `p1`'den gelmiştir. `p3` değişkeninin `y` değeri `char` olacaktır çünkü `y` `p2`'den gelmiştir. `println!` makrosunun çağrılması `p3.x = 5, p3.y = c` yazdıracaktır.

Bu örneğin amacı, bazı jenerik parametrelerin `impl` ile bazı jenerik parametrelerin ise metot tanımıyla bildirildiği bir durumu göstermektir. Burada `X1` ve `Y1` jenerik parametreleri `impl` sonrasında bildirilir çünkü onlar struct tanımıyla birlikte giderler. `X2` ve `Y2` jenerik parametreleri `fn karistir`'dan (fn mixup) sonra bildirilir çünkü sadece metotla ilgilidirler.

### Jenerikleri Kullanan Kodların Performansı

Jenerik tür parametrelerini kullanırken bir çalışma zamanı maliyeti olup olmadığını merak ediyor olabilirsiniz. İyi haber şu ki, jenerik türler kullanmak programınızı somut türler kullandığınızdan daha yavaş çalıştırmayacaktır.

Rust derleme zamanında jenerikleri kullanarak kodun *monomorfizasyonunu* (monomorphization - tektipleştirme) gerçekleştirerek bunu başarır. _Monomorfizasyon_, derlendiğinde kullanılan somut türleri doldurarak jenerik kodu özel/spesifik koda dönüştürme işlemidir. Bu süreçte derleyici, Liste 10-5'te jenerik fonksiyonu yaratmak için kullandığımız adımların tersini yapar: Derleyici jenerik kodun çağrıldığı tüm yerlere bakar ve jenerik kodun birlikte çağrıldığı somut türler için kod üretir.

Standart kütüphanenin jenerik `Option<T>` enum'ını kullanarak bunun nasıl çalıştığına bakalım:

```rust
let tamsayi = Some(5);
let ondalikli = Some(5.0);
```

Rust bu kodu derlediğinde monomorfizasyon uygular. Bu süreç sırasında derleyici `Option<T>` örneklerinde kullanılmış olan değerleri okur ve iki tür `Option<T>` tespit eder: Biri `i32` diğeri ise `f64`. Bu şekilde, `Option<T>`'nin jenerik tanımını `i32` ve `f64`'e özel iki tanıma genişletir ve böylece jenerik tanımı belirli olanlarla değiştirir.

Kodun monomorfize edilmiş (tektipleştirilmiş) hali aşağıdakine benzer (derleyici burada örnek olarak kullandıklarımızdan farklı isimler kullanır):

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

Jenerik `Option<T>`, derleyici tarafından oluşturulan belirli tanımlarla değiştirilir. Rust jenerik kodu her bir örnekte türü belirten bir koda derlediğinden, jenerikleri kullanmak için hiçbir çalışma zamanı maliyeti ödemeyiz. Kod çalıştığında, tıpkı her tanımı elle kopyalamışız gibi performans gösterir. Monomorfizasyon süreci, Rust'ın jeneriklerini çalışma zamanında son derece verimli hale getirir.