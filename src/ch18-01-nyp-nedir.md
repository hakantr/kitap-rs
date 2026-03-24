## Nesne Yönelimli Dillerin Özellikleri

Programlama dünyasında, bir dilin nesne yönelimli sayılması için hangi
özelliklere sahip olması gerektiği konusunda tam bir uzlaşma yoktur. Rust,
OOP dahil olmak üzere birçok programlama yaklaşımından etkilenmiştir; örneğin
13. bölümde fonksiyonel programlamadan gelen özellikleri görmüştük. Kabaca
bakarsak, nesne yönelimli dillerin ortak sayılan bazı özellikleri vardır:
nesneler, kapsülleme ve kalıtım. Şimdi bunların ne anlama geldiğine ve
Rust'ın bunları destekleyip desteklemediğine bakalım.

### Nesneler Veri ve Davranışı Birlikte Taşır

Erich Gamma, Richard Helm, Ralph Johnson ve John Vlissides'in yazdığı, _Gang of
Four_ kitabı olarak da anılan _Design Patterns: Elements of Reusable
Object-Oriented Software_, OOP tasarım desenlerini kataloglayan klasik bir
kaynak sayılır. Kitap OOP'yi şöyle tanımlar:

> Nesne yönelimli programlar nesnelerden oluşur. Bir **nesne**, hem veriyi hem
> de o veri üzerinde çalışan yordamları bir araya getirir. Bu yordamlar
> genellikle **metot** ya da **işlem** olarak adlandırılır.

Bu tanıma göre Rust nesne yönelimlidir: struct ve enum'lar veri taşır; `impl`
blokları da onlara metotlar kazandırır. Rust'ta bunlara doğrudan "nesne"
demesek de, Gang of Four tanımına göre aynı işlevi görürler.

### Uygulama Ayrıntılarını Gizleyen Kapsülleme

OOP ile sık ilişkilendirilen başka bir özellik de _kapsülleme (encapsulation)_
fikridir. Bunun anlamı, bir nesnenin uygulama ayrıntılarının o nesneyi kullanan
kod tarafından doğrudan erişilebilir olmamasıdır. Yani nesneyle etkileşmenin
tek yolu onun açık API'si olur; dışarıdaki kod nesnenin içini eşeleyip veriyi
veya davranışı doğrudan değiştiremez. Bunun avantajı, nesneyi kullanan kodu
bozmadan iç yapıyı değiştirebilmenizdir.

7. bölümde kapsüllemeyi nasıl denetlediğimizi görmüştük: `pub` anahtar
sözcüğüyle hangi modül, tür, fonksiyon ve metodun açık olacağına karar veririz;
geri kalan her şey varsayılan olarak gizlidir. Örneğin `i32` değerlerden oluşan
bir vektörü ve bu vektörün ortalamasını tutan `OrtalamaliKoleksiyon` adlı bir
struct tanımlayabiliriz. Böylece her ihtiyaç duyulduğunda ortalamayı yeniden
hesaplamak zorunda kalmayız; hesaplanmış değeri önbellekte tutarız.

<Listing number="18-1" file-name="src/lib.rs" caption="Tamsayı listesini ve bu listenin ortalamasını tutan `OrtalamaliKoleksiyon` struct'ı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-01/src/lib.rs}}
```

</Listing>

Struct `pub` olarak işaretlenmiştir; böylece başka kodlar onu kullanabilir.
Ama struct içindeki alanlar gizli kalır. Bu burada önemlidir; çünkü listeye
değer eklendiğinde ya da listeden değer çıkarıldığında ortalamanın da güncel
kalmasını istiyoruz. Bunun için struct üstünde `ekle`, `cikar` ve `ortalama`
metotlarını uygularız.

<Listing number="18-2" file-name="src/lib.rs" caption="`OrtalamaliKoleksiyon` üzerindeki açık `ekle`, `cikar` ve `ortalama` metotları">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-02/src/lib.rs:here}}
```

