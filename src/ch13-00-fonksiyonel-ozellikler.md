# Fonksiyonel Dil Özellikleri: Yineleyiciler (Iterators) ve Kapanışlar (Closures)

Rust'ın tasarımı mevcut birçok dilden ve teknikten ilham almıştır ve önemli etkilerden biri de _fonksiyonel programlamadır_ (functional programming). Fonksiyonel tarzda programlama genellikle fonksiyonları argüman olarak ileterek, diğer fonksiyonlardan döndürerek, daha sonra çalıştırılmak üzere değişkenlere atayarak (ve benzeri şekillerde) değer olarak kullanmayı içerir.

Bu bölümde, fonksiyonel programlamanın ne olup olmadığı tartışmasına girmeyeceğiz; bunun yerine Rust'ın, genellikle "fonksiyonel" olarak adlandırılan birçok dildeki özelliklere benzeyen bazı özelliklerini tartışacağız.

Daha spesifik olarak şunları ele alacağız:

- Bir değişkende saklayabileceğiniz, fonksiyona benzeyen bir yapı olan _Kapanışlar_ (Closures)
- Bir dizi ögeyi (element) işlemenin (processing) bir yolu olan _Yineleyiciler_ (Iterators)
- Bölüm 12'deki G/Ç (I/O) projesini geliştirmek için kapanışların ve yineleyicilerin nasıl kullanılacağı
- Kapanışların ve yineleyicilerin performansı (sürprizbozan: Düşündüğünüzden daha hızlıdırlar!)

Bölüm 6'da işlediğimiz desen eşleştirme ve enum'lar gibi, fonksiyonel tarzdan etkilenen bazı diğer Rust özelliklerini zaten ele almıştık. Kapanışlarda ve yineleyicilerde uzmanlaşmak (mastering), hızlı ve idiyomatik Rust kodu yazmanın önemli bir parçası olduğundan, bu bölümün tamamını onlara ayıracağız.