## `Drop` Trait'i ile Temizlik Sırasında Kod Çalıştırmak

Akıllı işaretçi deseni için önemli ikinci trait `Drop`tur. Bir değer kapsam
dışına çıkmak üzereyken ne olacağını özelleştirmenizi sağlar. `Drop`
uygulamasını herhangi bir tür için yazabilir ve böylece dosya, ağ bağlantısı
gibi kaynakları bırakmak için çalışacak kodu belirleyebilirsiniz.

`Drop`u akıllı işaretçiler bağlamında ele alıyoruz; çünkü `Drop` çoğunlukla
akıllı işaretçi yazarken kullanılır. Örneğin `Box<T>` bırakıldığında, kutunun
işaret ettiği öbek alanı serbest bırakılır.

Bazı dillerde, bazı türlerin örnekleriyle işiniz bittiğinde belleği ya da
kaynağı serbest bırakacak kodu programcının elle çağırması gerekir. Dosya
tanıtıcıları, soketler ve kilitler buna örnektir. Programcı unutursa sistem
zorlanabilir hatta çökebilir. Rust'ta ise bir değer kapsam dışına çıktığında
hangi kodun çalışacağını belirtirsiniz; derleyici bu kodu uygun yerlere
otomatik ekler.

Bir değer kapsam dışına çıktığında çalışacak kodu `Drop` trait'ini
uygulayarak belirlersiniz. Bu trait, `self`i değiştirilebilir referans olarak
alan `drop` adlı bir yöntem ister. Rust'ın `drop`u ne zaman çağırdığını görmek
için şimdilik bu yönteme `println!` ekleyelim.

Liste 15-14, kapsam dışına çıkınca mesaj basan `OzelAkilliIsaretci` yapısını
gösterir.

<Listing number="15-14" file-name="src/main.rs" caption="Temizlik kodunun yer alacagi `Drop` uygulamali `OzelAkilliIsaretci` yapisi">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

</Listing>

`Drop` trait'i prelude içinde olduğu için ayrıca kapsamaya almamız gerekmez.
`OzelAkilliIsaretci` için `Drop` uyguluyor ve `drop` içinde bir mesaj
yazdırıyoruz. Gerçek hayatta bu gövdeye kaynak temizleme mantığı konurdu.

`main` içinde iki `OzelAkilliIsaretci` oluşturup ardından
`OzelAkilliIsaretciler oluşturuldu` yazdırıyoruz. `main` sonunda bu değerler
kapsam dışına çıkar ve Rust `drop`u otomatik çağırır. Yani `drop`u bizim
elle çağırmamıza gerek yoktur.

Programı çalıştırınca şunu görürüz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

Rust değerler kapsam dışına çıktığında `drop`u otomatik çağırır. Değişkenler
oluşturulma sırasının tersine bırakılır; bu yüzden `d`, `c`den önce bırakılır.

<!-- Old headings. Do not remove or links may break. -->

<a id="dropping-a-value-early-with-std-mem-drop"></a>

Bazen bir değeri kapsam sonunu beklemeden daha erken temizlemek isteyebilirsiniz.
Örneğin kilit yöneten akıllı işaretçilerde, kilidi erkenden bırakıp aynı kapsam
içindeki başka kodun kilidi almasına izin vermek isteyebilirsiniz. Rust,
`Drop` trait'inin `drop` yöntemini elle çağırmanıza izin vermez; bunun yerine
standart kütüphanedeki `std::mem::drop` fonksiyonunu kullanırsınız.

Liste 15-14'teki `main`i değiştirip `drop` yöntemini elle çağırmak istersek,
Liste 15-15'teki kod çalışmaz.

<Listing number="15-15" file-name="src/main.rs" caption="Temizligi erken yapmak icin `Drop` trait'indeki `drop`u elle cagirmaya calismak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

</Listing>

Derlersek şu hatayı alırız:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

Hata açıkça `drop` çağrısının yasak olduğunu söyler. Buradaki _destructor_,
örneği temizleyen fonksiyon için kullanılan genel terimdir. Yapıcı
(_constructor_) nasıl örnek oluşturuyorsa, destructor da örneği temizler.

Rust buna izin vermez; çünkü kapsam sonunda yine otomatik `drop` çağrılırdı.
Böylece aynı değer iki kez temizlenmeye çalışılırdı.

Bu yüzden bir değeri erkenden bırakmak istiyorsak, `std::mem::drop`
fonksiyonunu çağırırız. Değeri fonksiyona argüman olarak veririz; prelude içinde
olduğu için ayrıca `use` yazmamız gerekmez. Liste 15-16 bunu gösterir.

<Listing number="15-16" file-name="src/main.rs" caption="Bir degeri kapsam disina cikmadan once acikca birakmak icin `std::mem::drop` cagirmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

</Listing>

Bu kodun çıktısı şöyledir:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

`OzelAkilliIsaretci oluşturuldu` ile `main bitmeden önce bırakıldı` satırları
arasında, bırakma mesajının yazdırılması `c` değerinin o noktada temizlendiğini
gösterir.

`Drop` uygulamasıyla verdiğiniz kodu birçok yaratıcı şekilde kullanabilirsiniz;
örneğin kendi bellek ayırıcınızı yazabilirsiniz. `Drop` ve Rust'ın sahiplik
sistemi sayesinde temizlik kodunu hatırlamak zorunda kalmazsınız; Rust bunu
sizin için yapar.

Ayrıca hâlâ kullanılan bir değeri yanlışlıkla temizlemekten de korkmanız
gerekmez. Referansların geçerliliğini koruyan sahiplik sistemi, `drop`un yalnızca
değer artık kullanılmıyorken bir kez çağrılmasını da sağlar.
