# Gelişmiş Özellikler

Buraya kadar Rust programlama dilinin en sık kullanılan kısımlarını öğrendiniz. 21. bölümde son bir proje daha yapmadan önce, ara sıra karşınıza çıkabilecek ama her gün kullanmayacağınız birkaç dil özelliğine bakacağız. Bilmediğiniz bir ayrıntıyla karşılaştığınızda bu bölümü başvuru kaynağı gibi kullanabilirsiniz. Burada ele alınan özellikler çok belirli durumlarda işe yarar. Sık sık eliniz gitmese bile, Rust'ın sunduğu imkânların tamamına genel olarak hâkim olmanızı istiyoruz.

Bu bölümde şunları ele alacağız:

- Güvensiz Rust: Rust'ın bazı garantilerinden bilinçli olarak çıkıp bu garantileri elle koruma sorumluluğunu üstlenmek
- Gelişmiş trait'ler (traits): ilişkili türler, varsayılan tür parametreleri, tam nitelikli sözdizimi, üst trait'ler ve trait'lerle ilişkili olarak newtype deseni
- Gelişmiş türler: newtype deseni hakkında daha fazlası, tür takma adları, never türü ve dinamik boyutlu türler
- Gelişmiş fonksiyonlar ve kapanışlar (closures): fonksiyon işaretçileri ve kapanış döndürme
- Makrolar: derleme zamanında daha fazla kod üreten kod tanımlama yolları

Bu bölüm, herkese hitap eden geniş bir Rust özellikleri seçkisi sunuyor. Haydi başlayalım!
