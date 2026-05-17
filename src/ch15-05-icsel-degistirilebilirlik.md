## `RefCell<T>` ve İçsel Değiştirilebilirlik Deseni

_İçsel değiştirilebilirlik (interior mutability)_, elde yalnızca değiştirilemez
referanslar olsa bile veriyi değiştirmeye izin veren bir Rust tasarım desenidir.
Normalde ödünç alma kuralları buna izin vermez. Bu desen, değişiklik ve ödünç
alma kurallarını yöneten normal Rust davranışını esnetmek için veri yapısının
içinde `unsafe` kod kullanır. `Unsafe`, kuralları derleyiciye değil bizim
manuel denetlediğimizi söyler; ayrıntısını 20. bölümde göreceğiz.

Bu deseni kullanan türleri ancak ödünç alma kurallarının çalışma zamanında da
izleneceğinden eminseniz kullanmalısınız. İçteki `unsafe` kod güvenli bir API
arkasına saklanır; dışarıdan bakınca tür yine değiştirilemez görünür.

Bu fikri, içsel değiştirilebilirliği kullanan `RefCell<T>` türü üzerinden
inceleyelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="enforcing-borrowing-rules-at-runtime-with-refcellt"></a>

### Ödünç Alma Kurallarını Çalışma Zamanında Uygulamak

`Rc<T>`den farklı olarak `RefCell<T>`, tuttuğu veri üzerinde tek sahipliği
temsil eder. Peki onu `Box<T>`den ayıran nedir? 4. bölümdeki ödünç alma
kurallarını hatırlayın:

- Aynı anda ya tek bir değiştirilebilir referansınız olabilir ya da istediğiniz
  kadar değiştirilemez referansınız olabilir; ikisi bir arada olamaz.
- Referanslar her zaman geçerli olmalıdır.

Referanslar ve `Box<T>` ile bu kuralların değişmezleri derleme zamanında
uygulanır. `RefCell<T>` ileyse _çalışma zamanında_ uygulanır. Referanslarla
kuralları bozarsanız derleyici hata verir. `RefCell<T>` ile bozarsanız program
`panic!` ile kapanır.

Derleme zamanında denetlemenin avantajı, hataların daha erken yakalanması ve
çalışma zamanı maliyeti olmamasıdır. Bu yüzden Rust'ta varsayılan yaklaşım
budur.

Çalışma zamanında denetlemenin avantajıysa, derleme zamanında fazla katı kalan
denetimlerin reddedeceği bazı bellek-güvenli senaryoları mümkün kılmasıdır.
Rust derleyicisi gibi durağan analiz araçları doğaları gereği temkinlidir.
Bazı özellikleri yalnızca kodu analiz ederek belirlemek imkânsızdır; en bilinen
örnek Duruş Problemi'dir.

Bu yüzden derleyici kurallara uyulduğundan emin değilse, doğru bir programı
bile reddedebilir. Bu rahatsız edicidir ama felaket değildir. Buna karşılık
yanlış programı kabul etseydi, Rust'ın verdiği güvencelere güvenemezdik.
`RefCell<T>`, kurallara uyduğunuzdan emin olduğunuz ama derleyicinin bunu
kanıtlayamadığı durumlarda işe yarar.

`RefCell<T>` de `Rc<T>` gibi yalnızca tek iş parçacıklı kullanım içindir.
Çok iş parçacıklı bağlamda kullanırsanız derleme hatası alırsınız. 16. bölümde,
aynı işlevselliğin çok iş parçacıklı sürümünü göreceğiz.

`Box<T>`, `Rc<T>` ve `RefCell<T>` arasında seçim yaparken akılda tutulacak kısa
özet şöyledir:

- `Rc<T>` aynı verinin birden çok sahibi olmasına izin verir; `Box<T>` ve
  `RefCell<T>` tek sahiplidir.
- `Box<T>`, derleme zamanında denetlenen değiştirilemez ya da değiştirilebilir
  ödünçler sunar; `Rc<T>` yalnızca derleme zamanında denetlenen değiştirilemez
  ödünçler sunar; `RefCell<T>` ise çalışma zamanında denetlenen her iki türü de
  sunar.
