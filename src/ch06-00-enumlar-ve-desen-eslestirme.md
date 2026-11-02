<!-- Old headings. Do not remove or links may break. -->
<a id="enums-and-pattern-matching"></a>

# Enum'lar ve Desen Eşleştirme

Bu bölümde, kısaca *enum* olarak da adlandırılan *numaralandırmaları* inceleyeceğiz. Enum'lar, olası *varyantlarını* numaralandırarak bir tür tanımlamanıza olanak tanır. İlk olarak, bir enum'ın verilerle birlikte anlamı da nasıl kodlayabildiğini göstermek için bir enum tanımlayıp kullanacağız. Daha sonra, bir değerin bir şey ya da hiçbir şey olabileceğini ifade eden ve özellikle kullanışlı bir enum olan `Option` türünü keşfedeceğiz. Ardından, `match` ifadesindeki *desen eşleştirmenin (pattern matching)*, bir enum'ın farklı değerleri için farklı kodlar çalıştırmayı nasıl kolaylaştırdığına bakacağız. Son olarak, kodunuzdaki enum'ları işlemek için kullanabileceğiniz pratik ve özlü birer kalıp olan `if let` ve `let...else` yapılarını ele alacağız.

> Not: C, C++ veya Java gibi dillerden geliyorsanız `enum` kavramını çoğunlukla sabit seçeneklerden veya isimlendirilmiş tam sayılardan oluşan basit listeler olarak tanıyor olabilirsiniz. Rust'taki enum'lar ise fonksiyonel programlama dillerindeki *cebirsel veri türlerine (algebraic data types)* — özellikle de *toplam türlere (sum types)* — karşılık gelir ve çok daha güçlüdür. Bölüm 5'te gördüğümüz `struct` yapıları birden fazla alanı **aynı anda** bir arada tutarken ("bu **ve** şu"), bir `enum` değeri tanımlanan varyantlardan herhangi bir anda **yalnızca biri** olabilir ("bu **veya** şu"); üstelik her varyant kendi içinde birbirinden tamamen farklı tür ve sayıda veri taşıyabilir. Bu tasarım `match` ile yapılan desen eşleştirmeyle birleştiğinde, geçersiz durumları daha tür düzeyinde imkânsız kılmanıza ve olası her varyantın eksiksiz ele alındığını derleme zamanında doğrulamanıza olanak tanır.

