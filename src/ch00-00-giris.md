# Giriş

> Not: Kitabın bu baskısı, [No Starch Press][nsp] tarafından basılı ve e-kitap formatında sunulan [The Rust Programming Language][nsprust] kitabıyla aynıdır.

[nsprust]: https://nostarch.com/rust-programming-language-3rd-edition
[nsp]: https://nostarch.com/

Rust'a giriş niteliğindeki _Rust Programlama Dili_ (_The Rust Programming Language_) kitabına hoş geldiniz. Rust programlama dili, daha hızlı ve daha güvenilir yazılımlar geliştirmenize yardımcı olur. Programlama dili tasarımında üst düzey ergonomi ile alt düzey kontrol genellikle birbiriyle çelişir; Rust ise bu ikileme meydan okur. Güçlü teknik kapasite ile mükemmel bir geliştirici deneyimini dengeleyen Rust, geleneksel olarak bu düzeydeki kontrolün beraberinde getirdiği zahmetlere katlanmak zorunda kalmadan alt düzey ayrıntıları (bellek kullanımı gibi) yönetme olanağı sunar.

## Rust Kimler İçin?

Rust, çeşitli nedenlerle pek çok kişi için ideal bir dildir. Gelin en önemli gruplardan birkaçına birlikte bakalım.

### Geliştirici Ekipleri

Rust, farklı düzeylerde sistem programlama bilgisine sahip geliştiricilerden oluşan büyük ekiplerin iş birliği içinde çalışması için üretken bir araç olduğunu kanıtlamıştır. Alt düzey kodlar, diğer pek çok dilde ancak kapsamlı testler ve deneyimli geliştiricilerin titiz kod incelemeleriyle yakalanabilen çeşitli sinsi hatalara açıktır. Rust'ta ise derleyici, *eşzamanlılık (concurrency)* hataları da dahil olmak üzere yakalanması güç bu hataları barındıran kodları derlemeyi reddederek bir bekçi rolü üstlenir. Ekip, derleyiciyle omuz omuza çalışarak vaktini hataların peşinden koşmak yerine programın mantığına odaklanmaya ayırabilir.

Rust ayrıca sistem programlama dünyasına çağdaş geliştirici araçları da kazandırır:

- Rust ile birlikte gelen bağımlılık yöneticisi ve derleme aracı olan Cargo; bağımlılıkları eklemeyi, derlemeyi ve yönetmeyi tüm Rust ekosistemi genelinde zahmetsiz ve tutarlı hale getirir.
- `rustfmt` biçimlendirme aracı, geliştiriciler arasında tutarlı bir kodlama stili sağlar.
- Rust Language Server (Rust Dil Sunucusu), kod tamamlama ve satır içi hata mesajları için tümleşik geliştirme ortamı (IDE) desteğine güç verir.

Geliştiriciler, Rust ekosistemindeki bu ve benzeri araçları kullanarak sistem düzeyinde kod yazarken yüksek üretkenliklerini koruyabilirler.

### Öğrenciler

Rust, sistem kavramlarını öğrenmek isteyen öğrenciler ve meraklılar içindir. Pek çok kişi işletim sistemi geliştirme gibi konuları Rust kullanarak öğrenmiştir. Topluluk son derece sıcakkanlıdır ve öğrencilerin sorularını yanıtlamaktan mutluluk duyar. Rust ekipleri, bu kitap gibi çalışmalar aracılığıyla sistem kavramlarını —özellikle programlamaya yeni başlayanlar başta olmak üzere— daha fazla insan için erişilebilir kılmayı amaçlar.

### Şirketler

İrili ufaklı yüzlerce şirket; komut satırı araçları, web servisleri, DevOps araçları, gömülü cihazlar, ses ve video analizi ile format dönüştürme, kripto paralar, biyoinformatik, arama motorları, Nesnelerin İnterneti uygulamaları, makine öğrenimi ve hatta Firefox web tarayıcısının önemli bölümleri dahil olmak üzere çok çeşitli görevler için üretim ortamında Rust kullanır.

