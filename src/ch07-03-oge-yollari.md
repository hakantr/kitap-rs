<!-- Old headings. Do not remove or links may break. -->

<a id="paths-for-referring-to-an-item-in-the-module-tree"></a>
<a id="modül-ağacındaki-bir-öğeye-başvurmak-için-yollar-paths"></a>
<a id="modul-agacindaki-bir-ogeye-basvurmak-icin-yollar-paths"></a>
<a id="modul-agacindaki-bir-ogeye-basvurmak-icin-yollar"></a>

## Modül Ağacındaki Bir Öğeye Başvurmak İçin Yollar

Rust'a modül ağacındaki bir öğeyi nerede bulacağını göstermek için, tıpkı bir dosya sisteminde gezinirken kullandığımız gibi bir *yol (path)* kullanırız. Bir fonksiyonu çağırmak için onun yolunu bilmemiz gerekir.

Bir yol iki biçimde yazılabilir:

- _Mutlak yol (absolute path)_, crate kökünden başlayan tam yoldur; harici bir crate'teki kod için crate adıyla, içinde bulunduğumuz crate'teki kod içinse doğrudan `crate` kelimesiyle başlar.
- _Göreli yol (relative path)_, geçerli modülden başlar ve `self`, `super` ya da geçerli modül içindeki bir tanımlayıcıyı kullanır.

Hem mutlak hem de göreli yollarda başlangıç ifadesini, birbirinden çift iki nokta üst üste (`::`) ile ayrılmış bir veya daha fazla tanımlayıcı izler.

Liste 7-1'e geri dönelim ve `bekleme_listesine_ekle` fonksiyonunu çağırmak istediğimizi varsayalım. Bu aslında şu soruyu sormakla aynıdır: `bekleme_listesine_ekle` fonksiyonunun yolu nedir? Liste 7-3, Liste 7-1'in bazı modül ve fonksiyonları çıkarılmış hâlini içerir.

Crate kökünde tanımlanan `restoranda_yemek_ye` adlı yeni bir fonksiyondan `bekleme_listesine_ekle` fonksiyonunu çağırmanın iki yolunu göstereceğiz. Bu yollar doğrudur; ancak örneğin bu hâliyle derlenmesini engelleyecek başka bir sorun daha vardır. Nedenini birazdan açıklayacağız.

`restoranda_yemek_ye` fonksiyonu, kütüphane crate'imizin açık API'sinin bir parçası olduğu için onu `pub` anahtar kelimesiyle işaretliyoruz. ["Yolları `pub` Anahtar Kelimesiyle Dışarıya Açma"][pub]<!-- ignore --> kısmında `pub` hakkında daha fazla ayrıntıya gireceğiz.

<Listing number="7-3" file-name="src/lib.rs" caption="`bekleme_listesine_ekle` fonksiyonunu mutlak ve göreli yollar kullanarak çağırma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

</Listing>

`restoranda_yemek_ye` içinde `bekleme_listesine_ekle` fonksiyonunu ilk çağırdığımızda mutlak bir yol kullanıyoruz. `bekleme_listesine_ekle` fonksiyonu `restoranda_yemek_ye` ile aynı crate içinde tanımlandığından, mutlak yolu başlatmak için `crate` anahtar kelimesini kullanabiliriz. Ardından `bekleme_listesine_ekle` fonksiyonuna ulaşana kadar birbirini izleyen modüllerin her birini yola dâhil ederiz. Aynı yapıya sahip bir dosya sistemi hayal edebilirsiniz: `bekleme_listesine_ekle` programını çalıştırmak için `/restoran_on_kisim/karsilama/bekleme_listesine_ekle` yolunu belirtirdik; crate kökünden başlamak üzere `crate` adını kullanmak, kabuğunuzda dosya sistemi kökünden başlamak için `/` kullanmaya benzer.

`restoranda_yemek_ye` içinde `bekleme_listesine_ekle` fonksiyonunu ikinci kez çağırdığımızda ise göreli bir yol kullanıyoruz. Yol, modül ağacında `restoranda_yemek_ye` ile aynı seviyede tanımlanmış olan `restoran_on_kisim` modülünün adıyla başlar. Bunun dosya sistemindeki karşılığı `restoran_on_kisim/karsilama/bekleme_listesine_ekle` yolunu kullanmak olurdu. Bir modül adıyla başlamak, yolun göreli olduğu anlamına gelir.

