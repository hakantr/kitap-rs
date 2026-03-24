# Giriş

> Not: Kitabın bu sürümü, [No Starch Press][nsp] tarafından basılı ve e-kitap formatında sunulan [The Rust Programming Language][nsprust] kitabı ile aynıdır.

[nsprust]: https://nostarch.com/rust-programming-language-3rd-edition
[nsp]: https://nostarch.com/

Rust hakkında giriş seviyesinde bir kitap olan _Rust Programlama Dili_ (The Rust Programming Language) kitabına hoş geldiniz.
Rust programlama dili, daha hızlı ve daha güvenilir yazılımlar geliştirmenize yardımcı olur.
Programlama dili tasarımında üst düzey ergonomi ile alt düzey kontrol genellikle birbirine ters düşer; Rust bu çatışmaya meydan okuyor.
Güçlü teknik kapasite ile harika bir geliştirici deneyimini dengeleyen Rust, geleneksel olarak bu tür bir kontrolle ilişkilendirilen tüm zorluklar olmadan alt düzey detayları (örneğin bellek kullanımı gibi) kontrol etme seçeneği sunar.

## Rust Kimler İçin?

Rust, birçok farklı nedenden ötürü pek çok insan için idealdir. Gelin en önemli gruplardan birkaçına bakalım.

### Geliştirici Ekipleri

Rust, farklı düzeylerde sistem programlama bilgisine sahip geliştiricilerden oluşan büyük ekipler arasında işbirliği yapmak için üretken bir araç olduğunu kanıtlıyor.
Alt düzey kodlar, diğer çoğu dilde ancak kapsamlı testler ve deneyimli geliştiriciler tarafından yapılan dikkatli kod incelemeleriyle yakalanabilen çeşitli ince hatalara eğilimlidir.
Rust'ta ise derleyici, eşzamanlılık (concurrency) hataları da dahil olmak üzere bu yakalaması zor hatalara sahip kodları derlemeyi reddederek bir bekçi (gatekeeper) rolü üstlenir.
Ekip, derleyiciyle birlikte çalışarak zamanını hataların peşinden koşmak yerine programın mantığına odaklanarak geçirebilir.

Rust ayrıca sistem programlama dünyasına çağdaş geliştirici araçları da getirir:

- Rust ile birlikte gelen bağımlılık yöneticisi ve derleme aracı olan Cargo, Rust ekosistemi genelinde bağımlılıkları eklemeyi, derlemeyi ve yönetmeyi zahmetsiz ve tutarlı hale getirir.
- `rustfmt` formatlama aracı, geliştiriciler arasında tutarlı bir kodlama stili sağlar.
- Rust Language Server, kod tamamlama ve satır içi hata mesajları için entegre geliştirme ortamı (IDE) entegrasyonuna güç verir.

Geliştiriciler, Rust ekosistemindeki bu ve diğer araçları kullanarak sistem düzeyinde kod yazarken dahi üretken olabilirler.

### Öğrenciler

Rust, sistem kavramları hakkında bilgi edinmek isteyen öğrenciler ve meraklılar içindir.
Birçok kişi işletim sistemi geliştirme gibi konuları Rust kullanarak öğrendi.
Topluluk çok misafirperverdir ve öğrencilerin sorularını yanıtlamaktan mutluluk duyar.
Bu kitap gibi çabalar aracılığıyla Rust ekipleri, sistem kavramlarını özellikle programlamaya yeni başlayanlar olmak üzere daha fazla insan için daha erişilebilir hale getirmek istiyor.

### Şirketler

İrili ufaklı yüzlerce şirket; komut satırı araçları, web servisleri, DevOps araçları, gömülü cihazlar, ses ve video analizi ve dönüştürmesi, kripto paralar, biyoinformatik, arama motorları, Nesnelerin İnterneti (IoT) uygulamaları, makine öğrenimi ve hatta Firefox web tarayıcısının önemli parçaları da dahil olmak üzere çok çeşitli görevler için üretim ortamında (production) Rust kullanıyor.

### Açık Kaynak Geliştiricileri

Rust; Rust programlama dilini, topluluğunu, geliştirici araçlarını ve kütüphanelerini inşa etmek isteyenler içindir.
Rust diline katkıda bulunmanızdan mutluluk duyarız.

### Hıza ve Kararlılığa Önem Verenler

Rust, bir dilde hız ve kararlılık (stability) arzulayanlar içindir.
Hız derken, hem Rust kodunun ne kadar hızlı çalışabildiğini hem de Rust'ın size ne kadar hızlı program yazdırdığını kastediyoruz.
Rust derleyicisinin kontrolleri, özellik eklemeleri ve yeniden yapılandırma yoluyla kararlılığı sağlar.
Bu durum, geliştiricilerin genellikle değiştirmekten korktuğu, bu tür kontrollere sahip olmayan dillerdeki kırılgan eski (legacy) kodlarla tam bir tezat oluşturur.
Rust, sıfır maliyetli soyutlamalar için çabalayarak (elle yazılmış kod kadar hızlı bir şekilde alt düzey koda derlenen üst düzey özellikler), güvenli kodun aynı zamanda hızlı kod olmasını sağlamaya çalışır.

