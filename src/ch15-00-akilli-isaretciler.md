# Akıllı İşaretçiler

İşaretçi, bellekteki bir adresi tutan değişkenler için kullanılan genel bir
kavramdır. Bu adres başka bir veriye karşılık gelir; yani o veriyi "işaret
eder". Rust'taki en yaygın işaretçi türü, 4. bölümde öğrendiğiniz
referanslardır. Referanslar `&` simgesiyle gösterilir ve işaret ettikleri
değeri ödünç alırlar. Veri göstermenin dışında özel bir yetenekleri yoktur;
ek maliyet de getirmezler.

_Akıllı işaretçiler (smart pointers)_ ise işaretçi gibi davranan ama buna ek
olarak meta veri ve ek yetenekler de taşıyan veri yapılarıdır. Akıllı
işaretçiler yalnızca Rust'a özgü değildir: Kökenleri C++'a uzanır ve başka
dillerde de bulunurlar. Rust'ın standart kütüphanesinde, referansların sunduğu
olanakların ötesine geçen farklı akıllı işaretçi türleri vardır. Genel fikri
incelemek için birkaç örneğe bakacağız; bunların arasında, _referans sayımı
(reference counting)_ kullanan bir akıllı işaretçi türü de olacak. Bu işaretçi,
sahip sayısını takip ederek ve ortada sahip kalmadığında veriyi temizleyerek,
aynı verinin birden fazla sahibi olmasına izin verir.

Rust'ta sahiplik ve ödünç alma kavramları bulunduğu için referanslarla akıllı
işaretçiler arasında ek bir fark daha vardır: Referanslar yalnızca veri ödünç
alırken, akıllı işaretçiler çoğu durumda işaret ettikleri verinin _sahibi_
olur.

Akıllı işaretçiler genellikle struct'larla gerçeklenir. Sıradan bir struct'tan
farklı olarak `Deref` ve `Drop` trait'lerini uygularlar. `Deref` trait'i,
akıllı işaretçi struct'ının örneğinin referans gibi davranmasını sağlar; böylece
kodunuzu hem referanslarla hem akıllı işaretçilerle çalışacak biçimde
yazabilirsiniz. `Drop` trait'i ise bir akıllı işaretçi örneği kapsam dışına
çıktığında çalışacak kodu özelleştirmenize izin verir. Bu bölümde her iki
trait'i de ele alacak ve akıllı işaretçiler için neden önemli olduklarını
göstereceğiz.

Akıllı işaretçi deseni, Rust'ta sık kullanılan genel bir tasarım deseni olduğu
için bu bölüm mevcut tüm akıllı işaretçileri kapsamayacak. Birçok kütüphanenin
kendi akıllı işaretçileri vardır; isterseniz siz de kendinizinkini
yazabilirsiniz. Biz standart kütüphanedeki en yaygın akıllı işaretçilere
odaklanacağız:

- `Box<T>`, değerleri yığında (_heap_) saklamak için
- `Rc<T>`, birden çok sahipliğe izin veren referans sayımlı tür
- `Ref<T>` ve `RefMut<T>`; bunlara `RefCell<T>` üzerinden erişilir ve ödünç alma
  kurallarını derleme zamanında değil çalışma zamanında denetler

Bunlara ek olarak, değiştirilemez bir türün içindeki değeri değiştirmeye izin
veren bir API sunduğu _içsel değiştirilebilirlik (interior mutability)_ desenini
de işleyeceğiz. Ayrıca referans döngülerini, bunların nasıl bellek sızıntısına
yol açabileceğini ve nasıl önlenebileceğini de konuşacağız.

Hadi başlayalım!
