<!-- Old headings. Do not remove or links may break. -->

<a id="putting-it-all-together-futures-tasks-and-threads"></a>
<a id="futures-tasks-and-threads"></a>
<a id="her-şeyi-birleştirmek-future-görev-ve-iş-parçacığı"></a>

## Future'lar, Görevler (Tasks) ve Thread'ler

[Bölüm 16'da][ch16]<!-- ignore --> gördüğümüz gibi, thread'ler (iş parçacıkları)
eşzamanlılığa yönelik bir yaklaşım sunar. Bu bölümde ise başka bir yaklaşım
gördük: future'lar ve akışlarla birlikte async kullanmak. Bir yöntemi diğerine
ne zaman tercih edeceğinizi merak ediyorsanız yanıt şudur: duruma göre değişir!
Üstelik pek çok durumda tercih thread'ler _veya_ async değil, thread'ler _ve_
async'tir.

Pek çok işletim sistemi onlarca yıldır thread tabanlı eşzamanlılık modelleri
sunmaktadır ve bunun sonucunda pek çok programlama dili de bunları destekler. Ne
var ki bu modellerin de kendine özgü ödünleşimleri vardır. Birçok işletim
sisteminde her bir thread için hatırı sayılır miktarda bellek kullanılır. Ayrıca
thread'ler yalnızca işletim sisteminiz ve donanımınız onları desteklediğinde bir
seçenektir. Yaygın masaüstü ve mobil bilgisayarların aksine, bazı gömülü
sistemlerde bir işletim sistemi bile bulunmaz; dolayısıyla thread'leri de
yoktur.

Asenkron model ise farklı —ve nihayetinde tamamlayıcı— bir ödünleşim kümesi
sunar. Asenkron modelde eşzamanlı işlemler kendi thread'lerini gerektirmez.
Bunun yerine, akışlar bölümünde senkron bir fonksiyondan iş başlatmak üzere
`trpl::spawn_task` kullandığımızda olduğu gibi görevler üzerinde çalışabilirler.
Bir görev bir thread'e benzer; ancak işletim sistemi tarafından yönetilmek
yerine kütüphane düzeyindeki kod —yani çalışma zamanı— tarafından yönetilir.

Thread başlatma API'leri ile görev başlatma API'lerinin birbirine bu kadar
benzemesinin bir nedeni vardır. Thread'ler, senkron işlem kümeleri için bir
sınır görevi görür; eşzamanlılık thread'ler _arasında_ mümkündür. Görevler ise
_asenkron_ işlem kümeleri için bir sınır görevi görür; bir görev kendi
gövdesindeki future'lar arasında geçiş yapabildiğinden, eşzamanlılık hem
görevler _arasında_ hem de görevlerin _içinde_ mümkündür. Son olarak future'lar,
Rust'ın en ince ayrıntılı eşzamanlılık birimidir ve her bir future, diğer
future'lardan oluşan bir ağacı temsil edebilir. Çalışma zamanı —daha doğrusu
onun yürütücüsü (*executor*)— görevleri yönetir, görevler de future'ları
yönetir. Bu bakımdan görevler; işletim sistemi yerine bir çalışma zamanı
tarafından yönetilmenin getirdiği ek yeteneklere sahip, çalışma zamanı
yönetimindeki hafif thread'lere benzer.

Bu durum, asenkron görevlerin her zaman thread'lerden daha iyi olduğu (ya da tam
tersinin geçerli olduğu) anlamına gelmez. Thread'lerle eşzamanlılık, bazı
açılardan `async` ile eşzamanlılığa kıyasla daha basit bir programlama
modelidir. Bu bir güçlü yön de olabilir, bir zayıf yön de. Thread'ler bir bakıma
“ateşle ve unut” (*fire and forget*) mantığındadır; bir future'ın doğal
karşılığına sahip olmadıklarından, bizzat işletim sistemi tarafından kesintiye
uğratılmadıkları sürece kesintisiz biçimde tamamlanana dek çalışırlar.

Üstelik görevler (en azından bazı çalışma zamanlarında) thread'ler arasında
taşınabildiği için, thread'ler ve görevler çoğu zaman birlikte çok iyi çalışır.
Nitekim perde arkasında, `spawn_blocking` ve `spawn_task` fonksiyonları da dahil
olmak üzere bu bölüm boyunca kullandığımız çalışma zamanı varsayılan olarak çok
thread'lidir! Pek çok çalışma zamanı, sistemin genel performansını artırmak
amacıyla thread'lerin o anki kullanım durumuna bağlı olarak görevleri thread'ler
arasında şeffaf biçimde taşıyan _iş çalma (work stealing)_ adlı bir yaklaşım
kullanır. Bu yaklaşım aslında hem thread'leri _hem de_ görevleri —ve dolayısıyla
future'ları— gerektirir.

Hangi yöntemi ne zaman kullanacağınızı düşünürken şu pratik kuralları göz önünde
bulundurabilirsiniz:

- İş _yüksek ölçüde paralelleştirilebiliyorsa_ (yani CPU bağımlıysa) —örneğin
  her bir parçası ayrı ayrı işlenebilecek büyük miktarda veriyi işlemek gibi—
  thread'ler daha iyi bir tercihtir.
- İş _yüksek ölçüde eşzamanlıysa_ (yani G/Ç bağımlıysa) —örneğin pek çok farklı
  kaynaktan farklı aralıklarda veya farklı hızlarda gelebilecek mesajları ele
  almak gibi— async daha iyi bir tercihtir.

