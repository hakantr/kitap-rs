<a id="advanced-features"></a>
# Gelişmiş Özellikler

Buraya kadar Rust programlama dilinin en sık kullanılan kısımlarını öğrendiniz. Bölüm 21'de son bir proje daha yapmadan önce, ara sıra karşınıza çıkabilecek ama her gün kullanmayacağınız birkaç dil özelliğine bakacağız. Bilmediğiniz bir yapıyla karşılaştığınızda bu bölümü bir başvuru kaynağı olarak kullanabilirsiniz. Burada ele alınan özellikler çok belirli durumlarda işe yarar. Sık sık eliniz gitmese bile, Rust'ın sunduğu imkânların tamamına genel olarak hâkim olmanızı istiyoruz.

Bu bölümde şunları ele alacağız:

- Güvenli olmayan (unsafe) Rust: Rust'ın bazı garantilerinden bilinçli olarak vazgeçme ve bu garantileri elle koruma sorumluluğunu üstlenme yolları
- Gelişmiş trait'ler: İlişkili türler, varsayılan tür parametreleri, tam nitelikli sözdizimi, üst trait'ler (*supertraits*) ve trait'lerle ilişkili olarak *newtype* deseni
- Gelişmiş türler: *Newtype* deseni hakkında daha fazlası, tür takma adları, *never* (`!`) türü ve dinamik boyutlu türler
- Gelişmiş fonksiyonlar ve kapanışlar: Fonksiyon işaretçileri ve kapanış döndürme
- Makrolar: Derleme zamanında daha fazla kod üreten kodlar tanımlama yolları

Bu bölüm, herkese hitap eden zengin bir Rust özellikleri seçkisi sunuyor! Haydi başlayalım!

> Not: Kendi uygulama kodunuzda `unsafe` bloklarına, prosedürel makrolara veya karmaşık ilişkili tür sınırlarına nadiren ihtiyaç duysanız bile, standart kütüphanenin (`std`) ve ekosistemdeki popüler kütüphanelerin (`serde`, `tokio`, `rayon` vb.) API imzalarını veya kaynak kodlarını okurken bu bölümdeki yapılarla sıkça karşılaşırsınız. Bu özellikleri tanımak, üçüncü parti kütüphanelerin belgelerini ve derleyici hata mesajlarını çok daha rahat yorumlamanızı sağlar.