Rust dili, diğer birçok kullanıcıyı da desteklemeyi umuyor; burada bahsedilenler sadece en büyük paydaşlardan bazıları.
Genel olarak Rust'ın en büyük hedefi, güvenlik _ve_ üretkenlik, hız _ve_ ergonomi sunarak programcıların on yıllardır kabul ettiği ödünleşimleri (trade-offs) ortadan kaldırmaktır.
Rust'a bir şans verin ve seçimlerinin sizin için işe yarayıp yaramadığına bakın.

## Bu Kitap Kimler İçin?

Bu kitap, başka bir programlama dilinde kod yazdığınızı varsayar, ancak hangisi olduğu konusunda herhangi bir varsayımda bulunmaz.
Materyali çok çeşitli programlama altyapılarından gelenler için geniş ölçüde erişilebilir hale getirmeye çalıştık.
Programlamanın _ne olduğu_ veya onun hakkında nasıl düşünülmesi gerektiği hakkında konuşarak çok fazla zaman harcamıyoruz.
Programlamaya tamamen yeniyseniz, özellikle programlamaya giriş sağlayan bir kitap okumanız sizin için daha faydalı olacaktır.

## Bu Kitap Nasıl Kullanılır?

Genel olarak, bu kitap sizin onu baştan sona sırayla okuduğunuzu varsayar.
İlerleyen bölümler önceki bölümlerdeki kavramlar üzerine inşa edilir ve önceki bölümler belirli bir konudaki ayrıntılara inmeyebilir, ancak bu konuyu ilerleyen bölümlerde tekrar ele alacaktır.

Bu kitapta iki tür bölüm bulacaksınız: kavram bölümleri ve proje bölümleri.
Kavram bölümlerinde Rust'ın bir yönü hakkında bilgi edineceksiniz.
Proje bölümlerinde ise şu ana kadar öğrendiklerinizi uygulayarak birlikte küçük programlar geliştireceğiz.
Bölüm 2, Bölüm 12 ve Bölüm 21 proje bölümleridir; geri kalanlar ise kavram bölümleridir.

**Bölüm 1**, Rust'ın nasıl kurulacağını, "Merhaba, dünya!" programının nasıl yazılacağını ve Rust'ın paket yöneticisi ve derleme aracı olan Cargo'nun nasıl kullanılacağını açıklar.
**Bölüm 2**, bir sayı tahmin oyunu geliştirmenizi sağlayarak Rust'ta program yazmaya yönelik uygulamalı bir giriştir.
Burada kavramları üst düzeyde ele alıyoruz; sonraki bölümler ek ayrıntılar sağlayacaktır.
Eğer hemen ellerinizi kirletmek (pratik yapmak) istiyorsanız, Bölüm 2 tam size göre.
Eğer bir sonrakine geçmeden önce her ayrıntıyı öğrenmeyi tercih eden, özellikle titiz bir öğreniciyseniz, Bölüm 2'yi atlayıp diğer programlama dillerindekine benzer Rust özelliklerini kapsayan **Bölüm 3**'e geçmek isteyebilirsiniz; daha sonra, öğrendiğiniz ayrıntıları uygulayacağınız bir proje üzerinde çalışmak istediğinizde Bölüm 2'ye dönebilirsiniz.

**Bölüm 4**'te Rust'ın sahiplik sistemini öğreneceksiniz.
**Bölüm 5** struct'ları ve metotları tartışıyor.
**Bölüm 6** enum'ları, `match` ifadelerini ve `if let` ile `let...else` kontrol akışı (control flow) yapılarını kapsıyor. Özel türler (custom types) oluşturmak için struct'ları ve enum'ları kullanacaksınız.

**Bölüm 7**'de Rust'ın modül sistemini, kodunuzu ve genel uygulama programlama arayüzünü (API) düzenlemek için gizlilik kurallarını öğreneceksiniz.
**Bölüm 8**, standart kütüphanenin sağladığı bazı yaygın koleksiyon veri yapılarını tartışır: vektörler, string'ler ve hash map'ler.
**Bölüm 9**, Rust'ın hata yönetimi (error-handling) felsefesini ve tekniklerini inceler.