Hem paralelliğe hem de eşzamanlılığa ihtiyaç duyuyorsanız, thread'ler ile async
arasında seçim yapmak zorunda değilsiniz. Her birinin en iyi olduğu rolü
üstlenmesine izin vererek ikisini özgürce bir arada kullanabilirsiniz. Örneğin
Liste 17-25, gerçek dünyadaki Rust kodlarında bu tür bir birleşimin oldukça
yaygın bir örneğini göstermektedir.

<Listing number="17-25" caption="Bir thread içinde bloklayıcı kodla mesaj göndermek ve mesajları bir async blok içinde beklemek" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:all}}
```

</Listing>

İşe bir asenkron kanal oluşturarak başlıyor, ardından `move` anahtar kelimesini
kullanarak kanalın gönderici tarafının (`gonderici`) sahipliğini alan bir thread
başlatıyoruz. Thread içinde 1'den 10'a kadar olan sayıları gönderiyor ve her
birinin arasında bir saniye uyuyoruz. Son olarak, bölüm boyunca yaptığımız gibi
`trpl::block_on` fonksiyonuna geçirilen bir async blokla oluşturulmuş future'ı
çalıştırıyoruz. Bu future içinde, gördüğümüz diğer mesaj iletimi örneklerinde
olduğu gibi bu mesajları (`mesaj`) `await` ile bekliyoruz.

Bölümü açtığımız senaryoya geri dönecek olursak; bir dizi video kodlama işini
(video kodlama hesaplama bağımlı olduğu için) adanmış bir thread kullanarak
çalıştırdığınızı, ancak bu işlemlerin tamamlandığını kullanıcı arayüzüne bir
asenkron kanal aracılığıyla bildirdiğinizi düşünün. Gerçek dünya kullanım
senaryolarında bu tür birleşimlerin sayısız örneği vardır.

> Not: **Tek Bakışta Eşzamanlılık Katmanları ve Senkron-Asenkron Köprüsü**
>
> Bölüm 16 ve Bölüm 17'de ele aldığımız üç temel yapı taşı, birbirinin rakibi
> değil farklı soyutlama katmanlarıdır:
>
> - **Future (Derleyici düzeyinde durum makinesi):** En küçük eşzamanlılık
>   birimidir. Yalnızca `.await` noktaları arasında yaşayacak değişkenler kadar
>   bellek kaplar; kendi başına çalışmaz, içinde bulunduğu görev tarafından
>   yoklandıkça işbirlikli (*cooperative*) olarak ilerler.
> - **Görev / Task (Çalışma zamanı yönetiminde birim):** Bir future ağacının
>   yürütme sınırıdır. Yürütücü (*executor*) tarafından kullanıcı alanında
>   çizelgelenir; bağlam değişimi işletim sistemi çekirdeğine inmeden çok düşük
>   maliyetle gerçekleştiği için binlerce eşzamanlı G/Ç bağlantısını yönetmeye
>   uygundur.
> - **Thread (İşletim sistemi yönetiminde birim):** Kendine ait ayrı bir yığın
>   (*stack*) belleği ayırır ve işletim sistemi çekirdeği tarafından kesmeli
>   (*preemptive*) olarak çizelgelenir. Fiziksel CPU çekirdeklerinde gerçek
>   paralellik sağladığı için yoğun hesaplama yüklerine uygundur.
>
> Hibrit mimarilerde bu katmanları birleştirirken Liste 17-25'teki inceliğe
> dikkat edin: sınırsız (*unbounded*) asenkron kanallarda `gonderici.send(i)`
> çağrısı karşı tarafı beklemeden anında tamamlandığı için senkron bir metottur
> ve `await` gerektirmeden doğrudan `std::thread` içinden çağrılabilir;
> `alici.recv().await` ise mesaj gelene dek bekleyebildiği için asenkrondur. Bu
> sayede kanallar, senkron thread dünyası ile asenkron görev dünyası arasında
> doğal ve güvenli bir köprü kurar.

<!-- Old headings. Do not remove or links may break. -->

<a id="summary"></a>

## Özet

Bu kitapta eşzamanlılık konusunu son kez görmüyorsunuz. [Bölüm 21'deki][ch21]<!-- ignore -->
proje, bu kavramları burada ele aldığımız daha basit örneklerden daha gerçekçi
bir durumda uygulayacak ve problemleri thread'lerle çözmek ile görevler ve
future'larla çözmeyi daha doğrudan karşılaştıracaktır.

Bu yaklaşımlardan hangisini seçerseniz seçin, Rust —ister yüksek verimli, yoğun
trafikli bir web sunucusu ister gömülü bir işletim sistemi için olsun— güvenli,
hızlı ve eşzamanlı kod yazmak üzere ihtiyaç duyduğunuz araçları size sunar.

Sıradaki bölümde, Rust programlarınız büyüdükçe problemleri modellemenin ve
çözümleri yapılandırmanın deyimsel (*idiomatic*) yollarından bahsedeceğiz. Ek
olarak, Rust'ın deyimsel kalıplarının nesne yönelimli programlamadan aşina
olabileceğiniz yaklaşımlarla nasıl bir ilişki kurduğunu tartışacağız.

[ch16]: ch16-00-eszamanlilik.html
[combining-futures]: ch17-03-daha-fazla-future.html#kendi-async-soyutlamalarımızı-kurmak
[streams]: ch17-04-akislar.html#akışlar-sırayla-gelen-futurelar
[ch21]: ch21-00-final-projesi-web-sunucusu.html
