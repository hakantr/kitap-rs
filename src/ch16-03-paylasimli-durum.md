## Paylaşımlı Durum Eşzamanlılığı

Mesaj iletimi, eşzamanlılığı yönetmenin iyi yollarından biridir; ama tek yol
bu değildir. Başka bir yöntem de birden fazla iş parçacığının aynı paylaşılan
veriye erişmesidir. Go belgelerindeki şu sözün bu bölümünü yeniden düşünün:
"Belleği paylaşarak iletişim kurmayın."

Peki belleği paylaşarak iletişim kurmak nasıl bir şeye benzerdi? Ayrıca mesaj
iletimini savunanlar neden bellek paylaşımından özellikle kaçınmayı öneriyor?

Bir bakıma, herhangi bir programlama dilindeki kanallar tek sahipliğe benzer;
çünkü bir değeri kanaldan gönderdiğinizde artık onu kullanmamanız gerekir.
Paylaşımlı bellekle eşzamanlılık ise çoklu sahipliğe benzer: Birden çok iş
parçacığı aynı anda aynı bellek konumuna erişebilir. 15. bölümde akıllı
işaretçilerin çoklu sahipliği nasıl mümkün kıldığını görmüştünüz. Çoklu
sahiplik, farklı sahiplerin yönetilmesini gerektirdiği için doğal olarak
karmaşıklık da getirir. Rust'ın tür sistemi ve sahiplik kuralları bu yönetimi
doğru yapma konusunda çok yardımcı olur. Bir örnek olarak, paylaşımlı bellek
eşzamanlılığında sık kullanılan temel yapılardan biri olan mutex'e bakalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-mutexes-to-allow-access-to-data-from-one-thread-at-a-time"></a>

### `Mutex<T>` ile Erişimi Denetlemek

_Mutex_, _mutual exclusion_ ifadesinin kısaltmasıdır; yani bir mutex, belli bir
anda yalnızca tek bir iş parçacığının bazı verilere erişmesine izin verir.
Mutex içindeki veriye erişmek için, iş parçacığı önce erişim istediğini
bildirmeli ve mutex'in _kilidini (lock)_ almalıdır. Kilit, mutex'in bir parçası
olan ve o anda veriye kimin özel erişimi olduğunu takip eden veri yapısıdır.
Bu yüzden mutex'in tuttuğu veriyi kilitleme sistemiyle _koruduğu_ söylenir.

Mutex'lerin kullanımı zor olmakla ünlüdür; çünkü iki kuralı sürekli akılda
tutmanız gerekir:

1. Veriyi kullanmadan önce kilidi almaya çalışmalısınız.
2. Mutex'in koruduğu veriyle işiniz bittiğinde, diğer iş parçacıkları da kilidi
   alabilsin diye kilidi bırakmalısınız.

Bunu gündelik bir benzetmeyle düşünelim: Tek mikrofonu olan bir panel
oturumunu hayal edin. Konuşmacılardan biri konuşmadan önce mikrofonu
istediğini belirtmelidir. Mikrofonu alınca istediği kadar konuşur; sonra da
sıradaki kişiye verir. Bir konuşmacı işini bitirince mikrofonu devretmeyi
unutursa, başka kimse konuşamaz. Paylaşılan mikrofonun yönetimi bozulursa
panel planlandığı gibi ilerlemez.

Mutex yönetimini doğru yapmak gerçekten zordur; işte bu yüzden birçok kişi
kanalları daha heyecan verici bulur. Ama Rust'ın tür sistemi ve sahiplik
kuralları sayesinde, kilitleme ile kilidi bırakma işlerini yanlış yapmanız
çok daha zordur.

#### `Mutex<T>` API'si

Mutex'in nasıl kullanıldığını görmek için önce tek iş parçacıklı çok basit bir
örnekle başlayalım; 16-12 numaralı liste bunu gösteriyor.