### Açık Kaynak Geliştiricileri

Rust; Rust programlama dilini, topluluğunu, geliştirici araçlarını ve kütüphanelerini inşa etmek isteyenler içindir. Rust diline katkıda bulunmanızdan büyük mutluluk duyarız.

### Hıza ve Kararlılığa Önem Verenler

Rust, bir dilde hız ve kararlılık arayanlar içindir. Hız derken hem Rust kodunun çalışma hızını hem de Rust'ın size program geliştirme sürecinde kazandırdığı hızı kastediyoruz. Rust derleyicisinin yaptığı denetimler, yeni özellik eklemeleri ve kodda yapılan yeniden düzenlemeler boyunca kararlılığı güvence altına alır. Bu durum, bu tür denetimlerden yoksun dillerde geliştiricilerin dokunmaktan çekindiği kırılgan eski kod tabanlarıyla tam bir tezat oluşturur. Rust, *sıfır maliyetli soyutlamalar (zero-cost abstractions)* —elle yazılmış alt düzey kod kadar hızlı çalışan koda derlenen üst düzey özellikler— hedefleyerek güvenli kodun aynı zamanda hızlı kod olmasını sağlamaya çalışır.

Rust dili, daha pek çok farklı kullanıcı kitlesini de desteklemeyi hedefler; burada değinilenler yalnızca en büyük paydaşlardan bazılarıdır. Özetle Rust'ın en büyük tutkusu; güvenliği _ve_ üretkenliği, hızı _ve_ ergonomiyi bir arada sunarak programcıların on yıllardır kabullendiği ödünleşimleri ortadan kaldırmaktır. Rust'a bir şans verin ve tasarım tercihlerinin sizin için işe yarayıp yaramadığını kendiniz görün.

## Bu Kitap Kimler İçin?

Bu kitap, daha önce başka bir programlama dilinde kod yazdığınızı varsayar; ancak bunun hangi dil olduğuna dair herhangi bir ön kabulde bulunmaz. İçeriği, çok çeşitli programlama geçmişlerinden gelen okuyucular için geniş ölçüde erişilebilir kılmaya özen gösterdik. Programlamanın _ne olduğu_ veya programlama mantığıyla nasıl düşünüleceği üzerine uzun uzadıya vakit harcamayacağız. Programlamaya tamamen yeniyseniz, öncelikle doğrudan programlamaya giriş niteliği taşıyan bir kitap okumanız sizin için çok daha faydalı olacaktır.

## Bu Kitap Nasıl Kullanılır?

Genel olarak bu kitap, bölümleri baştan sona sırasıyla okuduğunuzu varsayar. İlerleyen bölümler önceki bölümlerde tanıtılan kavramların üzerine inşa edilir; önceki bölümler ise belirli bir konunun tüm ayrıntılarına hemen inmeyip konuyu daha sonraki bir bölümde yeniden ele alabilir.

Bu kitapta iki tür bölüm bulacaksınız: kavram bölümleri ve proje bölümleri. Kavram bölümlerinde Rust'ın belirli bir yönünü öğreneceksiniz. Proje bölümlerinde ise o ana dek öğrendiklerinizi uygulayarak birlikte küçük programlar geliştireceğiz. Bölüm 2, Bölüm 12 ve Bölüm 21 proje bölümleridir; geri kalanlar ise kavram bölümleridir.

