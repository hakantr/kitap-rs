# Yaygın Koleksiyonlar

Rust'ın standart kütüphanesi, _koleksiyonlar_ adı verilen çok kullanışlı bir dizi veri yapısı içerir. Diğer birçok veri türü tek bir spesifik değeri temsil ederken, koleksiyonlar birden fazla değeri barındırabilir. Gömülü dizi ve demet türlerinin aksine, bu koleksiyonların işaret ettiği veriler yığında saklanır; bu da veri miktarının derleme zamanında bilinmesine gerek olmadığı ve program çalışırken büyüyüp küçülebileceği anlamına gelir. Her koleksiyon türünün farklı yetenekleri ve maliyetleri vardır, mevcut durumunuz için en uygun olanı seçmek zamanla geliştireceğiniz bir yetenektir. Bu bölümde, Rust programlarında çok sık kullanılan üç koleksiyonu ele alacağız:

- Bir _vektör_, yan yana duran değişken sayıda değeri depolamanızı sağlar.
- Bir _string_ (dizgi), karakterlerden oluşan bir koleksiyondur. Daha önce `String` türünden bahsetmiştik ancak bu bölümde onu derinlemesine inceleyeceğiz.
- Bir _hash map_ (karma harita), bir değeri belirli bir anahtarla ilişkilendirmenizi sağlar. Bu, _harita_ adı verilen daha genel bir veri yapısının özel bir uygulamasıdır.

Standart kütüphane tarafından sağlanan diğer koleksiyon türleri hakkında bilgi edinmek için [belgelere][collections] göz atabilirsiniz.

Vektörlerin, stringlerin ve hash maplerin nasıl oluşturulup güncelleneceğini ve her birini neyin özel kıldığını tartışacağız.

[collections]: ../std/collections/index.html