Göreli mi yoksa mutlak yol mu kullanacağınız, projenize göre vereceğiniz bir karardır ve öğeyi tanımlayan kodu, o öğeyi kullanan koddan ayrı mı yoksa onunla birlikte mi taşıma olasılığınızın daha yüksek olduğuna bağlıdır. Örneğin `restoran_on_kisim` modülünü ve `restoranda_yemek_ye` fonksiyonunu birlikte `musteri_deneyimi` adlı bir modülün içine taşırsak, `bekleme_listesine_ekle` için yazdığımız mutlak yolu güncellememiz gerekir; ancak göreli yol geçerliliğini korur. Buna karşılık `restoranda_yemek_ye` fonksiyonunu tek başına `yemek` adlı ayrı bir modüle taşırsak, `bekleme_listesine_ekle` çağrısının mutlak yolu aynı kalır ama göreli yolun güncellenmesi gerekir. Kod tanımlarını ve öğe çağrılarını birbirinden bağımsız olarak taşıma ihtimalimiz daha yüksek olduğundan, genel tercihimiz mutlak yolları kullanmaktır.

Şimdi Liste 7-3'ü derlemeyi deneyelim ve henüz neden derlenmediğini öğrenelim! Aldığımız hatalar Liste 7-4'te gösterilmiştir.

<Listing number="7-4" caption="Liste 7-3'teki kodu derlerken alınan derleyici hataları">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

</Listing>

Hata mesajları `karsilama` modülünün gizli olduğunu söylüyor (`module 'karsilama' is private`). Başka bir deyişle, `karsilama` modülü ve `bekleme_listesine_ekle` fonksiyonu için yazdığımız yollar doğrudur; ancak gizli bölümlere erişim iznimiz olmadığı için Rust bunları kullanmamıza izin vermez. Rust'ta tüm öğeler (fonksiyonlar, metotlar, struct'lar, enum'lar, modüller ve sabitler) üst modüllere karşı varsayılan olarak gizlidir. Bir fonksiyon veya struct gibi bir öğeyi gizli yapmak istiyorsanız onu bir modülün içine yerleştirirsiniz.

Üst modüldeki öğeler alt modüllerin içindeki gizli öğeleri kullanamaz; ancak alt modüllerdeki öğeler kendi ata modüllerindeki öğeleri kullanabilir. Bunun nedeni, alt modüllerin kendi uygulama ayrıntılarını sarmalayıp gizlemesi, buna karşılık tanımlandıkları dış bağlamı görebilmesidir. Benzetmemize devam edecek olursak, gizlilik kurallarını bir restoranın arka ofisi gibi düşünebilirsiniz: Orada olup bitenler restoran müşterilerine karşı gizlidir, ancak ofis yöneticileri işlettikleri restorandaki her şeyi görebilir ve yönetebilir.

Rust'ta modül sistemi, iç uygulama ayrıntılarını gizlemek varsayılan davranış olacak şekilde tasarlanmıştır. Bu sayede içteki kodun hangi kısımlarını dıştaki kodu bozmadan değiştirebileceğinizi net biçimde bilirsiniz. Yine de Rust, bir öğeyi açık hâle getiren `pub` anahtar kelimesini kullanarak alt modüllerdeki kodun iç kısımlarını dıştaki ata modüllere açma seçeneğini de size sunar.

<!-- Old headings. Do not remove or links may break. -->

<a id="exposing-paths-with-the-pub-keyword"></a>
<a id="yolları-pub-anahtar-kelimesiyle-açığa-çıkarmak-exposing"></a>
<a id="yollari-pub-anahtar-kelimesiyle-aciga-cikarmak-exposing"></a>
<a id="yollari-pub-anahtar-kelimesiyle-disariya-acma"></a>

### Yolları `pub` Anahtar Kelimesiyle Dışarıya Açma

