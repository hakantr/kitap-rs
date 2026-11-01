<!-- Old headings. Do not remove or links may break. -->
<a id="common-programming-concepts"></a>

# Yaygın Programlama Kavramları

Bu bölüm, neredeyse her programlama dilinde karşımıza çıkan kavramları ve bunların Rust'ta nasıl çalıştığını ele alır. Birçok programlama dili özünde pek çok ortak noktaya sahiptir. Bu bölümde sunulan kavramların hiçbiri yalnızca Rust'a özgü değildir; ancak bunları Rust bağlamında inceleyecek ve kullanımlarına ilişkin yerleşik kuralları açıklayacağız.

Bu bölümde özellikle değişkenler, temel veri türleri, fonksiyonlar, yorumlar ve kontrol akışı konularını öğreneceksiniz. Bu yapı taşları her Rust programında yer alacağından, bunları erkenden kavramak yola sağlam bir temelle çıkmanızı sağlayacaktır.

<!-- Old headings. Do not remove or links may break. -->
<a id="keywords"></a>
<a id="anahtar-kelimeler-keywords"></a>

> #### Anahtar Kelimeler
>
> Tıpkı diğer dillerde olduğu gibi Rust dilinde de yalnızca dilin kendisi tarafından kullanılmak üzere ayrılmış bir dizi _anahtar kelime (keyword)_ bulunur. Bu kelimeleri doğrudan değişken veya fonksiyon adı olarak kullanamayacağınızı unutmayın. Anahtar kelimelerin büyük çoğunluğunun özel bir anlamı vardır ve Rust programlarınızda çeşitli görevleri yerine getirmek için bunları kullanacaksınız; birkaçının ise şu an için herhangi bir işlevi yoktur ancak gelecekte Rust'a eklenebilecek özellikler için şimdiden ayrılmıştır. Anahtar kelimelerin tam listesini [Ek A][appendix_a]<!-- ignore --> bölümünde bulabilirsiniz.
>
> Yanlışlıkla bir anahtar kelimeyi (örneğin `type`, `match` veya `loop`) değişken ya da fonksiyon adı olarak tanımlarsanız derleyici `expected identifier, found keyword` (*tanımlayıcı bekleniyordu, anahtar kelime bulundu*) hatası verir. Harici bir veri biçimiyle (örneğin JSON içindeki `"type"` alanıyla) veya başka bir dilde yazılmış kodlarla entegre olurken bir anahtar kelimeyi isim olarak kullanmanız zorunlu olursa, kelimenin önüne `r#` ön ekini getirerek (`r#type` gibi) Ek A'da anlatılan _ham tanımlayıcı (raw identifier)_ sözdiziminden yararlanabilirsiniz.

[appendix_a]: ekler-01-anahtar-kelimeler.html
