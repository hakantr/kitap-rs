# Enum'lar ve Desen Eşleştirme

Bu bölümde, _enum_ olarak da adlandırılan numaralandırmalara bakacağız.

Enum'lar, olası varyantlarını (seçeneklerini) numaralandırarak bir tür tanımlamanıza olanak tanır. Öncelikle, bir enum'ın verilerle birlikte nasıl bir anlam ifade edebileceğini göstermek için bir enum tanımlayıp kullanacağız. Daha sonra, bir değerin bir şey ya da hiçbir şey (something or nothing) olabileceğini ifade eden `Option` adlı özellikle yararlı bir enum'ı inceleyeceğiz. Ardından, `match` ifadesindeki desen eşleştirmenin, bir enum'ın farklı değerleri için farklı kodlar çalıştırmayı nasıl kolaylaştırdığına bakacağız. Son olarak, `if let` yapısının kodunuzdaki enum'ları işlemek için kullanılabilecek ne kadar uygun ve özlü bir başka ifade (idiom) olduğunu ele alacağız.
