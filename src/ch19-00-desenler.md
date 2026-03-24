# Desenler ve Eşleştirme

Desenler, Rust'ta türlerin yapısına karşı eşleştirme yapmayı sağlayan özel bir
sözdizimidir. Basit ya da karmaşık veri yapıları üzerinde aynı fikirle
çalışırlar. Desenleri `match` ifadeleri ve başka yapılarla birlikte kullanmak,
programın kontrol akışı üzerinde daha ince denetim sağlar. Bir desen şu
parçaların bir birleşiminden oluşabilir:

- Sabit değerler
- Ayrıştırılmış dizi, enum, struct veya demetler
- Değişkenler
- Joker desenler
- Yer tutucular

Örneğin `x`, `(a, 3)` ve `Some(Renk::Kirmizi)` birer desendir. Desenlerin
geçerli olduğu bağlamlarda bu bileşenler, verinin şeklini tarif eder. Program
da bir değeri desenle karşılaştırıp, ilgili kod parçasının çalışmaya devam
edebilmesi için verinin doğru biçimde olup olmadığını belirler.

Bir deseni kullanmak için onu bir değerle karşılaştırırız. Eğer desen değerle
eşleşirse, değerin parçalarını kod içinde kullanabiliriz. 6. bölümde gördüğünüz
`match` ifadeleri buna örnekti. Değer desenin şekline uyuyorsa, o desende ad
verilmiş parçaları kullanabiliriz. Uymuyorsa o desenle ilişkili kod
çalıştırılmaz.

Bu bölüm, desenlerle ilgili her şey için bir başvuru bölümü niteliğinde.
Desenlerin kullanılabildiği yerleri, çürütülebilir ve çürütülemez desenler
arasındaki farkı ve görebileceğiniz temel desen sözdizimlerini ele alacağız.
Bölümün sonunda, birçok kavramı açık ve güçlü biçimde ifade etmek için
desenleri nasıl kullanacağınızı biliyor olacaksınız.