Liste 7-4'te `karsilama` modülünün gizli olduğunu bildiren hataya geri dönelim. Üst modüldeki `restoranda_yemek_ye` fonksiyonunun, alt modüldeki `bekleme_listesine_ekle` fonksiyonuna erişebilmesini istiyoruz; bu yüzden `karsilama` modülünü Liste 7-5'te gösterildiği gibi `pub` anahtar kelimesiyle işaretliyoruz.

<Listing number="7-5" file-name="src/lib.rs" caption="`restoranda_yemek_ye` içinden kullanabilmek için `karsilama` modülünü `pub` olarak bildirme">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs:here}}
```

</Listing>

Ne yazık ki Liste 7-5'teki kod da Liste 7-6'da gösterildiği gibi hâlâ derleyici hatalarıyla sonuçlanır.

<Listing number="7-6" caption="Liste 7-5'teki kodu derlerken alınan derleyici hataları">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

</Listing>

Ne oldu? `mod karsilama` tanımının önüne `pub` anahtar kelimesini eklemek modülü açık hâle getirir. Bu değişiklikle birlikte `restoran_on_kisim` modülüne erişebiliyorsak `karsilama` modülüne de erişebiliriz. Ancak `karsilama` modülünün _içeriği_ hâlâ gizlidir; modülü açık yapmak içindeki öğeleri otomatik olarak açık yapmaz. Bir modül üzerindeki `pub` anahtar kelimesi, yalnızca ata modüllerindeki kodların o modüle başvurmasına izin verir, içindeki koda erişmesini sağlamaz. Modüller birer kapsayıcı olduğundan yalnızca modülü açık hâle getirerek yapabileceğimiz pek bir şey yoktur; bir adım daha ileri gidip modül içindeki öğelerden birini veya birkaçını da açık hâle getirmeyi seçmeliyiz.

Liste 7-6'daki hatalar `bekleme_listesine_ekle` fonksiyonunun gizli olduğunu söylüyor (`function 'bekleme_listesine_ekle' is private`). Gizlilik kuralları modüllerin yanı sıra struct'lar, enum'lar, fonksiyonlar ve metotlar için de geçerlidir.

Liste 7-7'de olduğu gibi tanımının önüne `pub` anahtar kelimesini ekleyerek `bekleme_listesine_ekle` fonksiyonunu da açık hâle getirelim.

<Listing number="7-7" file-name="src/lib.rs" caption="`mod karsilama` ve `fn bekleme_listesine_ekle` tanımlarına `pub` anahtar kelimesini eklemek, fonksiyonu `restoranda_yemek_ye` içinden çağırmamızı sağlar.">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs:here}}
```

</Listing>

Artık kod derlenecektir! `pub` anahtar kelimesini eklemenin, gizlilik kuralları çerçevesinde bu yolları `restoranda_yemek_ye` içinde kullanmamıza neden izin verdiğini görmek için mutlak ve göreli yollara adım adım bakalım.

Mutlak yolda, crate'imizin modül ağacının kökü olan `crate` ile başlarız. `restoran_on_kisim` modülü crate kökünde tanımlanmıştır. `restoran_on_kisim` açık olmasa da `restoranda_yemek_ye` fonksiyonu `restoran_on_kisim` ile aynı modülde tanımlandığı için (yani `restoranda_yemek_ye` ve `restoran_on_kisim` aynı kapsamdaki kardeş öğeler olduğu için), `restoranda_yemek_ye` içinden `restoran_on_kisim` modülüne başvurabiliriz. Sırada `pub` ile işaretlenmiş `karsilama` modülü vardır. `karsilama` modülünün üst modülüne erişebildiğimiz için `karsilama` modülüne de erişebiliriz. Son olarak `bekleme_listesine_ekle` fonksiyonu `pub` ile işaretlenmiştir ve üst modülüne erişebildiğimiz için bu fonksiyon çağrısı sorunsuz çalışır!

