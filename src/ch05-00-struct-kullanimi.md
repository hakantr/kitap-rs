<!-- Old headings. Do not remove or links may break. -->
<a id="using-structs-to-structure-related-data"></a>

# İlgili Verileri Yapılandırmak İçin Struct Kullanımı

Bir *struct (yapı)*, anlamlı bir bütün oluşturan ilişkili birden fazla değeri bir araya getirip isimlendirmenizi sağlayan özel bir veri türüdür. Nesne yönelimli bir programlama diline aşinaysanız struct, bir nesnenin veri niteliklerine benzer. Bu bölümde, şimdiye kadar öğrendiklerinizden yola çıkarak *demet (tuple)* türü ile struct'ları karşılaştıracak, benzerlik ve farklarını ortaya koyacak ve verileri gruplandırmak için struct'ların ne zaman daha iyi bir seçenek olduğunu göstereceğiz.

Struct'ların nasıl tanımlanacağını ve örnekleneceğini inceleyeceğiz. Bir struct türüne ait davranışları belirlemek için *ilişkili fonksiyonların (associated functions)* —özellikle de *metot* olarak adlandırılan ilişkili fonksiyon türünün— nasıl tanımlanacağını ele alacağız. Struct'lar ve Bölüm 6'da göreceğimiz enum'lar, programınızın ele aldığı probleme özgü yeni türler oluşturmanın temel yapı taşlarıdır; bu türler sayesinde Rust'ın derleme zamanı tür denetiminden tam anlamıyla yararlanabilirsiniz.

> Not: C#, Java veya Python gibi nesne yönelimli dillerden geliyorsanız Rust'ta
> `class` anahtar kelimesinin ve klasik sınıf kalıtımı mekanizmasının
> bulunmadığını fark edeceksiniz. Rust, verinin bellekteki yapısını (`struct`
> tanımı) ve o veri üzerinde çalışan davranışları (`impl` blokları içindeki
> *metotlar* ve *ilişkili fonksiyonlar*) birbirinden net biçimde ayırır; türler
> arası ortak davranışları ise Bölüm 10'da göreceğimiz *trait* sistemiyle
> sağlar. Üstelik ham verileri anlamlı `struct` türleri altında gruplamak
> çalışma zamanında ek bir bellek ya da hız maliyeti getirmez; buna karşılık
> verilere anlamlı alan adları kazandırır. Ayrıca farklı kavramlar için ayrı
> struct türleri tanımlamak, bu türlerin yanlışlıkla birbirinin yerine
> kullanılmasını derleme aşamasında engeller; aynı türdeki iki alanın değerini
> karıştırmayı ise derleyici her zaman saptayamaz.
