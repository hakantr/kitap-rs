# Biçem Rehberi

## Düz Yazı

- Bölüm ve alt başlıklarda title case tercih edin. Örneğin
  `## Generating a Secret Number`, `## Generating a secret number` yerine
  tercih edilir.
- Bir terimi vurgularken tek tırnak yerine italik kullanın. Örneğin
  `is an *associated function* of`, `is an ‘associated function’ of`
  yerine tercih edilir.
- Metin içinde bir metoddan söz ederken parantez eklemeyin. Örneğin
  `read_line`, `read_line()` yerine tercih edilir.
- Satırları 80 karakter civarında sert sarın.
- Tek bir sözcük içinde kod ve düz metni karıştırmamaya çalışın. Örneğin
  ``Remember when we wrote `use std::io`?`` ifadesi,
  ``Remember when we `use`d `std::io`?`` ifadesine tercih edilir.

## Kod

- Uygun olduğunda, hangi dosyadan söz ettiğimiz açık olsun diye Markdown kod
  bloklarından önce dosya adını ekleyin.
- Kodda değişiklik yaparken hangi kısımların değiştiğini, hangilerinin aynı
  kaldığını açık gösterin. Nasıl yapılacağı henüz net değil.
- Mümkünse uzun satırları bölüp 80 karakter altında tutun.
- Komut satırı çıktısı içeren kod bloklarında `bash` sözdizimi renklendirmesi
  kullanın.

## Bağlantılar

Tüm betikler tamamlandıktan sonra:

- Basılı sürümde görünmemesi gereken bağlantıları yok sayılacak şekilde
  işaretleyin.
  - Buna HTML sürümünde bağlantı olarak kalması gereken tüm “Bölüm XX”
    kitap içi bağlantıları da dahildir.
- Kitap içi bağlantıları ve standart kütüphane API dokümantasyonu
  bağlantılarını göreceli yapın; böylece kitap ister çevrim dışı ister
  docs.rust-lang.org üzerinde okunsun düzgün çalışsın.
- Markdown bağlantıları kullanın ve basılı sürümde bunların
  `metin *url*` biçimine dönüştürüleceğini unutmayın. Bu yüzden bağlantıları,
  bu biçimde de doğal okunacak şekilde yazın.
