## Modül Ağacındaki Bir Öğeye Başvurmak İçin Yollar (Paths)

Rust'a modül ağacında bir öğeyi nerede bulacağını göstermek için, dosya sisteminde gezinirken (navigate) bir yol (path) kullandığımız şekilde bir yol kullanırız. Bir fonksiyonu çağırmak için onun yolunu bilmemiz gerekir.

Bir yol iki şekilde olabilir:

- _Mutlak yol_, crate kökünden başlayan tam yoldur; harici bir crate'den gelen kod için mutlak yol crate adıyla başlar ve mevcut crate'ten gelen kod için değişmez `crate` kelimesiyle başlar.
- _Göreceli yol_, geçerli modülden başlar ve mevcut modüldeki `self`, `super` veya bir tanımlayıcı kullanır.

Hem mutlak hem de göreceli yollar, birbirinden çift iki nokta üst üste (`::`) ile ayrılmış bir veya daha fazla tanımlayıcı tarafından takip edilir.

Liste 7-1'e dönecek olursak, `bekleme_listesine_ekle` (add_to_waitlist) fonksiyonunu çağırmak istediğimizi varsayalım. Bu aslında şu soruyu sormakla aynıdır: `bekleme_listesine_ekle` fonksiyonunun yolu nedir? Liste 7-3, bazı modülleri ve fonksiyonları çıkarılmış halde Liste 7-1'i içermektedir.

Crate kökünde tanımlanan `restoranda_yemek_ye` (eat_at_restaurant) adındaki yeni bir fonksiyondan `bekleme_listesine_ekle` fonksiyonunu çağırmanın iki yolunu göstereceğiz. Bu yollar doğrudur, ancak bu örneğin olduğu gibi derlenmesini engelleyecek başka bir sorun daha vardır. Nedenini birazdan açıklayacağız.

`restoranda_yemek_ye` fonksiyonu kütüphane (library) crate'imizin herkese açık API'sinin bir parçasıdır, bu yüzden onu `pub` anahtar kelimesiyle işaretliyoruz. ["Yolları `pub` Anahtar Kelimesiyle Açığa Çıkarmak" (Exposing Paths with the `pub` Keyword)][pub]<!-- ignore --> bölümünde `pub` hakkında daha fazla detaya gireceğiz.

<Listing number="7-3" file-name="src/lib.rs" caption="`bekleme_listesine_ekle` fonksiyonunu mutlak ve göreceli yollar kullanarak çağırmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

</Listing>

`restoranda_yemek_ye` fonksiyonu içinde `bekleme_listesine_ekle` fonksiyonunu ilk çağırdığımızda mutlak (absolute) bir yol kullanıyoruz. `bekleme_listesine_ekle` fonksiyonu `restoranda_yemek_ye` ile aynı crate içinde tanımlanmıştır, yani mutlak bir yolu başlatmak için `crate` anahtar kelimesini kullanabiliriz. Daha sonra `bekleme_listesine_ekle`'ye ulaşana kadar ardışık (successive) modüllerin her birini dahil ederiz. Aynı yapıya sahip bir dosya sistemi hayal edebilirsiniz: `bekleme_listesine_ekle` programını çalıştırmak için `/restoran_on_kisim/karsilama/bekleme_listesine_ekle` yolunu belirtirdik; crate kökünden başlamak için `crate` adını kullanmak, kabuğunuzda dosya sistemi kökünden başlamak için `/` kullanmaya benzer.

`restoranda_yemek_ye` fonksiyonu içinde `bekleme_listesine_ekle` fonksiyonunu ikinci kez çağırdığımızda ise göreceli (relative) bir yol kullanırız. Yol, modül ağacında `restoranda_yemek_ye` ile aynı seviyede tanımlanmış olan modülün adı `restoran_on_kisim` ile başlar. Buradaki dosya sistemi karşılığı, `restoran_on_kisim/karsilama/bekleme_listesine_ekle` yolunu kullanmak olurdu. Bir modül adıyla başlamak, yolun göreceli olduğu anlamına gelir.