Göreli yolda ise ilk adım dışında mantık mutlak yolla tamamen aynıdır: Yol, crate kökünden başlamak yerine `restoran_on_kisim` ile başlar. `restoran_on_kisim` modülü `restoranda_yemek_ye` ile aynı modül içinde tanımlandığından, `restoranda_yemek_ye` fonksiyonunun tanımlı olduğu modülden başlayan göreli yol geçerlidir. Ardından hem `karsilama` modülü hem de `bekleme_listesine_ekle` fonksiyonu `pub` ile işaretlendiği için yolun geri kalanı da erişilebilirdir ve fonksiyon çağrısı geçerlidir!

> Not: Rust'ta görünürlük yalnızca "tamamen gizli" veya "herkese açık (`pub`)" seçenekleriyle sınırlı değildir. Bir öğeyi dış dünyaya (harici crate'lere) açmadan yalnızca kendi projeniz içinde görünür kılmak istediğinizde kısıtlı görünürlük belirleyicilerini kullanabilirsiniz: `pub(crate)` öğeyi yalnızca geçerli crate genelinde açık yapar, `pub(super)` yalnızca bir üst modüle açar, `pub(in crate::yol)` ise yalnızca belirtilen ata modülün alt ağacına erişim izni verir. Özellikle kütüphane geliştirirken iç modüllerin birbiriyle paylaştığı yardımcı tür ve fonksiyonları `pub(crate)` ile işaretlemek, dış API'yi kirletmeden modülerliği korumanın en yaygın yoludur.

Kütüphane crate'inizi diğer projelerin de kodunuzu kullanabilmesi için paylaşmayı planlıyorsanız açık API'niz, crate'inizin kullanıcılarıyla yaptığınız ve kodunuzla nasıl etkileşime girebileceklerini belirleyen sözleşmenizdir. İnsanların crate'inize güvenle bağımlı olmasını kolaylaştırmak adına açık API'nizdeki değişiklikleri yönetirken dikkat edilmesi gereken pek çok nokta vardır. Bu konular bu kitabın kapsamı dışındadır; konuyla ilgileniyorsanız [Rust API Yönergeleri][api-guidelines] kılavuzunu inceleyebilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="best-practices-for-packages-with-a-binary-and-a-library"></a>
<a id="bir-i̇kili-binary-ve-kütüphane-library-i̇çeren-paketler-i̇çin-en-i̇yi-pratikler"></a>
<a id="bir-ikili-binary-ve-kütüphane-library-içeren-paketler-için-en-iyi-pratikler"></a>
<a id="bir-ikili-binary-ve-kutuphane-library-iceren-paketler-icin-en-iyi-pratikler"></a>