**Bölüm 1**, Rust'ın nasıl kurulacağını, “Merhaba, dünya!” programının nasıl yazılacağını ve Rust'ın paket yöneticisi ve derleme aracı olan Cargo'nun nasıl kullanılacağını açıklar. **Bölüm 2**, bir sayı tahmin oyunu geliştirmenizi sağlayarak Rust'ta program yazmaya uygulamalı bir giriş sunar. Burada kavramları genel hatlarıyla ele alırız; ek ayrıntıları ise ilerleyen bölümler sağlar. Hemen kolları sıvayıp pratik yapmak istiyorsanız, Bölüm 2 tam size göredir. Bir sonraki konuya geçmeden önce her ayrıntıyı öğrenmeyi tercih eden titiz bir okuyucuysanız, Bölüm 2'yi atlayıp doğrudan diğer programlama dillerindekilere benzer Rust özelliklerini ele alan **Bölüm 3**'e geçmek isteyebilirsiniz; daha sonra öğrendiğiniz ayrıntıları uygulayacağınız bir proje üzerinde çalışmak istediğinizde Bölüm 2'ye geri dönebilirsiniz.

**Bölüm 4**'te Rust'ın *sahiplik (ownership)* sistemini öğreneceksiniz. **Bölüm 5**, struct'ları (yapıları) ve metotları ele alır. **Bölüm 6** ise enum'ları, `match` ifadelerini ve `if let` ile `let...else` kontrol akışı yapılarını kapsar. Kendi özel türlerinizi oluşturmak için struct'ları ve enum'ları kullanacaksınız.

**Bölüm 7**'de Rust'ın modül sistemini ve kodunuz ile herkese açık uygulama programlama arayüzünüzü (API) düzenlemeye yönelik gizlilik kurallarını öğreneceksiniz. **Bölüm 8**, standart kütüphanenin sunduğu yaygın koleksiyon veri yapılarını ele alır: vektörler, string'ler ve hash map'ler. **Bölüm 9**, Rust'ın hata yönetimi felsefesini ve tekniklerini inceler.

**Bölüm 10**, birden fazla türe uygulanabilen kodlar tanımlama gücü veren *jenerikleri*, *trait'leri* ve *ömürleri (lifetimes)* derinlemesine inceler. **Bölüm 11** tamamen test yazmaya ayrılmıştır; Rust'ın sunduğu güvenlik garantilerine rağmen programınızın mantığının doğru çalıştığından emin olmak için testler gereklidir. **Bölüm 12**'de, dosyalar içinde metin arayan `grep` komut satırı aracının işlevlerinin bir alt kümesini barındıran kendi uygulamamızı geliştireceğiz. Bunun için önceki bölümlerde ele aldığımız pek çok kavramı kullanacağız.

**Bölüm 13**, Rust'ın fonksiyonel programlama dillerinden gelen özellikleri olan *kapanışları (closures)* ve *iteratörleri* inceler. **Bölüm 14**'te Cargo'yu daha derinlemesine inceleyecek ve kütüphanelerinizi başkalarıyla paylaşmaya yönelik en iyi uygulamaları konuşacağız. **Bölüm 15**, standart kütüphanenin sunduğu *akıllı işaretçileri (smart pointers)* ve bunların işlevselliğini mümkün kılan trait'leri ele alır.

**Bölüm 16**'da farklı eşzamanlı programlama modellerini adım adım inceleyecek ve Rust'ın birden fazla *thread (iş parçacığı)* ile korkusuzca programlama yapmanıza nasıl yardımcı olduğunu konuşacağız. **Bölüm 17**'de ise Rust'ın `async` ve `await` sözdizimini; *görevler (tasks)*, *future'lar* ve *akışlar (streams)* ile bunların sağladığı hafif eşzamanlılık modeliyle birlikte keşfederek bu temelin üzerine yenilerini inşa edeceğiz.

**Bölüm 18**, Rust'a özgü kullanım kalıplarının aşina olabileceğiniz nesne yönelimli programlama ilkeleriyle nasıl karşılaştırıldığını inceler. **Bölüm 19**, Rust programlarının genelinde fikirleri ifade etmenin güçlü yolları olan *desenler (patterns)* ve *desen eşleştirme (pattern matching)* üzerine bir başvuru kaynağıdır. **Bölüm 20** ise *güvenli olmayan (`unsafe`) Rust*, makrolar ve ömürler, trait'ler, türler, fonksiyonlar ile kapanışlar hakkında daha fazla ayrıntının yer aldığı, ilgi çekici gelişmiş konulardan oluşan zengin bir seçki sunar.