Göreceli veya mutlak bir yol kullanıp kullanmama seçimi projenize bağlı olarak vereceğiniz bir karardır ve bu seçim, öğe tanımlama kodunu, öğeyi kullanan koddan ayrı olarak mı yoksa birlikte mi taşıma (move) ihtimalinizin daha yüksek olduğuna bağlıdır. Örneğin, `restoran_on_kisim` modülünü ve `restoranda_yemek_ye` fonksiyonunu `musteri_deneyimi` (customer_experience) adında bir modüle taşırsak, `bekleme_listesine_ekle`'nin mutlak yolunu güncellememiz gerekir, ancak göreceli yol geçerliliğini korur. Bununla birlikte, eğer `restoranda_yemek_ye` fonksiyonunu ayrı olarak `yemek` (dining) adında bir modüle taşırsak, `bekleme_listesine_ekle` çağrısına giden mutlak yol aynı kalır, ancak göreceli yolun güncellenmesi gerekir. Genel tercihimiz mutlak yolları belirtmektir, çünkü kod tanımlarını ve öğe çağrılarını birbirinden bağımsız olarak taşımak isteme olasılığımız daha yüksektir.

Liste 7-3'ü derlemeye çalışalım ve henüz neden derlenmediğini öğrenelim! Aldığımız hatalar Liste 7-4'te gösterilmektedir.

<Listing number="7-4" caption="Liste 7-3'teki kod derlenirken alınan derleyici hataları">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

</Listing>

Hata mesajları `karsilama` (hosting) modülünün gizli olduğunu söylüyor. Başka bir deyişle, `karsilama` modülü ve `bekleme_listesine_ekle` fonksiyonu için doğru yollara sahibiz, ancak Rust gizli bölümlere erişimi olmadığı için bunları kullanmamıza izin vermiyor. Rust'ta tüm öğeler (fonksiyonlar, metotlar, struct'lar, enum'lar, modüller ve sabitler) varsayılan olarak üst modüllere gizlidir. Eğer bir fonksiyon veya struct gibi bir öğeyi gizli yapmak isterseniz, onu bir modülün içine koyarsınız.

Üst bir modüldeki öğeler alt modüllerin içindeki gizli öğeleri kullanamaz, ancak alt modüllerdeki öğeler ata modüllerindeki öğeleri kullanabilir. Bunun nedeni alt modüllerin uygulama ayrıntılarını (implementation details) sarmalaması ve gizlemesidir, ancak alt modüller tanımlandıkları bağlamı görebilirler. Metaforumuzla devam edecek olursak, gizlilik kurallarının bir restoranın arka ofisi (back office) gibi olduğunu düşünün: Orada olup bitenler restoran müşterilerine gizlidir, ancak ofis yöneticileri (managers) işlettikleri restorandaki her şeyi görebilir ve yapabilirler.

Rust modül sisteminin bu şekilde çalışmasını seçti, böylece iç uygulama detaylarını (inner implementation details) gizlemek varsayılan oldu. Bu şekilde, içteki kodun (inner code) hangi kısımlarını dıştaki kodu (outer code) bozmadan değiştirebileceğinizi bilirsiniz. Ancak Rust, bir öğeyi açık hale getirmek için `pub` anahtar kelimesini kullanarak alt modüllerin kodunun iç kısımlarını dıştaki ata modüllere açma seçeneğini de sunar.

### Yolları `pub` Anahtar Kelimesiyle Açığa Çıkarmak (Exposing)

Liste 7-4'te `karsilama` modülünün gizli olduğunu söyleyen hataya geri dönelim. Üst modüldeki `restoranda_yemek_ye` fonksiyonunun, alt modüldeki `bekleme_listesine_ekle` fonksiyonuna erişebilmesini istiyoruz, bu yüzden `karsilama` modülünü Liste 7-5'te gösterildiği gibi `pub` anahtar kelimesiyle işaretliyoruz.