**Bölüm 10**, birden fazla türe uygulanan kod tanımlama gücü veren generic'leri (jenerikler), trait'leri (özellikler) ve lifetime'ları (yaşam süreleri) derinlemesine inceler.
**Bölüm 11** tamamen test etme ile ilgilidir, ki bu Rust'ın güvenlik garantilerinde bile programınızın mantığının doğru olduğundan emin olmak için gereklidir.
**Bölüm 12**'de, dosyalar içinde metin arayan `grep` komut satırı aracının işlevlerinin bir alt kümesinin kendi uygulamamızı geliştireceğiz. Bunun için önceki bölümlerde tartıştığımız birçok kavramı kullanacağız.

**Bölüm 13**, fonksiyonel programlama dillerinden gelen Rust özellikleri olan closure'ları (kapanışlar) ve iterator'leri (yineleyiciler) inceler.
**Bölüm 14**'te Cargo'yu daha derinlemesine inceleyecek ve kütüphanelerinizi başkalarıyla paylaşmak için en iyi uygulamalar hakkında konuşacağız.
**Bölüm 15**, standart kütüphanenin sağladığı akıllı işaretçileri (smart pointers) ve onların işlevselliğini sağlayan trait'leri tartışır.

**Bölüm 16**'da farklı eşzamanlı (concurrent) programlama modellerini gözden geçirecek ve Rust'ın birden fazla thread'de (iş parçacığı) korkusuzca programlama yapmanıza nasıl yardımcı olduğunu konuşacağız.
**Bölüm 17**'de, Rust'ın async ve await sözdizimini; task'lar (görevler), future'lar ve stream'ler (akışlar) ve bunların sağladığı hafif eşzamanlılık modeliyle birlikte inceleyerek bu konuyu daha da ileri taşıyoruz.

**Bölüm 18**, Rust'ın deyimlerinin (idioms) aşina olabileceğiniz nesne yönelimli programlama (OOP) ilkeleriyle nasıl karşılaştırıldığına bakar.
**Bölüm 19**, Rust programları boyunca fikirleri ifade etmenin güçlü yolları olan pattern'ler (desenler) ve pattern matching (desen eşleştirme) üzerine bir referanstır.
**Bölüm 20**, unsafe (güvensiz) Rust, makrolar ve lifetime'lar, trait'ler, türler, fonksiyonlar ve closure'lar hakkında daha fazlasını içeren, ilgi çekici ileri düzey konuların bir karışımını içerir.

**Bölüm 21**'de alt düzey, çoklu iş parçacıklı (multithreaded) bir web sunucusu uygulayacağımız bir projeyi tamamlayacağız!

Son olarak, bazı ekler dil hakkında daha referans benzeri bir formatta faydalı bilgiler içerir.
**Ek A** Rust'ın anahtar kelimelerini, **Ek B** Rust'ın operatörlerini ve sembollerini, **Ek C** standart kütüphane tarafından sağlanan türetilebilir (derivable) trait'leri, **Ek D** bazı yararlı geliştirme araçlarını ve **Ek E** Rust sürümlerini (editions) kapsar.
**Ek F**'de kitabın çevirilerini bulabilirsiniz ve **Ek G**'de Rust'ın nasıl yapıldığını ve gecelik (nightly) Rust'ın ne olduğunu ele alacağız.

Bu kitabı okumanın yanlış bir yolu yoktur: Eğer ileri atlamak istiyorsanız, durmayın!
Herhangi bir kafa karışıklığı yaşarsanız önceki bölümlere geri dönmeniz gerekebilir. Ancak sizin için ne işe yarıyorsa onu yapın.

<span id="ferris"></span>

Rust öğrenme sürecinin önemli bir parçası, derleyicinin görüntülediği hata mesajlarını nasıl okuyacağınızı öğrenmektir: Bunlar sizi çalışan koda doğru yönlendirecektir.
Bu nedenle, derleyicinin her durumda size göstereceği hata mesajıyla birlikte, derlenmeyen birçok örnek sunacağız.
Rastgele bir örnek girip çalıştırırsanız, derlenmeyebileceğini bilin!
Çalıştırmaya çalıştığınız örneğin hata verip vermeyeceğini görmek için etrafındaki metni okuduğunuzdan emin olun.
Çoğu durumda, sizi derlenmeyen herhangi bir kodun doğru sürümüne yönlendireceğiz.
Ferris ayrıca, çalışması amaçlanmayan kodu ayırt etmenize yardımcı olacaktır:

| Ferris                                                                                                           | Anlamı                                            |
| ---------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Soru işaretli Ferris"/>                   | Bu kod derlenmez!                                 |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ellerini havaya kaldırmış Ferris"/>                 | Bu kod panik yaratır!                     |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Tek pençesi havada, omuz silken Ferris"/> | Bu kod istenen davranışı üretmez.                 |

Çoğu durumda, sizi derlenmeyen herhangi bir kodun doğru sürümüne yönlendireceğiz.

## Kaynak Kod

Bu kitabın oluşturulduğu kaynak dosyalar [GitHub][book]'ta bulunabilir.

[book]: https://github.com/rust-lang/book/tree/main/src