- `RefCell<T>` çalışma zamanında denetlenen değiştirilebilir ödünçlere izin
  verdiği için, kendisi değiştirilemez olsa bile içindeki değeri
  değiştirebilirsiniz.

Değiştirilemez bir değerin içindeki veriyi değiştirmek, işte bu içsel
değiştirilebilirlik desenidir.

<!-- Old headings. Do not remove or links may break. -->

<a id="interior-mutability-a-mutable-borrow-to-an-immutable-value"></a>

### İçsel Değiştirilebilirlik Kullanmak

Ödünç alma kurallarının sonucu olarak, elinizde değiştirilemez bir değer
varken onu değiştirilebilir olarak ödünç alamazsınız. Örneğin şu kod derlenmez:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

Derlerseniz şu hatayı alırsınız:

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

Bununla birlikte, bazı durumlarda bir değerin kendi metotları içinde kendini
değiştirmesi ama dış dünyaya değiştirilemez görünmesi faydalıdır. `RefCell<T>`
bunu yapmanın yollarından biridir. Tam anlamıyla kurallardan kaçmaz; yalnızca
kontrolü derleme zamanından çalışma zamanına taşır. Kuralı ihlal ederseniz
derleme hatası değil `panic!` alırsınız.

Şimdi `RefCell<T>`yi gerçekten işimize yarayan bir örnek üzerinde görelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-use-case-for-interior-mutability-mock-objects"></a>

#### Sahte Nesnelerle Test Yazmak

Test sırasında programcı bazen bir türün yerine başka bir tür kullanır; amaç
belirli davranışı gözlemek ve doğru uygulanıp uygulanmadığını denetlemektir.
Bu geçici türe _test double_ denir. Bunun özel bir biçimi olan _mock object_,
test boyunca neler olduğunu kaydeder; böylece doğru eylemlerin gerçekleşip
gerçekleşmediğini doğrulayabilirsiniz.

Rust'ta bazı dillerdeki anlamıyla nesne yoktur ve standart kütüphanede hazır
mock altyapısı gelmez. Ama aynı işi görecek struct'ları rahatlıkla
tanımlayabilirsiniz.

Şu senaryoyu test edelim: bir değerin üst sınıra ne kadar yaklaştığını izleyen
ve mevcut değerin sınıra yaklaşmasına göre mesaj gönderen bir kütüphane
yazacağız. Örneğin bir kullanıcının yapabileceği API çağrısı kotasını izlemek
için kullanılabilir.

Bu kütüphane yalnızca sınıra yakınlığı ve hangi eşiklerde hangi mesajın
gönderileceğini bilir. Mesajların nasıl gönderileceğini ise kütüphaneyi kullanan
uygulama sağlayacaktır. Bunun için `Iletici` adlı bir trait tanımlıyoruz.
Liste 15-20 kütüphane kodunu gösterir.

<Listing number="15-20" file-name="src/lib.rs" caption="Bir değerin üst sınıra ne kadar yaklaştığını izleyen ve belirli seviyelerde uyaran kütüphane">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

</Listing>

Buradaki önemli noktalardan biri, `Iletici` trait'inin `self`i değiştirilemez
referans alan `gonder` metodunu tanımlamasıdır. Sahte nesnemiz, gerçek nesneyle
aynı şekilde kullanılabilmek için bu arayüzü uygulamalıdır. İkinci önemli nokta
ise `SinirIzleyici` üzerindeki `deger_ata` davranışını test etmek istememizdir.
`deger` parametresine verdiğimiz şeyi değiştirebiliriz ama `deger_ata` bize
doğrulama yapacağımız bir sonuç döndürmez. Biz de “belirli bir `en_buyuk`
değeriyle oluşturulmuş `SinirIzleyici`, farklı sayılar verildiğinde doğru
iletileri gönderiyor mu?” sorusunu sınamak isteriz.