<Listing number="7-5" file-name="src/lib.rs" caption="`restoranda_yemek_ye` fonksiyonundan kullanabilmek için `karsilama` modülünü `pub` olarak bildirmek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs:here}}
```

</Listing>

Ne yazık ki, Liste 7-5'teki kod Liste 7-6'da gösterildiği gibi hala derleyici hatalarına neden olur.

<Listing number="7-6" caption="Liste 7-5'teki kodu derlerken alınan derleyici hataları">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

</Listing>

Ne oldu? `mod karsilama`'nın önüne `pub` anahtar kelimesini eklemek modülü açık hale getirir. Bu değişiklikle, eğer `restoran_on_kisim` modülüne erişebiliyorsak, `karsilama` modülüne de erişebiliriz. Ancak `karsilama` modülünün _içerikleri_ hala gizlidir; modülü açık hale getirmek onun içeriğini de açık hale getirmez. Bir modül üzerindeki `pub` anahtar kelimesi sadece ata modüllerindeki kodların o modüle atıfta bulunmasına (refer to) izin verir, ancak içindeki (inner) kodlara erişmesini sağlamaz. Modüller kapsayıcılar (containers) olduğundan, sadece modülü açık hale getirerek yapabileceğimiz pek bir şey yoktur; daha ileri gitmeli ve modül içindeki öğelerden birini veya birkaçını da açık hale getirmeyi seçmeliyiz.

Liste 7-6'daki hatalar `bekleme_listesine_ekle` fonksiyonunun gizli olduğunu söylüyor. Gizlilik kuralları, struct'lar, enum'lar, fonksiyonlar ve metotlar ile birlikte modüller için de geçerlidir.

Liste 7-7'de olduğu gibi tanımından önce `pub` anahtar kelimesini ekleyerek `bekleme_listesine_ekle` fonksiyonunu da açık hale getirelim.

<Listing number="7-7" file-name="src/lib.rs" caption="`mod karsilama` ve `fn bekleme_listesine_ekle` tanımlarına `pub` anahtar kelimesini eklemek, bu fonksiyonu `restoranda_yemek_ye` fonksiyonundan çağırmamızı sağlar.">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs:here}}
```

</Listing>

Artık kod derlenecek! `pub` anahtar kelimesini eklemenin gizlilik kurallarına uygun olarak bu yolları `restoranda_yemek_ye` içinde kullanmamıza neden izin verdiğini görmek için, mutlak ve göreceli yollara (absolute and relative paths) bakalım.

Mutlak yolda, crate'imizin modül ağacının kökü (root) olan `crate` ile başlarız. `restoran_on_kisim` modülü crate kökünde tanımlanmıştır. `restoran_on_kisim` açık olmasa da, `restoranda_yemek_ye` fonksiyonu `restoran_on_kisim` ile aynı modülde tanımlandığı için (yani, `restoranda_yemek_ye` ve `restoran_on_kisim` kardeş modüllerdir), `restoranda_yemek_ye` içinden `restoran_on_kisim` modülüne atıfta bulunabiliriz. Sırada `pub` ile işaretlenmiş `karsilama` modülü var. `karsilama`'nın üst modülüne (parent module) erişebiliyoruz, dolayısıyla `karsilama`'ya erişebiliriz. Son olarak, `bekleme_listesine_ekle` fonksiyonu `pub` ile işaretlenmiştir ve bu fonksiyonun üst modülüne erişebiliriz, dolayısıyla bu fonksiyon çağrısı çalışır!

Göreceli (relative) yolda mantık, ilk adım hariç, mutlak yolla aynıdır: Yol, crate kökünden başlamak yerine `restoran_on_kisim` ile başlar. `restoran_on_kisim` modülü, `restoranda_yemek_ye` fonksiyonu ile aynı modülde tanımlanmıştır, bu nedenle `restoranda_yemek_ye` fonksiyonunun tanımlı olduğu modülden başlayan göreceli yol çalışır. Ardından, `karsilama` ve `bekleme_listesine_ekle` fonksiyonları `pub` ile işaretlendiği için yolun geri kalanı çalışır ve bu fonksiyon çağrısı geçerlidir!

Kütüphane crate'inizi diğer projelerin de kodunuzu kullanabilmesi için paylaşmayı planlıyorsanız, açık API'niz, crate'inizin kullanıcılarıyla yapacağınız ve kodunuzla nasıl etkileşime girebileceklerini belirleyen sözleşmenizdir (contract). İnsanların crate'inize bağımlı olmasını kolaylaştırmak için açık API'nizde yapılacak değişiklikleri yönetmekle ilgili pek çok husus vardır. Bu hususlar bu kitabın kapsamı dışındadır; eğer bu konuyla ilgileniyorsanız, [Rust API Yönergeleri][api-guidelines] kılavuzuna bakabilirsiniz.

