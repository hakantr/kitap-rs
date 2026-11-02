<!-- Old headings. Do not remove or links may break. -->
<a id="common-collections"></a>
<a id="yaygin-koleksiyonlar"></a>

# Yaygın Koleksiyonlar

Rust'ın standart kütüphanesi, _koleksiyonlar (collections)_ adı verilen oldukça kullanışlı pek çok veri yapısı içerir. Diğer veri türlerinin çoğu belirli tek bir değeri temsil ederken, koleksiyonlar birden fazla değeri barındırabilir. Yerleşik dizi ve demet türlerinin aksine, bu koleksiyonların işaret ettiği veriler *öbekte (heap)* saklanır; bu da veri miktarının derleme zamanında bilinmesi gerekmediği ve program çalışırken büyüyüp küçülebileceği anlamına gelir. Her koleksiyon türünün farklı yetenekleri ve maliyetleri vardır; mevcut durumunuza en uygun olanı seçmek zamanla geliştireceğiniz bir beceridir. Bu bölümde, Rust programlarında çok sık kullanılan üç koleksiyonu ele alacağız:

- Bir *vektör (`Vec<T>`)*, değişken sayıda değeri bellekte yan yana saklamanızı sağlar.
- Bir *string (`String`)*, karakterlerden oluşan bir koleksiyondur. Daha önce `String` türünden bahsetmiştik; ancak bu bölümde onu derinlemesine inceleyeceğiz.
- Bir *hash map (`HashMap<K, V>`)*, bir değeri belirli bir anahtarla ilişkilendirmenizi sağlar. Bu tür, *eşlem (map)* adı verilen daha genel bir veri yapısının özel bir uygulamasıdır.

Standart kütüphane tarafından sağlanan diğer koleksiyon türleri hakkında bilgi edinmek için [standart kütüphane belgelendirmesine][collections] göz atabilirsiniz.

> Not: Koleksiyonların asıl elemanları dinamik olarak büyüyüp küçülebilmek için **öbekte** tutulurken, koleksiyon değişkeninin kendisi (öbekteki verinin işaretçisi, mevcut uzunluk ve ayrılan kapasite bilgisi) **yığında (stack)** sabit boyutlu küçük bir yapı olarak yer alır. Değişken kapsam dışına çıktığında ise öbekteki bellek, sahiplik kuralları uyarınca otomatik olarak temizlenir. Ayrıca çok sık kullanıldıkları için `Vec<T>` ve `String` ön tanımlı kapsamda (*prelude*) doğrudan hazır gelirken, `HashMap` ve diğer özel amaçlı koleksiyonlar `std::collections` modülünden `use` ile kapsama dahil edilir. İhtiyacınıza göre başvurabileceğiniz başlıca diğer standart koleksiyonlar şunlardır:
>
> - **`VecDeque<T>`**: Yalnızca sondan değil, hem baştan hem sondan hızlı ekleme ve çıkarma gerektiren çift uçlu kuyruk (*double-ended queue*) senaryoları için,
> - **`BTreeMap<K, V>`**: Anahtarların her zaman sıralı tutulması ve belirli bir aralıktaki anahtarların verimli biçimde sorgulanması gerektiğinde `HashMap<K, V>` yerine,
> - **`HashSet<T>` ve `BTreeSet<T>`**: Yinelenen elemanlara izin vermeyen benzersiz değer kümelerini sırasız (`HashSet`) veya sıralı (`BTreeSet`) olarak tutmak ve hızlı üyelik kontrolü yapmak için,
> - **`BinaryHeap<T>`**: Elemanlar arasından her zaman en büyük (en öncelikli) değere hızlıca ulaşmak istediğiniz öncelik kuyruğu (*priority queue*) yapıları için tercih edilir.

Şimdi vektörlerin, string'lerin ve hash map'lerin nasıl oluşturulup güncelleneceğini ve her birini neyin özel kıldığını inceleyeceğiz.

[collections]: https://doc.rust-lang.org/std/collections/index.html
