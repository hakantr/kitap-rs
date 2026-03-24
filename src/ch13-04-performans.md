<!-- Old headings. Do not remove or links may break. -->

<a id="comparing-performance-loops-vs-iterators"></a>

## Döngülerde (Loops) ve Yineleyicilerde (Iterators) Performans

Döngü mü yoksa yineleyici mi kullanacağınızı belirlemek için hangi uygulamanın daha hızlı olduğunu bilmeniz gerekir: `ara` fonksiyonunun açık bir `for` döngüsüne sahip olan versiyonu mu yoksa yineleyicilerle olan versiyonu mu.

Sör Arthur Conan Doyle'un _The Adventures of Sherlock Holmes_ (Sherlock Holmes'un Maceraları) kitabının tüm içeriğini bir `String`'e yükleyerek ve içeriğin içinde _the_ kelimesini arayarak bir kıyaslama çalıştırdık. İşte `ara` fonksiyonunun `for` döngüsünü kullanan sürümü ile yineleyicileri kullanan sürümündeki kıyaslama sonuçları:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

İki uygulamanın da benzer performansları var! Buradaki amacımız iki versiyonun eşdeğer olduğunu kanıtlamak değil; bu iki uygulamanın performans açısından nasıl karşılaştırılacağına dair genel bir fikir edinmek olduğu için kıyaslama kodunu burada açıklamayacağız.

Daha kapsamlı bir kıyaslama için, `icerik` olarak çeşitli boyutlardaki çeşitli metinleri, `sorgu` olarak farklı uzunluklardaki farklı sözcükleri ve diğer her tür varyasyonu kullanarak kontrol etmelisiniz. Mesele şu: Yineleyiciler üst düzey bir soyutlama olmalarına rağmen, alt düzey kodu kendiniz yazsaydınız olacak kodla yaklaşık olarak aynı şekilde derlenir. Yineleyiciler Rust'ın _sıfır maliyetli soyutlamalarından_ biridir; bununla, soyutlamayı kullanmanın (using the abstraction) hiçbir ek çalışma zamanı yükü getirmediğini kastediyoruz. Bu, C++'ın orijinal tasarımcısı ve uygulayıcısı Bjarne Stroustrup'un 2012 ETAPS "Foundations of C++" (C++'ın Temelleri) açılış sunumunda sıfır yükü tanımlamasına benzerdir:

> Genel olarak, C++ uygulamaları sıfır yük ilkesine uyar: Kullanmadığınız şeylerin parasını ödemezsiniz. Ve dahası: Kullandığınız şeyi elle kodlayarak daha iyisini yapamazsınız.

Pek çok durumda, yineleyicileri kullanan Rust kodu, elle yazacağınız aynı assembly (çevirici) koduna derlenir. Döngü açma ve dizi erişimindeki sınır denetimini ortadan kaldırma gibi optimizasyonlar uygulanır ve ortaya çıkan kodu son derece verimli hale getirir. Artık bunu bildiğinize göre, yineleyicileri ve kapanışları korkusuzca kullanabilirsiniz! Kodu daha üst düzeymiş gibi gösterirler ancak bunu yaptığınız için bir çalışma zamanı performans cezası uygulamazlar.

## Özet

Kapanışlar ve yineleyiciler, fonksiyonel programlama dili fikirlerinden esinlenilen Rust özellikleridir. Üst düzey fikirleri alt düzey performansta (low-level performance) açıkça ifade etme konusundaki Rust'ın kapasitesine katkıda bulunurlar. Kapanışların ve yineleyicilerin uygulamaları, çalışma zamanı performansının etkilenmeyeceği şekildedir. Bu, Rust'ın sıfır maliyetli soyutlamalar sağlamaya çabalama hedefinin bir parçasıdır.

G/Ç projemizin ifade gücünü artırdığımıza göre, şimdi projeyi dünyayla paylaşmamıza yardımcı olacak bazı `cargo` özelliklerine daha bakalım.

[references-and-borrowing]: ch04-02-referanslar-ve-odunc-alma.html#referanslar-ve-ödünç-alma-references-and-borrowing
[string-slices-as-parameters]: ch04-03-dilim-turu.html#parametre-olarak-string-dilimleri
[reference]: ../reference/trait-bounds.html