> #### Bir İkili (Binary) ve Kütüphane (Library) İçeren Paketler İçin En İyi Pratikler
>
> Bir paketin hem _src/main.rs_ ikili crate kökünü hem de _src/lib.rs_ kütüphane crate kökünü içerebileceğinden ve varsayılan olarak her iki crate'in de paket adına sahip olacağından bahsetmiştik. Tipik olarak, hem bir kütüphane hem de bir ikili crate içerme modeline sahip olan paketler, ikili crate'te, kütüphane crate'inde tanımlanan kodu çağıran bir çalıştırılabilir dosya başlatmaya yetecek kadar koda sahip olacaktır. Bu, kütüphane crate'inin kodunun paylaşılabileceği için diğer projelerin paketin sağladığı işlevlerin çoğundan yararlanmasını sağlar.
>
> Modül ağacı _src/lib.rs_ içinde tanımlanmalıdır. Ardından, paketin adıyla başlayan yollar kullanılarak herhangi bir açık öğe ikili crate içinde kullanılabilir. İkili crate, tıpkı tamamen dışarıdan bir crate'in kütüphane crate'ini kullanacağı gibi kütüphane crate'inin bir kullanıcısı haline gelir: Yalnızca açık API'yi kullanabilir. Bu sizin iyi bir API tasarlamanıza yardımcı olur; yalnızca yazar değil, aynı zamanda bir istemcisinizdir (client)!
>
> [Bölüm 12'de][ch12]<!-- ignore -->, hem ikili crate hem de kütüphane crate'i içeren bir komut satırı programı ile bu organizasyonel pratiği (organizational practice) göstereceğiz.

### `super` İle Göreceli (Relative) Yollara Başlamak

Göreceli (relative) yollar oluştururken, yolun (path) başında `super` kullanarak geçerli modül veya crate kökü yerine üst modülden başlayabiliriz. Bu, dosya sistemindeki yollarda bir üst dizine gitmek anlamına gelen `..` sözdizimi ile başlamaya benzer. `super` kullanmak, üst modülde (parent module) olduğunu bildiğimiz bir öğeye başvurmamızı sağlar; bu da, bir modül üst modülle yakından ilişkiliyken, üst modülün ileride modül ağacında başka bir yere taşınması ihtimaline karşı modül ağacını yeniden düzenlemeyi kolaylaştırabilir.

Liste 7-8'de bir şefin yanlış bir siparişi düzeltip bizzat müşteriye götürdüğü durumu modelleyen kodu düşünün. `restoran_arka_kisim` modülünde tanımlanan `yanlis_siparisi_duzelt` (fix_incorrect_order) fonksiyonu, `siparisi_teslim_et` (deliver_order) fonksiyonunun yolunu belirterek ve `super` ile başlayarak, üst modülde tanımlı olan `siparisi_teslim_et` fonksiyonunu çağırır.

<Listing number="7-8" file-name="src/lib.rs" caption="Bir fonksiyonun `super` ile başlayan göreceli (relative) bir yolla çağrılması">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

</Listing>

`yanlis_siparisi_duzelt` fonksiyonu `restoran_arka_kisim` modülü içindedir, bu yüzden `restoran_arka_kisim`'in üst modülüne (bu durumda kök (root) olan `crate`'tir) gitmek için `super` kullanabiliriz. Oradan itibaren `siparisi_teslim_et` fonksiyonunu arar ve buluruz. Başarılı! Crate'in modül ağacını yeniden düzenlemeye karar verirsek, `restoran_arka_kisim` modülünün ve `siparisi_teslim_et` fonksiyonunun birbirleriyle aynı ilişki içinde kalma ihtimalinin yüksek olduğunu ve birlikte taşınacaklarını düşünüyoruz. Bu nedenle, `super` kullandık ki bu kod başka bir modüle taşındığında gelecekte kodu güncelleyecek daha az yerimiz olsun.

### Struct'ları ve Enum'ları Herkese Açık (Public) Yapmak

