<!-- Old headings. Do not remove or links may break. -->

<a id="extensible-concurrency-with-the-sync-and-send-traits"></a>
<a id="extensible-concurrency-with-the-send-and-sync-traits"></a>

## `Send` ve `Sync` ile Genişletilebilir Eşzamanlılık

İlginç bir nokta şu: Bu bölümde şimdiye kadar gördüğümüz eşzamanlılık
özelliklerinin neredeyse tamamı dilin kendisinden değil, standart
kütüphaneden geliyor. Yani eşzamanlılığı ele alırken seçenekleriniz yalnızca
dilin veya standart kütüphanenin sunduklarıyla sınırlı değil; kendi
eşzamanlılık araçlarınızı yazabilir ya da başkalarının yazdığı çözümleri
kullanabilirsiniz.

Buna rağmen, dilin içine gömülü temel eşzamanlılık kavramları arasında
standart kütüphanedeki `std::marker` trait'leri olan `Send` ve `Sync` özel bir
yer tutar.

<!-- Old headings. Do not remove or links may break. -->

<a id="allowing-transference-of-ownership-between-threads-with-send"></a>

### İş Parçacıkları Arasında Sahiplik Aktarmak

`Send` işaretleyici trait'i, bu trait'i uygulayan bir türün değerlerinin
iş parçacıkları arasında taşınabildiğini söyler. Rust'taki türlerin neredeyse
tamamı `Send` uygular; ama `Rc<T>` gibi bazı istisnalar vardır. `Rc<T>`,
`Send` olamaz; çünkü bir `Rc<T>` değerini `clone` edip kopyanın sahipliğini
başka bir iş parçacığına aktarsaydınız, iki iş parçacığı da referans sayısını
aynı anda güncelleyebilirdi. Bu nedenle `Rc<T>`, iş parçacığı güvenliğinin
performans maliyetini ödemek istemediğiniz tek iş parçacıklı senaryolar için
tasarlanmıştır.

Dolayısıyla Rust'ın tür sistemi ve trait sınırları, bir `Rc<T>` değerini
yanlışlıkla iş parçacıkları arasında güvensiz şekilde göndermenizi engeller.
Bunu 16-14 numaralı listede denediğimizde `` the trait `Send` is not
implemented for `Rc<Mutex<i32>>` `` hatasını aldık. `Send` uygulayan `Arc<T>`
kullandığımız anda kod derlendi.

Tamamı `Send` türlerinden oluşan bir tür de otomatik olarak `Send` kabul edilir.
Ham işaretçiler hariç, ilkel türlerin neredeyse tamamı `Send`'dir; ham
işaretçilere 20. bölümde döneceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="allowing-access-from-multiple-threads-with-sync"></a>

### Birden Fazla İş Parçacığından Erişim

`Sync` işaretleyici trait'i, bu trait'i uygulayan türlere birden fazla
iş parçacığından referans vermenin güvenli olduğunu belirtir. Başka bir deyişle
bir `T` türü için `&T` (`T`'ye değiştirilemez referans) güvenle başka bir
iş parçacığına gönderilebiliyorsa, o `T` türü `Sync` uygular. `Send`'e benzer
biçimde ilkel türlerin tamamı `Sync` uygular; tamamen `Sync` türlerden oluşan
türler de yine `Sync` olur.

Akıllı işaretçi `Rc<T>`, `Send` uygulamamasının sebebiyle aynı nedenle `Sync`
de uygulamaz. 15. bölümde gördüğümüz `RefCell<T>` ile onun akrabası olan
`Cell<T>` ailesi de `Sync` değildir. Çünkü `RefCell<T>`'nin çalışma zamanında
yaptığı ödünç denetimi iş parçacığı güvenli değildir. Buna karşılık akıllı
işaretçi `Mutex<T>`, `Sync` uygular ve  [“`Mutex<T>` için Paylaşımlı Erişim”]
[shared-access]<!-- ignore --> bölümünde gördüğünüz gibi birden fazla iş
parçacığı arasında erişim paylaşmak için kullanılabilir.

### `Send` ve `Sync`'i Elle Uygulamak Güvensizdir

Tamamı `Send` ve `Sync` uygulayan parçalardan oluşan türler bu trait'leri
zaten otomatik olarak aldığı için, çoğu zaman bunları elle uygulamamız gerekmez.
Üstelik bunlar işaretleyici trait olduğu için uygulanacak bir metod da yoktur;
esas amaçları eşzamanlılığa ilişkin bazı değişmezleri zorunlu kılmaktır.

Bu trait'leri elle uygulamak, `unsafe` Rust kodu yazmayı gerektirir.
`unsafe` Rust'ı 20. bölümde ele alacağız. Şimdilik bilinmesi gereken nokta şu:
`Send` ve `Sync` parçalardan oluşmayan yeni eşzamanlı türler tasarlamak,
güvenlik güvencelerini koruyabilmek için çok dikkatli düşünmeyi gerektirir.
[“The Rustonomicon”][nomicon], bu güvenceler ve onları nasıl koruyacağınız
hakkında daha ayrıntılı bilgi içerir.

## Özet

Bu kitapta eşzamanlılık konusunu son kez görmüyorsunuz: Bir sonraki bölüm
eşzamansız programlamaya odaklanacak ve 21. bölümdeki proje bu bölümdeki
kavramları, burada gördüğümüz küçük örneklerden daha gerçekçi bir senaryoda
kullanacak.

Daha önce de değindiğimiz gibi, Rust'ın eşzamanlılığı ele alış biçiminin çok
az bir kısmı dilin parçasıdır; birçok çözüm crate'ler halinde sunulur. Bu
crate'ler standart kütüphaneden daha hızlı gelişir. Bu yüzden çok iş parçacıklı
senaryolarda kullanılacak güncel ve güçlü crate'leri çevrim içi araştırmayı
ihmal etmeyin.

Rust standart kütüphanesi, mesaj iletimi için kanallar ve `Mutex<T>` ile
`Arc<T>` gibi eşzamanlı bağlamlarda güvenle kullanılabilen akıllı işaretçiler
sağlar. Tür sistemi ile ödünç alma denetleyicisi, bu çözümleri kullanan
kodunuzun veri yarışına ya da geçersiz referanslara sürüklenmemesini sağlar.
Kodunuz bir kez derlendikten sonra, başka dillerde peşine düşmesi zor olan
hatalara kapılmadan birden fazla iş parçacığında güvenle çalışacağına
inanabilirsiniz. Eşzamanlı programlama artık korkulacak bir şey değil:
Programlarınızı korkusuzca eşzamanlı hale getirin!

[shared-access]: ch16-03-paylasimli-durum.html#mutext-için-paylaşımlı-erişim
[nomicon]: ../nomicon/index.html
