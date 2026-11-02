<!-- Old headings. Do not remove or links may break. -->
<a id="generic-types-traits-and-lifetimes"></a>
<a id="jenerik-türler-traitler-ve-ömürler"></a>
<a id="jenerik-turler-traitler-ve-omurler"></a>

# Jenerik Türler, Trait'ler ve Ömürler

Her programlama dili, kavram tekrarını etkili biçimde ele almak için çeşitli araçlar sunar. Rust'ta bu araçlardan biri *jeneriklerdir (generics)*; yani somut türlerin veya diğer özelliklerin yerine geçen soyut yer tutuculardır. Kod derlenip çalıştırıldığında yerlerine ne geleceğini önceden bilmeden, jeneriklerin davranışını veya diğer jeneriklerle nasıl bir ilişki kurduğunu ifade edebiliriz.

Tıpkı fonksiyonların aynı kodu birden fazla somut değer üzerinde çalıştırabilmek için değeri önceden bilinmeyen parametreler alması gibi, fonksiyonlar `i32` veya `String` gibi somut bir tür yerine jenerik bir türde parametreler de alabilir. Aslında jenerikleri Bölüm 6'da `Option<T>`, Bölüm 8'de `Vec<T>` ve `HashMap<K, V>`, Bölüm 9'da ise `Result<T, E>` ile zaten kullandık. Bu bölümde kendi türlerinizi, fonksiyonlarınızı ve metotlarınızı jeneriklerle nasıl tanımlayacağınızı keşfedeceksiniz!

İlk olarak, kod tekrarını azaltmak için tekrarlayan mantığı ayrı bir fonksiyona çıkarmayı gözden geçireceğiz. Ardından aynı tekniği kullanarak, yalnızca parametre türleri bakımından farklılaşan iki fonksiyondan jenerik bir fonksiyon elde edeceğiz. Ayrıca struct ve enum tanımlarında jenerik türlerin nasıl kullanıldığını da inceleyeceğiz.

Daha sonra, davranışı jenerik bir biçimde tanımlamak için *trait*'leri nasıl kullanacağınızı öğreneceksiniz. Bir jenerik türü rastgele herhangi bir türü kabul etmek yerine yalnızca belirli bir davranışa sahip türleri kabul edecek şekilde sınırlandırmak için trait'leri jenerik türlerle birleştirebilirsiniz.

Son olarak, referansların birbiriyle nasıl ilişkili olduğu konusunda derleyiciye bilgi veren özel bir jenerik çeşidi olan *ömürleri (lifetimes)* ele alacağız. Ömürler, ödünç alınan değerler hakkında derleyiciye yeterli bilgi sunmamızı sağlar; böylece derleyici, bizim yardımımız olmadan doğrulayabileceğinden çok daha fazla senaryoda referansların geçerli kalacağını garanti altına alabilir.

> Not: Rust'ın tür sisteminde **jenerikler**, **trait'ler** ve **ömürler** birbirini tamamlayan üçlü bir yapı oluşturur:
>
> - **Jenerik türler (`<T>`)**, bir fonksiyonun veya veri yapısının *hangi türlerle* çalışabileceğini soyutlar (*"Ne tür bir veri?"*).
> - **Trait'ler (`T: PartialOrd` gibi sınırlarla)**, bu soyut türlerin sahip olması gereken *davranışları* tanımlar ve kısıtlar (*"Bu tür neler yapabilir?"*).
> - **Ömürler (`'a`)**, ödünç alınan referansların *geçerlilik kapsamlarını* birbiriyle ilişkilendirir (*"Bu referans hangi kapsam boyunca geçerlidir?"*).
>
> En önemli nokta ise bu üç mekanizmanın tamamının **derleme zamanında** çözümlenmesidir. Derleyici, jenerik tanımları programda kullanılan her somut tür için özelleştirilmiş makine koduna dönüştürür, trait sınırlarını doğrular ve ömür kurallarını denetledikten sonra ömür bildirimlerini ikili dosyadan tamamen siler. Böylece çalışma zamanında hiçbir ek bellek veya hız maliyeti ödemeden (*sıfır maliyetli soyutlama*) hem esnek hem de tam tür ve bellek güvenliğine sahip kod yazabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="removing-duplication-by-extracting-a-function"></a>
<a id="bir-fonksiyonu-çıkararak-tekrarlamayı-kaldırmak"></a>
<a id="bir-fonksiyonu-cikararak-tekrarlamayi-kaldirmak"></a>

## Fonksiyon Çıkararak Kod Tekrarını Kaldırma