> #### Hem İkili Hem de Kütüphane Crate'i İçeren Paketler İçin En İyi Uygulamalar
>
> Bir paketin hem _src/main.rs_ ikili crate kökünü hem de _src/lib.rs_ kütüphane crate kökünü barındırabileceğinden ve her iki crate'in de varsayılan olarak paketin adını taşıyacağından bahsetmiştik. Genellikle bu düzende yapılandırılan paketlerde ikili crate, yalnızca çalıştırılabilir programı başlatıp kütüphane crate'inde tanımlanan kodu çağıracak kadar kod içerir. Kütüphane crate'inin kodu paylaşılabildiği için bu yapı, paketin sunduğu işlevselliğin büyük bölümünden diğer projelerin de yararlanabilmesini sağlar.
>
> Modül ağacı _src/lib.rs_ içinde tanımlanmalıdır. Böylece dışarıya açık her öğe, yollara paketin adıyla başlanarak ikili crate içinde kullanılabilir (örneğin _src/main.rs_ içinden `crate::` yerine `restoran::` ile başvurulur). İkili crate, tıpkı tamamen harici bir crate gibi kütüphane crate'inin bir kullanıcısı hâline gelir ve yalnızca açık API'yi kullanabilir. Bu yaklaşım iyi bir API tasarlamanıza yardımcı olur; çünkü kütüphanenin yalnızca yazarı değil, aynı zamanda ilk istemcisisinizdir!
>
> [Bölüm 12'de][ch12]<!-- ignore -->, hem ikili crate hem de kütüphane crate'i içeren bir komut satırı programıyla bu organizasyon yaklaşımını uygulamalı olarak göstereceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="starting-relative-paths-with-super"></a>
<a id="super-i̇le-göreceli-relative-yollara-başlamak"></a>
<a id="super-ile-göreceli-relative-yollara-başlamak"></a>
<a id="super-ile-goreceli-relative-yollara-baslamak"></a>
<a id="super-ile-goreli-yollara-baslama"></a>

### `super` ile Göreli Yollara Başlama

Yolun başında `super` kullanarak geçerli modül veya crate kökü yerine doğrudan üst modülden başlayan göreli yollar oluşturabiliriz. Bu, bir dosya sistemi yoluna bir üst dizine çıkmak anlamına gelen `..` sözdizimiyle başlamaya benzer. `super` kullanmak, üst modülde yer aldığını bildiğimiz bir öğeye başvurmamızı sağlar; modül üst modülüyle yakından ilişkiliyse ve üst modülün ileride modül ağacında başka bir konuma taşınma ihtimali varsa bu yöntem modül ağacını yeniden düzenlemeyi kolaylaştırır.

Bir şefin hatalı bir siparişi düzeltip bizzat müşteriye götürdüğü durumu modelleyen Liste 7-8'deki kodu ele alalım. `restoran_arka_kisim` modülünde tanımlanan `yanlis_siparisi_duzelt` fonksiyonu, `super` ile başlayan yolu belirterek üst modülde tanımlı `siparisi_teslim_et` fonksiyonunu çağırır.

<Listing number="7-8" file-name="src/lib.rs" caption="`super` ile başlayan göreli bir yol kullanarak fonksiyon çağırma">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

</Listing>

`yanlis_siparisi_duzelt` fonksiyonu `restoran_arka_kisim` modülü içinde yer aldığından, `restoran_arka_kisim` modülünün üst modülüne — bu örnekte kök modül olan `crate` kapsamına — çıkmak için `super` kullanabiliriz. Oradan `siparisi_teslim_et` fonksiyonunu arar ve buluruz. Başardık! `restoran_arka_kisim` modülü ile `siparisi_teslim_et` fonksiyonunun birbiriyle ilişkili kalacağını ve crate'in modül ağacını yeniden düzenlemeye karar verirsek birlikte taşınacaklarını düşünüyoruz. Bu nedenle `super` kullandık; böylece bu kod gelecekte başka bir modüle taşındığında güncellememiz gereken yer sayısı daha az olacaktır.

<!-- Old headings. Do not remove or links may break. -->

<a id="making-structs-and-enums-public"></a>
<a id="structları-ve-enumları-herkese-açık-public-yapmak"></a>
<a id="struct-ları-ve-enum-ları-herkese-açık-public-yapmak"></a>
<a id="structlari-ve-enumlari-herkese-acik-public-yapmak"></a>
<a id="structlari-ve-enumlari-acik-yapma"></a>

### Struct'ları ve Enum'ları Açık Yapma

Struct'ları ve enum'ları açık olarak tanımlamak için de `pub` anahtar kelimesini kullanabiliriz; ancak `pub` kullanımının struct'lar ve enum'larla ilgili birkaç ek ayrıntısı vardır. Bir struct tanımının önüne `pub` koyarsak struct'ın kendisini açık yaparız, fakat struct'ın alanları gizli kalmaya devam eder. Her bir alanı duruma göre tek tek açık yapıp yapmamayı seçebiliriz. Liste 7-9'da açık bir `tost` alanına ve gizli bir `mevsim_meyvesi` alanına sahip olan açık bir `restoran_arka_kisim::Kahvalti` struct'ı tanımladık. Bu yapı, müşterinin yemekle birlikte gelen ekmek türünü seçebildiği, ancak yemeğe hangi meyvenin eşlik edeceğine mevsime ve stok durumuna göre şefin karar verdiği gerçek bir restoran işleyişini modeller. Eldeki meyveler hızla değiştiği için müşteriler meyveyi seçemez, hatta hangi meyveyi alacaklarını bile göremez.

<Listing number="7-9" file-name="src/lib.rs" caption="Bazı alanları açık, bazı alanları gizli olan bir struct">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

</Listing>

`restoran_arka_kisim::Kahvalti` struct'ındaki `tost` alanı açık olduğu için `restoranda_yemek_ye` içinde nokta gösterimini kullanarak `tost` alanına yazabilir ve bu alanı okuyabiliriz. Buna karşılık `mevsim_meyvesi` alanı gizli olduğundan `restoranda_yemek_ye` içinde bu alanı kullanamayız. Nasıl bir derleyici hatası aldığınızı görmek için `mevsim_meyvesi` alanının değerini değiştiren satırı yorumdan çıkarmayı deneyin!

Ayrıca `restoran_arka_kisim::Kahvalti` gizli bir alana sahip olduğu için, bu struct'ın bir `Kahvalti` örneği oluşturan açık bir ilişkili fonksiyon sunması gerektiğine dikkat edin (burada ona `yaz` adını verdik). `Kahvalti` böyle bir fonksiyona sahip olmasaydı, `restoranda_yemek_ye` içinde gizli `mevsim_meyvesi` alanına değer atayamayacağımız için `Kahvalti` türünden bir örnek de oluşturamazdık.

Buna karşılık bir enum'ı açık yaparsak tüm varyantları da otomatik olarak açık olur. Liste 7-10'da gösterildiği gibi yalnızca `enum` anahtar kelimesinden önce `pub` yazmamız yeterlidir.

<Listing number="7-10" file-name="src/lib.rs" caption="Bir enum'ı açık olarak tanımlamak, tüm varyantlarını da açık hâle getirir.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

</Listing>

`Meze` enum'ını açık hâle getirdiğimiz için `restoranda_yemek_ye` içinde `Corba` ve `Salata` varyantlarını doğrudan kullanabiliriz.

Varyantları açık olmadıkça enum'lar pek işe yaramaz; her seferinde tüm enum varyantlarına tek tek `pub` eklemek zorunda kalmak zahmetli olacağından enum varyantlarının varsayılan durumu açık olmaktır. Struct'lar ise alanları dışarıya açık olmadan da çoğu zaman oldukça kullanışlıdır; bu nedenle struct alanları, `pub` ile aksi belirtilmedikçe her şeyin varsayılan olarak gizli olduğu genel kuralı izler.

> Not: `pub struct` ile `pub enum` arasındaki bu fark, iki veri yapısının tasarım amacından kaynaklanır. Bir struct alanlarını bir arada tutar ve çoğu zaman alanlar arasında korunması gereken *tutarlılık kuralları (invariants)* barındırır: En az bir alanı gizli bıraktığınızda dışarıdaki kod ne doğrudan `Kahvalti { ... }` sözdizimiyle denetimsiz bir örnek oluşturabilir ne de gizli alanı değiştirebilir (Liste 7-9'daki yorum satırını açtığınızda `error[E0616]: field 'mevsim_meyvesi' of struct 'Kahvalti' is private` hatası alırsınız); tüm örnekler `Kahvalti::yaz` gibi kuralları koruyan açık fonksiyonlar üzerinden oluşturulmak zorundadır. Benzer kural demet struct'ları için de geçerlidir; örneğin `pub struct Nokta(pub i32, i32);` tanımında ilk alan açıkken ikinci alan gizlidir. Bir enum ise olası durumlardan yalnızca birini temsil eder ve temel kullanım biçimi `match` ya da `if let` ile tüm varyantları kapsamlı (*exhaustive*) biçimde eşleştirmektir; varyantlardan biri gizli olsaydı dışarıdaki kod bu eşleştirmeyi yapamazdı. Bir enum varyantının taşıdığı verinin iç ayrıntılarını gizlemek istediğinizde ise varyantın içine alanları gizli bir `pub struct` yerleştirebilirsiniz.

Henüz ele almadığımız ve `pub` ile ilgili bir durum daha var; o da modül sisteminin son özelliği olan `use` anahtar kelimesidir. Önce `use` anahtar kelimesini tek başına inceleyecek, ardından `pub` ve `use` anahtar kelimelerini nasıl birleştireceğimizi göstereceğiz.

[pub]: ch07-03-oge-yollari.html#yolları-pub-anahtar-kelimesiyle-dışarıya-açma
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-io-projesi.html
