<!-- Old headings. Do not remove or links may break. -->

<a id="comparing-performance-loops-vs-iterators"></a>
<a id="performance-in-loops-vs-iterators"></a>
<a id="döngülerde-loops-ve-yineleyicilerde-iterators-performans"></a>
<a id="dongulerde-loops-ve-yineleyicilerde-iterators-performans"></a>
<a id="donguler-ve-iteratorlerde-performans"></a>

## Döngüler ve İteratörlerde Performans

Döngü mü yoksa iteratör mü kullanacağınıza karar vermek için hangi uygulamanın daha hızlı olduğunu bilmeniz gerekir: `ara` fonksiyonunun açık bir `for` döngüsü kullanan sürümü mü, yoksa iteratörlerle yazılan sürümü mü?

Sir Arthur Conan Doyle'un _The Adventures of Sherlock Holmes_ (Sherlock Holmes'un Maceraları) kitabının tüm metnini bir `String` içine yükleyip metinde _the_ kelimesini aradığımız bir performans kıyaslaması (*benchmark*) çalıştırdık. `ara` fonksiyonunun `for` döngüsü kullanan sürümü ile iteratör kullanan sürümünün kıyaslama sonuçları şöyledir:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

Her iki uygulama da birbirine çok yakın bir performansa sahiptir! Buradaki amacımız iki sürümün birebir eşdeğer olduğunu kanıtlamak değil, bu iki uygulamanın performans açısından birbirine kıyasla nasıl sonuç verdiğine dair genel bir fikir edinmek olduğundan kıyaslama kodunu burada açıklamayacağız.

Daha kapsamlı bir kıyaslama için `icerik` olarak farklı boyutlarda çeşitli metinleri, `sorgu` olarak farklı uzunluklarda çeşitli kelimeleri ve diğer pek çok varyasyonu denemeniz gerekir. Buradaki asıl nokta şudur: İteratörler üst düzey bir soyutlama olsa da, alt düzey kodu bizzat kendiniz yazmışsınız gibi neredeyse aynı makine koduna derlenir. İteratörler Rust'ın _sıfır maliyetli soyutlamalarından (zero-cost abstractions)_ biridir; bununla, soyutlamayı kullanmanın çalışma zamanında hiçbir ek yük getirmediğini kastediyoruz. Bu yaklaşım, C++ dilinin ilk tasarımcısı ve uygulayıcısı Bjarne Stroustrup'un 2012 ETAPS konferansındaki “Foundations of C++” (C++'ın Temelleri) başlıklı açılış sunumunda _sıfır ek yük (zero-overhead)_ ilkesini tanımlama biçimiyle örtüşür:

> Genel olarak C++ uygulamaları sıfır ek yük ilkesine uyar: Kullanmadığınız şeyin bedelini ödemezsiniz. Dahası, kullandığınız şeyi elle kodlayarak daha iyisini yazamazsınız.

Pek çok durumda iteratör kullanan Rust kodu, elle yazacağınız assembly koduyla birebir aynı çıktıyı üretecek şekilde derlenir. *Döngü açma (loop unrolling)* ve dizi erişimlerindeki *sınır denetimini (bounds checking)* ortadan kaldırma gibi derleyici optimizasyonları devreye girerek ortaya çıkan kodu son derece verimli hale getirir. Artık bunu bildiğinize göre iteratörleri ve kapanışları hiçbir çekince duymadan kullanabilirsiniz! Kodu üst düzey ve okunaklı tutarken çalışma zamanında herhangi bir performans kaybına yol açmazlar.

> Not: Klasik indeksli `for i in 0..v.len() { ... v[i] ... }` döngülerine kıyasla iteratör zincirlerinin neden çoğu zaman en az onlar kadar hızlı — hatta bazen daha hızlı — çalıştığını ve performansı doğru ölçmenin inceliklerini şu iki noktada özetleyebiliriz:
>
> 1. **Sınır Denetimlerinin Kaldırılması, Yazmaçlar ve Otomatik Vektörleştirme (SIMD):** Rust bellek güvenliğini korumak adına her `v[i]` indeks erişiminde `i < v.len()` sınır denetimi uygular. Karmaşık indeks aritmetiği içeren döngülerde derleyici (LLVM), indeksin asla sınırı aşmayacağını her zaman statik olarak kanıtlayamayabilir; kalan sınır denetimleri ise hem fazladan koşullu dallanma buyruğu üretir hem de **otomatik vektörleştirmeyi (SIMD — tek buyrukla birden fazla veriyi paralel işleme)** engelleyebilir. Buna karşılık `v.iter()`, `zip`, `map` ve `fold` gibi iteratör zincirleri dilimin bellekteki sınırları boyunca yapısal olarak güvenli biçimde ilerler. Derleyici bu sayede sınır denetimlerini güvenle kaldırabilir, ara değerleri bellek yerine doğrudan işlemci **yazmaçlarında (*registers*)** tutabilir ve örneğin ses kod çözücülerdeki doğrusal tahmin hesaplamaları (*linear prediction coding*) gibi yoğun sayısal döngülerde döngü açma ile SIMD buyruklarını çok daha etkin biçimde uygulayabilir.
> 2. **Gerçekçi Performans Ölçümü (`--release` ve Kıyaslama Kütüphaneleri):** Sıfır maliyetli soyutlamalar, derleyicinin fonksiyonları satır içine açma (*inlining*) ve döngü birleştirme optimizasyonlarına dayanır. Varsayılan `cargo run` veya `cargo test` komutlarının kullandığı `dev` profilinde derleme hızını korumak için bu optimizasyonlar kapalıdır; bu modda iteratör zincirleri yanıltıcı biçimde yavaş çalışabilir. Bu nedenle hız ölçümlerini her zaman Bölüm 14'te inceleyeceğimiz `release` profiliyle (`cargo build --release` veya `cargo run --release`) yapmalı; hassas mikro-kıyaslamalarda ise ısınma turları ve istatistiksel sapma analizi sunan `criterion` veya `divan` gibi kıyaslama (*benchmark*) crate'lerini kullanmalısınız.

<!-- Old headings. Do not remove or links may break. -->

<a id="summary"></a>
<a id="ozet"></a>

## Özet

Kapanışlar ve iteratörler, fonksiyonel programlama dillerindeki fikirlerden esinlenen Rust özellikleridir. Bu özellikler, Rust'ın üst düzey fikirleri alt düzey kod performansıyla net bir biçimde ifade edebilme yeteneğine katkıda bulunur. Kapanışların ve iteratörlerin çalışma biçimi, çalışma zamanı performansını olumsuz etkilemeyecek şekilde tasarlanmıştır. Bu yaklaşım, Rust'ın sıfır maliyetli soyutlamalar sunma hedefinin önemli bir parçasıdır.

G/Ç projemizin ifade gücünü artırdığımıza göre, şimdi projeyi dünyayla paylaşmamıza yardımcı olacak bazı `cargo` özelliklerine bakalım.