<Listing number="16-12" file-name="src/main.rs" caption="Sade olması için `Mutex<T>` API'sini tek iş parçacıklı bağlamda incelemek">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

</Listing>

Birçok türde olduğu gibi `Mutex<T>` değerini de `new` ilişkili fonksiyonuyla
oluştururuz. İçindeki veriye erişmek için `lock` metoduyla kilidi alırız. Bu
çağrı, kilidi alma sırası bize gelene kadar mevcut iş parçacığını bloklar.

Kilidi elinde tutan başka bir iş parçacığı paniklerse `lock` çağrısı hata
verebilir. Böyle bir durumda artık hiç kimse kilidi alamayacağı için, burada
`unwrap` kullanıp bu iş parçacığının da paniklemesini seçtik.

Kilidi aldıktan sonra, burada `sayi` adını verdiğimiz dönüş değerini içerideki
veriye ait değiştirilebilir referans gibi kullanabiliriz. Tür sistemi, `kilit`
değerinin içindeki veriyi kullanmadan önce gerçekten kilit aldığımızdan emin
olur. `kilit` değişkeninin türü `i32` değil `Mutex<i32>` olduğu için, içerideki
`i32` değeri kullanabilmek adına _zorunlu olarak_ `lock` çağırırız. Bunu
unutamayız; tür sistemi başka türlü izin vermez.

`lock` çağrısı, bizim `unwrap` ile ele aldığımız `LockResult` içine sarılmış
bir `MutexGuard` döndürür. `MutexGuard`, `Deref` uygular; böylece içerideki
veriyi işaret eder. Ayrıca `Drop` uygulaması sayesinde bir `MutexGuard` kapsam
dışına çıktığında kilit otomatik olarak bırakılır. Bu da içteki kapsamın sonunda
olur. Sonuç olarak kilidi bırakmayı unutup mutex'i başka iş parçacıklarının
kullanmasına engel olma riski yaşamayız; kilit bırakma işi kendiliğinden olur.

Kilidi bıraktıktan sonra mutex değerini ekrana yazdırabilir ve içerideki `i32`
değerini `6` yaptığımızı görebiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="sharing-a-mutext-between-multiple-threads"></a>

#### `Mutex<T>` İçin Paylaşımlı Erişim

Şimdi `Mutex<T>` kullanarak bir değeri birden fazla iş parçacığı arasında
paylaştırmayı deneyelim. 10 iş parçacığı oluşturup her birinin sayacı `1`
artırmasını isteyeceğiz; böylece sayaç `0`'dan `10`'a çıkacak. 16-13 numaralı
listedeki örnek derleyici hatası verecek ve bu hata üzerinden `Mutex<T>` ile
çalışırken Rust'ın bize nasıl yardım ettiğini daha iyi anlayacağız.

<Listing number="16-13" file-name="src/main.rs" caption="Her biri `Mutex<T>` ile korunan sayacı artıran on iş parçacığı">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

</Listing>

16-12 numaralı listedekine benzer şekilde, `Mutex<T>` içindeki bir `i32`
değerini tutmak için `sayac` değişkeni oluşturuyoruz. Ardından bir sayı aralığı
üzerinde dönerek 10 iş parçacığı başlatıyoruz. `thread::spawn` çağrısına,
sayacı iş parçacığına taşıyan, `lock` metoduyla `Mutex<T>` kilidini alan ve
sonra içerdeki değere `1` ekleyen aynı kapanışı veriyoruz. Bir iş parçacığı
kapanışı bitirdiğinde `sayi` kapsam dışına çıkar ve kilit bırakılır; böylece
başka bir iş parçacığı kilidi alabilir.

Ana iş parçacığında bütün tutamaçları bir vektörde topluyoruz. Sonra 16-2
numaralı listedeki gibi her tutamaç üzerinde `join` çağırarak tüm iş
parçacıklarının bitmesini bekliyoruz. En sonunda ana iş parçacığı kilidi alıp
programın sonucunu yazdırıyor.

