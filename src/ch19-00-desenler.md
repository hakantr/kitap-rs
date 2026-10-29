# Desenler ve Eşleştirme

<a id="patterns-and-matching"></a>

_Desenler (patterns)_, Rust'ta hem karmaşık hem de basit türlerin yapısına karşı
eşleştirme yapmayı sağlayan özel bir sözdizimidir. Desenleri `match` ifadeleri
ve diğer yapılarla birlikte kullanmak, programın kontrol akışı üzerinde size
daha fazla denetim sunar. Bir desen aşağıdaki bileşenlerin bir birleşiminden
oluşabilir:

- Hazır değerler (*literals*)
- Yapısı bozulmuş (*destructured*) diziler, enum'lar, yapılar veya demetler
- Değişkenler
- Joker desenler (*wildcards*)
- Yer tutucular (*placeholders*)

Örnek bazı desenler arasında `x`, `(a, 3)` ve `Some(Renk::Kirmizi)` sayılabilir.
Desenlerin geçerli olduğu bağlamlarda bu bileşenler verinin şeklini tarif eder.
Ardından programımız değerleri bu desenlere karşı eşleştirerek belirli bir kod
parçasını çalıştırmaya devam etmek için doğru veri şekline sahip olup olmadığını
belirler.

Bir deseni kullanmak için onu bir değerle karşılaştırırız. Desen değerle
eşleşirse kodumuzda değerin parçalarını kullanırız. Bölüm 6'da madeni para
ayırma makinesi örneği gibi desen kullanan `match` ifadelerini hatırlayın. Değer
desenin şekline uyuyorsa adlandırılmış parçaları kullanabiliriz. Uymuyorsa
desenle ilişkili kod çalışmaz.

Bu bölüm, desenlerle ilgili her şey için bir başvuru kılavuzu niteliğindedir.
Desenleri kullanmanın geçerli olduğu yerleri, çürütülebilir (*refutable*) ve
çürütülemez (*irrefutable*) desenler arasındaki farkı ve karşılaşabileceğiniz
farklı desen sözdizimi türlerini ele alacağız. Bölümün sonunda pek çok kavramı
açık ve anlaşılır bir biçimde ifade etmek için desenleri nasıl kullanacağınızı
biliyor olacaksınız.

> Not: Rust'ta desen eşleştirme aynı anda iki temel görevi yerine getirir:
> verinin belirli bir şekle uyup uymadığını **sınamak** ve uyuyorsa içindeki
> alanları **değişkenlere bağlayarak yapısını bozmak (*destructuring*)**. Üstelik
> desenler Rust'ın sahiplik kurallarıyla doğrudan bütünleşiktir: `Copy` olmayan
> bir değeri desende bir değişkene bağladığınızda sahiplik taşınır; referans
> üzerinden eşleştirme yaptığınızda ise derleyici bağlanan değişkenleri otomatik
> olarak referans (`&T` veya `&mut T`) haline getirir.
