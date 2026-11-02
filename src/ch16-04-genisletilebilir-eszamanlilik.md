<!-- Old headings. Do not remove or links may break. -->

<a id="extensible-concurrency-with-the-sync-and-send-traits"></a>
<a id="extensible-concurrency-with-the-send-and-sync-traits"></a>
<a id="extensible-concurrency-with-send-and-sync"></a>

## `Send` ve `Sync` ile Genişletilebilir Eşzamanlılık

İlginç bir şekilde, bu bölümde şimdiye dek ele aldığımız eşzamanlılık
özelliklerinin neredeyse tamamı dilin kendisinin değil, standart kütüphanenin
bir parçasıydı. Eşzamanlılığı yönetme seçenekleriniz dil veya standart
kütüphaneyle sınırlı değildir; kendi eşzamanlılık özelliklerinizi yazabilir veya
başkalarının yazdıklarını kullanabilirsiniz.

Bununla birlikte, standart kütüphaneden ziyade doğrudan dilin içine yerleşik
olan temel eşzamanlılık kavramları arasında `std::marker` modülündeki `Send` ve
`Sync` trait'leri yer alır.

<!-- Old headings. Do not remove or links may break. -->

<a id="allowing-transference-of-ownership-between-threads-with-send"></a>
<a id="transferring-ownership-between-threads"></a>
<a id="iş-parçacıkları-arasında-sahiplik-aktarmak"></a>
<a id="is-parcaciklari-arasinda-sahiplik-aktarmak"></a>

### Thread'ler Arasında Sahiplik Aktarma

`Send` _işaretleyici trait'i (marker trait)_, `Send` uygulayan türe ait
değerlerin sahipliğinin thread'ler arasında aktarılabileceğini belirtir.
Rust'taki neredeyse her tür `Send` uygular; ancak aralarında `Rc<T>` türünün de
bulunduğu bazı istisnalar vardır: `Rc<T>` bu trait'i uygulayamaz, çünkü bir
`Rc<T>` değerini klonlayıp klonun sahipliğini başka bir thread'e aktarmayı
deneseydiniz, her iki thread de referans sayısını aynı anda güncelleyebilirdi.
Bu nedenle `Rc<T>`, thread güvenliğinin getirdiği performans maliyetini ödemek
istemediğiniz tek thread'li durumlarda kullanılmak üzere uygulanmıştır.

Dolayısıyla Rust'ın tür sistemi ve trait sınırları, bir `Rc<T>` değerini
thread'ler arasında yanlışlıkla güvensiz bir biçimde göndermenizi kesin olarak
engeller. Liste 16-14'te bunu yapmayı denediğimizde `` the trait `Send` is not
implemented for `Rc<std::sync::Mutex<i32>>` `` (`Send` trait'i `Rc<Mutex<i32>>`
için uygulanmamıştır) hatasını almıştık. `Send` trait'ini uygulayan `Arc<T>`
türüne geçtiğimizde ise kod derlenmişti.

Tamamen `Send` türlerinden oluşan her tür de otomatik olarak `Send` olarak
işaretlenir. Bölüm 20'de ele alacağımız _ham işaretçiler (raw pointers)_ dışında
neredeyse tüm ilkel türler `Send`'dir.

<!-- Old headings. Do not remove or links may break. -->

<a id="allowing-access-from-multiple-threads-with-sync"></a>
<a id="accessing-from-multiple-threads"></a>
<a id="birden-fazla-iş-parçacığından-erişim"></a>
<a id="birden-fazla-is-parcacigindan-erisim"></a>

### Birden Fazla Thread'den Erişim