Bu örneğin derlenmeyeceğini söylemiştik. Şimdi nedenine bakalım:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

Hata mesajı `sayac` değerinin döngünün önceki yinelemesinde taşındığını
söylüyor. Yani Rust bize, kilitli `sayac` değerinin sahipliğini birden fazla iş
parçacığına taşıyamayacağımızı anlatıyor. Bunu, 15. bölümde gördüğümüz çoklu
sahiplik yaklaşımıyla düzeltmeye çalışalım.

#### Birden Fazla İş Parçacığıyla Çoklu Sahiplik

15. bölümde bir değere birden fazla sahip vermek için `Rc<T>` akıllı
işaretçisini kullanmıştık. Aynı şeyi burada da yapıp ne olacağına bakalım.
16-14 numaralı listede `Mutex<T>` değerini `Rc<T>` içine sarıyor ve iş
parçacığına taşımadan önce `Rc<T>` değerini klonluyoruz.

<Listing number="16-14" file-name="src/main.rs" caption="Birden fazla iş parçacığının `Mutex<T>` değerine sahip olabilmesi için `Rc<T>` kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

</Listing>

Yine derliyoruz ve... bu kez farklı hatalar alıyoruz. Derleyici bize çok şey
öğretiyor:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

Bu mesajın en önemli kısmı şudur:
`` `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely ``.
Derleyici bunun nedenini de söylüyor:
`` the trait `Send` is not implemented for `Rc<std::sync::Mutex<i32>>` ``.
Bir sonraki bölümde `Send` trait'ini ayrıntılı konuşacağız. Şimdilik şunu
bilin: `Send`, iş parçacıklarıyla kullanılacak türlerin eşzamanlı bağlamlara
uygun olduğunu garanti eden trait'lerden biridir.

Ne yazık ki `Rc<T>` iş parçacıkları arasında paylaşmak için güvenli değildir.
`Rc<T>` referans sayısını yönettiğinde, her `clone` çağrısında sayıyı artırır,
her klon düşürüldüğünde de azaltır. Fakat bu sayacın güncellenmesi sırasında
başka bir iş parçacığının araya girmesini önleyecek bir eşzamanlılık ilkelini
kullanmaz. Bu da yanlış sayımlara, dolayısıyla çok sinsi hatalara, bellek
sızıntılarına ya da işimiz bitmeden bir değerin düşürülmesine yol açabilir.
Burada bize gereken şey, `Rc<T>`ye çok benzeyen ama referans sayısını iş
parçacığı güvenli biçimde güncelleyen bir türdür.

#### `Arc<T>` ile Atomik Referans Sayımı

Neyse ki `Arc<T>`, `Rc<T>` gibi davranan ama eşzamanlı ortamlarda güvenle
kullanılabilen bir türdür. Buradaki _a_, _atomic_ sözcüğünden gelir; yani bu
tür _atomik referans sayımlıdır (atomically reference-counted)_. Atomikler,
burada ayrıntısına girmeyeceğimiz ayrı bir eşzamanlılık ilkelidir. Daha fazla
bilgi için standart kütüphanedeki [`std::sync::atomic`][atomic]<!-- ignore -->
belgelerine bakabilirsiniz. Şimdilik bilmeniz gereken, atomik türlerin ilkel
türler gibi çalıştığı ama iş parçacıkları arasında güvenle paylaşılabildiğidir.

Şu soru akla gelebilir: Madem öyle, neden bütün ilkel türler atomik değil ya da
standart kütüphane türleri varsayılan olarak neden `Arc<T>` kullanmıyor?
Cevap şu: İş parçacığı güvenliği bir performans bedeli getirir ve bu bedeli
yalnızca gerçekten gerektiğinde ödemek istersiniz. Tek bir iş parçacığında
çalışıyorsanız, atomik güvenceleri zorunlu kılmak zorunda kalmayan kod daha
hızlı çalışabilir.

Örneğimize geri dönelim: `Arc<T>` ile `Rc<T>` aynı API'yi sunar. Bu nedenle
programı düzeltmek için yalnızca `use` satırını, `new` çağrısını ve `clone`
çağrısını değiştirmemiz yeterlidir. 16-15 numaralı listedeki kod sonunda
derlenir ve çalışır.

<Listing number="16-15" file-name="src/main.rs" caption="Birden fazla iş parçacığı arasında sahipliği paylaşabilmek için `Mutex<T>` değerini `Arc<T>` ile sarmalamak">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

</Listing>

Bu kod aşağıdaki çıktıyı üretir:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Sonuç: 10
```