**Bölüm 21**'de alt düzey, *çok kanallı (multithreaded)* bir web sunucusu geliştireceğimiz bir projeyi tamamlayacağız!

Son olarak ekler bölümü, dil hakkında başvuru kaynağı niteliğinde faydalı bilgiler barındırır. **Ek A** Rust'ın anahtar kelimelerini, **Ek B** Rust'ın operatörlerini ve sembollerini, **Ek C** standart kütüphane tarafından sağlanan türetilebilir trait'leri, **Ek D** faydalı geliştirme araçlarını ve **Ek E** *Rust sürümlerini (editions)* kapsar. **Ek F**'de kitabın çevirilerini bulabilir, **Ek G**'de ise Rust'ın nasıl geliştirildiğini ve *"Nightly Rust"* (gecelik derleme) sürümünün ne olduğunu öğrenebilirsiniz.

Bu kitabı okumanın yanlış bir yolu yoktur: İlerideki bölümlere atlamak isterseniz hiç çekinmeyin! Herhangi bir kafa karışıklığı yaşarsanız önceki bölümlere geri dönmeniz gerekebilir; ancak sizin için hangi yöntem en verimliyse onu izleyin.

<span id="ferris"></span>

Rust öğrenme sürecinin önemli bir parçası, derleyicinin gösterdiği hata mesajlarını nasıl okuyacağınızı öğrenmektir: Bu mesajlar sizi çalışan koda doğru yönlendirecektir. Bu doğrultuda kitap boyunca derlenmeyen pek çok örneği, derleyicinin her durumda size göstereceği hata mesajıyla birlikte paylaşacağız. Rastgele bir kod örneğini yazıp çalıştırmayı denediğinizde derlenmeyebileceğini unutmayın! Çalıştırmak istediğiniz örneğin bilerek hata üretmek üzere yazılıp yazılmadığını görmek için çevresindeki metni mutlaka okuyun. Çoğu durumda, derlenmeyen her kodun doğru sürümüne sizi adım adım ulaştıracağız. Ferris de çalışması amaçlanmayan kodları ayırt etmenize yardımcı olacaktır:

| Ferris                                                                                                                | Anlamı                             |
| --------------------------------------------------------------------------------------------------------------------- | ---------------------------------- |
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Soru işaretli Ferris"/>                        | Bu kod derlenmez!                  |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ellerini havaya kaldırmış Ferris"/>                      | Bu kod panik oluşturur!            |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Tek pençesi havada, omuz silken Ferris"/> | Bu kod istenen davranışı üretmez.  |

Çoğu durumda, derlenmeyen her kodun doğru sürümüne sizi adım adım ulaştıracağız.

> Not: Rust derleyicisi (`rustc`), hata ve uyarı mesajlarını İngilizce olarak üretir; ancak hata kodları (örneğin `E0382`), satır işaretçileri ve `help:` önerileri son derece açık ve yol göstericidir. Kendi terminalinizde göreceğiniz çıktılarla birebir eşleşmesi için kitap boyunca derleyici mesajlarını orijinal haliyle koruyup Türkçe açıklamalarla ayrıntılı biçimde ele alacağız. Ayrıca karşılaştığınız herhangi bir hata kodunun kapsamlı açıklamasına terminalde `rustc --explain E0382` gibi bir komutla dilediğiniz an ulaşabilirsiniz.

## Kaynak Kod

Bu kitabın oluşturulduğu orijinal İngilizce kaynak dosyalarına [GitHub üzerindeki resmi `rust-lang/book` deposundan][book], elinizdeki Türkçe çevirinin kaynak dosyalarına ise [`hakantr/kitap-rs` deposundan][kitap-rs] ulaşabilirsiniz.

[book]: https://github.com/rust-lang/book/tree/main/src
[kitap-rs]: https://github.com/hakantr/kitap-rs