</Listing>

`ekle`, `cikar` ve `ortalama`, `OrtalamaliKoleksiyon` içindeki veriye erişmenin
veya onu değiştirmenin tek yoludur. `ekle` ile listeye öğe eklenince ya da
`cikar` ile çıkarılınca, her iki metot da gizli `ortalamayi_guncelle` metodunu
çağırarak `ortalama` alanını günceller.

`liste` ile `ortalama` alanlarını gizli bırakarak dışarıdaki kodun `liste`
alanına doğrudan müdahale etmesini engelleriz. Yoksa `liste` değişir ama
`ortalama` senkron kalmayabilir. `ortalama` metodu ise dışarıdaki kodun
ortalama değerini okuyabilmesini sağlar; ama onu doğrudan değiştirmesine izin
vermez.

Bu kapsülleme sayesinde, gelecekte `OrtalamaliKoleksiyon`'ın iç yapısını
kolayca değiştirebiliriz. Örneğin `liste` alanı için `Vec<i32>` yerine
`HashSet<i32>` kullanabiliriz. Dış dünyaya açık `ekle`, `cikar` ve `ortalama`
metotlarının imzaları değişmediği sürece, bu türü kullanan kodun değişmesine
gerek kalmazdı.

Eğer bir dilin nesne yönelimli sayılabilmesi için kapsüllemeyi desteklemesi
şartsa, Rust bu koşulu karşılar. Çünkü kodun farklı parçalarında `pub`
kullanıp kullanmamak, uygulama ayrıntılarını kapsüllemenizi sağlar.

### Tür Sistemi ve Kod Paylaşımı Olarak Kalıtım

_Kalıtım (inheritance)_, bir nesnenin başka bir nesnenin tanımından öğeler
devralmasıdır. Böylece üst nesnenin verisini ve davranışını tekrar yazmadan
kazanır.

Eğer bir dilin nesne yönelimli sayılması için kalıtım şartsa, Rust bu anlamda
nesne yönelimli değildir. Bir struct'ın başka bir struct'ın alanlarını ve
metotlarını doğrudan miras almasını sağlayan yerleşik bir mekanizma yoktur.

Yine de kalıtımı genelde iki nedenle kullanırsınız. Birincisi kod tekrarını
azaltmaktır. Bir tür için tanımladığınız davranışı başka bir türde de yeniden
kullanmak istersiniz. Rust'ta bunu sınırlı ölçüde, trait'lerdeki varsayılan
metot uygulamalarıyla yapabilirsiniz.

İkinci neden tür sistemidir: bir alt türün, üst türün kullanılabildiği
yerlerde kullanılabilmesini istemek. Buna _çok biçimlilik (polymorphism)_ denir.

> ### Çok Biçimlilik
>
> Birçok kişi çok biçimliliği kalıtımla eş anlamlı sanır; ama aslında daha geniş
> bir kavramdır. Kodun birden fazla türde veriyle çalışabilmesini ifade eder.
> Kalıtımlı dillerde bu genellikle alt sınıflar üzerinden sağlanır.
>
> Rust bunun yerine jenerikler ve trait sınırları kullanır. Buna bazen
> _sınırlı parametrik çok biçimlilik_ denir.

Rust, kalıtımı doğrudan sunmayarak farklı bir ödünleşim seçmiştir. Kalıtım çoğu
zaman gerekenden fazla kod paylaşımına yol açabilir. Alt türler, üst türün her
özelliğini her zaman paylaşmamalıdır; ama kalıtım onları paylaşmaya zorlayabilir.
Bu da tasarımı daha az esnek hale getirir.

Bu yüzden Rust, çalışma zamanında çok biçimlilik elde etmek için kalıtım yerine
trait nesnelerini kullanır. Şimdi trait nesnelerinin nasıl çalıştığına bakalım.