`Sync` işaretleyici trait'i, `Sync` uygulayan bir türe birden fazla thread'den
referans verilmesinin güvenli olduğunu belirtir. Başka bir deyişle, herhangi bir
`T` türü, `&T` (`T`'ye ait değiştirilemez referans) `Send` trait'ini
uyguluyorsa —yani bu referans başka bir thread'e güvenle gönderilebiliyorsa—
`Sync` uygular. `Send`'e benzer şekilde, ham işaretçiler dışındaki tüm ilkel
türler `Sync` uygular ve tamamen `Sync` uygulayan türlerden oluşan türler de
`Sync` uygular.

`Rc<T>` akıllı işaretçisi, `Send` trait'ini uygulamamasıyla aynı gerekçelerle
`Sync` trait'ini de uygulamaz. Bölüm 15'te ele aldığımız `RefCell<T>` türü ve
onunla ilişkili `Cell<T>` türleri ailesi de `Sync` uygulamaz. `RefCell<T>`'nin
çalışma zamanında yürüttüğü ödünç alma denetimi thread güvenli değildir. Buna
karşılık `Mutex<T>` akıllı işaretçisi `Sync` uygular ve
[“`Mutex<T>`'ye Paylaşımlı Erişim”][shared-access]<!-- ignore --> bölümünde
gördüğünüz gibi erişimi birden fazla thread ile paylaşmak için kullanılabilir.

> Not: `Sync` trait'inin kavramsal tanımı tek bir özlü kurala dayanır:
> **`T: Sync` olması, ancak ve ancak `&T: Send` olması demektir.** Yani bir
> değere ait değiştirilemez referansı (`&T`) başka bir thread'e güvenle
> gönderebiliyorsanız `T` türü `Sync`'tir. Peki ölçüt neden `&mut T` değil de
> `&T`'dir? Çünkü `&mut T` derleme zamanında zaten *tekil (münhasır)* erişimi
> garanti eder; ortada aynı veriye erişen ikinci bir thread olamaz. Birden fazla
> thread'in aynı veriye eşzamanlı erişimi yalnızca paylaşımlı `&T` referansları
> üzerinden gerçekleşir ve tehlike, `&T` üzerinden senkronizasyonsuz _içsel
> değiştirilebilirlik_ sunulduğunda doğar. Standart kütüphanedeki türler bu dört
> olasılığı somut biçimde sergiler:
>
> - **Hem `Send` hem `Sync` (`i32`, `String`, `Vec<i32>`, `Arc<i32>`, `Mutex<i32>`):**
>   Hem sahipliği başka bir thread'e taşınabilir hem de `&T` referansı birden
>   fazla thread arasında güvenle paylaşılabilir. (`Mutex<T>` erişimi kilitlerle
>   sıraya soktuğundan, `T: Send` olduğu sürece `Mutex<T>` hem `Send` hem de
>   `Sync` olur.)
> - **`Send` iken `!Sync` (`Cell<T>` ve `RefCell<T>`, `T: Send` iken):** Bu
>   türlerin tek bir sahibi vardır; değerin tamamını bir thread'den diğerine
>   taşımak (`Send`) güvenlidir, çünkü eski thread artık ona erişemez. Ancak
>   `&Cell<T>` veya `&RefCell<T>` başka bir thread'e gönderilebilseydi (`Sync`),
>   iki thread atomik olmayan iç durumu veya ödünç alma sayacını aynı anda
>   değiştirerek veri yarışına yol açardı; bu yüzden `!Sync`'tirler.
> - **Hem `!Send` hem `!Sync` (`Rc<T>`, ham işaretçiler `*const T` / `*mut T`):**
>   `Rc<T>` klonlandığında aynı atomik olmayan referans sayacını paylaşan ikinci
>   bir sahip oluşturur. Klonu başka bir thread'e taşımak (`Send`) da, `&Rc<T>`
>   referansını paylaşıp karşı thread'de `.clone()` çağrılmasına olanak tanımak
>   (`Sync`) da sayaç üzerinde veri yarışı yaratacağından `Rc<T>` ne `Send` ne
>   de `Sync`'tir.
> - **`!Send` iken `Sync` (`MutexGuard<'_, T>`, `T: Sync` iken):** İşletim
>   sistemi düzeyindeki pek çok kilit mekanizması, kilidi edinen thread ile
>   serbest bırakan thread'in **aynı thread** olmasını şart koşar. `MutexGuard`
>   düşürüldüğünde (`Drop`) kilidi açtığı için başka bir thread'e taşınamaz
>   (`!Send`); aksi hâlde kilit yanlış thread üzerinde açılırdı. Buna karşılık
>   kilidi tutan thread'in `&MutexGuard<'_, T>` referansını (örneğin
>   `std::thread::scope` ile) diğer thread'lere ödünç vermesi güvenlidir (`Sync`);
>   çünkü diğer thread'ler yalnızca `&T` üzerinden okuma yapar ve kilidin
>   açılması yine asıl sahibi olan thread'de gerçekleşir.

<!-- Old headings. Do not remove or links may break. -->

<a id="implementing-send-and-sync-manually-is-unsafe"></a>
<a id="send-ve-synci-elle-uygulamak-guvensizdir"></a>

### `Send` ve `Sync`'i Elle Uygulamak Güvensizdir

Tamamen `Send` ve `Sync` trait'lerini uygulayan başka türlerden oluşan türler de
`Send` ve `Sync`'i otomatik olarak uyguladığından, bu trait'leri elle
uygulamamız gerekmez. İşaretleyici trait'ler oldukları için uygulanacak herhangi
bir metotları bile yoktur. Yalnızca eşzamanlılıkla ilgili _değişmezleri
(invariants)_ zorunlu kılmak için yararlıdırlar.

Bu trait'leri elle uygulamak, güvenli olmayan (`unsafe`) Rust kodu yazmayı
gerektirir. Güvenli olmayan Rust kodunun kullanımını Bölüm 20'de ele alacağız;
şimdilik bilmeniz gereken önemli nokta, `Send` ve `Sync` parçalardan oluşmayan
yeni eşzamanlı türler oluşturmanın güvenlik güvencelerini koruyabilmek adına çok
dikkatli düşünmeyi gerektirdiğidir. [“The Rustonomicon”][nomicon], bu güvenceler
ve bunların nasıl korunacağı hakkında daha fazla bilgi içerir.

> Not: `Send` ve `Sync`, `#[derive(...)]` özniteliğiyle değil derleyicinin
> _otomatik trait (auto trait)_ mekanizmasıyla uygulanır: bir struct veya enum'ın
> tüm alanları `Send` (veya `Sync`) ise türün kendisi de bu trait'i kendiliğinden
> kazanır. Buna karşılık kendi veri yapınızın içinde ham işaretçi (`*const T` /
> `*mut T`) kullandığınızda —örneğin standart kütüphanedeki `Vec<T>`, `Arc<T>`
> veya `Mutex<T>` gibi alt düzey yapılar tasarlarken— derleyici otomatik uygulamayı
> devre dışı bırakır ve thread güvenliği sözleşmesini bizzat doğrulayarak
> `unsafe impl Send for ...` / `unsafe impl Sync for ...` yazmanızı bekler.

<!-- Old headings. Do not remove or links may break. -->

<a id="summary"></a>
<a id="ozet"></a>

## Özet

Bu kitapta eşzamanlılık konusunu son kez görmüyorsunuz: Bir sonraki bölüm
asenkron (`async`) programlamaya odaklanacak ve Bölüm 21'deki proje, bu
bölümdeki kavramları burada ele aldığımız daha küçük örneklerden daha gerçekçi
bir senaryoda kullanacak.

Daha önce de belirttiğimiz gibi, Rust'ın eşzamanlılığı ele alış biçiminin çok
küçük bir kısmı dilin kendi parçası olduğundan, pek çok eşzamanlılık çözümü
crate'ler olarak uygulanmıştır. Bu crate'ler standart kütüphaneden daha hızlı
evrilir; dolayısıyla çok thread'li durumlarda kullanabileceğiniz güncel ve en
gelişmiş crate'ler için çevrim içi araştırma yapmayı unutmayın.

Rust standart kütüphanesi, mesaj iletimi için kanallar ve eşzamanlı bağlamlarda
güvenle kullanılabilen `Mutex<T>` ile `Arc<T>` gibi akıllı işaretçi türleri
sunar. Tür sistemi ve ödünç alma denetleyicisi, bu çözümleri kullanan kodların
veri yarışlarına veya geçersiz referanslara yol açmamasını güvence altına alır.
Kodunuzu derlemeyi başardığınızda, diğer dillerde sıkça rastlanan ve izini
sürmesi güç hatalarla karşılaşmadan birden fazla thread üzerinde sorunsuzca
çalışacağından emin olabilirsiniz. Eşzamanlı programlama artık korkulacak bir
kavram değil: İlerleyin ve programlarınızı korkusuzca eşzamanlı hâle getirin!

[shared-access]: ch16-03-paylasimli-durum.html#mutext-için-paylaşımlı-erişim
[nomicon]: ../nomicon/index.html
