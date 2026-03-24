# Jenerik Türler, Traitler ve Ömürler

Her programlama dili, kavramların tekrarını etkin bir şekilde ele almak için araçlara sahiptir. Rust'ta bu araçlardan biri _jeneriklerdir_: somut türler veya diğer özellikler için soyut yedeklerdir. Kod derlenip çalıştırılırken yerlerinde ne olacağını bilmeden jeneriklerin davranışını veya diğer jeneriklerle nasıl ilişki kurduklarını ifade edebiliriz.

Fonksiyonlar, `i32` veya `String` gibi somut bir tür yerine bir tür jenerik türe ait parametreler alabilir; tıpkı aynı kodu birden fazla somut değer üzerinde çalıştırmak için bilinmeyen değerlere sahip parametreler aldıkları gibi. Aslında, Bölüm 6'da `Option<T>` ile, Bölüm 8'de `Vec<T>` ve `HashMap<K, V>` ile ve Bölüm 9'da `Result<T, E>` ile jenerikleri zaten kullandık. Bu bölümde, kendi türlerinizi, fonksiyonlarınızı ve metotlarınızı jeneriklerle nasıl tanımlayacağınızı keşfedeceksiniz!

Önce kod tekrarını azaltmak için bir fonksiyonun nasıl çıkarılacağını gözden geçireceğiz. Ardından, yalnızca parametrelerinin türleri bakımından farklılık gösteren iki fonksiyondan jenerik bir fonksiyon oluşturmak için aynı tekniği kullanacağız. Ayrıca struct ve enum tanımlarında jenerik türlerin nasıl kullanılacağını açıklayacağız.

Daha sonra, davranışı jenerik bir şekilde tanımlamak için traitleri nasıl kullanacağınızı öğreneceksiniz. Herhangi bir tür yerine, yalnızca belirli bir davranışa sahip türleri kabul etmesi adına bir jenerik türü kısıtlamak için traitleri jenerik türlerle birleştirebilirsiniz.

Son olarak, derleyiciye referansların birbiriyle nasıl ilişki kurduğu hakkında bilgi veren bir çeşit jenerik olan _ömürleri_ tartışacağız. Ömürler, ödünç alınan değerler hakkında derleyiciye yeterli bilgi vermemizi sağlar, böylece referansların bizim yardımımız olmadan olabileceğinden daha fazla durumda geçerli kalacağından emin olabilir.

## Bir Fonksiyonu Çıkararak Tekrarlamayı Kaldırmak

Jenerikler, kod tekrarını kaldırmak için belirli türleri birden fazla türü temsil eden bir yer tutucuyla değiştirmemizi sağlar. Jenerik sözdizimine dalmadan önce, belirli değerleri birden fazla değeri temsil eden bir yer tutucuyla değiştiren bir fonksiyon çıkararak jenerik türleri içermeyen bir şekilde tekrarı nasıl kaldıracağımıza bakalım. Sonra, jenerik bir fonksiyonu çıkarmak için aynı tekniği uygulayacağız! Bir fonksiyona çıkarabileceğiniz tekrarlayan kodu nasıl tanıyacağınızı görerek, jenerikleri kullanabilecek tekrarlayan kodları tanımaya başlayacaksınız.

Bir listedeki en büyük sayıyı bulan, Liste 10-1'deki kısa programla başlayacağız.

<Listing number="10-1" file-name="src/main.rs" caption="Bir sayı listesindeki en büyük sayıyı bulmak">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

</Listing>

`sayi_listesi` değişkeninde bir tamsayı listesi saklıyoruz ve listedeki ilk sayının referansını `en_buyuk` adlı bir değişkene yerleştiriyoruz. Ardından listedeki tüm sayılar üzerinde yineleme yapıyoruz ve eğer o anki sayı `en_buyuk` içinde depolanan sayıdan daha büyükse o değişkendeki referansı değiştiriyoruz. Ancak, o anki sayı şimdiye kadar görülen en büyük sayıdan küçük veya ona eşitse değişken değişmez ve kod listedeki bir sonraki sayıya geçer. Listedeki tüm sayıları değerlendirdikten sonra, `en_buyuk` en büyük sayıya referans vermelidir, bu örnekte o da 100'dür.

Şimdi iki farklı sayı listesindeki en büyük sayıyı bulmakla görevlendirildik. Bunu yapmak için, Liste 10-2'de gösterildiği gibi Liste 10-1'deki kodu kopyalamayı ve aynı mantığı programın iki farklı yerinde kullanmayı seçebiliriz.

<Listing number="10-2" file-name="src/main.rs" caption="*İki* sayı listesindeki en büyük sayıyı bulmak için kod">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

</Listing>

Bu kod çalışmasına rağmen, kodu kopyalamak yorucu ve hataya açıktır. Ayrıca, kodu değiştirmek istediğimizde birden fazla yerde güncellemeyi de unutmamalıyız.

Bu tekrarı ortadan kaldırmak için, parametre olarak aktarılan herhangi bir tamsayı listesi üzerinde çalışan bir fonksiyon tanımlayarak bir soyutlama oluşturacağız. Bu çözüm kodumuzu daha açık hale getirir ve bir listedeki en büyük sayıyı bulma kavramını soyut bir şekilde ifade etmemizi sağlar.

Liste 10-3'te, en büyük sayıyı bulan kodu `en_buyuk_bul` adlı bir fonksiyona çıkarıyoruz. Ardından Liste 10-2'deki iki listedeki en büyük sayıyı bulmak için fonksiyonu çağırıyoruz. Bu fonksiyonu gelecekte sahip olabileceğimiz diğer herhangi bir `i32` değer listesinde de kullanabiliriz.

<Listing number="10-3" file-name="src/main.rs" caption="İki listedeki en büyük sayıyı bulmak için soyutlanmış kod">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

</Listing>

`en_buyuk_bul` fonksiyonu, fonksiyona aktarabileceğimiz herhangi bir somut `i32` değer dilimini temsil eden `liste` adında bir parametreye sahiptir. Sonuç olarak, fonksiyonu çağırdığımızda, kod aktardığımız belirli değerler üzerinde çalışır.

Özetle, Liste 10-2'deki kodu Liste 10-3'e dönüştürmek için attığımız adımlar şunlardır:

1. Tekrarlanan kodu belirleyin.
1. Tekrarlanan kodu fonksiyonun gövdesine çıkarın ve o kodun girdilerini ve dönüş değerlerini fonksiyon imzasında belirtin.
1. Bunun yerine fonksiyonu çağırmak için tekrarlanan kodun iki örneğini güncelleyin.

Sırada, kod tekrarını azaltmak için aynı adımları jeneriklerle kullanacağız. Fonksiyon gövdesinin belirli değerler yerine soyut bir `liste` üzerinde çalışabilmesi gibi, jenerikler de kodun soyut türler üzerinde çalışmasına izin verir.

Örneğin, iki fonksiyonumuz olduğunu varsayalım: biri `i32` değerlerinden oluşan bir dilimdeki en büyük öğeyi, diğeri ise `char` değerlerinden oluşan bir dilimdeki en büyük öğeyi bulan bir fonksiyon. Bu tekrarı nasıl ortadan kaldırırdık? Hadi öğrenelim!