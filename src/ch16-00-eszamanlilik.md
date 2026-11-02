<!-- Old headings. Do not remove or links may break. -->
<a id="fearless-concurrency"></a>
<a id="korkusuz-eşzamanlılık"></a>
<a id="korkusuz-eszamanlilik"></a>

# Korkusuz Eşzamanlılık (Fearless Concurrency)

Eşzamanlı programlamayı güvenli ve verimli bir şekilde ele almak, Rust'ın temel hedeflerinden bir diğeridir. Bir programın farklı bölümlerinin birbirinden bağımsız olarak yürütüldüğü _eşzamanlı programlama (concurrent programming)_ ve bir programın farklı bölümlerinin aynı anda yürütüldüğü _paralel programlama (parallel programming)_, giderek daha fazla bilgisayarın birden çok işlemcisinden yararlanmasıyla birlikte her geçen gün daha büyük önem kazanmaktadır. Geçmişten bugüne bu bağlamlarda program yazmak hep zor ve hataya açık olmuştur. Rust bu durumu değiştirmeyi hedefliyor.

Başlangıçta Rust ekibi, bellek güvenliğini sağlamanın ve eşzamanlılık sorunlarını önlemenin farklı yöntemlerle çözülmesi gereken iki ayrı zorluk olduğunu düşünüyordu. Zamanla ekip, sahiplik ve tür sistemlerinin hem bellek güvenliğini _hem de_ eşzamanlılık sorunlarını yönetmeye yardımcı olan çok güçlü bir araç seti olduğunu keşfetti! Sahiplik ve tür denetimi sayesinde Rust'ta pek çok eşzamanlılık hatası, çalışma zamanı hatası yerine derleme zamanı hatası hâline gelir. Böylece çalışma zamanında ortaya çıkan bir eşzamanlılık hatasının oluştuğu koşulları birebir yeniden üretmeye çalışarak uzun zaman harcamak zorunda kalmazsınız; hatalı kod derlenmez ve derleyici sorunu açıklayan bir hata mesajı gösterir. Sonuç olarak kodunuzu canlı ortama (üretime) aldıktan sonra değil, henüz üzerinde çalışırken düzeltebilirsiniz. Rust'ın bu yönüne _korkusuz eşzamanlılık_ adını verdik. Korkusuz eşzamanlılık, fark edilmesi güç sinsi hatalardan arınmış ve yeni hatalara yol açmadan kolayca yeniden düzenlenebilen kodlar yazmanıza olanak tanır.

> Not: Anlatımı sade tutmak adına, pek çok sorundan söz ederken daha kesin bir
> ifadeyle _eşzamanlı ve/veya paralel_ demek yerine kısaca _eşzamanlı_
> diyeceğiz. Bu bölüm boyunca _eşzamanlı_ ifadesini gördüğünüz her yerde
> zihninizde bunu _eşzamanlı ve/veya paralel_ olarak düşünebilirsiniz. Bu
> ayrımın daha büyük önem taşıdığı bir sonraki bölümde (Bölüm 17'de `async` ve
> `await` yapısını incelerken) daha kesin bir dil kullanacağız.

Pek çok dil, eşzamanlılık sorunlarını ele almak için sunduğu çözümler konusunda katı (dogmatik) bir yaklaşım benimser. Örneğin Erlang, _mesaj iletimine (message passing)_ dayalı eşzamanlılık için zarif olanaklar sunarken _thread'ler (iş parçacıkları)_ arasında durum paylaşmak için yalnızca dolaylı ve anlaşılması güç yollar barındırır. Olası çözümlerin yalnızca bir alt kümesini desteklemek üst düzey diller için makul bir stratejidir; çünkü üst düzey bir dil, soyutlama kazanmak uğruna denetimin bir kısmından vazgeçmenin getireceği yararları vaat eder. Buna karşılık alt düzey dillerden, her durumda en yüksek performansı sağlayan çözümü sunmaları ve donanım üzerinde daha az soyutlama katmanı bulundurmaları beklenir. Bu nedenle Rust, problemleri kendi durumunuza ve gereksinimlerinize en uygun biçimde modelleyebilmeniz için çeşitli araçlar sunar.

Bu bölümde ele alacağımız konular şunlardır:

- Aynı anda birden fazla kod parçasını çalıştırmak için thread'lerin nasıl
  oluşturulacağı
- Kanalların thread'ler arasında mesaj gönderdiği _mesaj iletimli_ eşzamanlılık
- Birden fazla thread'in aynı veriye erişebildiği _paylaşımlı durum
  (shared-state)_ eşzamanlılığı
- Rust'ın eşzamanlılık güvencelerini, standart kütüphanenin sunduğu türlerin
  yanı sıra kullanıcı tanımlı türlere de genişleten `Sync` ve `Send` trait'leri

> Not: Bilgisayar biliminde sıkça birbiriyle karıştırılan **eşzamanlılık
> (*concurrency*)** ve **paralellik (*parallelism*)** kavramları arasındaki
> farkı ve Rust'ın yaklaşımını şu iki temel noktayla özetleyebiliriz:
>
> - **Eşzamanlılık yapı ile, paralellik yürütme ile ilgilidir:** *Eşzamanlılık*,
>   birden fazla bağımsız işi birinin tamamlanmasını beklemeden diğerinde de
>   ilerleme kaydedebilecek şekilde **yapısal olarak yönetmektir**; tek
>   çekirdekli bir işlemci bile görevler arasında hızla geçiş yaparak eşzamanlı
>   ilerleme sağlayabilir (Bölüm 17'de göreceğimiz `async`/`await` modelinin tek
>   bir thread üzerinde çok sayıda G/Ç görevini nöbetleşe yürütmesi gibi).
>   *Paralellik* ise birden fazla işin çok çekirdekli donanım üzerinde fiziksel
>   olarak **kelimenin tam anlamıyla aynı anda çalışmasıdır** (birden fazla
>   işletim sistemi thread'inin farklı CPU çekirdeklerinde eş anlı hesaplama
>   yapması gibi).
> - **Veri yarışlarının (*data races*) derleme zamanında önlenmesi:** İki veya
>   daha fazla thread'in aynı bellek alanına senkronizasyon olmaksızın erişip en
>   az birinin yazma işlemi yapması *veri yarışı* doğurur. Rust'ın Bölüm 4'te
>   gördüğümüz temel ödünç alma kuralı —aynı anda ya birden fazla değiştirilemez
>   referans (`&T`) ya da yalnızca tek bir değiştirilebilir referans (`&mut T`)
>   bulunabilmesi— `Send` ve `Sync` trait'leri aracılığıyla thread sınırları
>   boyunca da zorunlu tutulur. Böylece başka dillerde yalnızca çalışma
>   zamanında ve belirli yük koşullarında tesadüfen patlak veren veri yarışları,
>   Rust'ta henüz kod derlenirken yakalanıp engellenir.