Jenerikler, kod tekrarını ortadan kaldırmak amacıyla belirli türlerin yerine birden fazla türü temsil eden bir yer tutucu koymamızı sağlar. Jenerik sözdizimine geçmeden önce, jenerik türler kullanmadan kod tekrarını nasıl giderebileceğimize bakalım: belirli değerlerin yerine birden fazla değeri temsil eden bir parametre koyacak şekilde ayrı bir fonksiyon çıkaracağız. Ardından aynı tekniği jenerik bir fonksiyon çıkarmak için uygulayacağız! Bir fonksiyona aktarabileceğiniz tekrarlayan kodları nasıl tespit edeceğinizi gördükçe, jeneriklerden yararlanabilecek tekrarlayan kodları da kolayca fark etmeye başlayacaksınız.

Bir listedeki en büyük sayıyı bulan Liste 10-1'deki kısa programla başlayalım.

<Listing number="10-1" file-name="src/main.rs" caption="Bir sayı listesindeki en büyük sayıyı bulma">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

</Listing>

`sayi_listesi` değişkeninde bir tam sayı listesi saklar ve listedeki ilk sayının referansını `en_buyuk` adlı değişkene yerleştiririz. Ardından listedeki tüm sayılar üzerinde sırayla dolaşırız; o anki `sayi` değeri `en_buyuk` değişkeninin işaret ettiği sayıdan büyükse, `en_buyuk` içindeki referansı bu yeni sayıyı gösterecek şekilde güncelleriz. Buna karşılık, o anki sayı şimdiye dek görülen en büyük sayıdan küçük veya ona eşitse değişken değişmez ve kod listedeki bir sonraki sayıya geçer. Listedeki tüm sayılar incelendikten sonra `en_buyuk` değişkeni listedeki en büyük sayıyı işaret eder; bu örnekte söz konusu değer `100`'dür.

Şimdi iki farklı sayı listesindeki en büyük sayıyı bulmamız istendiğini düşünelim. Bunu yapmak için Liste 10-2'de gösterildiği gibi Liste 10-1'deki kodu kopyalayıp aynı mantığı programın iki farklı yerinde kullanmayı seçebiliriz.

<Listing number="10-2" file-name="src/main.rs" caption="*İki* sayı listesindeki en büyük sayıyı bulan kod">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

</Listing>

Bu kod çalışsa da kodu bu şekilde kopyalamak hem zahmetli hem de hataya açıktır. Üstelik mantıkta bir değişiklik yapmak istediğimizde kodu birden fazla yerde güncellemeyi unutmamamız gerekir.

Bu tekrarı ortadan kaldırmak için, parametre olarak geçirilen herhangi bir tam sayı listesi üzerinde çalışan bir fonksiyon tanımlayarak bir soyutlama oluşturacağız. Bu çözüm kodumuzu daha anlaşılır hâle getirir ve bir listedeki en büyük sayıyı bulma kavramını soyut bir biçimde ifade etmemizi sağlar.

Liste 10-3'te en büyük sayıyı bulan kodu `en_buyuk_bul` adlı bir fonksiyona çıkarıyoruz. Ardından Liste 10-2'deki iki listede yer alan en büyük sayıları bulmak için bu fonksiyonu çağırıyoruz. Gelecekte elimizde olabilecek başka herhangi bir `i32` değer listesi için de aynı fonksiyonu kullanabiliriz.

<Listing number="10-3" file-name="src/main.rs" caption="İki listedeki en büyük sayıyı bulmak üzere soyutlanmış kod">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

</Listing>

`en_buyuk_bul` fonksiyonu, fonksiyona geçirebileceğimiz herhangi bir somut `i32` dilimini (`&[i32]`) temsil eden `liste` adında bir parametreye sahiptir. Sonuç olarak fonksiyonu çağırdığımızda kod, o çağrıda geçirdiğimiz belirli değerler üzerinde çalışır.

Özetle, Liste 10-2'deki kodu Liste 10-3'teki hâline dönüştürürken izlediğimiz adımlar şunlardır:

1. Tekrarlanan kodu belirleyin.
1. Tekrarlanan kodu bir fonksiyonun gövdesine çıkarın; bu kodun girdilerini ve dönüş değerini fonksiyon imzasında belirtin.
1. Tekrarlanan kodun bulunduğu iki yeri, aynı kodu tekrar yazmak yerine bu fonksiyonu çağıracak şekilde güncelleyin.

Bir sonraki adımda, kod tekrarını azaltmak için aynı adımları jeneriklerle uygulayacağız. Tıpkı fonksiyon gövdesinin belirli değerler yerine soyut bir `liste` parametresi üzerinde çalışabilmesi gibi, jenerikler de kodun soyut türler üzerinde çalışmasına olanak tanır.

Örneğin elimizde iki fonksiyon olduğunu varsayalım: biri `i32` değerlerinden oluşan bir dilimdeki en büyük öğeyi, diğeri ise `char` değerlerinden oluşan bir dilimdeki en büyük öğeyi bulsun. Bu tekrarı nasıl ortadan kaldırırız? Gelin birlikte öğrenelim!
