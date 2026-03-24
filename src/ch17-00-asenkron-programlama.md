# Eşzamansız Programlamanın Temelleri: Async, Await, Future ve Akışlar

Bilgisayardan yapmasını istediğimiz birçok işlem biraz zaman alabilir. Bu uzun
süren işler tamamlanmayı beklerken başka bir şeyler de yapabilsek güzel olurdu.
Modern bilgisayarlar aynı anda birden fazla iş üzerinde çalışmak için iki temel
yaklaşım sunar: paralellik ve eşzamanlılık. Buna karşılık programlarımızın
mantığı çoğu zaman daha doğrusal yazılır. Programın hangi işlemleri yapacağını
ve bir fonksiyonun hangi noktalarda duraklayıp başka bir bölümün devreye
girebileceğini, her kod parçasının tam olarak hangi sırayla ve nasıl
çalışacağını en baştan tek tek tarif etmek zorunda kalmadan ifade edebilmek
isteriz. _Eşzamansız programlama (asynchronous programming)_, tam da bunu
sağlayan bir soyutlamadır: kodu, olası duraklama noktaları ve sonunda üretilecek
sonuçlar üzerinden ifade etmemize izin verir; geri kalan koordinasyon
ayrıntılarını da bizim yerimize üstlenir.

Bu bölüm, 16. bölümde iş parçacıklarıyla gördüğümüz paralellik ve eşzamanlılık
yaklaşımını temel alıp alternatif bir yol tanıtıyor: Rust'ın future'ları,
akışları, `async` ve `await` sözdizimi ve eşzamansız işlemlerin yürütülmesini
yöneten üçüncü taraf çalışma zamanı crate'leri.

Bir örnek düşünelim. Diyelim ki aile kutlamasından hazırladığınız bir videoyu
dışa aktarıyorsunuz. Bu işlem dakikalar, hatta saatler sürebilir. Video dışa
aktarma, kullanabildiği kadar CPU ve GPU gücü tüketir. Eğer tek CPU çekirdeğiniz
olsaydı ve işletim sistemi bu dışa aktarmayı bitene kadar durdurmasaydı, yani
işlem _senkron_ çalışsaydı, bu görev sürerken bilgisayarda başka hiçbir şey
yapamazdınız. Bu oldukça can sıkıcı olurdu. Neyse ki işletim sistemleri bu tür
işleri görünmez biçimde yeterince sık bölerek aynı anda başka işlerin de
yürümesine olanak tanır.

Şimdi de başkasının paylaştığı bir videoyu indirdiğinizi düşünün. Bu da zaman
alabilir; ama CPU'yu video dışa aktarma kadar meşgul etmez. Burada CPU, ağdan
veri gelmesini beklemek zorundadır. Veri gelmeye başlar başlamaz okumaya
başlayabilirsiniz; ama tamamının ulaşması zaman alabilir. Veri geldikten sonra
bile, video büyükse hepsini belleğe almak bir iki saniye sürebilir. Bu kısa
gibi görünse de, saniyede milyarlarca işlem yapabilen modern işlemciler için
uzun bir süredir. Yine işletim sistemi, ağ çağrısı tamamlanana kadar beklerken
CPU'nun başka işler yapmasına olanak tanımak için programı görünmez biçimde
keser.

Video dışa aktarma, _CPU-bağımlı_ ya da _hesaplama-bağımlı_ bir işe örnektir.
Sınırını, CPU veya GPU'nun işleme hızı ve bu hızın ne kadarını bu işe
ayırabildiği belirler. Video indirme ise _G/Ç-bağımlı (I/O-bound)_ bir işlemdir;
burada sınır, bilgisayarın _girdi/çıktı_ hızıdır. Ağ üzerinden veri ne kadar
hızlı gelirse işlem de ancak o kadar hızlı ilerleyebilir.

Her iki örnekte de işletim sisteminin görünmez kesmeleri bir tür eşzamanlılık
sağlar. Ancak bu eşzamanlılık tüm program düzeyindedir: işletim sistemi bir
programı durdurup başka programlara çalışma fırsatı verir. Çoğu durumda ise,
programlarımızı işletim sisteminden daha ayrıntılı bildiğimiz için, onun
göremediği eşzamanlılık fırsatlarını biz fark edebiliriz.

Örneğin dosya indirmelerini yöneten bir araç yazıyorsak, tek bir indirme
başlatmanın arayüzü kilitlememesi gerekir; kullanıcılar aynı anda birden fazla
indirme de başlatabilmelidir. Oysa ağla etkileşen birçok işletim sistemi API'si
_bloklayıcıdır (blocking)_; yani işlediği veri tamamen hazır olana kadar
programın ilerleyişini durdurur.

> Not: Aslında düşünürseniz çoğu fonksiyon çağrısı da böyledir. Ama
> _bloklayıcı_ terimi genellikle dosyalar, ağ ya da bilgisayardaki başka
> kaynaklarla etkileşen çağrılar için kullanılır; çünkü özellikle bu tür
> durumlarda işlemin _bloklamayan_ olması program açısından önemli fayda sağlar.

Ana iş parçacığımızı bloklamaktan kaçınmak için her dosya indirmesine ayrı bir
iş parçacığı ayırabilirdik. Ancak bu iş parçacıklarının kullandığı sistem
kaynaklarının ek maliyeti bir noktadan sonra sorun olur. İdeal olan, çağrının
baştan bloklamaması ve bizim yalnızca programın tamamlamasını istediğimiz bir
iş kümesini tanımlayıp, bunların en iyi sırayla ve biçimde nasıl çalıştırılacağı
kararını çalışma zamanına bırakmamızdır.

