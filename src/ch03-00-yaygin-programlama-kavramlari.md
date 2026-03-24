# Yaygın Programlama Kavramları

Bu bölüm, neredeyse her programlama dilinde ortaya çıkan kavramları ve
bunların Rust'ta nasıl çalıştığını kapsar. Çoğu programlama dili temelinde
birbiriyle pek çok ortak noktaya sahiptir. Bu bölümde sunulan kavramların
hiçbiri Rust'a özgü değildir, ancak bunları Rust bağlamında tartışacağız
ve bunların kullanımı etrafındaki gelenekleri (conventions) açıklayacağız.

Özellikle değişkenler, temel veri türleri (basic types),
fonksiyonlar (functions), yorumlar (comments) ve kontrol akışı (control flow)
hakkında bilgi edineceksiniz. Bu temeller her Rust programında bulunacaktır
ve bunları erkenden öğrenmek size başlamanız için güçlü bir temel sağlayacaktır.

> #### Anahtar Kelimeler (Keywords)
>
> Rust dili, tıpkı diğer dillerde olduğu gibi, yalnızca dilin kendisi tarafından
> kullanılmak üzere ayrılmış bir dizi _anahtar kelimeye_ (keywords) sahiptir.
> Bu kelimeleri değişken veya fonksiyon isimleri olarak kullanamayacağınızı
> unutmayın. Anahtar kelimelerin çoğunun özel anlamları vardır ve bunları
> Rust programlarınızda çeşitli görevleri yerine getirmek için kullanacaksınız;
> birkaçının ise şu an onlarla ilişkili bir işlevselliği yoktur ancak gelecekte
> Rust'a eklenebilecek işlevsellikler için ayrılmıştır (reserved).
> Anahtar kelimelerin listesini [Ek A][appendix_a]<!-- ignore -->'da bulabilirsiniz.

[appendix_a]: ekler-01-anahtar-kelimeler.md