Başardık! `0`'dan `10`'a kadar saydık. Bu çok etkileyici görünmeyebilir; ama
`Mutex<T>` ve iş parçacığı güvenliği hakkında pek çok şey öğrenmiş olduk.
Aslında aynı yapı, yalnızca sayaç artırmak değil çok daha karmaşık hesaplar
için de kullanılabilir. Bu stratejiyle bir hesabı bağımsız parçalara bölebilir,
parçaları iş parçacıklarına dağıtabilir ve sonunda her parçanın sonucunu
`Mutex<T>` aracılığıyla ortak sonuca yansıtabilirsiniz.

Şunu da unutmayın: Basit sayısal işlemler yapıyorsanız, standart
kütüphanedeki [`std::sync::atomic` modülü][atomic]<!-- ignore --> altında
`Mutex<T>`den daha sade türler bulunur. Bu türler ilkel verilere güvenli,
eşzamanlı ve atomik erişim sağlar. Biz burada `Mutex<T>`yi ilkel bir türle
kullanmayı özellikle seçtik; çünkü amacımız öncelikle `Mutex<T>`nin nasıl
çalıştığını göstermekti.

<!-- Old headings. Do not remove or links may break. -->

<a id="similarities-between-refcelltrct-and-mutextarct"></a>

### `RefCell<T>`/`Rc<T>` ile `Mutex<T>`/`Arc<T>` Karşılaştırması

`sayac` değişkeninin değiştirilemez tanımlandığını ama içindeki değere
değiştirilebilir referans alabildiğimizi fark etmiş olabilirsiniz. Bu da
`Mutex<T>`nin, `Cell` ailesindeki türler gibi içsel değiştirilebilirlik
sağladığını gösterir. 15. bölümde `Rc<T>` içindeki değeri değiştirebilmek için
`RefCell<T>` kullanmıştık; burada da aynı işi `Arc<T>` içindeki değer için
`Mutex<T>` ile yapıyoruz.

Dikkat edilmesi gereken başka bir nokta daha var: `Mutex<T>` kullanırken Rust
sizi her türlü mantık hatasından koruyamaz. 15. bölümde, `Rc<T>` kullanırken
iki değerin birbirini işaret etmesiyle referans döngüsü kurulabileceğini ve
bunun bellek sızıntısına yol açabileceğini görmüştünüz. Benzer biçimde
`Mutex<T>` de _kilitlenme (deadlock)_ riskini taşır. Bu durum, bir işlemin iki
kaynak için kilit alması gerektiğinde ve iki iş parçacığının bu kilitlerden
birer tanesini alıp birbirini sonsuza kadar beklemesiyle ortaya çıkar.

Kilitlenmeler ilginizi çekiyorsa, bir kilitlenme içeren küçük bir Rust programı
yazmayı deneyin. Ardından başka dillerde mutex kullanan sistemlerde uygulanan
kilitlenme azaltma stratejilerini araştırın ve benzerini Rust'ta kurmayı
deneyin. Standart kütüphanedeki `Mutex<T>` ve `MutexGuard` API belgeleri bu
konuda yararlı bilgiler içerir.

Bölümü `Send` ve `Sync` trait'lerinden ve bunları özel türlerle nasıl
kullanabileceğimizden söz ederek tamamlayacağız.

[atomic]: ../std/sync/atomic/index.html