Struct'ları ve enum'ları da herkese açık olarak belirtmek için `pub` anahtar kelimesini kullanabiliriz, ancak struct'lar ve enum'lar ile `pub` kullanımının birkaç ek detayı vardır. Eğer bir struct tanımının öncesinde `pub` kullanırsak, struct'ı herkese açık yaparız, ancak struct'ın alanları hala gizli kalacaktır. Her bir alanı (field) duruma göre (case-by-case) herkese açık yapıp yapmamayı seçebiliriz. Liste 7-9'da, herkese açık bir `tost` alanı (field) ve gizli bir `mevsim_meyvesi` (seasonal_fruit) alanına sahip, açık bir `restoran_arka_kisim::Kahvalti` (Breakfast) struct'ı tanımladık. Bu durum, müşterinin yemekle birlikte gelen ekmek türünü seçebildiği, ancak şefin o mevsimde nelerin mevcut olduğuna ve stokta ne olduğuna göre hangi meyvenin eşlik edeceğine karar verdiği bir restorandaki durumu modeller. Mevcut olan meyve hızlı değiştiği için müşteriler meyveyi seçemezler ya da hangi meyveyi alacaklarını dahi göremezler.

<Listing number="7-9" file-name="src/lib.rs" caption="Bazı açık alanlara ve bazı gizli alanlara sahip bir struct">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

</Listing>

`restoran_arka_kisim::Kahvalti` struct'ındaki `tost` alanı açık olduğu için, `restoranda_yemek_ye` fonksiyonunda nokta (dot) gösterimini kullanarak `tost` alanını yazabilir (değiştirebilir) veya okuyabiliriz. Dikkat edin ki `mevsim_meyvesi` alanı gizli olduğu için, onu `restoranda_yemek_ye` içinde kullanamayız. Nasıl bir hata aldığınızı görmek için `mevsim_meyvesi` alanının değerini değiştiren satırın (yorum satırının) yorumunu (uncomment) kaldırmayı deneyin!

Ayrıca, `restoran_arka_kisim::Kahvalti` struct'ı gizli bir alana sahip olduğu için, bu struct'ın `Kahvalti`'nın bir örneğini oluşturan herkese açık ilişkili bir fonksiyon sağlaması gerektiğine dikkat edin (burada onu `yaz` (summer) olarak adlandırdık). `Kahvalti` struct'ı böyle bir fonksiyona sahip olmasaydı, `restoranda_yemek_ye` içinde `Kahvalti`'nın bir örneğini oluşturamazdık, çünkü `restoranda_yemek_ye` içinde gizli olan `mevsim_meyvesi` alanının değerini ayarlayamazdık.

Buna karşın, bir enum'ı herkese açık yaparsak, tüm varyantları da açık olur. Liste 7-10'da gösterildiği gibi yalnızca `enum` anahtar kelimesinden önce `pub` kullanmamız yeterlidir.

<Listing number="7-10" file-name="src/lib.rs" caption="Bir enum'ı açık olarak atamak, onun tüm varyantlarını da açık hale getirir.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

</Listing>

`Meze` (Appetizer) enum'ını açık hale getirdiğimiz için, `restoranda_yemek_ye` içinde `Corba` (Soup) ve `Salata` (Salad) varyantlarını kullanabiliriz.

Varyantları herkese açık olmadıkça enum'lar pek yararlı olmazlar; her durumda tüm enum varyantlarına `pub` eklemek can sıkıcı olurdu, bu nedenle enum varyantları için varsayılan (default) durum açık olmalarıdır. Struct'lar alanları açık olmadan da çoğu zaman kullanışlıdır, bu nedenle struct alanları, aksi `pub` ile belirtilmedikçe her şeyin varsayılan olarak gizli olduğu genel kuralını takip eder.

Henüz ele almadığımız ve `pub`'ı ilgilendiren bir durum daha vardır; o da son modül sistemi özelliğimiz olan `use` anahtar kelimesidir. Önce `use` anahtar kelimesini tek başına ele alacağız ve ardından `pub` ve `use`'u birlikte nasıl kullanacağımızı göstereceğiz.

[pub]: ch07-03-oge-yollari.html#yolları-pub-anahtar-kelimesiyle-açığa-çıkarmak-exposing
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-io-projesi.html
