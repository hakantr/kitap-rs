# Korkusuz Eşzamanlılık

Rust'ın büyük hedeflerinden biri de eşzamanlı programlamayı güvenli ve verimli
bir şekilde ele almaktır. _Eşzamanlı programlama (concurrent programming)_,
programın farklı bölümlerinin birbirinden bağımsız ilerlemesidir. _Paralel
programlama (parallel programming)_ ise farklı bölümlerin gerçekten aynı anda
çalışmasıdır. Bilgisayarlar çok çekirdekli işlemcilerden daha fazla
yararlandıkça bu iki yaklaşım da giderek daha önemli hale geliyor. Tarihsel
olarak bu tür programlar yazmak zordu ve hataya açıktı. Rust bunu değiştirmeyi
amaçlıyor.

İlk başta Rust ekibi, bellek güvenliğini sağlamakla eşzamanlılık sorunlarını
önlemenin farklı yöntemlerle çözülecek iki ayrı problem olduğunu düşünüyordu.
Zamanla sahiplik ve tür sisteminin, hem bellek güvenliğini hem de
eşzamanlılıktaki sorunları yönetmek için çok güçlü araçlar sunduğunu fark
ettiler. Sahiplik ve tür denetiminden yararlanıldığı için, eşzamanlılığa dair
pek çok hata Rust'ta çalışma zamanı hatası değil derleme zamanı hatası olur.
Böylece çalışma zamanında ortaya çıkan bir hatayı tekrar üretmeye uzun zaman
harcamak yerine, yanlış kod doğrudan derlenmez ve size sorunu açıklayan bir
hata verir. Sonuç olarak kodunuzu üretime çıktıktan sonra değil, daha
yazarken düzeltebilirsiniz. Rust'ın bu yönüne _korkusuz eşzamanlılık_ adı
veriliyor. Korkusuz eşzamanlılık, ince ve sinsi hatalardan uzak; aynı zamanda
yeniden düzenlemesi kolay kod yazmanızı sağlar.

> Not: Sade tutmak için bu bölümde birçok sorundan söz ederken daha kesin bir
> ayrım yapıp "_eşzamanlı ve/veya paralel_" demek yerine yalnızca
> "_eşzamanlı_" diyeceğiz. Bu bölüm boyunca "_eşzamanlı_" gördüğünüz yerlerde
> zihninizde "_eşzamanlı ve/veya paralel_" diye okuyun. Bir sonraki bölümde
> bu ayrım daha önemli hale geldiğinde daha net konuşacağız.

Birçok dil, eşzamanlılık sorunlarına sunduğu çözümler konusunda oldukça katı
davranır. Örneğin Erlang, mesaj iletimine dayalı eşzamanlılık için zarif
olanaklar sunar; ama iş parçacıkları arasında durum paylaşmak için pek açık
yollar vermez. Olası çözümlerin yalnızca bir kısmını desteklemek, üst seviye
diller için makul bir yaklaşımdır; çünkü bu diller belli soyutlamaları
kazanmak için biraz denetimden vazgeçmeyi zaten vaat eder. Buna karşılık daha
alt seviyeli dillerden, eldeki duruma göre en iyi performansı verecek çözümü
sunması ve donanım üstünde daha az soyutlama kurması beklenir. Bu yüzden Rust,
sorununuzu ve gereksinimlerinizi hangi model daha iyi karşılıyorsa onu
kullanabilmeniz için çeşitli araçlar sunar.

Bu bölümde şu konuları işleyeceğiz:

- Aynı anda birden fazla kod parçası çalıştırmak için iş parçacıkları nasıl
  oluşturulur?
- Kanalların iş parçacıkları arasında mesaj taşıdığı _mesaj iletimli
  eşzamanlılık_
- Birden çok iş parçacığının aynı veriye eriştiği _paylaşımlı durum
  eşzamanlılığı_
- Rust'ın eşzamanlılık güvencelerini, standart kütüphanedeki türlerin yanında
  kullanıcı tanımlı türlere de taşıyan `Sync` ve `Send` trait'leri