Rust'ın _async_ (yani _asynchronous_) soyutlaması tam olarak bunu sağlar. Bu
bölümde şu başlıkları ele alacağız:

- Rust'ın `async` ve `await` sözdizimi nasıl kullanılır, eşzamansız fonksiyonlar
  bir çalışma zamanı ile nasıl yürütülür?
- `async` modeli, 16. bölümde gördüğümüz bazı eşzamanlılık problemlerini çözmek
  için nasıl kullanılır?
- Çok iş parçacıklı yapı ile `async`, birbirini tamamlayan çözümler olarak
  hangi durumlarda birlikte kullanılabilir?

Ama önce, `async`'in pratikte nasıl çalıştığını görmeden önce, paralellik ile
eşzamanlılık arasındaki farkı biraz daha netleştirelim.

## Paralellik ve Eşzamanlılık

Şimdiye kadar paralellik ile eşzamanlılığı çoğu yerde neredeyse eş anlamlı gibi
kullandık. Artık bunları biraz daha dikkatli ayırmamız gerekiyor; çünkü
çalışmaya başladıkça aradaki farklar görünür hale gelecek.

Bir yazılım projesindeki işi bir ekibin nasıl paylaştırabileceğini düşünün.
Tek bir kişiye birden fazla görev verebilir, her kişiye tek bir görev
atayabilir ya da bu iki yaklaşımı karıştırabilirsiniz.

Bir kişi, görevlerin hiçbiri bitmeden önce birkaç farklı iş üzerinde sırayla
çalışıyorsa bu _eşzamanlılık_tır. Bunu uygulamanın bir yolu, bilgisayarınızda
iki farklı proje açık tutmaya benzer: birinden sıkıldığınızda ya da bir yerde
takıldığınızda ötekine geçersiniz. Tek kişisinizdir; iki işte tam anlamıyla
aynı anda ilerleyemezsiniz. Ama aralarında gidip gelerek ikisini de yavaş yavaş
ilerletebilirsiniz (bkz. Şekil 17-1).

<figure>

<img src="img/trpl17-01.svg" class="center" alt="A ve B Görevi olarak etiketlenmiş kutularda alt görevlerin ve bunlar arasında geçişlerin gösterildiği diyagram." />

<figcaption>Şekil 17-1: A ve B görevleri arasında geçiş yapılan eşzamanlı bir iş akışı</figcaption>

</figure>

Ekipteki herkes bir görev alıp onun üzerinde tek başına çalışıyorsa bu
_paralellik_tir. Ekipteki her kişi tam anlamıyla aynı anda ilerleme
kaydedebilir (bkz. Şekil 17-2).

<figure>

<img src="img/trpl17-02.svg" class="center" alt="A ve B Görevi üzerinde bağımsız ilerleyen alt görevlerin gösterildiği paralel iş akışı diyagramı." />

<figcaption>Şekil 17-2: A ve B görevlerinin bağımsız yürüdüğü paralel iş akışı</figcaption>

</figure>

Bu iki iş akışında da görevler arasında koordinasyon gerekebilir. Diyelim ki
bir kişiye verilen görevin ötekilerden tamamen bağımsız olduğunu sandınız ama
aslında önce başka birinin işini bitirmesi gerekiyordu. Bu durumda işin bir
kısmı paralel yapılabilirken, bir kısmı da aslında _seri_ olarak, yani birbiri
ardına ilerlemek zorundadır (bkz. Şekil 17-3).

<figure>

<img src="img/trpl17-03.svg" class="center" alt="Bir görevin başka bir görevin sonucunu beklediği kısmen paralel iş akışı diyagramı." />

<figcaption>Şekil 17-3: A3 adımının B3 sonucunu beklediği kısmen paralel bir iş akışı</figcaption>

</figure>

Aynı şekilde, kendi işlerinizden birinin başka bir işinize bağlı olduğunu fark
edebilirsiniz. Bu durumda eşzamanlı çalışmanız da kısmen seri hale gelir.

Paralellik ile eşzamanlılık bazen birbirinin içine de geçer. Bir iş arkadaşınız
siz bir görevi bitirene kadar beklemek zorundaysa, büyük olasılıkla tüm
enerjinizi o göreve verip onun önünü açarsınız. Böylece artık ne onunla paralel
çalışıyorsunuzdur ne de kendi diğer işleriniz arasında eşzamanlı geçiş
yapıyorsunuzdur.

Aynı temel dinamik yazılım ve donanım için de geçerlidir. Tek CPU çekirdekli
bir makinede işlemci aynı anda yalnızca tek bir iş yapabilir; ama yine de
eşzamanlı çalışabilir. İş parçacıkları, süreçler ve `async` gibi araçlarla
bilgisayar bir işi durdurup diğerlerine geçebilir, sonra yeniden ilkine döner.
Birden fazla CPU çekirdeği olan makinede ise gerçekten paralel çalışma da
mümkündür. Bir çekirdek bir işi yaparken diğeri tamamen farklı bir işi aynı
anda yürütebilir.

Rust'ta `async` kod genellikle eşzamanlı yürür. Donanıma, işletim sistemine ve
kullandığımız eşzamanlı çalışma zamanına bağlı olarak, bu eşzamanlılık arka
planda paralellik de kullanabilir.

Şimdi Rust'ta eşzamansız programlamanın gerçekten nasıl çalıştığına bakalım.
