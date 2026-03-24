## Her Şeyi Birleştirmek: Future, Görev ve İş Parçacığı

16. bölümde gördüğümüz gibi, iş parçacıkları eşzamanlılığa yaklaşmanın bir
yolunu sunar. Bu bölümde ise başka bir yol gördük: `async`, future ve akışlar.
Hangisini ne zaman seçeceğinizi merak ediyorsanız kısa cevap şu: duruma göre.
Üstelik çoğu zaman seçim iş parçacıkları _veya_ `async` değil, iş parçacıkları
_ve_ `async` olur.

Birçok işletim sistemi onlarca yıldır iş parçacığı temelli eşzamanlılık
modelleri sunuyor; bu yüzden birçok programlama dili de onları destekliyor.
Ama bunun bedelleri var. Pek çok işletim sisteminde her iş parçacığı kayda
değer miktarda bellek kullanır. Ayrıca iş parçacıkları yalnızca işletim sistemi
ve donanım destekliyorsa mümkündür. Masaüstü ve mobil sistemlerin aksine bazı
gömülü sistemlerde işletim sistemi bile yoktur; dolayısıyla iş parçacıkları da
yoktur.

`async` modeli farklı ve sonuçta tamamlayıcı bir ödünleşim seti sunar. `async`
yaklaşımında eşzamanlı işlemler için ayrı ayrı iş parçacıkları gerekmez.
Bunun yerine işlemler, akışlar bölümünde `trpl::spawn_task` ile yaptığımız gibi
görevler üstünde çalışabilir. Görev, iş parçacığına benzer; ama onu işletim
sistemi değil, kütüphane düzeyindeki kod, yani çalışma zamanı yönetir.

İş parçacığı başlatma API'leri ile görev başlatma API'lerinin birbirine
benzemesinin iyi bir nedeni var. İş parçacıkları, senkron işlem kümeleri için
bir sınır görevi görür; eşzamanlılık iş parçacıkları _arasında_ mümkündür.
Görevler ise eşzamansız işlem kümeleri için bir sınırdır; eşzamanlılık hem
görevler _arasında_ hem de görevlerin _içinde_ mümkündür, çünkü bir görev
gövdesindeki future'lar arasında geçiş yapabilir. Son olarak future'lar, Rust'ın
en ince taneli eşzamanlılık birimidir ve her future başka future'lardan oluşan
bir ağacı temsil edebilir. Çalışma zamanı, daha doğrusu onun _yürütücüsü
(executor)_, görevleri yönetir; görevler de future'ları yönetir. Bu yönüyle
görevler, işletim sistemi yerine çalışma zamanı tarafından yönetilen hafif iş
parçacıkları gibidir.

Bu, `async` görevlerin her zaman iş parçacıklarından daha iyi olduğu anlamına
gelmez; tersi de doğru değil. İş parçacıklarıyla eşzamanlılık, bazı bakımlardan
`async` ile eşzamanlılıktan daha basit bir programlama modelidir. Bu hem güçlü
yanı hem de zayıf yanı olabilir. İş parçacıkları çoğu zaman "başlat ve bırak"
gibidir; yerleşik bir future karşılıkları yoktur. İşletim sistemi müdahale
etmedikçe başladıkları işi sonuna kadar götürürler.

Öte yandan iş parçacıkları ile görevler çoğu zaman birlikte çok iyi çalışır;
çünkü bazı çalışma zamanlarında görevler iş parçacıkları arasında taşınabilir.
Hatta bu bölümde kullandığımız çalışma zamanı, `spawn_blocking` ve `spawn_task`
işlevleri dahil, varsayılan olarak çok iş parçacıklıdır. Birçok çalışma zamanı,
iş parçacıklarının o anki kullanımına göre görevleri şeffaf biçimde aralarında
taşıyan _iş çalma (work stealing)_ yaklaşımını kullanır. Böylece sistemin genel
performansı iyileşir. Bu yaklaşım, hem iş parçacıklarını hem görevleri hem de
dolayısıyla future'ları birlikte gerektirir.

Hangi yöntemi nerede kullanacağınıza karar verirken şu pratik kuralları akılda
tutabilirsiniz:

- İş çok iyi _paralelleştirilebiliyorsa_ yani CPU-bağımlıysa, örneğin büyük bir
  veri kümesini bağımsız parçalara ayırıp işleyebiliyorsanız, iş parçacıkları
  daha iyi seçimdir.
- İş çok iyi _eşzamanlıysa_ yani G/Ç-bağımlıysa, örneğin farklı kaynaklardan
  farklı hızlarda gelen iletileri ele alıyorsanız, `async` daha iyi seçimdir.

Hem paralellik hem eşzamanlılık gerekiyorsa, iş parçacıkları ile `async`
arasında seçim yapmak zorunda değilsiniz. İkisini rahatça birlikte
kullanabilirsiniz. Böylece her biri en iyi olduğu rolü üstlenir. 17-25 numaralı
liste, gerçek Rust kodunda sık rastlanan böyle bir birleşime örnek gösteriyor.

<Listing number="17-25" caption="Bloklayıcı kodla bir iş parçacığında ileti göndermek ve iletileri bir async blok içinde beklemek" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:all}}
```

</Listing>

Önce bir eşzamansız kanal oluşturuyoruz. Sonra kanalın gönderici tarafının
sahipliğini `move` ile alan bir iş parçacığı başlatıyoruz. İş parçacığının
içinde 1'den 10'a kadar sayıları gönderiyor ve her birinin arasında bir saniye
bekliyoruz. Son olarak, bölüm boyunca yaptığımız gibi `trpl::block_on` içine
verilen bir async bloktan üretilen future'ı çalıştırıyoruz. Bu future içinde de
diğer mesaj iletimi örneklerinde olduğu gibi iletileri bekliyoruz.

Bölümün başındaki video örneğine geri dönersek, video kodlama görevlerini ayrı
bir iş parçacığında çalıştırdığınızı düşünün; çünkü video kodlama CPU-bağımlı
bir iştir. Ama bu işlemler bittiğinde kullanıcı arayüzünü bir eşzamansız kanal
üzerinden bilgilendirebilirsiniz. Gerçek dünyada bu tür birleşimlerin sayısız
örneği vardır.

## Özet

Bu kitapta eşzamanlılık konusunu son kez görmüyorsunuz. 21. bölümdeki proje,
buradaki küçük örneklerden daha gerçekçi bir senaryoda bu kavramları
uygulayacak; ayrıca iş parçacıklarıyla çözüm üretmeyi görevler ve future'larla
çözüm üretmeyle daha doğrudan karşılaştıracak.

Hangi yaklaşımı seçerseniz seçin, Rust size güvenli, hızlı ve eşzamanlı kod
yazmak için gerekli araçları verir; ister yüksek trafiğe sahip bir web sunucusu
yazın, ister gömülü bir işletim sistemi.

Sıradaki bölümde, Rust programları büyüdükçe problemleri modellemenin ve
çözümleri düzenlemenin deyimsel yollarını konuşacağız. Ayrıca Rust'ın
deyişlerinin nesne yönelimli programlamadan aşina olabileceğiniz kalıplarla
nasıl ilişki kurduğunu da ele alacağız.

[ch16]: http://localhost:3000/ch16-00-concurrency.html
[combining-futures]: ch17-03-daha-fazla-future.html#kendi-async-soyutlamalarımızı-kurmak
[streams]: ch17-04-akislar.html#akışlar-sırayla-gelen-futurelar
[ch21]: ch21-00-final-projesi-web-sunucusu.html