Gerçekten e-posta ya da mesaj göndermek yerine, yalnızca gönderilmesi istenen
mesajları kaydeden bir sahte nesneye ihtiyacımız var. Sahte nesnenin örneğini
oluşturup `SinirIzleyici`ye verecek, sonra `deger_ata` çağıracak ve sonrasında
beklediğimiz mesajların kaydedilip kaydedilmediğine bakacağız. Liste 15-21 bu
yönde bir girişimi gösteriyor; ama ödünç alma denetleyicisi buna izin vermiyor.

<Listing number="15-21" file-name="src/lib.rs" caption="Odunc alma denetleyicisinin izin vermedigi `SahteIletici` gerceklemesi denemesi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

</Listing>

Bu test kodu, gönderilen iletileri tutmak için `Vec<String>` kullanan
`SahteIletici` yapısını tanımlar. Boş ileti listesiyle başlayan örnekler
oluşturmayı kolaylaştırmak için `yeni` ilişkili fonksiyonunu da ekleriz.
Ardından `Iletici` trait'ini uygularız.

Testte, `SinirIzleyici`ye `deger` olarak `80` verdiğimizde, yani `100`lük
sınırın yüzde 75'ini geçtiğimizde ne olduğunu sınarız. Önce yeni bir
`SahteIletici`, sonra ona referans verilen bir `SinirIzleyici` oluştururuz.
`deger_ata(80)` çağırdıktan sonra, sahte ileticinin bir mesaj kaydetmiş olmasını
bekleriz.

Ama burada bir sorun var:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

`gonder`, `self`i değiştirilemez referans aldığı için `SahteIletici` içindeki
ileti listesini değiştiremiyoruz. Hata mesajının önerdiği gibi trait'i ve
uygulamayı `&mut self` yapamayız; çünkü sırf test kolaylığı için `Iletici`
trait'ini değiştirmek istemiyoruz.

İşte burada içsel değiştirilebilirlik devreye girer. `gonderilen_iletiler`
alanını `RefCell<T>` içine alırız; böylece `gonder` metodu `self`
değiştirilemez referans alsa bile, içerideki veriyi değiştirebilir. Liste 15-22
bunu gösterir.

<Listing number="15-22" file-name="src/lib.rs" caption="Dış değer değiştirilemez sayılırken iç değeri değiştirmek için `RefCell<T>` kullanmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

</Listing>

`gonderilen_iletiler` alanı artık `Vec<String>` değil
`RefCell<Vec<String>>`dir. `yeni` içinde boş vektörün etrafına yeni bir
`RefCell` örneği sararız.

`gonder` uygulamasında ilk parametre hâlâ `self`in değiştirilemez ödüncüdür;
trait tanımıyla uyumludur. `self.gonderilen_iletiler` üzerinde `borrow_mut`
çağırarak içteki vektöre değiştirilebilir erişim alır, ardından `push`
kullanarak iletiyi kaydederiz.

Doğrulamada da vektörün boyuna bakmak için `borrow` çağırıp değiştirilemez
referans alırız.

<!-- Old headings. Do not remove or links may break. -->

<a id="keeping-track-of-borrows-at-runtime-with-refcellt"></a>

#### Ödünçleri Çalışma Zamanında İzlemek

Normal referanslarda `&` ve `&mut` kullanırız. `RefCell<T>` ileyse `borrow` ve
`borrow_mut` kullanırız. `borrow`, `Ref<T>`; `borrow_mut` ise `RefMut<T>`
döndürür. Her iki tür de `Deref` uyguladığı için normal referanslar gibi
davranabilir.

`RefCell<T>`, o anda etkin olan `Ref<T>` ve `RefMut<T>` akıllı işaretçilerinin
sayısını izler. `borrow` her çağrıldığında değiştirilemez ödünç sayısını
artırır. `Ref<T>` kapsam dışına çıkınca sayı bir azalır. Tıpkı derleme zamanı
kuralları gibi, `RefCell<T>` de aynı anda çok sayıda değiştirilemez ödünç ya da
yalnızca tek değiştirilebilir ödünç olmasına izin verir.

Kural ihlali yaparsak, referanslarda olduğu gibi derleme hatası değil çalışma
zamanında `panic!` alırız. Liste 15-23, Liste 15-22'deki `gonder`
uygulamasının bilerek bozulmuş sürümüdür: aynı kapsam içinde iki
değiştirilebilir ödünç oluşturmaya çalışıyoruz.

