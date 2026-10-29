<!-- Old headings. Do not remove or links may break. -->
<a id="functional-language-features-iterators-and-closures"></a>
<a id="fonksiyonel-dil-özellikleri-yineleyiciler-iterators-ve-kapanışlar-closures"></a>
<a id="fonksiyonel-dil-ozellikleri-yineleyiciler-iterators-ve-kapanislar-closures"></a>
<a id="fonksiyonel-dil-ozellikleri-iteratorler-ve-kapanislar"></a>

# Fonksiyonel Dil Özellikleri: İteratörler ve Kapanışlar

Rust'ın tasarımı mevcut pek çok dil ve teknikten ilham almıştır; bu esin kaynaklarının en önemlilerinden biri de *fonksiyonel programlamadır (functional programming)*. Fonksiyonel tarzda programlama; çoğu zaman fonksiyonları başka fonksiyonlara argüman olarak geçirmek, diğer fonksiyonlardan döndürmek veya daha sonra çalıştırılmak üzere değişkenlere atamak gibi yollarla birer değer olarak kullanmayı içerir.

Bu bölümde fonksiyonel programlamanın ne olup olmadığı tartışmasına girmeyeceğiz; bunun yerine Rust'ın, genellikle "fonksiyonel" olarak nitelendirilen pek çok dildeki özelliklere benzeyen bazı yeteneklerini inceleyeceğiz.

Özellikle şu konuları ele alacağız:

- Bir değişkende saklayabileceğiniz, fonksiyon benzeri bir yapı olan *kapanışlar (closures)*
- Bir dizi öğeyi sırayla işlemenin bir yolu olan *iteratörler (yineleyiciler / iterators)*
- Bölüm 12'deki G/Ç projesini iyileştirmek için kapanışların ve iteratörlerin nasıl kullanılacağı
- Kapanışların ve iteratörlerin performansı (sürprizi şimdiden bozalım: Düşündüğünüzden daha hızlıdırlar!)

Desen eşleştirme ve enum'lar gibi fonksiyonel yaklaşımdan etkilenen diğer bazı Rust özelliklerini daha önce ele almıştık. Kapanışlara ve iteratörlere hâkim olmak, hızlı ve deyimsel Rust kodu yazmanın önemli bir parçası olduğundan bu bölümün tamamını bu iki konuya ayıracağız.

> Not: Pek çok yüksek seviyeli dilde fonksiyonel programlama desenleri — kapanışlar, `map` ve `filter` gibi zincirleme *iteratör adaptörleri* ya da `Option` ve `Result` üzerindeki `unwrap_or_else`, `and_then` gibi *kombinatörler* — öbek tahsisleri, dinamik çağrılar veya ara koleksiyon kopyaları nedeniyle çalışma zamanında ek maliyet doğurabilir. Rust'ta ise bu yapıların tamamı **sıfır maliyetli soyutlama (*zero-cost abstraction*)** ilkesiyle tasarlanmıştır: Derleyici her kapanış için benzersiz bir somut tür üretir, tembel (*lazy*) iteratör zincirlerini ara bellek ayırmadan tek bir geçişte birleştirir ve çağrıları satır içine (*inlining*) açar. Sonuç olarak, okunabilirliği yüksek ve bildirimsel fonksiyonel Rust kodu, elle yazılmış alt seviye bir döngüyle aynı verimlilikte makine koduna derlenir.
