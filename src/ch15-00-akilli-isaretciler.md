<!-- Old headings. Do not remove or links may break. -->
<a id="smart-pointers"></a>
<a id="akıllı-işaretçiler"></a>
<a id="akilli-isaretciler"></a>

# Akıllı İşaretçiler (Smart Pointers)

_İşaretçi (pointer)_, bellekteki bir adresi barındıran değişkenler için kullanılan genel bir kavramdır. Bu adres başka bir veriye başvurur, diğer bir deyişle o veriyi “işaret eder”. Rust'taki en yaygın işaretçi türü, Bölüm 4'te öğrendiğiniz referanslardır. Referanslar `&` sembolüyle gösterilir ve işaret ettikleri değeri ödünç alırlar. Veriye başvurmak dışında özel bir yetenekleri yoktur ve herhangi bir ek maliyet getirmezler.

_Akıllı işaretçiler_ ise bir işaretçi gibi davranan, ancak bunun yanı sıra ek meta veriler ve yetenekler de barındıran veri yapılarıdır. Akıllı işaretçi kavramı yalnızca Rust'a özgü değildir; kökeni C++ diline dayanır ve başka dillerde de bulunur. Rust'ın standart kütüphanesinde, referansların sunduğu işlevselliğin ötesine geçen çeşitli akıllı işaretçiler tanımlanmıştır. Genel kavramı keşfetmek için aralarında _referans sayımlı (reference counting)_ bir akıllı işaretçi türünün de bulunduğu birkaç farklı örneği inceleyeceğiz. Bu işaretçi, sahiplerin sayısını takip ederek bir verinin birden fazla sahibi olmasına olanak tanır ve hiçbir sahip kalmadığında veriyi temizler.

Sahiplik ve ödünç alma kavramlarına sahip olan Rust'ta, referanslar ile akıllı işaretçiler arasında ek bir fark daha vardır: Referanslar veriyi yalnızca ödünç alırken, akıllı işaretçiler çoğu durumda işaret ettikleri verinin _sahibidir_.

Akıllı işaretçiler genellikle struct'lar kullanılarak oluşturulur. Sıradan bir struct'tan farklı olarak akıllı işaretçiler `Deref` ve `Drop` trait'lerini uygular. `Deref` trait'i, akıllı işaretçi struct'ının bir örneğinin referans gibi davranmasını sağlar; böylece hem referanslarla hem de akıllı işaretçilerle çalışabilen kodlar yazabilirsiniz. `Drop` trait'i ise bir akıllı işaretçi örneği kapsam dışına çıktığında çalıştırılacak kodu özelleştirmenize olanak tanır. Bu bölümde her iki trait'i de ele alacak ve akıllı işaretçiler için neden önemli olduklarını göstereceğiz.

Akıllı işaretçi deseni Rust'ta sıkça kullanılan genel bir tasarım deseni olduğundan, bu bölüm mevcut tüm akıllı işaretçileri kapsamayacaktır. Pek çok kütüphanenin kendi akıllı işaretçileri vardır, hatta siz de kendinizinkini yazabilirsiniz. Biz burada standart kütüphanedeki en yaygın akıllı işaretçileri ele alacağız:

- `Box<T>`: Değerleri öbekte (_heap_) ayırmak için
- `Rc<T>`: Çoklu sahipliği mümkün kılan referans sayımlı tür
- `Ref<T>` ve `RefMut<T>`: Ödünç alma kurallarını derleme zamanı yerine çalışma zamanında uygulayan `RefCell<T>` türü üzerinden erişilen türler

Bunlara ek olarak, değiştirilemez bir türün kendi içindeki bir değeri değiştirmek üzere bir API sunduğu _içsel değiştirilebilirlik (interior mutability)_ desenini inceleyeceğiz. Ayrıca _referans döngülerini (reference cycles)_, bunların nasıl bellek sızıntısına yol açabileceğini ve nasıl önlenebileceğini de ele alacağız.

> Not: Aslında Bölüm 8'de tanıştığınız `String` ve `Vec<T>` gibi türler de öbekteki belleğin sahibi oldukları, kapasite ve uzunluk gibi ek meta veriler tuttukları ve `Deref` ile `Drop` trait'lerini uyguladıkları için birer akıllı işaretçidir. Bu bölüme başlarken Rust'taki işaretçi ve akıllı işaretçi ailesini kuşbakışı olarak şöyle özetleyebiliriz:
>
> - **`&T` ve `&mut T` (Normal referanslar):** Verinin sahibi olmadan yalnızca ödünç alır; ek bellek veya çalışma zamanı maliyeti yoktur.
> - **`Box<T>`:** Veriyi yığın (*stack*) yerine öbekte tutar ve derleme zamanında denetlenen **tek sahiplik** sağlar.
> - **`Rc<T>` (ve çok kanallı eşdeğeri `Arc<T>`):** Referans sayımıyla aynı veriye **çoklu sahiplik** olanağı tanır (`Arc<T>` türünü Bölüm 16'da göreceğiz).
> - **`RefCell<T>` (ve çok kanallı eşdeğeri `Mutex<T>`):** *İçsel değiştirilebilirlik* sağlar ve ödünç alma kurallarını derleme zamanı yerine **çalışma zamanında** denetler.
> - **`Deref` ve `Drop` trait'leri:** Bir struct'ı akıllı işaretçi yapan temel yapı taşlarıdır; `Deref` referans benzeri başvuru çözme (`*`) ve otomatik tür zorlaması sağlarken, `Drop` değer kapsam dışına çıktığında bellek ve kaynak temizliğini otomatikleştirir.

Haydi başlayalım!
