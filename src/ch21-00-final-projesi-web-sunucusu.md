# Final Proje: Çok İş Parçacıklı Bir Web Sunucusu Geliştirmek

Uzun bir yol geldik; kitabın sonuna ulaştık. Bu bölümde, son bölümlerde öğrendiğimiz kavramların bir kısmını bir araya getiren son bir projeyi birlikte geliştireceğiz. Aynı zamanda önceki bölümlerden bazı önemli fikirleri de kısaca pekiştireceğiz.

Bu final projesinde, tarayıcıda "Merhaba!" diyen basit bir web sunucusu yazacağız. Hedefimiz, Şekil 21-1'deki gibi görünen bir sayfa üretmek.

Web sunucusunu şu planla oluşturacağız:

1. TCP ve HTTP hakkında ihtiyaç duyduğumuz kadar bilgi edinmek
2. Bir soket üzerinden TCP bağlantılarını dinlemek
3. Az sayıda HTTP isteğini ayrıştırmak
4. Geçerli bir HTTP yanıtı oluşturmak
5. İş parçacığı havuzu ile sunucunun aktarım kapasitesini artırmak

<img alt="127.0.0.1:8080 adresini ziyaret eden tarayıcı ekran görüntüsü; sayfada “Merhaba! Rust'tan selam” yazıyor" src="img/trpl21-01.png" class="center" style="width: 50%;" />

<span class="caption">Şekil 21-1: Finalde birlikte geliştireceğimiz proje</span>

Başlamadan önce iki noktayı belirtelim. Birincisi, burada kullanacağımız yöntem Rust ile web sunucusu geliştirmenin en iyi ya da en pratik yolu değildir. Topluluk üyeleri, [crates.io](https://crates.io/) üzerinde üretime uygun pek çok web sunucusu ve iş parçacığı havuzu crate'i yayımlamıştır. Ama bu bölümün amacı sizi hazır çözümü kullanmaya yöneltmek değil, alttaki fikirleri öğretmektir. Rust bir sistem programlama dili olduğu için hangi soyutlama düzeyinde çalışacağımıza biz karar veririz.

İkincisi, bu bölümde `async` ve `await` kullanmayacağız. İş parçacığı havuzu kurmak tek başına yeterince büyük bir iştir; buna bir de asenkron çalışma zamanı eklemeyeceğiz. Yine de, burada karşılaşacağımız bazı problemlerde `async` ve `await`'in nasıl devreye girebileceğine ara ara değineceğiz. Sonuçta 17. bölümde de belirttiğimiz gibi, birçok asenkron çalışma zamanı işlerini yönetmek için zaten iş parçacığı havuzları kullanır.

Bu nedenle temel HTTP sunucusunu ve iş parçacığı havuzunu elle yazacağız. Böylece ileride kullanabileceğiniz hazır crate'lerin arkasındaki genel yaklaşımı daha sağlam biçimde öğrenmiş olacaksınız.