<Listing number="15-23" file-name="src/lib.rs" caption="Aynı kapsamda iki değiştirilebilir referans oluşturup `RefCell<T>`nin panic vermesini görmek">

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

</Listing>

Önce `borrow_mut`ten dönen `RefMut<T>` için `ilk_odunc` değişkenini, ardından
aynı kapsamda ikinci bir `RefMut<T>` için `ikinci_odunc`u oluşturuyoruz. Bu,
aynı kapsamda iki değiştirilebilir referans demektir ve yasaktır. Kod derlenir
ama test başarısız olur:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

Kodun `already borrowed: BorrowMutError` mesajıyla paniklediğine dikkat edin.
`RefCell<T>`, kuralları çalışma zamanında işte böyle uygular.

Bu yaklaşımın bedeli vardır: hataları geliştirme sürecinde daha geç fark
edebilirsiniz ve çalışma zamanında küçük de olsa ek maliyet oluşur. Buna
rağmen, yalnızca değiştirilemez değerlerin izin verildiği bağlamda kendini
değiştirebilen sahte nesneler yazmak gibi durumlarda `RefCell<T>` çok işe yarar.

### Değiştirilebilir Verinin Birden Fazla Sahibi Olmasına İzin Vermek

`RefCell<T>` çok sık `Rc<T>` ile birlikte kullanılır. `Rc<T>`, verinin birden
çok sahibi olmasına izin verir ama yalnızca değiştirilemez erişim sunar. Eğer
`Rc<T>` içinde `RefCell<T>` taşırsanız, hem birden çok sahipliğe hem de
değiştirilebilirliğe sahip olursunuz.

Liste 15-18'de `Rc<T>` kullanarak bir listenin birden çok yerde
paylaşılabildiğini görmüştük. Ama `Rc<T>` yalnızca değiştirilemez değerleri
tuttuğu için, liste oluştuktan sonra içindeki değerleri değiştiremiyorduk.
Şimdi `RefCell<T>` ekleyerek bunu mümkün kılacağız. Liste 15-24 bunu gösterir.

<Listing number="15-24" file-name="src/main.rs" caption="Değiştirilebilir `Liste` oluşturmak için `Rc<RefCell<i32>>` kullanmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

</Listing>

Önce `Rc<RefCell<i32>>` türünden `deger` oluşturuyoruz ki sonra doğrudan
erişebilelim. Ardından `a` listesini, `deger`i taşıyan `Dugum` varyantıyla
kuruyoruz. Burada `deger`i klonlamamız gerekir; böylece içteki `5`in sahipliği
hem `deger`de hem `a`da olur.

`a` listesini `Rc<T>` içine sarıyoruz ki `b` ve `c` oluşturulurken ikisi de
`a`yı işaret edebilsin.

Listeler kurulduktan sonra `deger`e `10` eklemek istiyoruz. Bunun için
`borrow_mut` çağırıyoruz. Rust'ın otomatik başvuru çözmesi, `Rc<T>`yi içteki
`RefCell<T>`ye indirger. `borrow_mut`, `RefMut<T>` döndürür; biz de bunu
çözerek iç değeri değiştiririz.

`a`, `b` ve `c`yi yazdırdığımızda hepsinin artık `15` içerdiğini görürüz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

Bu teknik oldukça kullanışlıdır. Dışarıdan bakınca değiştirilemez bir `Liste`
gibi görünür; ama `RefCell<T>`nin sunduğu API ile gerektiğinde iç veriyi
değiştirebiliriz. Çalışma zamanındaki ödünç denetimi veri yarışlarını önler;
bazı veri yapılarında biraz performans kaybına karşılık bu esneklik gayet
değerlidir. Elbette `RefCell<T>` çok iş parçacıklı kodda çalışmaz; onun güvenli
karşılığı olan `Mutex<T>`yi 16. bölümde göreceğiz.

[wheres-the---operator]: ch05-03-metotlar.html#--operatörüne-ne-oldu
